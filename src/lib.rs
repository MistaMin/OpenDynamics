//! OpenDynamics: five dynamics sections in a fixed order, each on its own tab.
//!
//! ```text
//! in -> input gain -> Decompressor -> Gate/Expander -> Compressor -> De-Expander -> Limiter
//!    -> output gain -> out
//! ```
//!
//! The decompressor comes first: it restores transients that earlier compression flattened,
//! so the gate and compressor work on the restored dynamics (owner, 2026-11-24). Sections keep
//! their index (parameter ids and order are unchanged); [`CHAIN`] is the order they run in.
//!
//! ```text
//! ```
//!
//! Every section is a pure gain multiplier (see `crates/dynamics`). A section that is off
//! passes its input through a delay equal to its latency, so the plugin's latency does not
//! jump when one is switched; On/Off crossfades over 10 ms so a switch never clicks.
//!
//! Zero Latency (global): every look-ahead drops out and the plugin reports 0 samples. The
//! output dips to silence for 5 ms around the switch, since the delays change length.
//!
//! Auto Gain (global, see `autogain`): the output is matched to the input's loudness, after
//! the meters. A listening aid: off when printing.
//!
//! Delta (global): plays what the sections changed, the output minus the input lined up for
//! latency. With Auto Gain on too, the loudness is matched on the normal output, so the
//! difference plays at the matched level. The meters keep showing the processed signal.

mod autogain;
mod text;
mod shared;
mod ui;

use std::sync::{Arc, LazyLock};

use olm_api::{Kind, Module, ParamDesc};
use openplugin::core::EditorState;
use openplugin::prelude::*;

pub use shared::Shared;
pub use ui::make_editor;

/// One tab: its id prefix, its name, whether it starts on.
pub struct SectionInfo {
    pub key: &'static str,
    pub label: &'static str,
    /// Host index of the section's On switch; its module parameters follow it.
    pub on: usize,
    pub base: usize,
    pub count: usize,
}

pub const SECTIONS: usize = 5;
/// The order the sections run in, and their tabs from left to right (section indices).
pub const CHAIN: [usize; SECTIONS] = [3, 0, 1, 2, 4];
const KEYS: [(&str, &str, bool); SECTIONS] =
    [("gate", "Gate / Expander", false), ("comp", "Compressor", true), ("deexp", "De-Expander", false), ("decomp", "Decompressor", false), ("limit", "Limiter", true)];

pub const P_INPUT: usize = 0;
pub const P_OUTPUT: usize = 1;
pub const P_ZERO_LATENCY: usize = 2;
pub const P_AUTO_GAIN: usize = 3;
pub const P_DELTA: usize = 4;

fn modules() -> [Box<dyn Module>; SECTIONS] {
    [
        Box::new(dynamics::Gate::default()),
        Box::new(dynamics::Compressor::default()),
        Box::new(dyn_deexpand::DeExpander::default()),
        Box::new(dyn_decompress::Decompressor::default()),
        Box::new(dynamics::Limiter::default()),
    ]
}

