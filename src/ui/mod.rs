//! The editor: five tabs across the top, one page per section, meters on the rails.
//!
//! Built from OpenLimiter's kit (`kit.rs`, `theme.rs`, `raster.rs`), so the knobs, pills, keys,
//! sliders, colours and four themes are the house ones. The page follows the focus of the
//! Harmonic Compressor editor:
//!
//! * Top bar: ON, the section's name, its one defining switch (Gate mode, Limiter Clean), and
//!   the global Auto Gain and Zero Latency.
//! * One display: four seconds of level history (input level, output level, and the gain the
//!   section applies hanging from the top edge), with the static curve drawn in its left square
//!   on the same dBFS scale. The live dot on the curve sits at the height the newest output
//!   level arrives at, so the curve and the history read as one picture.
//! * Chips above the display open a panel of sliders over it (shape, timing, sidechain,
//!   ceiling): the settings that are set once. One panel at a time; closed by default.
//! * Below the display, the section's main controls as big knobs.
//!
//! What never moves (panels, grids, captions, knob faces) is painted once into the canvas's
//! cached layer; values, the history, the curve, an open settings panel and meters are painted
//! on top every frame that something moved.

#![allow(dead_code)] // the kit is shared with OpenLimiter: not every piece is used here

pub mod analyser;
pub mod kit;
pub mod raster;
pub mod theme;
mod view3d;

use std::collections::VecDeque;
use std::sync::Arc;

use openplugin::core::editor::{AccessAction, AccessNode, AccessRange, AccessRole, DamageRect, DamageTracker, Key, Modifiers, ParamMenu, ParamMenuAction, SizeRange, TextEdit, Tooltip};
use openplugin::core::{Editor, EditorHost, EditorState, Frame};
use openplugin::dri::goodlookindri::scene::{Point, Rect, Scene};
use openplugin::dri::{ControlSet, SceneCanvas};

use crate::shared::{Live, Shared};
use crate::{INFO, LAYOUT, P_AUTO_GAIN, P_DELTA, P_INPUT, P_OUTPUT, P_ZERO_LATENCY, SECTIONS};
use kit::{inside, rect, Cell, Kind, Size};
use raster::R;
use theme::{load_theme, store_theme, theme, Theme, ThemeId};

pub const WIDTH: u32 = 1040;
pub const HEIGHT: u32 = 720;

const TAB_LABELS: [&str; SECTIONS] = ["GATE / EXPANDER", "COMPRESSOR", "DE-EXPANDER", "DECOMPRESSOR", "LIMITER"];
const THEME_BUTTON: R = (952.0, 14.0, 72.0, 24.0);
const PAGE: R = (100.0, 60.0, 840.0, 648.0);
const TOP_Y: f32 = PAGE.1 + 12.0;
/// Global, shown on every tab at the page's top right.
const ZERO_LATENCY: R = (PAGE.0 + PAGE.2 - 136.0, TOP_Y, 120.0, 22.0);
const AUTO_GAIN: R = (ZERO_LATENCY.0 - 128.0, TOP_Y, 120.0, 22.0);
const DELTA: R = (AUTO_GAIN.0 - 88.0, TOP_Y, 80.0, 22.0);
/// Where a section's own top-bar switch starts (Gate mode, Limiter Clean).
const TOP_X: f32 = PAGE.0 + 240.0;
/// The display: level history across, the static curve in its left square.
const DISPLAY: R = (140.0, 136.0, 772.0, 352.0);
/// The spectrograph lane under the level history, on the same time axis.
const LANE: R = (DISPLAY.0, DISPLAY.1 + DISPLAY.3 + 6.0, DISPLAY.2, 56.0);
/// Spectrograph bands, log-spaced from `SPEC_LO` to `SPEC_HI` Hz, and the dB change that
/// shows at full strength.
const BANDS: usize = 24;
const SPEC_LO: f32 = 40.0;
const SPEC_HI: f32 = 16_000.0;
const SPEC_FULL_DB: f32 = 12.0;
/// Shades each way in the spectrograph lane.
const SHADES: i32 = 12;
/// Samples per spectrum (85 ms at 48 kHz).
const SPEC_SIZE: usize = 4096;
/// The settings chips, above the display.
const CHIP_Y: f32 = 106.0;
const CHIP_W: f32 = 110.0;
const PANEL_W: f32 = 420.0;
const PANEL_ROW: f32 = 32.0;
/// The display's level scale, dBFS (history and curve share it).
const LO: f32 = -60.0;
const HI: f32 = 6.0;
/// The gain band hanging from the display's top: this many dB over this share of its height.
const GR_DB: f32 = 24.0;
const GR_SHARE: f32 = 0.3;
/// History length in editor ticks (about 4 s at 60 Hz).
/// History entries per second (by elapsed time: hosts tick at 20 to 60 Hz) and kept.
const HIST_HZ: f32 = 30.0;
const TRACE_LEN: usize = HISTORY_S * HIST_HZ as usize;
const HISTORY_S: usize = 4;
/// Main knobs: big, centred in a row under the display.
const KNOB_Y: f32 = 578.0;
/// Swaps the display: the 3D spectrum large, the history small.
/// Right-aligned to the display's right edge, mirroring the first settings chip at its left.
const VIEW3D_BUTTON: R = (DISPLAY.0 + DISPLAY.2 - 90.0, CHIP_Y, 90.0, 22.0);
const KNOB_STEP: f32 = 136.0;
const METER_TOP: f32 = 72.0;
const METER_BOTTOM: f32 = 588.0;
/// Meter scale, dBFS.
const M_LO: f32 = -60.0;
const M_HI: f32 = 6.0;

/// Section `i`'s tab: tabs sit in chain order, left to right.
fn tab_rect(i: usize) -> R {
    let pos = crate::CHAIN.iter().position(|&s| s == i).unwrap_or(i);
    (196.0 + pos as f32 * 148.0, 12.0, 142.0, 30.0)
}

fn tab_dot(i: usize) -> (f32, f32) {
    let r = tab_rect(i);
    (r.0 + 14.0, r.1 + r.3 * 0.5)
}

fn dot_hit(i: usize) -> R {
    let (x, y) = tab_dot(i);
    (x - 10.0, y - 10.0, 20.0, 20.0)
}

fn chip_rect(i: usize) -> R {
    (DISPLAY.0 + i as f32 * (CHIP_W + 8.0), CHIP_Y, CHIP_W, 22.0)
}