/// Defaults that differ from a module's own: the decompressor's ceiling is off, because the
/// Limiter tab comes after it.
fn default_for(key: &str, d: &ParamDesc) -> f32 {
    match (key, d.id) {
        ("decomp", "ceilingOn") => 0.0,
        _ => d.default,
    }
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// Dynamics controls move in whole dB and whole ratio steps (owner, 2026-11-24): every dB
/// and ratio parameter with whole-number ends gets one step per unit.
fn whole_steps(d: &ParamDesc) -> u32 {
    let whole = |v: f32| (v - v.round()).abs() < 1e-6;
    match d.kind {
        Kind::Db | Kind::Ratio if d.steps == 0 && whole(d.min) && whole(d.max) => (d.max - d.min).round() as u32 + 1,
        _ => d.steps,
    }
}

fn info_for(key: &'static str, label: &'static str, d: &ParamDesc) -> ParamInfo {
    let unit = match d.kind {
        Kind::Db => Unit::Db,
        Kind::Ms => Unit::Ms,
        Kind::Hz => Unit::Hz,
        Kind::Percent => Unit::Percent,
        Kind::Ratio => Unit::Ratio,
        Kind::Plain => Unit::None,
        Kind::Toggle => Unit::Toggle,
        Kind::Choice(_) => Unit::Choice,
    };
    let mut info = ParamInfo::new(leak(format!("{key}.{}", d.id)), leak(format!("{label} {}", d.name)), d.min, d.max, default_for(key, d), unit)
        .with_steps(whole_steps(d))
        .with_group(label);
    if d.log {
        info = info.with_taper(Taper::Log);
    }
    // Value text hosts show and read, the same as the editor's.
    info = match d.kind {
        Kind::Db => info.with_text(text::db, text::parse_db),
        Kind::Ratio if key == "decomp" => info.with_text(text::upward_ratio, text::parse_upward_ratio),
        Kind::Ratio => info.with_text(text::ratio, text::parse_ratio),
        Kind::Ms => info.with_text(text::ms, text::parse_ms),
        Kind::Hz => info.with_text(text::hz, text::parse_hz),
        Kind::Plain => info.with_text(text::plain, text::parse_plain),
        _ => info,
    };
    if let Kind::Choice(labels) = d.kind {
        info = info.with_choices(labels);
    }
    info
}

static TABLE: LazyLock<(Vec<ParamInfo>, Vec<SectionInfo>)> = LazyLock::new(|| {
    let mut params = vec![
        ParamInfo::new("input", "Input Gain", -24.0, 24.0, 0.0, Unit::Db).with_steps(49).with_group("Global").with_text(text::db, text::parse_db),
        ParamInfo::new("output", "Output Gain", -24.0, 24.0, 0.0, Unit::Db).with_steps(49).with_group("Global").with_text(text::db, text::parse_db),
        ParamInfo::new("zeroLatency", "Zero Latency", 0.0, 1.0, 0.0, Unit::Toggle).with_group("Global"),
        ParamInfo::new("autoGain", "Auto Gain", 0.0, 1.0, 0.0, Unit::Toggle).with_group("Global"),
        ParamInfo::new("delta", "Delta", 0.0, 1.0, 0.0, Unit::Toggle).with_group("Global"),
    ];
    let mods = modules();
    let mut sections = Vec::new();
    for ((key, label, on), m) in KEYS.iter().zip(mods.iter()) {
        let on_index = params.len();
        params.push(ParamInfo::new(leak(format!("{key}.on")), leak(format!("{label} On")), 0.0, 1.0, f32::from(u8::from(*on)), Unit::Toggle).with_group(label));
        let base = params.len();
        for d in m.params() {
            params.push(info_for(key, label, d));
        }
        sections.push(SectionInfo { key, label, on: on_index, base, count: m.params().len() });
    }
    (params, sections)
});

pub static LAYOUT: LazyLock<&'static [SectionInfo]> = LazyLock::new(|| &TABLE.1);

pub static INFO: LazyLock<PluginInfo> = LazyLock::new(|| {
    PluginInfo::new("com.opengrid.opendynamics", "OpenDynamics", "OpenGrid", env!("CARGO_PKG_VERSION"), Category::Dynamics, &TABLE.0)
        .with_url("https://github.com/MistaMin/OpenDynamics")
        .with_editor(true)
});

static LAYOUTS: &[IoConfig] = &[
    IoConfig::STEREO,
    IoConfig { input: SpeakerLayout::Stereo, output: SpeakerLayout::Stereo, sidechain: Some(SpeakerLayout::Stereo) },
    IoConfig { input: SpeakerLayout::Stereo, output: SpeakerLayout::Stereo, sidechain: Some(SpeakerLayout::Mono) },
];

/// The most latency any setting can add, ms: three 10 ms look-aheads plus the decompressor's
/// ceiling look-ahead and true-peak reader, with room to spare.
const MAX_LATENCY_MS: f32 = 45.0;
const FADE_MS: f32 = 10.0;
/// Fade out (and back in) around a Zero Latency switch.
const DIP_MS: f32 = 5.0;
/// Delta fades in and out over this.
const DELTA_GLIDE_MS: f32 = 50.0;

struct Section {
    module: Box<dyn Module>,
    on: bool,
    /// 0 = off, 1 = on; moves toward `on` one step per sample.
    fade: f32,
    dry: [Delay; 2],
}

/// A plain ring delay for the dry path of a section.
#[derive(Default)]
struct Delay {
    buf: Vec<f32>,
    pos: usize,
}

impl Delay {
    fn alloc(&mut self, len: usize) {
        self.buf = vec![0.0; len.max(1)];
        self.pos = 0;
    }

    fn clear(&mut self) {
        self.buf.iter_mut().for_each(|v| *v = 0.0);
    }

    #[inline]
    fn push(&mut self, x: f32, d: usize) -> f32 {
        let n = self.buf.len();
        if n == 0 || d == 0 {
            return x;
        }
        self.buf[self.pos] = x;
        let r = self.buf[(self.pos + n - d.min(n - 1)) % n];
        self.pos = (self.pos + 1) % n;
        r
    }
}

pub struct OpenDynamics {
    values: Vec<f32>,
    sections: Vec<Section>,
    gain: [f32; 2],
    gain_target: [f32; 2],
    smooth: f32,
    fade_step: f32,
    sample_rate: f32,
    max_block: usize,
    dry: [Vec<f32>; 2],
    mono: Vec<f32>,
    view: [f32; 16],
    shared: Arc<Shared>,
    /// Zero Latency as asked (parameter) and as running in the modules.
    zero_want: bool,
    zero: bool,
    /// Output gain for the switch: 1 normally, 0 at the moment the modules change.
    dip: f32,
    dip_step: f32,
    autogain: autogain::AutoGain,
    /// Delta as asked and how far it is faded in (0..1, a 50 ms glide so it never clicks).
    delta: bool,
    delta_mix: f32,
    /// Samples left in which the sidechain counts as present (it carried signal recently).
    key_hold: usize,
    /// Diagnosis: samples of exact silence while the input had signal, and whether logged.
    silent_for: usize,
    silence_logged: bool,
    delta_step: f32,
    /// The input after the input gain, delayed by the latency, for the editor's spectrum.
    pre: [Vec<f32>; 2],
    pre_delay: [Delay; 2],
}

/// A panic inside the host is caught by the SDK and the block is silenced, which leaves no
/// trace. This writes the message and place to the user's log folder (once per process) so a
/// silent plugin can be diagnosed: `~/Library/Logs/OpenDynamics/panic.log` on macOS, the
/// settings folder elsewhere.
/// Appends one line to `~/Library/Logs/OpenDynamics/panic.log` (macOS) or the settings folder.
fn log_line(msg: &str) {
    if cfg!(test) {
        return;
    }
    let path = if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join("Library/Logs/OpenDynamics/panic.log"))
    } else {
        ui::theme::pref_file("panic.log")
    };
    if let Some(path) = path {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            use std::io::Write;
            let _ = writeln!(f, "{} {msg}", env!("CARGO_PKG_VERSION"));
        }
    }
}

fn log_panics() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let path = if cfg!(target_os = "macos") {
                std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join("Library/Logs/OpenDynamics/panic.log"))
            } else {
                ui::theme::pref_file("panic.log")
            };
            if let Some(path) = path {
                if let Some(dir) = path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
                    use std::io::Write;
                    let _ = writeln!(f, "{} {info}", env!("CARGO_PKG_VERSION"));
                }
            }
            prev(info);
        }));
    });
}

impl Default for OpenDynamics {
    fn default() -> Self {
        log_panics();
        let values: Vec<f32> = INFO.params.iter().map(|p| p.default).collect();
        let sections = modules().into_iter().map(|module| Section { module, on: false, fade: 0.0, dry: Default::default() }).collect();
        let mut p = Self {
            values: values.clone(),
            sections,
            gain: [1.0; 2],
            gain_target: [1.0; 2],
            smooth: 1.0,
            fade_step: 1.0,
            sample_rate: 48_000.0,
            max_block: 0,
            dry: Default::default(),
            mono: Vec::new(),
            view: [0.0; 16],
            shared: Arc::new(Shared::default()),
            zero_want: false,
            zero: false,
            dip: 1.0,
            dip_step: 1.0,
            autogain: autogain::AutoGain::new(48_000.0, false),
            delta: false,
            delta_mix: 0.0,
            key_hold: 0,
            silent_for: 0,
            silence_logged: false,
            delta_step: 1.0,
            pre: Default::default(),
            pre_delay: Default::default(),
        };
        for (i, v) in values.iter().enumerate() {
            p.set_param_index(i, *v);
        }
        for s in &mut p.sections {
            s.fade = if s.on { 1.0 } else { 0.0 };
        }
        p
    }
}