/// One control, by the section's local parameter index.
#[derive(Clone, Copy)]
enum Item {
    /// A big knob in the main row; a slider in a settings panel.
    Knob(usize, &'static str),
    /// Keys side by side, this wide.
    Keys(usize, &'static str, f32),
    Pill(usize, &'static str),
}

/// What a section shows: its main knobs, its top-bar switch, its settings panels.
struct Page {
    main: &'static [Item],
    top: &'static [Item],
    panels: &'static [(&'static str, &'static [Item])],
}

/// Main controls are what is turned while listening; a panel holds what is set once.
fn page(tab: usize) -> Page {
    use Item::*;
    match tab {
        0 => {
            use dynamics::gate::index as g;
            Page {
                main: &[Knob(g::THRESHOLD, "Threshold"), Knob(g::RATIO, "Ratio"), Knob(g::RANGE, "Range"), Knob(g::ATTACK, "Attack"), Knob(g::HOLD, "Hold"), Knob(g::RELEASE, "Release")],
                top: &[Keys(g::MODE, "Mode", 150.0)],
                panels: &[
                    ("SHAPE", &[Knob(g::KNEE, "Knee"), Knob(g::HYSTERESIS, "Hysteresis"), Knob(g::STEREO_LINK, "Link")]),
                    ("TIMING", &[Knob(g::LOOKAHEAD, "Look-Ahead"), Pill(g::AUTO_ATTACK, "Auto Attack"), Pill(g::AUTO_RELEASE, "Auto Release")]),
                    ("SIDECHAIN", &[Keys(g::DETECTOR, "Detector", 140.0), Knob(g::RMS_WINDOW, "RMS Window"), Keys(g::KEY_SOURCE, "Key", 160.0), Knob(g::KEY_HIGH_PASS, "Key HP"), Knob(g::KEY_LOW_PASS, "Key LP"), Knob(g::KEY_BOOST, "Key Boost"), Knob(g::KEY_BOOST_FREQ, "Boost Freq"), Knob(g::KEY_BOOST_Q, "Boost Q")]),
                ],
            }
        }
        1 => {
            use dynamics::comp::index as c;
            Page {
                main: &[Knob(c::THRESHOLD, "Threshold"), Knob(c::RATIO, "Ratio"), Knob(c::ATTACK, "Attack"), Knob(c::RELEASE, "Release"), Knob(c::MAKEUP, "Make-Up"), Knob(c::MIX, "Mix")],
                top: &[],
                panels: &[
                    ("SHAPE", &[Knob(c::KNEE, "Knee"), Knob(c::STEREO_LINK, "Link")]),
                    ("TIMING", &[Knob(c::LOOKAHEAD, "Look-Ahead"), Pill(c::AUTO_ATTACK, "Auto Attack"), Pill(c::AUTO_RELEASE, "Auto Release")]),
                    ("SIDECHAIN", &[Keys(c::DETECTOR, "Detector", 140.0), Knob(c::RMS_WINDOW, "RMS Window"), Keys(c::KEY_SOURCE, "Key", 160.0), Knob(c::KEY_HIGH_PASS, "Key HP"), Knob(c::KEY_LOW_PASS, "Key LP"), Knob(c::KEY_BOOST, "Key Boost"), Knob(c::KEY_BOOST_FREQ, "Boost Freq"), Knob(c::KEY_BOOST_Q, "Boost Q")]),
                ],
            }
        }
        2 => {
            use dyn_deexpand::index as d;
            Page {
                main: &[Knob(d::THRESHOLD, "Threshold"), Knob(d::RATIO, "Ratio"), Knob(d::RANGE, "Range"), Knob(d::ATTACK, "Attack"), Knob(d::RELEASE, "Release"), Knob(d::MIX, "Mix")],
                top: &[],
                panels: &[
                    ("SHAPE", &[Knob(d::KNEE, "Knee"), Knob(d::FLOOR, "Floor"), Knob(d::STEREO_LINK, "Link")]),
                    ("TIMING", &[Knob(d::HOLD, "Hold"), Knob(d::LOOKAHEAD, "Look-Ahead"), Pill(d::AUTO_ATTACK, "Auto Attack"), Pill(d::AUTO_RELEASE, "Auto Release")]),
                    ("SIDECHAIN", &[Keys(d::DETECTOR, "Detector", 140.0), Knob(d::DETECTOR_WINDOW, "RMS Window"), Keys(d::KEY_SOURCE, "Key", 160.0), Knob(d::KEY_HIGH_PASS, "Key HP"), Knob(d::KEY_LOW_PASS, "Key LP"), Knob(d::KEY_BOOST, "Key Boost"), Knob(d::KEY_BOOST_FREQ, "Boost Freq"), Knob(d::KEY_BOOST_Q, "Boost Q")]),
                ],
            }
        }
        3 => {
            use dyn_decompress::index as d;
            Page {
                main: &[Knob(d::THRESHOLD, "Threshold"), Knob(d::RATIO, "Ratio"), Knob(d::RANGE, "Range"), Knob(d::ATTACK, "Attack"), Knob(d::RELEASE, "Release"), Knob(d::MIX, "Mix")],
                top: &[],
                panels: &[
                    ("SHAPE", &[Knob(d::KNEE, "Knee"), Knob(d::TRIM, "Trim"), Knob(d::STEREO_LINK, "Link")]),
                    ("TIMING", &[Knob(d::LOOKAHEAD, "Look-Ahead"), Pill(d::AUTO_ATTACK, "Auto Attack"), Pill(d::AUTO_RELEASE, "Auto Release")]),
                    ("SIDECHAIN", &[Keys(d::DETECTOR, "Detector", 210.0), Knob(d::TRANSIENT_CREST, "Crest"), Knob(d::DETECTOR_WINDOW, "RMS Window"), Keys(d::KEY_SOURCE, "Key", 160.0), Knob(d::KEY_HIGH_PASS, "Key HP"), Knob(d::KEY_LOW_PASS, "Key LP"), Knob(d::KEY_BOOST, "Key Boost"), Knob(d::KEY_BOOST_FREQ, "Boost Freq"), Knob(d::KEY_BOOST_Q, "Boost Q")]),
                    ("CEILING", &[Pill(d::CEILING_ON, "Ceiling"), Pill(d::MATCH, "Match"), Knob(d::CEILING, "Ceiling"), Keys(d::CEILING_TYPE, "Type", 200.0), Knob(d::MATCH_TARGET, "Match Crest")]),
                ],
            }
        }
        _ => {
            use dynamics::limiter::index as l;
            Page {
                main: &[Knob(l::THRESHOLD, "Threshold"), Knob(l::CEILING, "Ceiling"), Knob(l::ATTACK, "Attack"), Knob(l::RELEASE, "Release")],
                top: &[Pill(l::CLEAN, "Clean")],
                panels: &[
                    ("SHAPE", &[Knob(l::STEREO_LINK, "Link")]),
                    ("TIMING", &[Knob(l::BASS_HOLD, "Bass Hold"), Pill(l::AUTO_ATTACK, "Auto Attack"), Pill(l::AUTO_RELEASE, "Auto Release")]),
                ],
            }
        }
    }
}

/// Rows a panel needs: one per slider or keys, one per run of switches.
fn panel_rows(items: &[Item]) -> usize {
    let mut n = 0;
    let mut prev_pill = false;
    for it in items {
        let pill = matches!(it, Item::Pill(..));
        if !(pill && prev_pill) {
            n += 1;
        }
        prev_pill = pill;
    }
    n
}

/// Where panel `p` of `tab` opens: under its chip, kept inside the display.
fn panel_rect(tab: usize, p: usize) -> R {
    let items = page(tab).panels[p].1;
    let h = 34.0 + panel_rows(items) as f32 * PANEL_ROW + 6.0;
    let x = chip_rect(p).0.clamp(DISPLAY.0 + 8.0, DISPLAY.0 + DISPLAY.2 - PANEL_W - 8.0);
    (x, DISPLAY.1 + 8.0, PANEL_W, h)
}

/// A control that does nothing in the current setting is drawn faded.
/// Each keyed section's (boost frequency, boost, boost Q), local indices.
fn key_boost(tab: usize) -> Option<(usize, usize, usize)> {
    match tab {
        0 => Some((dynamics::gate::index::KEY_BOOST_FREQ, dynamics::gate::index::KEY_BOOST, dynamics::gate::index::KEY_BOOST_Q)),
        1 => Some((dynamics::comp::index::KEY_BOOST_FREQ, dynamics::comp::index::KEY_BOOST, dynamics::comp::index::KEY_BOOST_Q)),
        2 => Some((dyn_deexpand::index::KEY_BOOST_FREQ, dyn_deexpand::index::KEY_BOOST, dyn_deexpand::index::KEY_BOOST_Q)),
        3 => Some((dyn_decompress::index::KEY_BOOST_FREQ, dyn_decompress::index::KEY_BOOST, dyn_decompress::index::KEY_BOOST_Q)),
        _ => None,
    }
}

fn dimmed(tab: usize, local: usize, v: &dyn Fn(usize) -> f32) -> bool {
    // The boost's frequency and width do nothing at 0 dB.
    if let Some((f, g, q)) = key_boost(tab) {
        if (local == f || local == q) && v(g).abs() < 1e-3 {
            return true;
        }
    }
    match tab {
        0 => {
            use dynamics::gate::index as g;
            let gate = v(g::MODE) < 0.5;
            (gate && (local == g::RATIO || local == g::KNEE)) || (local == g::RMS_WINDOW && v(g::DETECTOR) < 0.5)
        }
        1 => local == dynamics::comp::index::RMS_WINDOW && v(dynamics::comp::index::DETECTOR) < 0.5,
        2 => local == dyn_deexpand::index::DETECTOR_WINDOW && v(dyn_deexpand::index::DETECTOR) < 0.5,
        3 => {
            use dyn_decompress::index as d;
            (local == d::TRANSIENT_CREST && v(d::DETECTOR) < 1.5)
                || ((local == d::CEILING || local == d::CEILING_TYPE) && v(d::CEILING_ON) < 0.5)
                || (local == d::MATCH_TARGET && v(d::MATCH) < 0.5)
                || (local == d::RANGE && v(d::MATCH) >= 0.5)
        }
        4 => {
            use dynamics::limiter::index as l;
            // Clean sets the release itself.
            let clean = v(l::CLEAN) >= 0.5;
            (local == l::BASS_HOLD && !clean) || ((local == l::RELEASE || local == l::AUTO_RELEASE) && clean)
        }
        _ => false,
    }
}

/// Controls Zero Latency switches off: every look-ahead, and the decompressor's true-peak
/// ceiling (the instant ceiling reads sample peaks).
fn zero_latency_dims(tab: usize, local: usize) -> bool {
    match tab {
        0 => local == dynamics::gate::index::LOOKAHEAD,
        1 => local == dynamics::comp::index::LOOKAHEAD,
        2 => local == dyn_deexpand::index::LOOKAHEAD,
        3 => local == dyn_decompress::index::LOOKAHEAD || local == dyn_decompress::index::CEILING_TYPE,
        _ => false,
    }
}

/// The upward sections lift; the others cut.
fn lifts(tab: usize) -> bool {
    tab == 2 || tab == 3
}

/// Below this band level (dB) the spectrograph shows no change.
const SPEC_GATE_DB: f32 = -90.0;
/// A band this far under the loudest one carries no weight in the spectrograph.
const SPEC_WEIGHT_DB: f32 = 40.0;

/// y of a spectrograph band's lower and upper edge (band 0 at the bottom).
fn band_y(k: usize) -> (f32, f32) {
    let h = LANE.3 / BANDS as f32;
    let bottom = LANE.1 + LANE.3;
    (bottom - k as f32 * h, bottom - (k + 1) as f32 * h)
}

/// A moving gain readout: whole dB (tenths change every frame and cannot be read), signed,
/// and never "-0".
fn whole_db(v: f32) -> String {
    let r = v.round();
    if r == 0.0 { "0".into() } else { format!("{r:+.0}") }
}

/// The decompressor's ratio is the x of 1:x, in whole steps.
fn upward_ratio(v: f32) -> String {
    format!("1:{:.0}", v.round().max(1.0))
}

/// Output level (dBFS) of the section's static curve at an input level, as the editor draws it.
fn curve_out(tab: usize, l: f64, v: &dyn Fn(usize) -> f32) -> f64 {
    let p = |i: usize| v(i) as f64;
    let mix_db = |gain_db: f64, mix_pct: f64, wet_db: f64| {
        let m = mix_pct / 100.0;
        20.0 * ((1.0 - m) + m * 10f64.powf((gain_db + wet_db) / 20.0)).max(1e-9).log10()
    };
    match tab {
        0 => {
            use dynamics::gate::index as g;
            l + dynamics::gate::target(p(g::MODE) < 0.5, l, p(g::THRESHOLD), p(g::RATIO), p(g::RANGE), p(g::KNEE))
        }
        1 => {
            use dynamics::comp::index as c;
            let g = dynamics::curve::compressor(l, p(c::THRESHOLD), p(c::RATIO), p(c::KNEE));
            l + mix_db(g, p(c::MIX), p(c::MAKEUP))
        }
        2 => {
            use dyn_deexpand::index as d;
            let g = dyn_deexpand::static_gain_db(l, p(d::THRESHOLD), p(d::RATIO), p(d::KNEE), p(d::RANGE), p(d::FLOOR));
            l + mix_db(g, p(d::MIX), 0.0)
        }
        3 => {
            use dyn_decompress::index as d;
            let g = dyn_decompress::static_gain_db(l, p(d::THRESHOLD), 1.0 / p(d::RATIO).max(1.0), p(d::KNEE), p(d::RANGE));
            let mut out = l + mix_db(g, p(d::MIX), 0.0) + p(d::TRIM);
            if p(d::CEILING_ON) >= 0.5 {
                out = out.min(p(d::CEILING));
            }
            out
        }
        _ => {
            use dynamics::limiter::index as li;
            (l + p(li::CEILING) - p(li::THRESHOLD)).min(p(li::CEILING))
        }
    }
}

/// The threshold the display marks.
fn threshold(tab: usize, v: &dyn Fn(usize) -> f32) -> f32 {
    match tab {
        0 => v(dynamics::gate::index::THRESHOLD),
        1 => v(dynamics::comp::index::THRESHOLD),
        2 => v(dyn_deexpand::index::THRESHOLD),
        3 => v(dyn_decompress::index::THRESHOLD),
        _ => v(dynamics::limiter::index::THRESHOLD),
    }
}

/// Display height of a level, dBFS.
fn level_y(db: f32) -> f32 {
    let n = ((db - LO) / (HI - LO)).clamp(0.0, 1.0);
    DISPLAY.1 + DISPLAY.3 - n * DISPLAY.3
}

/// Curve square: x of an input level, dBFS (the square is as wide as the display is high).
fn curve_x(db: f32) -> f32 {
    DISPLAY.0 + ((db - LO) / (HI - LO)).clamp(0.0, 1.0) * DISPLAY.3
}

/// Depth of the gain band under the display's top edge.
fn gain_y(g: f32) -> f32 {
    DISPLAY.1 + (g.abs() / GR_DB).min(1.0) * DISPLAY.3 * GR_SHARE
}

fn meter_y(db: f32) -> f32 {
    let n = ((db - M_LO) / (M_HI - M_LO)).clamp(0.0, 1.0);
    METER_BOTTOM - n * (METER_BOTTOM - METER_TOP)
}

fn to_db(v: f32) -> f32 {
    if v > 1e-9 {
        20.0 * v.log10()
    } else {
        -200.0
    }
}

pub struct DynEditor {
    host: EditorHost,
    controls: ControlSet,
    cells: Vec<Cell>,
    /// How many of `cells` (at the end) belong to the open settings panel.
    panel_cells: usize,
    values: Vec<f32>,
    tab: usize,
    /// The open settings panel of the tab, if any.
    panel: Option<usize>,
    theme: ThemeId,
    shared: Option<Arc<Shared>>,
    live: [Live; SECTIONS],
    /// Per section: the gain it applied and its detector level, one entry per tick.
    trace: [VecDeque<f32>; SECTIONS],
    level: [VecDeque<f32>; SECTIONS],
    meters: [[f32; 2]; 2],
    auto_gain_db: f32,
    /// Spectrograph: input and output spectra now, the change per band per tick (dB, out - in).
    spec: [analyser::Spectrum; 2],
    spec_cols: VecDeque<[f32; BANDS]>,
    taps: [Vec<f32>; 4],
    /// The 3D view is on (an editor setting, not saved with the session) and its rows.
    view3d: bool,
    terrain: view3d::Terrain,
    terrain_img: view3d::TerrainImage,
    /// Time not yet turned into history entries, s.
    hist_acc: f32,
    modifiers: Modifiers,
    keys_active: bool,
    canvas: SceneCanvas,
    damage: DamageTracker,
}

pub fn make_editor(host: EditorHost) -> Option<Box<dyn Editor>> {
    Some(Box::new(DynEditor::new(host)))
}

impl DynEditor {
    pub fn new(host: EditorHost) -> Self {
        let mut e = Self {
            controls: ControlSet::new(host.clone()),
            host,
            cells: Vec::new(),
            panel_cells: 0,
            values: INFO.params.iter().map(|p| p.default).collect(),
            tab: 1,
            panel: None,
            theme: load_theme(),
            shared: None,
            live: [Live::default(); SECTIONS],
            trace: std::array::from_fn(|_| VecDeque::from(vec![0.0; TRACE_LEN])),
            level: std::array::from_fn(|_| VecDeque::from(vec![-200.0; TRACE_LEN])),
            meters: [[-200.0; 2]; 2],
            auto_gain_db: 0.0,
            spec: std::array::from_fn(|_| analyser::Spectrum::new(SPEC_SIZE, BANDS, SPEC_LO, SPEC_HI).instant()),
            spec_cols: VecDeque::from(vec![[0.0; BANDS]; TRACE_LEN]),
            taps: std::array::from_fn(|_| vec![0.0; view3d::SPEC3_SIZE]),
            view3d: false,
            terrain: view3d::Terrain::default(),
            terrain_img: view3d::TerrainImage::default(),
            hist_acc: 0.0,
            modifiers: Modifiers::NONE,
            keys_active: true,
            canvas: SceneCanvas::new(WIDTH as f32, HEIGHT as f32),
            damage: DamageTracker::new(),
        };
        e.build();
        e
    }

    fn t(&self) -> &'static Theme {
        theme(self.theme)
    }

    fn local(&self, local: usize) -> f32 {
        self.values.get(LAYOUT[self.tab].base + local).copied().unwrap_or(0.0)
    }

    /// Fades a section control that does nothing now and sets how it reads.
    fn section_cell(&self, cell: Cell) -> Cell {
        let local = cell.param - LAYOUT[self.tab].base;
        let v = |i: usize| self.local(i);
        let zero = self.values[P_ZERO_LATENCY] >= 0.5;
        let mut cell = cell.dim(dimmed(self.tab, local, &v) || (zero && zero_latency_dims(self.tab, local)));
        if self.tab == 3 && local == dyn_decompress::index::RATIO {
            cell = cell.format(upward_ratio);
        }
        cell
    }

    /// The cells of the open tab and panel, plus the global ones. Panel cells come last.
    fn make_cells(&self) -> (Vec<Cell>, usize) {
        let sec = &LAYOUT[self.tab];
        let pg = page(self.tab);
        let mut cells = vec![
            Cell::knob(Size::Small, P_INPUT, "Input", 10.0, 610.0),
            Cell::knob(Size::Small, P_OUTPUT, "Output", 950.0, 610.0),
            Cell::pill(sec.on, "On", (PAGE.0 + 16.0, TOP_Y, 64.0, 22.0)),
            Cell::pill(P_ZERO_LATENCY, "Zero Latency", ZERO_LATENCY),
            Cell::pill(P_AUTO_GAIN, "Auto Gain", AUTO_GAIN),
            Cell::pill(P_DELTA, "Delta", DELTA),
        ];
        let mut x = TOP_X;
        for it in pg.top {
            let (cell, w) = match *it {
                Item::Keys(i, label, w) => (Cell::segmented(sec.base + i, label, (x, TOP_Y, w, 22.0)), w),
                Item::Pill(i, label) => (Cell::pill(sec.base + i, label, (x, TOP_Y, 90.0, 22.0)), 90.0),
                Item::Knob(..) => continue,
            };
            cells.push(self.section_cell(cell));
            x += w + 8.0;
        }
        let total = pg.main.len() as f32 * KNOB_STEP - (KNOB_STEP - Size::Big.cell().0);
        let x0 = PAGE.0 + (PAGE.2 - total) * 0.5;
        for (k, it) in pg.main.iter().enumerate() {
            if let Item::Knob(i, label) = *it {
                cells.push(self.section_cell(Cell::knob(Size::Big, sec.base + i, label, x0 + k as f32 * KNOB_STEP, KNOB_Y)));
            }
        }
        let before = cells.len();
        if let Some(p) = self.panel {
            let (px, py, pw, _) = panel_rect(self.tab, p);
            let items = pg.panels[p].1;
            let mut y = py + 34.0;
            let mut k = 0;
            while k < items.len() {
                match items[k] {
                    Item::Knob(i, label) => {
                        cells.push(self.section_cell(Cell::new(Kind::FaderH, sec.base + i, label, (px + 16.0, y, pw - 32.0, 24.0))));
                        k += 1;
                    }
                    Item::Keys(i, label, w) => {
                        cells.push(self.section_cell(Cell::segmented(sec.base + i, label, (px + 16.0 + 96.0, y, w, 24.0))));
                        k += 1;
                    }
                    Item::Pill(..) => {
                        let mut x = px + 16.0;
                        while let Some(Item::Pill(i, label)) = items.get(k) {
                            cells.push(self.section_cell(Cell::pill(sec.base + i, *label, (x, y, 120.0, 24.0))));
                            x += 128.0;
                            k += 1;
                        }
                    }
                }
                y += PANEL_ROW;
            }
        }
        let n = cells.len() - before;
        (cells, n)
    }

    /// New controls for the open tab, carrying every value over without a gesture.
    fn build(&mut self) {
        (self.cells, self.panel_cells) = self.make_cells();
        let focus = self.controls.focused();
        let mut set = ControlSet::new(self.host.clone());
        for c in &self.cells {
            set.add(kit::control_for(c));
        }
        set.set_modifiers(self.modifiers);
        for c in &self.cells {
            set.param_changed(c.param, self.values[c.param]);
        }
        set.set_focus(focus.filter(|&f| f < self.cells.len()));
        self.controls = set;
        self.damage.add_all();
    }

    /// Picks up values the controls changed (drags, keys, typing) and redraws.
    fn sync(&mut self) {
        let changed = self.controls.take_changed();
        if changed.is_empty() {
            return;
        }
        for id in changed {
            if let Some(c) = self.controls.get(id) {
                let p = c.param;
                self.values[p] = c.value();
            }
        }
        if self.dims_changed() {
            self.build();
        }
        self.damage.add_all();
    }

    /// A value changed which controls are faded (a mode, a detector, a switch).
    fn dims_changed(&self) -> bool {
        let (fresh, _) = self.make_cells();
        fresh.iter().zip(&self.cells).any(|(a, b)| a.dim != b.dim)
    }

    fn select(&mut self, tab: usize) {
        if tab != self.tab && tab < SECTIONS {
            self.tab = tab;
            self.panel = None;
            self.build();
        }
    }

    /// Opens settings panel `p` (closing any other), or closes it if it is open.
    fn set_view3d(&mut self, on: bool) {
        self.view3d = on;
        self.damage.add_all();
    }

    fn toggle_panel(&mut self, p: usize) {
        if p < page(self.tab).panels.len() {
            self.open_panel(if self.panel == Some(p) { None } else { Some(p) });
        }
    }

    fn open_panel(&mut self, p: Option<usize>) {
        self.panel = p.filter(|&p| p < page(self.tab).panels.len());
        self.build();
    }

    fn toggle_section(&mut self, i: usize) {
        let on = LAYOUT[i].on;
        let v = if self.values[on] >= 0.5 { 0.0 } else { 1.0 };
        self.values[on] = v;
        self.host.set(on, v);
        self.controls.param_changed(on, v);
        self.damage.add_all();
    }

    // ---- painting ----

    fn paint_layer(&self, s: &mut Scene) {
        let t = self.t();
        self.paint_layer_main(s);
        // Tabs and meter tracks: they change only with the tab and the sections' on states,
        // which are in the layer's key.
        for i in 0..SECTIONS {
            let r = tab_rect(i);
            let open = i == self.tab;
            let on = self.values[LAYOUT[i].on] >= 0.5;
            let fill = if open { t.panel } else { t.inset };
            s.fill_round(rect(r), 8.0, fill, fill, true);
            s.stroke_round(rect(r), 8.0, if open { t.accent } else { t.border }, if open { 1.5 } else { 1.0 });
            let (dx, dy) = tab_dot(i);
            let dot = Scene::ellipse_at(Point::new(dx, dy), 4.5);
            if on {
                s.fill_ellipse_solid(dot, t.accent);
            } else {
                s.stroke_ellipse(dot, t.dim, 1.2);
            }
            s.text(Point::new(r.0 + 26.0, dy), TAB_LABELS[i], if open { t.text } else { t.dim }, 9.5, false);
        }
        for x0 in [22.0f32, 962.0] {
            for c in 0..2 {
                let x = x0 + c as f32 * 18.0;
                s.fill_round(Rect::new(x, METER_TOP, 14.0, METER_BOTTOM - METER_TOP), 3.0, t.seg_off, t.seg_off, true);
            }
        }
    }

    fn paint_layer_main(&self, s: &mut Scene) {
        let t = self.t();
        s.fill_rect(Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32), t.bg, t.bg, true);
        s.text(Point::new(20.0, 27.0), "OpenDynamics", t.text, 15.0, false);
        // Rails and the page.
        kit::panel(s, t, (8.0, 60.0, 84.0, 648.0));
        kit::panel(s, t, (948.0, 60.0, 84.0, 648.0));
        kit::panel(s, t, PAGE);
        for (x, label) in [(50.0, "IN"), (990.0, "OUT")] {
            s.text(Point::new(x, METER_BOTTOM + 12.0), label, t.dim, 9.0, true);
        }
        for rail in [22.0, 962.0] {
            for db in [6.0f32, 0.0, -6.0, -12.0, -24.0, -36.0, -48.0, -60.0] {
                let y = meter_y(db);
                s.line(Point::new(rail + 40.0, y), Point::new(rail + 44.0, y), t.grid_strong, 1.0);
                s.text(Point::new(rail + 47.0, y), format!("{db:.0}"), t.dim, 8.0, false);
            }
        }
        // Page title.
        s.text(Point::new(PAGE.0 + 96.0, TOP_Y + 11.0), LAYOUT[self.tab].label.to_uppercase(), t.text, 12.0, false);
        if self.view3d {
            self.paint_3d_layer(s);
        } else {
            // Display: level grid every 12 dB, a line each second.
            kit::plot(s, t, DISPLAY);
            let bottom = DISPLAY.1 + DISPLAY.3;
            let mut db = 0.0f32;
            while db >= LO - 1e-3 {
                let y = level_y(db);
                s.line(Point::new(DISPLAY.0, y), Point::new(DISPLAY.0 + DISPLAY.2, y), if db == 0.0 { t.grid_strong } else { t.grid }, 1.0);
                kit::text_right(s, DISPLAY.0 - 4.0, y, &format!("{db:.0}"), t.dim, 8.0);
                db -= 12.0;
            }
            kit::text_right(s, DISPLAY.0 - 4.0, DISPLAY.1 + 5.0, "dBFS", t.dim, 8.0);
            for k in 0..=HISTORY_S {
                let x = DISPLAY.0 + DISPLAY.2 * k as f32 / HISTORY_S as f32;
                if k > 0 && k < HISTORY_S {
                    s.line(Point::new(x, DISPLAY.1), Point::new(x, bottom), t.grid, 1.0);
                    s.line(Point::new(x, LANE.1), Point::new(x, LANE.1 + LANE.3), t.grid, 1.0);
                }
                let label = if k == HISTORY_S { "NOW".to_string() } else { format!("-{} s", HISTORY_S - k) };
                let y = LANE.1 + LANE.3 + 11.0;
                match k {
                    0 => s.text(Point::new(x, y), label, t.dim, 8.0, false),
                    k if k == HISTORY_S => kit::text_right(s, x, y, &label, t.dim, 8.0),
                    _ => s.text(Point::new(x, y), label, t.dim, 8.0, true),
                }
            }
            // Spectrograph lane: frequency up the side.
            kit::plot(s, t, LANE);
            for (f, label) in [(100.0f32, "100"), (1000.0, "1k"), (10_000.0, "10k")] {
                let k = (f / SPEC_LO).ln() / (SPEC_HI / SPEC_LO).ln() * (BANDS - 1) as f32;
                let y = LANE.1 + LANE.3 - (k + 0.5) * LANE.3 / BANDS as f32;
                kit::text_right(s, DISPLAY.0 - 4.0, y, label, t.dim, 8.0);
            }
            // The curve square: a plate over the history, its input grid, the unity line.
            let plate = t.plot_bg.with_alpha(0.85);
            s.fill_rect(Rect::new(DISPLAY.0, DISPLAY.1, DISPLAY.3, DISPLAY.3), plate, plate, true);
            s.stroke_round(Rect::new(DISPLAY.0, DISPLAY.1, DISPLAY.3, DISPLAY.3), 0.0, t.grid_strong, 1.0);
            for db in [-48.0f32, -36.0, -24.0, -12.0, 0.0] {
                let x = curve_x(db);
                s.line(Point::new(x, DISPLAY.1), Point::new(x, bottom), t.grid, 1.0);
                s.text(Point::new(x, bottom - 8.0), format!("{db:.0}"), t.dim, 8.0, true);
            }
            // Top left: a low input never gives a high output, so no curve passes here.
            s.text(Point::new(DISPLAY.0 + 8.0, DISPLAY.1 + 11.0), "CURVE  IN -> OUT dBFS", t.dim, 8.0, false);
            kit::dashed(s, &[Point::new(curve_x(LO), level_y(LO)), Point::new(curve_x(HI), level_y(HI))], 4.0, 4.0, t.grid_strong, 1.0);
            // The gain scale beside the band at the top.
            let (gr_col, sign) = if lifts(self.tab) { (t.ok, "+") } else { (t.over, "-") };
            for g in [0.0f32, 12.0, 24.0] {
                let label = if g == 0.0 { "0".to_string() } else { format!("{sign}{g:.0}") };
                s.text(Point::new(DISPLAY.0 + DISPLAY.2 + 4.0, gain_y(g)), label, gr_col, 8.0, false);
            }
            s.text(Point::new(DISPLAY.0 + DISPLAY.2 + 4.0, gain_y(GR_DB) + 12.0), if lifts(self.tab) { "LIFT" } else { "GR" }, gr_col, 8.0, false);
        }
        kit::button(s, t, VIEW3D_BUTTON, "3D VIEW", self.view3d);
        // Settings chips.
        for (i, (label, _)) in page(self.tab).panels.iter().enumerate() {
            let open = self.panel == Some(i);
            let r = chip_rect(i);
            kit::button(s, t, r, label, open);
            // A small chevron: down to open, up when open.
            let (cx, cy, d) = (r.0 + r.2 - 14.0, r.1 + r.3 * 0.5, if open { -2.5 } else { 2.5 });
            let ink = if open { t.on_accent } else { t.dim };
            kit::curve(s, vec![Point::new(cx - 4.0, cy - d), Point::new(cx, cy + d), Point::new(cx + 4.0, cy - d)], ink, 1.4);
        }
        for c in &self.cells[..self.cells.len() - self.panel_cells] {
            kit::paint_static(s, t, c);
        }
        kit::button(s, t, THEME_BUTTON, "THEME", false);
    }

    fn paint_live(&self, s: &mut Scene) {
        let t = self.t();
        // Tabs: the gain bar under each (the tab itself is in the cached layer).
        for i in 0..SECTIONS {
            let r = tab_rect(i);
            let on = self.values[LAYOUT[i].on] >= 0.5;
            // A thin gain bar under each tab: how hard that section works right now.
            let g = self.live[i].gain[0].abs().max(self.live[i].gain[1].abs()).min(GR_DB);
            if on && g > 0.05 {
                let w = (r.2 - 20.0) * g / GR_DB;
                s.fill_rect(Rect::new(r.0 + 10.0, r.1 + r.3 - 4.0, w, 2.0), t.accent, t.accent, true);
            }
        }
        let tab = self.tab;
        if self.view3d {
            self.paint_3d_terrain_cached(s);
            self.paint_3d_live(s);
        } else {
            let v = |i: usize| self.local(i);
            let on = self.values[LAYOUT[tab].on] >= 0.5;
            let ink = if on { t.accent } else { t.dim };
            let bottom = DISPLAY.1 + DISPLAY.3;
            let gr_col = if lifts(tab) { t.ok } else { t.over };
            // History, oldest at the left: input level, output level, gain band from the top.
            let x_at = |k: usize| DISPLAY.0 + DISPLAY.2 * k as f32 / (TRACE_LEN - 1) as f32;
            let input: Vec<Point> = self.level[tab].iter().enumerate().map(|(k, l)| Point::new(x_at(k), level_y(*l))).collect();
            let output: Vec<Point> = self.level[tab].iter().zip(&self.trace[tab]).enumerate().map(|(k, (l, g))| Point::new(x_at(k), level_y(l + g))).collect();
            let band: Vec<Point> = self.trace[tab].iter().enumerate().map(|(k, g)| Point::new(x_at(k), gain_y(*g))).collect();
            // Fills only right of the curve square (its plate hides them there; fills are the cost):
            // the gap between input and output, the output down to the floor, the gain band.
            let sq = DISPLAY.0 + DISPLAY.3;
            let first = ((sq - DISPLAY.0) / DISPLAY.2 * (TRACE_LEN - 1) as f32).floor() as usize;
            kit::between(s, &input[first..], &output[first..], t.in_fill.with_alpha(t.in_fill_alpha));
            kit::area(s, &output[first..], bottom, t.out_fill.with_alpha(t.out_fill_alpha));
            kit::area(s, &band[first..], DISPLAY.1, gr_col.with_alpha(0.25));
            // Lines: faint over the curve square, full to its right.
            let faint = 0.3;
            kit::curve(s, input[..=first].to_vec(), t.in_line.with_alpha(faint), 1.0);
            kit::curve(s, output[..=first].to_vec(), t.out_line.with_alpha(faint), 1.0);
            kit::curve(s, band[..=first].to_vec(), gr_col.with_alpha(faint), 1.0);
            kit::curve(s, input[first..].to_vec(), t.in_line, 1.0);
            kit::curve(s, output[first..].to_vec(), if on { t.out_line } else { t.dim }, 1.5);
            kit::curve(s, band[first..].to_vec(), gr_col, 1.2);
            // Threshold across the history.
            let thr = threshold(tab, &v);
            if (LO..=HI).contains(&thr) {
                kit::dashed(s, &[Point::new(sq, level_y(thr)), Point::new(DISPLAY.0 + DISPLAY.2, level_y(thr))], 4.0, 4.0, t.dim, 1.0);
                if self.panel.is_none() {
                    // On a plate: the tag sits over the moving history.
                    let tag = format!("THR {thr:.0} dB");
                    let (w, right, y) = (goodlookindri_shell::text::width(&tag, 8.0), DISPLAY.0 + DISPLAY.2 - 6.0, (level_y(thr) - 9.0).max(DISPLAY.1 + 8.0));
                    s.fill_round(Rect::new(right - w - 5.0, y - 6.5, w + 10.0, 13.0), 3.0, t.plot_bg, t.plot_bg, true);
                    kit::text_right(s, right, y, &tag, t.text, 8.0);
                }
            }
            // The curve square (its plate and grid are in the cached layer).
            if (LO..=HI).contains(&thr) {
                kit::dashed(s, &[Point::new(curve_x(thr), DISPLAY.1 + 20.0), Point::new(curve_x(thr), bottom)], 2.0, 3.0, t.dim, 1.0);
            }
            let pts: Vec<Point> = (0..=120)
                .map(|k| {
                    let l = LO + (HI - LO) * k as f32 / 120.0;
                    Point::new(curve_x(l), level_y(curve_out(tab, l as f64, &v) as f32))
                })
                .collect();
            kit::curve(s, pts, ink, 2.0);
            // The live dot: detector level in, level plus the gain applied now (the newest output).
            let live = self.live[tab];
            if live.active {
                let l = live.level[0].max(live.level[1]);
                if l > LO {
                    let g = if live.gain[0].abs() >= live.gain[1].abs() { live.gain[0] } else { live.gain[1] };
                    let p = Point::new(curve_x(l), level_y(l + g));
                    s.fill_ellipse_solid(Scene::ellipse_at(p, 5.0), t.text);
                    s.stroke_ellipse(Scene::ellipse_at(p, 5.0), t.panel, 1.5);
                }
            }
            // Spectrograph: per tick and band, energy the sections took out (red) or added (green).
            // Vertical runs of one shade are one rect (the cost is the commands, not the area).
            let col_w = DISPLAY.2 / (TRACE_LEN - 1) as f32;
            for (k, col) in self.spec_cols.iter().enumerate() {
                let x = (DISPLAY.0 + col_w * (k as f32 - 0.5)).max(LANE.0);
                let w = (DISPLAY.0 + col_w * (k as f32 + 0.5)).min(LANE.0 + LANE.2) - x;
                // One row per pixel, interpolated between bands; runs of one shade are one rect.
                let shade = |d: f32| -> i32 { ((d / SPEC_FULL_DB).clamp(-1.0, 1.0) * SHADES as f32).round() as i32 };
                let rows = LANE.3 as usize;
                let at = |r: usize| -> f32 {
                    let p = ((r as f32 + 0.5) / rows as f32 * BANDS as f32 - 0.5).clamp(0.0, (BANDS - 1) as f32);
                    let (i, f) = (p.floor() as usize, p.fract());
                    col[i] + (col[(i + 1).min(BANDS - 1)] - col[i]) * f
                };
                let mut b = 0;
                while b < rows {
                    let q = shade(at(b));
                    let mut e = b + 1;
                    while e < rows && shade(at(e)) == q {
                        e += 1;
                    }
                    if q != 0 {
                        let c = if q < 0 { t.over } else { t.ok }.with_alpha(0.12 + 0.73 * q.unsigned_abs() as f32 / SHADES as f32);
                        let bottom = LANE.1 + LANE.3;
                        s.fill_rect(Rect::new(x, bottom - e as f32, w, (e - b) as f32), c, c, true);
                    }
                    b = e;
                }
            }
            {
                let cap = "ENERGY CHANGE BY FREQUENCY";
                let w = goodlookindri_shell::text::width(cap, 8.0);
                s.fill_round(Rect::new(LANE.0 + 4.0, LANE.1 + 3.0, w + 10.0, 12.0), 3.0, t.plot_bg, t.plot_bg, true);
                s.text(Point::new(LANE.0 + 9.0, LANE.1 + 9.0), cap, t.dim, 8.0, false);
            }
        }
        // Readouts beside the chips, right to left.
        let mut right = VIEW3D_BUTTON.0 - 12.0;
        let y = CHIP_Y + 11.0;
        let now = *self.trace[tab].back().unwrap_or(&0.0);
        let gr = format!("{} {} dB", if lifts(tab) { "LIFT" } else { "GR" }, whole_db(now));
        kit::text_right(s, right, y, &gr, t.text, 11.0);
        right -= goodlookindri_shell::text::width(&gr, 11.0) + 12.0;
        if self.values[P_AUTO_GAIN] >= 0.5 {
            let ag = format!("AUTO GAIN {} dB", whole_db(self.auto_gain_db));
            kit::text_right(s, right, y, &ag, t.dim, 9.0);
            right -= goodlookindri_shell::text::width(&ag, 9.0) + 12.0;
        }
        if self.shared.as_ref().is_some_and(|sh| sh.key_present()) {
            kit::text_right(s, right, y, "KEY IN", t.ok, 9.0);
        }
        // The open settings panel, over the display.
        if let Some(p) = self.panel {
            let r = panel_rect(tab, p);
            kit::panel(s, t, r);
            s.stroke_round(rect(r), 8.0, t.accent, 1.0);
            s.text(Point::new(r.0 + 16.0, r.1 + 17.0), page(tab).panels[p].0, t.text, 9.5, false);
            for c in &self.cells[self.cells.len() - self.panel_cells..] {
                if matches!(c.kind, Kind::Segmented) {
                    s.text(Point::new(r.0 + 16.0, c.rect.1 + c.rect.3 * 0.5), &c.label, t.dim, 9.0, false);
                }
                kit::paint_static(s, t, c);
            }
        }
        // Meters.
        for (m, x0) in [(0usize, 22.0f32), (1, 962.0)] {
            for c in 0..2 {
                let x = x0 + c as f32 * 18.0;
                let db = self.meters[m][c];
                if db > M_LO {
                    let y = meter_y(db);
                    let col = if db > 0.0 { t.over } else { t.seg_on };
                    s.fill_round(Rect::new(x, y, 14.0, METER_BOTTOM - y), 3.0, col, col, true);
                }
            }
        }
        // Control values.
        for (cell, c) in self.cells.iter().zip(self.controls.iter()) {
            let focused = self.keys_active && self.controls.focused().is_some_and(|f| self.controls.get(f).is_some_and(|fc| fc.param == cell.param));
            kit::paint_value(s, t, cell, c, c.value(), focused);
        }
    }
}

impl Editor for DynEditor {
    fn size(&self) -> (u32, u32) {
        (WIDTH, HEIGHT)
    }