fn db_to_gain(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

impl OpenDynamics {
    fn run(&mut self, left: &mut [f32], right: &mut [f32], key: Option<[&[f32]; 2]>) {
        let n = left.len();
        if self.dip <= 0.0 && self.zero != self.zero_want {
            self.apply_zero_latency();
        }
        self.autogain.feed_raw(left, right);
        let mut peak_in = [0.0f32; 2];
        for i in 0..n {
            for c in 0..2 {
                self.gain[c] += (self.gain_target[c] - self.gain[c]) * self.smooth;
            }
            left[i] *= self.gain[0];
            right[i] *= self.gain[1];
            peak_in[0] = peak_in[0].max(left[i].abs());
            peak_in[1] = peak_in[1].max(right[i].abs());
        }
        let lat = self.total_latency();
        for i in 0..n {
            self.pre[0][i] = self.pre_delay[0].push(left[i], lat);
            self.pre[1][i] = self.pre_delay[1].push(right[i], lat);
        }
        let step = self.fade_step;
        for si in CHAIN {
            let s = &mut self.sections[si];
            let lat = s.module.latency();
            if !s.on && s.fade <= 0.0 {
                // Off: a delay as long as the section's latency, nothing else.
                for i in 0..n {
                    left[i] = s.dry[0].push(left[i], lat);
                    right[i] = s.dry[1].push(right[i], lat);
                }
                self.shared.section(si, [0.0; 2], [-200.0; 2], false);
                continue;
            }
            if s.on && s.fade <= 0.0 {
                // Coming on from off: start from clean state, not from what it held before.
                s.module.reset();
            }
            self.dry[0][..n].copy_from_slice(left);
            self.dry[1][..n].copy_from_slice(right);
            match key {
                Some([kl, kr]) => s.module.process_keyed(left, right, kl, kr),
                None => s.module.process_keyed(left, right, &self.dry[0][..n], &self.dry[1][..n]),
            }
            let target = if s.on { 1.0 } else { 0.0 };
            for i in 0..n {
                let dl = s.dry[0].push(self.dry[0][i], lat);
                let dr = s.dry[1].push(self.dry[1][i], lat);
                if s.fade != target {
                    s.fade = if s.fade < target { (s.fade + step).min(1.0) } else { (s.fade - step).max(0.0) };
                }
                if s.fade < 1.0 {
                    left[i] = dl + (left[i] - dl) * s.fade;
                    right[i] = dr + (right[i] - dr) * s.fade;
                }
            }
            let got = s.module.view(&mut self.view);
            let lift = s.module.lift_db();
            let red = s.module.meters().reduction_db;
            // Signed gain per channel from the read-out; the modules that only lift or only cut
            // agree on slot 0..2 = gain, 2..4 = detector level.
            let (g, d) = if got >= 4 { ([self.view[0], self.view[1]], [self.view[2], self.view[3]]) } else { ([lift - red; 2], [-200.0; 2]) };
            self.shared.section(si, g, d, true);
        }
        self.shared.push_taps([&self.pre[0][..n], &self.pre[1][..n]], [left, right]);
        let mut peak_out = [0.0f32; 2];
        let out = db_to_gain(self.values[P_OUTPUT]);
        let dip_to = if self.zero == self.zero_want { 1.0 } else { 0.0 };
        for i in 0..n {
            if self.dip != dip_to {
                self.dip = if self.dip < dip_to { (self.dip + self.dip_step).min(1.0) } else { (self.dip - self.dip_step).max(0.0) };
            }
            let g = out * self.dip;
            left[i] *= g;
            right[i] *= g;
            peak_out[0] = peak_out[0].max(left[i].abs());
            peak_out[1] = peak_out[1].max(right[i].abs());
        }
        self.shared.peaks(peak_in, peak_out);
        self.watch_silence(n, peak_in, peak_out);
        self.autogain.feed_out(left, right);
        let delta_to = if self.delta { 1.0 } else { 0.0 };
        if self.delta_mix != 0.0 || delta_to != 0.0 {
            for i in 0..n {
                if self.delta_mix != delta_to {
                    self.delta_mix = if self.delta_mix < delta_to { (self.delta_mix + self.delta_step).min(1.0) } else { (self.delta_mix - self.delta_step).max(0.0) };
                }
                // The input copy is the same level as the sections saw, so scale it like the output.
                let m = self.delta_mix * out * self.dip;
                left[i] -= m * self.pre[0][i];
                right[i] -= m * self.pre[1][i];
            }
        }
        self.autogain.apply(left, right);
        self.shared.set_auto_gain(self.autogain.gain_db());
    }

    /// Diagnosis (QA 2026-11-24, the VST3 in a host went silent for good, not reproduced on demand):
    /// when the input has signal but the output has been exactly zero for a second, the state
    /// that can silence the plugin is written once to the panic log's folder.
    fn watch_silence(&mut self, n: usize, peak_in: [f32; 2], peak_out: [f32; 2]) {
        if peak_in[0].max(peak_in[1]) > 1e-4 && peak_out[0].max(peak_out[1]) == 0.0 {
            self.silent_for += n;
        } else {
            self.silent_for = 0;
        }
        if self.silent_for > self.sample_rate as usize && !self.silence_logged {
            self.silence_logged = true;
            let on: Vec<bool> = self.sections.iter().map(|s| s.on).collect();
            let fade: Vec<f32> = self.sections.iter().map(|s| s.fade).collect();
            let msg = format!(
                "silent output with input: on {on:?} fade {fade:?} delta {} mix {} dip {} zero {} want {} latency {} in {} out {} gain {:?} autogain {} sr {} block {}",
                self.delta, self.delta_mix, self.dip, self.zero, self.zero_want, self.total_latency(), self.values[P_INPUT], self.values[P_OUTPUT], self.gain, self.autogain.gain_db(), self.sample_rate, self.max_block
            );
            log_line(&msg);
        }
    }

    /// Switches every module to the asked Zero Latency mode and starts them clean (their
    /// delays change length, so what they held no longer lines up).
    fn apply_zero_latency(&mut self) {
        self.zero = self.zero_want;
        for s in &mut self.sections {
            s.module.set_zero_latency(self.zero);
            s.module.reset();
            s.dry[0].clear();
            s.dry[1].clear();
        }
    }

    fn total_latency(&self) -> usize {
        self.sections.iter().map(|s| s.module.latency()).sum()
    }
}

impl Plugin for OpenDynamics {
    fn state_is_parameters_only(&self) -> bool {
        true
    }

    fn info(&self) -> &'static PluginInfo {
        &INFO
    }

    fn prepare(&mut self, sample_rate: f32, max_block: usize, _channels: usize) {
        self.sample_rate = sample_rate.max(1.0);
        self.max_block = max_block.max(1);
        let dry_len = (MAX_LATENCY_MS * 1e-3 * self.sample_rate).ceil() as usize + 2;
        for s in &mut self.sections {
            s.module.prepare(self.sample_rate, self.max_block);
            s.dry[0].alloc(dry_len);
            s.dry[1].alloc(dry_len);
            s.fade = if s.on { 1.0 } else { 0.0 };
        }
        self.dry = [vec![0.0; self.max_block], vec![0.0; self.max_block]];
        self.pre = [vec![0.0; self.max_block], vec![0.0; self.max_block]];
        self.pre_delay[0].alloc(dry_len);
        self.pre_delay[1].alloc(dry_len);
        self.mono = vec![0.0; self.max_block];
        self.smooth = 1.0 - (-1.0 / (0.005 * self.sample_rate)).exp();
        self.fade_step = 1.0 / (FADE_MS * 1e-3 * self.sample_rate);
        self.dip_step = 1.0 / (DIP_MS * 1e-3 * self.sample_rate);
        self.delta_step = 1.0 / (DELTA_GLIDE_MS * 1e-3 * self.sample_rate);
        self.delta_mix = if self.delta { 1.0 } else { 0.0 };
        if self.zero != self.zero_want {
            self.apply_zero_latency();
        }
        self.dip = 1.0;
        self.autogain = autogain::AutoGain::new(self.sample_rate, self.values[P_AUTO_GAIN] >= 0.5);
        self.gain = self.gain_target;
        self.shared.set_sample_rate(self.sample_rate);
    }

    fn reset(&mut self) {
        for s in &mut self.sections {
            s.module.reset();
            s.dry[0].clear();
            s.dry[1].clear();
            s.fade = if s.on { 1.0 } else { 0.0 };
        }
        self.pre_delay[0].clear();
        self.pre_delay[1].clear();
        self.gain = self.gain_target;
    }

    fn set_param(&mut self, id: &str, value: f32) -> bool {
        match INFO.params.iter().position(|p| p.id == id) {
            Some(i) => self.set_param_index(i, value),
            None => false,
        }
    }

    fn set_param_index(&mut self, index: usize, value: f32) -> bool {
        let Some(info) = INFO.params.get(index) else { return false };
        let value = info.clamp(value);
        self.values[index] = value;
        if index == P_INPUT {
            self.gain_target = [db_to_gain(value); 2];
            return true;
        }
        if index == P_OUTPUT {
            return true;
        }
        if index == P_DELTA {
            self.delta = value >= 0.5;
            return true;
        }
        if index == P_AUTO_GAIN {
            self.autogain.on = value >= 0.5;
            return true;
        }
        if index == P_ZERO_LATENCY {
            self.zero_want = value >= 0.5;
            if self.max_block == 0 {
                // Not running yet: switch now, nothing to fade.
                self.apply_zero_latency();
            }
            return true;
        }
        for (s, l) in self.sections.iter_mut().zip(LAYOUT.iter()) {
            if index == l.on {
                s.on = value >= 0.5;
                return true;
            }
            if (l.base..l.base + l.count).contains(&index) {
                s.module.set_param(index - l.base, value);
                return true;
            }
        }
        false
    }

    fn param(&self, id: &str) -> Option<f32> {
        INFO.params.iter().position(|p| p.id == id).map(|i| self.values[i])
    }

    fn set_offline(&mut self, offline: bool) {
        for s in &mut self.sections {
            s.module.set_offline(offline);
        }
    }

    fn supported_layouts(&self) -> &'static [IoConfig] {
        LAYOUTS
    }

    fn process(&mut self, io: &mut [&mut [f32]], frames: usize) {
        self.process_buses(io, None, frames);
    }

    fn process_buses(&mut self, io: &mut [&mut [f32]], sidechain: Option<&[&[f32]]>, frames: usize) {
        if self.max_block == 0 {
            return;
        }
        let key: Option<[&[f32]; 2]> = sidechain.and_then(|side| match side {
            [kl, kr, ..] if kl.len() >= frames && kr.len() >= frames => Some([&kl[..frames], &kr[..frames]]),
            [k] if k.len() >= frames => Some([&k[..frames], &k[..frames]]),
            _ => None,
        });
        // KEY IN means a key that carries signal: hosts hand over the sidechain bus even when
        // nothing is routed to it (a host's sidechain pins with nothing connected), and that silence is not a key. Held 1 s.
        let heard = key.is_some_and(|[a, b]| a.iter().chain(b.iter()).any(|v| v.abs() > 1e-5));
        self.key_hold = if heard { self.sample_rate as usize } else { self.key_hold.saturating_sub(frames) };
        self.shared.set_key_present(self.key_hold > 0);
        let mut mono = std::mem::take(&mut self.mono);
        let Some((first, rest)) = io.split_first_mut() else { return };
        let mut start = 0;
        while start < frames {
            let n = (frames - start).min(self.max_block);
            let k = key.map(|[a, b]| [&a[start..start + n], &b[start..start + n]]);
            let l = &mut first[start..start + n];
            match rest.first_mut() {
                Some(r) => self.run(l, &mut r[start..start + n], k),
                None => {
                    // Mono: run as two equal channels and keep the left.
                    let r = &mut mono[..n];
                    r.copy_from_slice(l);
                    self.run(l, r, k);
                }
            }
            start += n;
        }
        self.mono = mono;
    }

    fn latency(&self) -> usize {
        self.total_latency()
    }

    fn max_latency(&self) -> Option<usize> {
        Some((MAX_LATENCY_MS * 1e-3 * self.sample_rate).ceil() as usize)
    }

    fn meters(&self) -> MeterFrame {
        let mut m = MeterFrame::SILENT;
        m.reduction_db = self.sections.iter().filter(|s| s.on).map(|s| s.module.meters().reduction_db).fold(0.0, f32::max);
        m
    }

    fn editor_state(&self) -> Option<EditorState> {
        Some(self.shared.clone())
    }

    fn note_ports(&self) -> NotePorts {
        NotePorts { input: false, output: false }
    }

    /// Fixed from the first release on: LV2 sessions find the plugin by it.
    fn lv2_uri(&self) -> Option<&'static str> {
        Some("https://opengrid.studio/plugins/opendynamics")
    }

    /// Fixed from the first release on: AAX sessions find the plugin by it.
    fn aax_ids(&self) -> Option<openplugin::core::AaxIds> {
        Some(openplugin::core::AaxIds { manufacturer: *b"OpGd", product: *b"ODyn", mono: *b"ODyM", stereo: *b"ODyS" })
    }
}

openplugin::targets::export_clap!(OpenDynamics, editor = make_editor);
openplugin::targets::export_lv2!(OpenDynamics, editor = make_editor);
openplugin::targets::export_shell!(OpenDynamics, editor = make_editor);

#[cfg(test)]
mod tests {
    use super::*;

    fn block(p: &mut OpenDynamics, l: &mut [f32], r: &mut [f32]) {
        let n = l.len();
        let mut io: [&mut [f32]; 2] = [l, r];
        p.process(&mut io, n);
    }

    #[test]
    fn params_are_unique_and_every_section_is_there() {
        let ids: std::collections::HashSet<_> = INFO.params.iter().map(|p| p.id).collect();
        assert_eq!(ids.len(), INFO.params.len());
        assert_eq!(LAYOUT.len(), SECTIONS);
        let last = LAYOUT.last().unwrap();
        assert_eq!(last.base + last.count, INFO.params.len());
    }

    /// Owner rule (2026-11-24): every dB and ratio control moves in whole steps, defaults
    /// included, so a host or a typed value lands on a whole number.
    #[test]
    fn dynamics_controls_move_in_whole_steps() {
        for p in INFO.params.iter().filter(|p| matches!(p.unit, Unit::Db | Unit::Ratio)) {
            assert!(p.steps > 1, "{} is not stepped", p.id);
            assert_eq!(p.default, p.default.round(), "{} default {}", p.id, p.default);
            for v in [p.min + 0.37, (p.min + p.max) * 0.5 + 0.21, p.max - 0.4] {
                let c = p.clamp(v);
                assert!((c - c.round()).abs() < 1e-4, "{}: {v} -> {c}", p.id);
            }
        }
    }