    fn size_range(&self) -> SizeRange {
        SizeRange::fixed(WIDTH, HEIGHT)
    }

    fn bind(&mut self, state: Option<EditorState>) {
        self.shared = state.and_then(|s| s.downcast::<Shared>().ok());
    }

    fn param_changed(&mut self, index: usize, plain: f32) {
        let Some(v) = self.values.get_mut(index) else { return };
        if *v == plain {
            return;
        }
        *v = plain;
        self.controls.param_changed(index, plain);
        let _ = self.controls.take_changed();
        if self.dims_changed() {
            self.build();
        }
        self.damage.add_all();
    }

    fn tick(&mut self, dt: f32) {
        let Some(sh) = self.shared.clone() else { return };
        // History entries by elapsed time (the axis reads seconds whatever the tick rate).
        self.hist_acc += dt.max(0.0);
        let pushes = ((self.hist_acc * HIST_HZ) as usize).min(4);
        self.hist_acc = (self.hist_acc - pushes as f32 / HIST_HZ).min(1.0 / HIST_HZ);
        for i in 0..SECTIONS {
            let l = sh.take_section(i);
            self.live[i] = l;
            let g = if l.gain[0].abs() >= l.gain[1].abs() { l.gain[0] } else { l.gain[1] };
            for _ in 0..pushes {
                let tr = &mut self.trace[i];
                tr.pop_front();
                tr.push_back(if l.active { g } else { 0.0 });
                let lv = &mut self.level[i];
                lv.pop_front();
                lv.push_back(if l.active { l.level[0].max(l.level[1]) } else { -200.0 });
            }
        }
        self.auto_gain_db = sh.auto_gain_db();
        // Spectrograph: what the sections did to each band, output against the aligned input.
        for (c, tap) in self.taps.iter_mut().enumerate() {
            sh.read_tap(c, tap);
        }
        let sr = sh.sample_rate();
        let [a, b, c, d] = &self.taps;
        self.spec[0].update(a, Some(b), sr, dt);
        self.spec[1].update(c, Some(d), sr, dt);
        self.terrain.feed(&self.taps, sr, dt);
        // Change per band (dB), averaged with its neighbours, then weighted by where the
        // track's energy is: a band 40 dB under the loudest one shows nothing, so a broadband
        // gain lights up the bands that carry the music, not the empty ones.
        let (si, so) = (&self.spec[0].db, &self.spec[1].db);
        let loud = si.iter().chain(so.iter()).fold(SPEC_GATE_DB, |m, v| m.max(*v));
        let delta = |k: usize| if si[k].max(so[k]) < SPEC_GATE_DB { 0.0 } else { so[k] - si[k] };
        let prev = *self.spec_cols.back().unwrap_or(&[0.0; BANDS]);
        let mut col = [0.0f32; BANDS];
        for (k, v) in col.iter_mut().enumerate() {
            let (a, b) = (k.saturating_sub(1), (k + 1).min(BANDS - 1));
            let d = (delta(a) + 2.0 * delta(k) + delta(b)) * 0.25;
            let w = (1.0 - (loud - si[k].max(so[k])) / SPEC_WEIGHT_DB).clamp(0.0, 1.0);
            *v = 0.5 * prev[k] + 0.5 * d * w;
        }
        for _ in 0..pushes {
            self.spec_cols.pop_front();
            self.spec_cols.push_back(col);
        }
        let (pin, pout) = sh.take_peaks();
        let fall = 24.0 * dt;
        for c in 0..2 {
            self.meters[0][c] = to_db(pin[c]).max(self.meters[0][c] - fall);
            self.meters[1][c] = to_db(pout[c]).max(self.meters[1][c] - fall);
        }
        // shortcut: the whole moving layer is repainted each tick; per-part damage like
        // OpenLimiter's when CPU in the editor matters.
        self.damage.add_all();
    }