    /// The chain: decompressor first (restores transients before the gate and compressor),
    /// limiter last, every section once.
    #[test]
    fn chain_order() {
        let mut seen = CHAIN;
        seen.sort_unstable();
        assert_eq!(seen, [0, 1, 2, 3, 4]);
        assert_eq!(LAYOUT[CHAIN[0]].key, "decomp");
        assert_eq!(LAYOUT[CHAIN[1]].key, "gate");
        assert_eq!(LAYOUT[CHAIN[SECTIONS - 1]].key, "limit");
    }

    /// What hosts show (automation lanes, tooltips) follows the display rules and reads back:
    /// every numeric parameter at min, middle and max prints at most three digits per number,
    /// whole dB and ratios, no "0.x" except the key boost's Q (an EQ control), no "-0", and the
    /// printed text parses to the same value within its printed precision.
    #[test]
    fn host_text_follows_the_display_rules() {
        for p in INFO.params.iter().filter(|p| !matches!(p.unit, Unit::Toggle | Unit::Choice)) {
            for n in [0.0f32, 0.5, 1.0] {
                let v = p.from_normalized(n);
                let t = p.format(v);
                for num in t.split(|c: char| !(c.is_ascii_digit() || c == '.')).filter(|x| !x.is_empty()) {
                    assert!(num.chars().filter(|c| c.is_ascii_digit()).count() <= 3, "{}: {t:?}", p.id);
                    assert!(!num.starts_with("0.") || p.id.ends_with("keyBoostQ"), "{}: {t:?}", p.id);
                }
                assert!(!t.starts_with("-0"), "{}: {t:?}", p.id);
                if matches!(p.unit, Unit::Db | Unit::Ratio) {
                    assert!(!t.contains('.'), "{}: {t:?}", p.id);
                }
                let back = p.parse(&t).unwrap_or_else(|| panic!("{}: cannot read back {t:?}", p.id));
                let tol = (v.abs() * 0.006).max(0.006);
                assert!((back - p.clamp(v)).abs() <= tol.max(0.5 * (p.max - p.min) / p.steps.max(1) as f32), "{}: {v} -> {t:?} -> {back}", p.id);
            }
        }
    }

    /// Writes docs/PARAMETERS.md for the manual: every parameter by section, as hosts show it.
    #[test]
    #[ignore]
    fn dump_manual_params() {
        let mut out = String::from("# OpenDynamics parameter reference\n\nGenerated from the plugin (`cargo test --release --lib dump_manual_params -- --ignored`). Every parameter, by section (the order hosts list them), with its range and default as hosts and the editor show them.\n");
        let mut group = "";
        for p in INFO.params.iter() {
            if p.group != group {
                group = p.group;
                out += &format!("\n## {group}\n\n| Parameter | Range or choices | Default |\n|---|---|---|\n");
            }
            let (range, default) = match p.unit {
                Unit::Toggle => ("Off, On".to_string(), if p.default >= 0.5 { "On" } else { "Off" }.to_string()),
                Unit::Choice => (p.choices.join(", "), p.choices.get(p.default as usize).copied().unwrap_or("").to_string()),
                _ => (format!("{} to {}", p.format(p.min), p.format(p.max)), p.format(p.default)),
            };
            out += &format!("| {} | {range} | {default} |\n", p.name);
        }
        std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/PARAMETERS.md"), out).expect("write");
    }

    #[test]
    fn all_off_is_a_wire() {
        let mut p = OpenDynamics::default();
        for l in LAYOUT.iter() {
            p.set_param_index(l.on, 0.0);
        }
        p.prepare(48_000.0, 256, 2);
        let n = 2048;
        let src: Vec<f32> = (0..n).map(|i| ((i as f32) * 0.1).sin() * 0.5).collect();
        let (mut l, mut r) = (src.clone(), src.clone());
        block(&mut p, &mut l, &mut r);
        let lat = p.latency();
        assert!(lat > 0 && lat < 1000, "{lat}");
        assert_eq!(&l[lat..], &src[..n - lat]);
    }

    #[test]
    fn zero_latency_reports_and_is_zero() {
        let mut p = OpenDynamics::default();
        for l in LAYOUT.iter() {
            p.set_param_index(l.on, 1.0);
        }
        p.set_param_index(LAYOUT[3].base + dyn_decompress::index::CEILING_ON, 1.0);
        p.prepare(48_000.0, 256, 2);
        assert!(p.latency() > 0);
        p.set_param("zeroLatency", 1.0);
        // The switch waits for the 5 ms dip.
        let mut l = vec![0.0f32; 256];
        let mut r = l.clone();
        for _ in 0..4 {
            block(&mut p, &mut l, &mut r);
        }
        assert_eq!(p.latency(), 0);
        // An impulse comes out on its own sample.
        let mut l = vec![0.0f32; 256];
        l[10] = 0.25;
        let mut r = l.clone();
        for _ in 0..8 {
            block(&mut p, &mut vec![0.0; 256], &mut vec![0.0; 256]);
        }
        block(&mut p, &mut l, &mut r);
        assert!(l[10].abs() > 0.0 && l[..10].iter().all(|v| *v == 0.0), "{:?}", &l[..12]);
        // And back.
        p.set_param("zeroLatency", 0.0);
        for _ in 0..4 {
            block(&mut p, &mut vec![0.0; 256], &mut vec![0.0; 256]);
        }
        assert!(p.latency() > 0);
    }

    #[test]
    fn zero_latency_switch_does_not_click() {
        let mut p = OpenDynamics::default();
        p.prepare(48_000.0, 64, 2);
        let mut prev = 0.0f32;
        let mut worst = 0.0f32;
        for b in 0..200 {
            if b == 100 {
                p.set_param("zeroLatency", 1.0);
            }
            let mut l: Vec<f32> = (0..64).map(|i| ((b * 64 + i) as f32 * 0.02).sin() * 0.5).collect();
            let mut r = l.clone();
            block(&mut p, &mut l, &mut r);
            for v in l {
                worst = worst.max((v - prev).abs());
                prev = v;
            }
        }
        assert_eq!(p.latency(), 0);
        assert!(worst < 0.02, "{worst}");
    }

    /// The spectrum's input copy lines up with the output: with every section off they match
    /// sample for sample, so the editor shows no change.
    #[test]
    fn spectrum_taps_line_up() {
        let mut p = OpenDynamics::default();
        for l in LAYOUT.iter() {
            p.set_param_index(l.on, 0.0);
        }
        p.prepare(48_000.0, 512, 2);
        for b in 0..8 {
            let mut l: Vec<f32> = (0..512).map(|i| ((b * 512 + i) as f32 * 0.03).sin() * 0.5).collect();
            let mut r = l.clone();
            block(&mut p, &mut l, &mut r);
        }
        let (mut pre, mut post) = (vec![0.0f32; 2048], vec![0.0f32; 2048]);
        p.shared.read_tap(0, &mut pre);
        p.shared.read_tap(2, &mut post);
        assert!(post.iter().any(|v| *v != 0.0));
        assert_eq!(pre, post);
    }

    /// Delta plays what the sections changed: the normal output minus Delta's output is the
    /// input delayed by the latency, sample for sample; with every section off it is silence.
    #[test]
    fn delta_is_the_difference() {
        let run = |delta: bool, all_off: bool| {
            let mut p = OpenDynamics::default();
            if all_off {
                for l in LAYOUT.iter() {
                    p.set_param_index(l.on, 0.0);
                }
            }
            p.set_param("comp.threshold", -30.0);
            p.set_param("delta", if delta { 1.0 } else { 0.0 });
            p.prepare(48_000.0, 512, 2);
            let src: Vec<f32> = (0..48_000).map(|i| ((i as f32) * 0.031).sin() * 0.5 * (1.0 + (i as f32 * 0.0004).sin())).collect();
            let (mut l, mut r) = (src.clone(), src.clone());
            for (a, b) in l.chunks_mut(512).zip(r.chunks_mut(512)) {
                block(&mut p, a, b);
            }
            (l, p.latency(), src)
        };
        let (silent, ..) = run(true, true);
        assert!(silent.iter().all(|v| *v == 0.0), "all off + delta is not silent");
        let (normal, lat, src) = run(false, false);
        let (delta, lat2, _) = run(true, false);
        assert_eq!(lat, lat2);
        assert!(delta.iter().any(|v| v.abs() > 1e-3), "the compressor changed nothing");
        for i in lat..normal.len() {
            assert!((normal[i] - delta[i] - src[i - lat]).abs() < 1e-5, "sample {i}");
        }
    }

    /// DSP cost: every section on, 48 kHz, 256-sample blocks, 10 s of noise; prints the share
    /// of one core. `cargo test --release --lib dsp_cost -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn dsp_cost() {
        for zero in [false, true] {
            let mut p = OpenDynamics::default();
            for l in LAYOUT.iter() {
                p.set_param_index(l.on, 1.0);
            }
            p.set_param("zeroLatency", if zero { 1.0 } else { 0.0 });
            p.prepare(48_000.0, 256, 2);
            let mut seed = 1u32;
            let src: Vec<f32> = (0..256)
                .map(|_| {
                    seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    0.6 * ((seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5)
                })
                .collect();
            let blocks = 48_000 * 10 / 256;
            let t0 = std::time::Instant::now();
            for _ in 0..blocks {
                let (mut l, mut r) = (src.clone(), src.clone());
                block(&mut p, &mut l, &mut r);
            }
            let s = t0.elapsed().as_secs_f64();
            println!("all sections on, zero latency {zero}: {:.2} % of one core ({:.3} s for 10 s)", 100.0 * s / 10.0, s);
        }
    }