    fn damage(&mut self) -> Option<Vec<DamageRect>> {
        self.sync();
        self.damage.pending().map(<[DamageRect]>::to_vec)
    }

    fn render(&mut self, scale: f32) -> Frame<'_> {
        self.sync();
        if self.view3d {
            self.update_terrain_image(scale);
        }
        // What the cached layer shows, hashed: tab, theme, 3D, open panel, faded controls and
        // which sections are on (their tabs and the meter tracks live in it).
        let parts = [
            self.tab as u64,
            self.theme as u64,
            self.view3d as u64,
            self.panel.map_or(0, |p| p as u64 + 1),
            self.cells.iter().filter(|c| c.dim).fold(0u64, |h, c| h.wrapping_mul(31).wrapping_add(c.param as u64)),
            (0..SECTIONS).fold(0u64, |h, i| h << 1 | u64::from(self.values[LAYOUT[i].on] >= 0.5)),
        ];
        let key = parts.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, v| (h ^ v).wrapping_mul(0x0100_0000_01b3));
        let mut layer = Scene::new();
        self.paint_layer(&mut layer);
        let mut live = Scene::new();
        self.paint_live(&mut live);
        let damage = self.damage.pending();
        let frame = self.canvas.frame_damaged(scale, key, |dl| dl.append(layer), |dl| dl.append(live), damage);
        self.damage.clear();
        frame
    }

    fn modifiers_changed(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
        self.controls.set_modifiers(modifiers);
    }

    fn mouse_down(&mut self, x: f32, y: f32, clicks: u32) {
        for i in 0..SECTIONS {
            if inside(dot_hit(i), x, y) {
                self.toggle_section(i);
                return;
            }
            if inside(tab_rect(i), x, y) {
                self.select(i);
                return;
            }
        }
        if inside(THEME_BUTTON, x, y) {
            self.theme = self.theme.next();
            store_theme(self.theme);
            self.damage.add_all();
            return;
        }
        for i in 0..page(self.tab).panels.len() {
            if inside(chip_rect(i), x, y) {
                self.toggle_panel(i);
                return;
            }
        }
        if inside(VIEW3D_BUTTON, x, y) {
            self.set_view3d(!self.view3d);
            return;
        }
        // A click on the display outside an open panel closes it.
        if let Some(p) = self.panel {
            if inside(DISPLAY, x, y) && !inside(panel_rect(self.tab, p), x, y) {
                self.open_panel(None);
                return;
            }
        }
        self.controls.mouse_down(x, y, clicks);
        self.sync();
    }

    fn mouse_move(&mut self, x: f32, y: f32) {
        self.controls.mouse_move(x, y);
        self.sync();
    }

    fn mouse_up(&mut self, x: f32, y: f32) {
        self.controls.mouse_up(x, y);
        self.sync();
    }

    fn wheel(&mut self, x: f32, y: f32, dy: f32) {
        self.controls.wheel(x, y, dy);
        self.sync();
    }

    fn right_mouse_down(&mut self, x: f32, y: f32) -> bool {
        self.controls.context_click(x, y)
    }

    fn take_param_menu(&mut self) -> Option<ParamMenu> {
        self.controls.take_param_menu()
    }

    fn param_menu_action(&mut self, param: u32, action: ParamMenuAction) -> Option<String> {
        let r = self.controls.param_menu_action(param, action);
        self.sync();
        r
    }

    fn tooltips(&self) -> Vec<Tooltip> {
        let mut tips = self.controls.tooltips();
        for i in 0..SECTIONS {
            tips.push(Tooltip { rect: DamageRect::from_tuple(dot_hit(i)), text: format!("Switch {} on or off", LAYOUT[i].label) });
        }
        for (i, (label, _)) in page(self.tab).panels.iter().enumerate() {
            tips.push(Tooltip { rect: DamageRect::from_tuple(chip_rect(i)), text: format!("Open or close the {} settings", label.to_lowercase()) });
        }
        tips.push(Tooltip {
            rect: DamageRect::from_tuple((DISPLAY.0, DISPLAY.1, DISPLAY.3, DISPLAY.3)),
            text: "Static curve: output level for a steady input level, on the same dBFS scale as the history. The dot is the detector level now and the gain applied now.".into(),
        });
        tips.push(Tooltip {
            rect: DamageRect::from_tuple((DISPLAY.0 + DISPLAY.3, DISPLAY.1, DISPLAY.2 - DISPLAY.3, DISPLAY.3)),
            text: "Last 4 seconds: detector level (light), output level (line), and the gain the section applies (band from the top).".into(),
        });
        tips.push(Tooltip { rect: DamageRect::from_tuple(VIEW3D_BUTTON), text: "Show the 3D spectrum large and the history small".into() });
        tips.push(Tooltip {
            rect: DamageRect::from_tuple(LANE),
            text: "Energy change by frequency over the last 4 seconds: output against the input, lined up for latency, weighted by how much of the track sits in each band. Red: the sections took energy out there; green: they added it.".into(),
        });
        tips
    }

    /// The controls, then one switch per settings chip (so a closed panel is still reachable).
    fn accessibility_nodes(&self) -> Vec<AccessNode> {
        let mut nodes = self.controls.accessibility_nodes();
        for (i, (label, _)) in page(self.tab).panels.iter().enumerate() {
            let open = self.panel == Some(i);
            nodes.push(AccessNode {
                label: format!("{} settings", label.to_lowercase()),
                value: if open { "open" } else { "closed" }.into(),
                role: AccessRole::Switch,
                on: open,
                rect: DamageRect::from_tuple(chip_rect(i)),
                focused: false,
                enabled: true,
            });
        }
        nodes.push(AccessNode {
            label: "3D spectrum view".into(),
            value: if self.view3d { "on" } else { "off" }.into(),
            role: AccessRole::Switch,
            on: self.view3d,
            rect: DamageRect::from_tuple(VIEW3D_BUTTON),
            focused: false,
            enabled: true,
        });
        nodes
    }

    fn accessibility_action(&mut self, node: usize, action: AccessAction) -> bool {
        let n = self.controls.accessibility_nodes().len();
        if node >= n {
            let chips = page(self.tab).panels.len();
            if action == AccessAction::Press && node - n < chips {
                self.toggle_panel(node - n);
                return true;
            }
            if action == AccessAction::Press && node - n == chips {
                self.set_view3d(!self.view3d);
                return true;
            }
            return false;
        }
        let r = self.controls.accessibility_action(node, action);
        self.sync();
        r
    }

    fn accessibility_range(&self, node: usize) -> Option<AccessRange> {
        self.controls.accessibility_range(node)
    }

    fn accessibility_set_value(&mut self, node: usize, value: f64) -> bool {
        let r = self.controls.accessibility_set_value(node, value);
        self.sync();
        r
    }

    fn accessibility_focus(&mut self, node: usize) -> bool {
        self.controls.accessibility_focus(node)
    }

    fn wants_keys(&self) -> bool {
        self.controls.wants_keys()
    }

    fn key_down(&mut self, key: Key, modifiers: Modifiers) -> bool {
        let r = self.controls.key_down(key, modifiers);
        self.sync();
        r
    }

    fn key_up(&mut self, key: Key, modifiers: Modifiers) -> bool {
        self.controls.key_up(key, modifiers)
    }

    fn take_text_edit(&mut self) -> Option<TextEdit> {
        self.controls.take_text_edit()
    }

    fn text_edit_commit(&mut self, text: &str) -> bool {
        let r = self.controls.text_edit_commit(text);
        self.sync();
        r
    }

    fn text_edit_cancel(&mut self) {
        self.controls.text_edit_cancel();
    }

    fn focus_changed(&mut self, focused: bool) {
        self.keys_active = focused;
        if !focused {
            self.controls.release_keys();
        }
        self.damage.add_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Closed, then each settings panel of the tab open.
    fn panel_states(tab: usize) -> Vec<Option<usize>> {
        std::iter::once(None).chain((0..page(tab).panels.len()).map(Some)).collect()
    }

    /// Every parameter of every section, and the globals, has a control on some tab (a
    /// control in a settings panel counts).
    #[test]
    fn every_parameter_has_a_control() {
        let mut e = DynEditor::new(EditorHost::detached());
        let mut seen = vec![false; INFO.params.len()];
        for tab in 0..SECTIONS {
            e.select(tab);
            for p in panel_states(tab) {
                e.open_panel(p);
                for c in &e.cells {
                    seen[c.param] = true;
                }
            }
        }
        let missing: Vec<&str> = seen.iter().enumerate().filter(|(_, s)| !**s).map(|(i, _)| INFO.params[i].id).collect();
        assert!(missing.is_empty(), "no control for {missing:?}");
    }

    #[test]
    fn controls_stay_inside_the_page_and_do_not_overlap() {
        let mut e = DynEditor::new(EditorHost::detached());
        let within = |c: &R, r: R| c.0 >= r.0 && c.1 >= r.1 && c.0 + c.2 <= r.0 + r.2 && c.1 + c.3 <= r.1 + r.3;
        for tab in 0..SECTIONS {
            e.select(tab);
            for p in panel_states(tab) {
                e.open_panel(p);
                let n = e.cells.len() - e.panel_cells;
                let on_page: Vec<&Cell> = e.cells.iter().filter(|c| c.param != P_INPUT && c.param != P_OUTPUT).collect();
                for c in &on_page {
                    assert!(within(&c.rect, PAGE), "tab {tab} panel {p:?}: {} at {:?}", c.label, c.rect);
                }
                if let Some(p) = p {
                    let r = panel_rect(tab, p);
                    assert!(within(&r, DISPLAY), "tab {tab}: panel {p} at {r:?} leaves the display");
                    for c in &e.cells[n..] {
                        assert!(within(&c.rect, r), "tab {tab} panel {p}: {} at {:?} outside {r:?}", c.label, c.rect);
                    }
                }
                // Main controls stay clear of the display; chips stay clear of everything.
                for c in &e.cells[..n] {
                    let ov = |r: R| c.rect.0 < r.0 + r.2 && r.0 < c.rect.0 + c.rect.2 && c.rect.1 < r.1 + r.3 && r.1 < c.rect.1 + c.rect.3;
                    assert!(!ov(DISPLAY), "tab {tab}: {} covers the display", c.label);
                    for i in 0..page(tab).panels.len() {
                        assert!(!ov(chip_rect(i)), "tab {tab}: {} covers chip {i}", c.label);
                    }
                }
                for (i, a) in on_page.iter().enumerate() {
                    for b in &on_page[i + 1..] {
                        let ov = a.rect.0 < b.rect.0 + b.rect.2 && b.rect.0 < a.rect.0 + a.rect.2 && a.rect.1 < b.rect.1 + b.rect.3 && b.rect.1 < a.rect.1 + a.rect.3;
                        assert!(!ov, "tab {tab} panel {p:?}: {} overlaps {}", a.label, b.label);
                    }
                }
            }
        }
    }

    /// Frame cost per tab at 2x with live signal (plugin-gui-performance, section 3a):
    /// `cargo test --release --lib frame_cost -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn frame_cost() {
        let (w, h) = (WIDTH as f64, HEIGHT as f64);
        for (tab, panel, v3) in (0..SECTIONS).flat_map(|t| [(t, None, false), (t, Some(page(t).panels.len() - 1), false), (t, None, true)]) {
            let shared = Arc::new(Shared::default());
            let mut e = DynEditor::new(EditorHost::detached());
            e.bind(Some(shared.clone() as EditorState));
            e.param_changed(LAYOUT[tab].on, 1.0);
            e.param_changed(P_AUTO_GAIN, 1.0);
            e.select(tab);
            e.open_panel(panel);
            e.set_view3d(v3);
            // Fill the history and the 3D terrain first (4 s), so a full view is timed.
            let mut seed = 11u32;
            for f in 0..(HISTORY_S as f32 * HIST_HZ) as usize {
                let g = -6.0 - 6.0 * (f as f32 * 0.4).sin();
                shared.section(tab, [g, g * 0.9], [-20.0 + g, -22.0 + g], true);
                let pre: Vec<f32> = (0..1600)
                    .map(|_| {
                        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                        0.3 * ((seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5)
                    })
                    .collect();
                let post: Vec<f32> = pre.iter().map(|v| v * 10f32.powf(g / 20.0)).collect();
                shared.push_taps([&pre, &pre], [&post, &post]);
                e.tick(1.0 / 30.0);
            }
            let _ = e.render(2.0);
            let (mut area, mut time, frames) = (0.0f64, std::time::Duration::ZERO, 30);
            for f in 0..frames {
                let g = -6.0 - 6.0 * (f as f32 * 0.4).sin();
                for i in 0..SECTIONS {
                    shared.section(i, [g, g * 0.9], [-20.0 + g, -22.0 + g], true);
                }
                shared.peaks([0.5, 0.4], [0.3, 0.25]);
                shared.set_auto_gain(-3.0 + 0.1 * f as f32);
                // Worst case for the spectrograph: noise in every band, a gain that moves.
                let mut seed = 3u32 + f as u32;
                let pre: Vec<f32> = (0..1600)
                    .map(|_| {
                        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                        0.3 * ((seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5)
                    })
                    .collect();
                let post: Vec<f32> = pre.iter().map(|v| v * 10f32.powf(g / 20.0)).collect();
                shared.push_taps([&pre, &pre], [&post, &post]);
                e.tick(1.0 / 30.0);
                area += match e.damage() {
                    Some(r) => r.iter().map(|d| (d.w.min(w as f32 * 2.0) * d.h.min(h as f32 * 2.0)) as f64).sum::<f64>(),
                    None => w * h * 4.0,
                };
                let t0 = std::time::Instant::now();
                let _ = e.render(2.0);
                time += t0.elapsed();
                if std::env::var("FRAMES").is_ok() && v3 {
                    print!("{:.0} ", t0.elapsed().as_secs_f64() * 1000.0);
                }
            }
            let state = panel.map_or("closed", |p| page(tab).panels[p].0);
            let state = if v3 { "3D" } else { state };
            println!("{:16} {state:9} dirty {:5.1}% of window per frame, render {:6.2} ms per frame", LAYOUT[tab].label, 100.0 * area / frames as f64 / (w * h * 4.0), time.as_secs_f64() * 1000.0 / frames as f64);
        }
    }

    /// Which live-layer commands cost the most at 2x (profiling aid). Run with `--ignored`.
    #[test]
    #[ignore]
    fn live_layer_cost_by_command() {
        use goodlookindri_shell::raster::Framebuffer;
        let tab: usize = std::env::var("TAB").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
        let shared = Arc::new(Shared::default());
        let mut e = DynEditor::new(EditorHost::detached());
        e.bind(Some(shared.clone() as EditorState));
        e.param_changed(LAYOUT[tab].on, 1.0);
        e.select(tab);
        e.set_view3d(std::env::var("V3").is_ok());
        let mut seed = 3u32;
        for f in 0..TRACE_LEN {
            let g = -6.0 - 6.0 * (f as f32 * 0.4).sin();
            shared.section(tab, [g, g], [-20.0 + g, -20.0 + g], true);
            let pre: Vec<f32> = (0..800)
                .map(|_| {
                    seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    0.3 * ((seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5)
                })
                .collect();
            let post: Vec<f32> = pre.iter().map(|v| v * 10f32.powf(g / 20.0)).collect();
            shared.push_taps([&pre, &pre], [&post, &post]);
            e.tick(1.0 / 30.0);
        }
        let mut live = Scene::new();
        e.paint_live(&mut live);
        openplugin::dri::goodlookindri::hardware::set_device_scale(2.0);
        let mut fb = Framebuffer::new(WIDTH as usize * 2, HEIGHT as usize * 2);
        let mut costs: Vec<(f64, usize, String)> = Vec::new();
        for (i, c) in live.cmds.iter().enumerate() {
            let mut one = Scene::new();
            one.cmds.push(c.clone());
            let t0 = std::time::Instant::now();
            for _ in 0..3 {
                fb.replay(&one, 2.0);
            }
            let name = format!("{c:?}");
            costs.push((t0.elapsed().as_secs_f64() * 1000.0 / 3.0, i, name.chars().take(90).collect()));
        }
        let total: f64 = costs.iter().map(|c| c.0).sum();
        let mut by_kind: std::collections::BTreeMap<String, (f64, usize)> = Default::default();
        for c in &costs {
            let k = c.2.split(|ch: char| !ch.is_alphanumeric()).next().unwrap_or("").to_string();
            let e = by_kind.entry(k).or_default();
            e.0 += c.0;
            e.1 += 1;
        }
        for (k, (ms, n)) in &by_kind {
            println!("KIND {k:12} {n:6} cmds {ms:7.2} ms");
        }
        costs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        println!("{} commands, {total:.2} ms", costs.len());
        for c in costs.iter().take(25) {
            println!("{:7.3} ms  #{:4}  {}", c.0, c.1, c.2);
        }
    }

    /// Screenshots with live signal: `<SHOTS>/<tab>-<theme>-<scale>x.ppm`. Run with `--ignored`.
    #[test]
    #[ignore]
    fn screenshots() {
        let dir = std::env::var("SHOTS").unwrap_or_else(|_| format!("{}/build/shots", env!("CARGO_MANIFEST_DIR")));
        std::fs::create_dir_all(&dir).expect("dir");
        let scale: f32 = std::env::var("SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
        let shots = ThemeId::ALL.into_iter().flat_map(|th| {
            (0..SECTIONS).flat_map(move |tab| {
                let panels = if matches!(th, ThemeId::Dark | ThemeId::Light) { page(tab).panels.len() } else { 0 };
                std::iter::once((th, tab, None, false)).chain(std::iter::once((th, tab, None, true))).chain((0..panels).map(move |p| (th, tab, Some(p), false)))
            })
        });
        for (th, tab, panel, v3) in shots {
            {
                let shared = Arc::new(Shared::default());
                let mut e = DynEditor::new(EditorHost::detached());
                e.bind(Some(shared.clone() as EditorState));
                e.theme = th;
                e.param_changed(LAYOUT[tab].on, 1.0);
                e.param_changed(P_AUTO_GAIN, 1.0);
                e.select(tab);
                // Four seconds of a drum-like level: hits every half second, decaying.
                for f in 0..TRACE_LEN {
                    let t = (f % 15) as f32;
                    let lvl = -14.0 - 2.5 * t;
                    let g = match tab {
                        2 | 3 => (6.0 - t * 0.6).max(0.0) * if tab == 2 { 0.6 } else { 1.0 },
                        _ => -(10.0 - t * 0.8).max(0.0),
                    };
                    for i in 0..SECTIONS {
                        shared.section(i, [g, g * 0.92], [lvl, lvl - 1.5], i == tab);
                    }
                    let pk = 10f32.powf(lvl / 20.0);
                    shared.peaks([pk * 1.2, pk], [pk, pk * 0.85]);
                    shared.set_auto_gain(-2.6);
                    // Audio for the spectrograph: a mix of tones and noise, through the gain.
                    let mut seed = 9u32 + f as u32;
                    let pre: Vec<f32> = (0..800)
                        .map(|i| {
                            let n = (f * 800 + i) as f32 / 48_000.0;
                            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                            let noise = (seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5;
                            let tau = std::f32::consts::TAU;
                            pk * (0.5 * (tau * 80.0 * n).sin() + 0.25 * (tau * 440.0 * n).sin() + 0.12 * (tau * 2500.0 * n).sin() + 0.1 * noise)
                        })
                        .collect();
                    let post: Vec<f32> = pre.iter().map(|v| v * 10f32.powf(g / 20.0)).collect();
                    shared.push_taps([&pre, &pre], [&post, &post]);
                    e.tick(1.0 / 30.0);
                }
                e.open_panel(panel);
                e.set_view3d(v3);
                e.damage.add_all();
                let f = e.render(scale);
                let mut out = format!("P6\n{} {}\n255\n", f.width, f.height).into_bytes();
                for px in f.pixels {
                    out.extend_from_slice(&[(px >> 16) as u8, (px >> 8) as u8, *px as u8]);
                }
                let state = if v3 { "-3d".to_string() } else { panel.map_or(String::new(), |p| format!("-{}", page(tab).panels[p].0.to_lowercase())) };
                std::fs::write(format!("{dir}/{}{state}-{}-{scale:.0}x.ppm", LAYOUT[tab].key, th.name()), out).expect("write");
            }
        }
    }

    /// Every tab with signal, static and live layers: no text cut by its clip, no two labels
    /// overlapping, nothing under 8 pt (plugin-gui-performance, text audit).
    #[test]
    fn text_is_whole_apart_and_readable() {
        use goodlookindri_shell::text;
        use openplugin::dri::goodlookindri::scene::Cmd;
        let mut bad = Vec::new();
        for (tab, panel, v3) in (0..SECTIONS).flat_map(|t| panel_states(t).into_iter().flat_map(move |p| [(t, p, false), (t, p, true)])) {
            let shared = Arc::new(Shared::default());
            let mut e = DynEditor::new(EditorHost::detached());
            e.bind(Some(shared.clone() as EditorState));
            e.param_changed(LAYOUT[tab].on, 1.0);
            e.param_changed(P_AUTO_GAIN, 1.0);
            e.select(tab);
            e.open_panel(panel);
            e.set_view3d(v3);
            for _ in 0..5 {
                shared.section(tab, [-0.04, -0.02], [-20.0, -21.0], true);
                shared.peaks([0.5, 0.4], [0.3, 0.25]);
                shared.set_auto_gain(-12.6);
                e.tick(1.0 / 30.0);
            }
            let mut s = Scene::new();
            e.paint_layer(&mut s);
            e.paint_live(&mut s);
            let key = format!("{}/{}", LAYOUT[tab].key, panel.map_or("closed", |p| page(tab).panels[p].0));
            let mut clips: Vec<(f32, f32, f32, f32)> = vec![];
            let mut texts: Vec<(f32, f32, f32, f32, String, f32)> = vec![];
            for c in &s.cmds {
                match c {
                    Cmd::ClipRound { rect, .. } => clips.push((rect.x, rect.y, rect.x + rect.w, rect.y + rect.h)),
                    Cmd::PopClip => {
                        clips.pop();
                    }
                    Cmd::Text { at, text: t, size, center, .. } => {
                        let w = text::width(t, *size);
                        let x0 = if *center { at.x - w * 0.5 } else { at.x };
                        let (y0, y1) = (at.y - size * 0.36, at.y + size * 0.36);
                        if let Some(cl) = clips.iter().fold(None::<(f32, f32, f32, f32)>, |a, c| Some(a.map_or(*c, |a| (a.0.max(c.0), a.1.max(c.1), a.2.min(c.2), a.3.min(c.3))))) {
                            if x0 < cl.0 - 0.5 || x0 + w > cl.2 + 0.5 || y0 < cl.1 - 0.5 || y1 > cl.3 + 0.5 {
                                bad.push(format!("CLIPPED {key}: {t:?} size {size}"));
                            }
                        }
                        if *size < 8.0 {
                            bad.push(format!("SMALL {key}: {t:?} size {size}"));
                        }
                        if t.contains("-0.0") || t.trim_start().starts_with("-0 ") || t == "-0" {
                            bad.push(format!("MINUS ZERO {key}: {t:?}"));
                        }
                        texts.push((x0, y0, x0 + w, y1, t.clone(), *size));
                    }
                    _ => {}
                }
            }
            for i in 0..texts.len() {
                for j in i + 1..texts.len() {
                    let (a, b) = (&texts[i], &texts[j]);
                    if a.0 < b.2 - 0.5 && b.0 < a.2 - 0.5 && a.1 < b.3 - 0.5 && b.1 < a.3 - 0.5 && a.4 != b.4 && !a.4.trim().is_empty() && !b.4.trim().is_empty() {
                        bad.push(format!("OVERLAP {key}: {:?} ({}) and {:?} ({})", a.4, a.5, b.4, b.5));
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }

    /// The displayed text of every control at its minimum, middle and maximum (QA display
    /// rules). Prints the table; `cargo test --release --lib display_table -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn display_table() {
        let mut e = DynEditor::new(EditorHost::detached());
        let mut seen = std::collections::HashSet::new();
        for tab in 0..SECTIONS {
            e.select(tab);
            for p in panel_states(tab) {
                e.open_panel(p);
                for c in e.cells.clone() {
                    let Some(info) = INFO.params.get(c.param) else { continue };
                    if !seen.insert(c.param) || matches!(info.unit, openplugin::core::Unit::Toggle | openplugin::core::Unit::Choice) {
                        continue;
                    }
                    let mid = info.from_normalized(0.5);
                    println!("{:28} | {:>12} | {:>12} | {:>12}", info.id, c.text(info.min), c.text(mid), c.text(info.max));
                }
            }
        }
    }

    /// The readouts beside the chips (GR or LIFT, AUTO GAIN, KEY IN) at their widest never
    /// reach a settings chip or the 3D VIEW button, on any tab.
    #[test]
    fn chip_row_readouts_keep_clear() {
        use openplugin::dri::goodlookindri::scene::Cmd;
        for tab in 0..SECTIONS {
            let shared = Arc::new(Shared::default());
            let mut e = DynEditor::new(EditorHost::detached());
            e.bind(Some(shared.clone() as EditorState));
            e.param_changed(P_AUTO_GAIN, 1.0);
            e.select(tab);
            let worst = if lifts(tab) { 24.0 } else { -24.0 };
            shared.section(tab, [worst, worst], [-20.0, -20.0], true);
            shared.set_auto_gain(-24.0);
            shared.set_key_present(true);
            e.tick(1.0 / 30.0);
            let mut sc = Scene::new();
            e.paint_live(&mut sc);
            let mut boxes: Vec<R> = (0..page(tab).panels.len()).map(chip_rect).collect();
            boxes.push(VIEW3D_BUTTON);
            for c in &sc.cmds {
                if let Cmd::Text { at, text, size, center, .. } = c {
                    if (at.y - (CHIP_Y + 11.0)).abs() > 1.0 {
                        continue;
                    }
                    let w = goodlookindri_shell::text::width(text, *size);
                    let x0 = if *center { at.x - w * 0.5 } else { at.x };
                    for b in &boxes {
                        assert!(x0 + w <= b.0 - 4.0 || x0 >= b.0 + b.2 + 4.0, "tab {tab}: {text:?} at {x0:.0}..{:.0} touches {b:?}", x0 + w);
                    }
                }
            }
        }
    }

    /// IN/OUT meters (sample peak, instant rise, 24 dB/s fall, no hold): a full-scale peak
    /// reads 0 dB on the next tick, -6 dB reads -6, and after the signal stops the reading falls
    /// 24 dB per second of ticks, whatever the tick rate; silence never shows a number.
    #[test]
    fn meters_read_and_fall_as_specified() {
        for hz in [30.0f32, 60.0] {
            let shared = Arc::new(Shared::default());
            let mut e = DynEditor::new(EditorHost::detached());
            e.bind(Some(shared.clone() as EditorState));
            shared.peaks([1.0, 0.5012], [0.5012, 1.0]);
            e.tick(1.0 / hz);
            assert!(e.meters[0][0].abs() < 0.01 && (e.meters[0][1] + 6.0).abs() < 0.01, "{:?}", e.meters);
            assert!((e.meters[1][0] + 6.0).abs() < 0.01 && e.meters[1][1].abs() < 0.01);
            for _ in 0..hz as usize {
                shared.peaks([0.0; 2], [0.0; 2]);
                e.tick(1.0 / hz);
            }
            assert!((e.meters[0][0] + 24.0).abs() < 0.05, "{hz} Hz: {}", e.meters[0][0]);
            // a louder peak during the fall takes over at once
            shared.peaks([0.25, 0.25], [0.25, 0.25]);
            e.tick(1.0 / hz);
            assert!((e.meters[0][0] + 12.04).abs() < 0.05, "{}", e.meters[0][0]);
        }
    }

    #[test]
    fn renders_every_tab_in_every_theme() {
        let mut e = DynEditor::new(EditorHost::detached());
        for th in ThemeId::ALL {
            e.theme = th;
            for tab in 0..SECTIONS {
                e.select(tab);
                for p in panel_states(tab) {
                    e.open_panel(p);
                    e.damage.add_all();
                    let f = e.render(1.0);
                    assert_eq!((f.width, f.height), (WIDTH as usize, HEIGHT as usize));
                }
            }
        }
    }
}