    /// Host QA 2026-11-24: Gate Range at its maximum left the whole plugin silent for
    /// good. Every section's extremes must keep the output finite and the plugin alive.
    #[test]
    fn extremes_never_kill_the_output() {
        let mut p = OpenDynamics::default();
        for l in LAYOUT.iter() {
            p.set_param_index(l.on, 1.0);
        }
        p.prepare(48_000.0, 512, 2);
        let mut seed = 5u32;
        let mut play = |p: &mut OpenDynamics, n: usize| -> f32 {
            let mut peak = 0.0f32;
            for b in 0..n {
                let mut l: Vec<f32> = (0..512)
                    .map(|i| {
                        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                        let t = (b * 512 + i) as f32;
                        if (b / 20) % 2 == 0 { 0.5 * (t * 0.01 * (1.0 + t * 1e-5)).sin() } else { 0.3 * ((seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5) }
                    })
                    .collect();
                let mut r = l.clone();
                block(p, &mut l, &mut r);
                for v in l.iter().chain(&r) {
                    assert!(v.is_finite());
                    peak = peak.max(v.abs());
                }
            }
            peak
        };
        assert!(play(&mut p, 40) > 0.01);
        let n = INFO.params.len();
        for i in 0..n {
            let info = &INFO.params[i];
            if LAYOUT.iter().any(|l| l.on == i) {
                continue;
            }
            for v in [info.min, info.max] {
                p.set_param_index(i, v);
                play(&mut p, 20);
            }
            p.set_param_index(i, info.default);
            let alive = play(&mut p, 120);
            assert!(alive > 0.01, "after {} extremes the output stayed silent ({alive})", info.id);
        }
    }

    /// KEY IN: a silent sidechain bus is not a key; signal on it is, for a second after.
    #[test]
    fn key_in_needs_signal() {
        let mut p = OpenDynamics::default();
        p.prepare(48_000.0, 512, 2);
        let (mut l, mut r) = (vec![0.1f32; 512], vec![0.1f32; 512]);
        let silent = vec![0.0f32; 512];
        let tone: Vec<f32> = (0..512).map(|i| (i as f32 * 0.1).sin() * 0.2).collect();
        let mut io: [&mut [f32]; 2] = [&mut l, &mut r];
        p.process_buses(&mut io, Some(&[&silent[..], &silent[..]]), 512);
        assert!(!p.shared.key_present());
        p.process_buses(&mut io, Some(&[&tone[..], &tone[..]]), 512);
        assert!(p.shared.key_present());
        for _ in 0..200 {
            p.process_buses(&mut io, Some(&[&silent[..], &silent[..]]), 512);
        }
        assert!(!p.shared.key_present());
    }

    /// Profile (wiki optimisation gate, step 1): each section alone and all together, 48 kHz,
    /// 64-sample blocks (1.333 ms budget), 20 s of a loud noise-and-tone mix with gain reduction.
    /// Prints mean us per block, % of the block budget, and % of one core.
    /// `cargo test --release --lib dsp_profile -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn dsp_profile() {
        let configs: Vec<(String, Vec<usize>)> = std::iter::once(("all off".to_string(), vec![]))
            .chain(LAYOUT.iter().enumerate().map(|(i, l)| (l.key.to_string(), vec![i])))
            .chain(std::iter::once(("all on".to_string(), (0..SECTIONS).collect())))
            .collect();
        let n = 64;
        let secs = 20;
        let mut seed = 3u32;
        let src: Vec<f32> = (0..48_000)
            .map(|i| {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                0.4 * (i as f32 * 0.02).sin() + 0.3 * ((seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5)
            })
            .collect();
        for round in 0..2 {
            for (name, on) in &configs {
                let mut p = OpenDynamics::default();
                for (i, l) in LAYOUT.iter().enumerate() {
                    p.set_param_index(l.on, if on.contains(&i) { 1.0 } else { 0.0 });
                }
                p.prepare(48_000.0, n, 2);
                let blocks = 48_000 * secs / n;
                let (mut l, mut r) = (vec![0.0f32; n], vec![0.0f32; n]);
                // warm-up excluded
                for b in 0..200 {
                    l.copy_from_slice(&src[(b * n) % 47_936..][..n]);
                    r.copy_from_slice(&l);
                    block(&mut p, &mut l, &mut r);
                }
                let t0 = std::time::Instant::now();
                for b in 0..blocks {
                    l.copy_from_slice(&src[(b * n) % 47_936..][..n]);
                    r.copy_from_slice(&l);
                    block(&mut p, &mut l, &mut r);
                }
                let s = t0.elapsed().as_secs_f64();
                let us = s / blocks as f64 * 1e6;
                if round == 1 {
                    println!("{name:8} {us:7.2} us/block  {:5.2} % of budget  {:5.2} % of a core", us / 1333.3 * 100.0, s / secs as f64 * 100.0);
                }
            }
        }
    }

    #[test]
    fn switching_a_section_does_not_click() {
        let mut p = OpenDynamics::default();
        p.prepare(48_000.0, 64, 2);
        // A loud tone, compressed hard; then the compressor goes off mid-stream.
        p.set_param("comp.threshold", -40.0);
        let mut prev = 0.0f32;
        let mut worst = 0.0f32;
        for b in 0..200 {
            if b == 100 {
                p.set_param("comp.on", 0.0);
            }
            let mut l: Vec<f32> = (0..64).map(|i| ((b * 64 + i) as f32 * 0.02).sin() * 0.5).collect();
            let mut r = l.clone();
            block(&mut p, &mut l, &mut r);
            for v in l {
                worst = worst.max((v - prev).abs());
                prev = v;
            }
        }
        // A 0.5-peak sine at this rate moves at most 0.01 a sample; a click would jump.
        assert!(worst < 0.02, "{worst}");
    }
}
