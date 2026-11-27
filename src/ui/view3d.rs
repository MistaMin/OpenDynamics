//! The 3D view: a spectrogram terrain over the full band, the history small under it.
//!
//! Frequency runs across (20 Hz to 20 kHz, the front row spans the whole width), time recedes
//! up and back, and level is height, after OpenEQ's 3D spectrogram. Each row is the input's
//! spectrum with the output's inside it: the body is what came out, red on top is what the
//! sections took away, green what they added. Rows are opaque and drawn back to front, so near
//! ridges hide the ground behind them.
//!
//! Cost (plugin-gui-performance): every fill is a column fill (`kit::area`, `kit::between`),
//! the body is one opaque fill that both shades and hides, and only every other stored row is
//! drawn, chosen by the row (not its age) so the surface scrolls instead of shimmering.

use std::collections::VecDeque;

use openplugin::dri::goodlookindri::color::Color as C;
use openplugin::dri::goodlookindri::scene::{Point, Scene};

use super::{analyser, kit, lifts, raster, DynEditor, DISPLAY, GR_DB, HI, HISTORY_S, LANE, LO, TRACE_LEN};

/// Bands across the full band.
pub const BANDS3: usize = 96;
/// Samples per terrain spectrum (170 ms at 48 kHz): 5.9 Hz bins, so the bass bands are not
/// all one bin (flat steps with hard corners).
pub const SPEC3_SIZE: usize = 8192;
/// Points drawn per band: the ridges are Catmull-Rom splines through the bands.
const SUB: usize = 3;
pub const F3_LO: f32 = 20.0;
pub const F3_HI: f32 = 20_000.0;
/// Rows per second and rows kept (3 s of depth), by elapsed time, whatever the host's tick.
pub const ROW_HZ: f32 = 10.0;
pub const ROWS3: usize = 30;
const SECONDS3: usize = 3;
/// Level range drawn as height, dB on the analyser's scale (a full-scale sine reads 0).
const DB3_LO: f32 = -90.0;
const DB3_HI: f32 = -12.0;
/// Where the terrain stands in the display.
const FRONT_GAP: f32 = 24.0;
const BACK_AT: f32 = 0.24;
const FRONT_H: f32 = 0.42;
const BACK_SCALE: f32 = 0.5;
/// Front-edge frequency labels.
const F_LABELS: [(f32, &str); 9] = [(50.0, "50"), (100.0, "100"), (200.0, "200"), (500.0, "500"), (1000.0, "1k"), (2000.0, "2k"), (5000.0, "5k"), (10_000.0, "10k"), (20_000.0, "20k")];

/// The 3D view's ground: dark in dark themes, light in light ones. The energy colours run
/// from the ground's own tone toward strong colour, so loud always stands out.
#[derive(Clone, Copy)]
pub struct Well {
    pub bg: C,
    pub grid: C,
    pub text: C,
    pub ridge: C,
    heat: [(f32, u32); 5],
}

const DARK_WELL: Well = Well {
    bg: C(0xff0a_0e14),
    grid: C(0xff26_2e38),
    text: C(0xff8a_949b),
    ridge: C(0xffff_ffff),
    // Deep blue, blue, cyan, ice, near white.
    heat: [(0.0, 0x14_286e), (0.35, 0x1f_6fd0), (0.65, 0x2e_c8e0), (0.85, 0xb4_f0f5), (1.0, 0xfa_faf0)],
};

const LIGHT_WELL: Well = Well {
    bg: C(0xfff4_f6f9),
    grid: C(0xffcf_d6df),
    text: C(0xff5a_6470),
    ridge: C(0xff10_1820),
    // Pale blue, sky, blue, deep blue, navy.
    heat: [(0.0, 0xd6_e4f2), (0.35, 0x8c_b8e6), (0.65, 0x2f_7fd6), (0.85, 0x1c_4fa8), (1.0, 0x0c_2460)],
};

/// The well for a theme: light when the theme's plots are light, then on the theme's own
/// plot colour (so it sits in a cream theme as well as a white one).
pub fn well(t: &super::theme::Theme) -> Well {
    let c = t.plot_bg.0;
    let luma = 0.2126 * ((c >> 16) & 255) as f32 + 0.7152 * ((c >> 8) & 255) as f32 + 0.0722 * (c & 255) as f32;
    if luma > 128.0 { Well { bg: t.plot_bg, ..LIGHT_WELL } } else { DARK_WELL }
}

/// Gain reduction is always this one red, and lift this one green, whatever the level or
/// depth: the energy colours never use red or green, so the two can't be confused.
const GR_COLOUR: C = C(0xffe8_3a3a);
const LIFT_COLOUR: C = C(0xff2f_b36a);
/// Energy steps: a row's body is drawn in runs of one step.
const HEAT_STEPS: f32 = 12.0;

impl Well {
    /// The energy colour for a level 0..1.
    fn heat(&self, v: f32) -> C {
        let v = v.clamp(0.0, 1.0);
        let i = self.heat.iter().position(|(at, _)| *at >= v).unwrap_or(self.heat.len() - 1).max(1);
        let ((a, ca), (b, cb)) = (self.heat[i - 1], self.heat[i]);
        let f = if b > a { (v - a) / (b - a) } else { 1.0 };
        C(0xff00_0000 | raster::blend(ca, C(cb), f))
    }
}

/// One stored row: input and output levels per band, 0..1 of the height.
pub type Row3 = ([f32; BANDS3], [f32; BANDS3]);

pub struct Terrain {
    pub spec: [analyser::Spectrum; 2],
    pub rows: VecDeque<Row3>,
    /// Rows written so far: picks which rows are drawn, so the choice scrolls with them.
    pub written: u64,
    /// Time since the last row, s.
    acc: f32,
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            spec: std::array::from_fn(|_| analyser::Spectrum::new(SPEC3_SIZE, BANDS3, F3_LO, F3_HI).instant()),
            rows: VecDeque::from(vec![([0.0; BANDS3], [0.0; BANDS3]); ROWS3]),
            written: 0,
            acc: 0.0,
        }
    }
}

fn norm(db: f32) -> f32 {
    ((db - DB3_LO) / (DB3_HI - DB3_LO)).clamp(0.0, 1.0)
}

/// An opaque colour `t` of the way from `a` to `b`.
fn mix(a: C, b: C, t: f32) -> C {
    C(0xff00_0000 | raster::blend(a.0 & 0x00ff_ffff, b, t))
}

/// Row at depth `t` (0 front, 1 back): left, width, base y, height.
fn row_geom(t: f32) -> (f32, f32, f32, f32) {
    let depth = 1.0 - (1.0 - t) * (1.0 - t);
    let scale = 1.0 - (1.0 - BACK_SCALE) * depth;
    let front = DISPLAY.1 + DISPLAY.3 - FRONT_GAP;
    let back = DISPLAY.1 + DISPLAY.3 * BACK_AT;
    let w = DISPLAY.2 * scale;
    (DISPLAY.0 + (DISPLAY.2 - w) * 0.5, w, front + (back - front) * depth, DISPLAY.3 * FRONT_H * scale)
}

fn x_of(f: f32) -> f32 {
    DISPLAY.0 + DISPLAY.2 * (f / F3_LO).ln() / (F3_HI / F3_LO).ln()
}

/// Catmull-Rom through the band values, `SUB` points per band, kept within 0..1.
fn spline(v: &[f32; BANDS3]) -> Vec<f32> {
    let at = |k: isize| v[k.clamp(0, BANDS3 as isize - 1) as usize];
    let mut out = Vec::with_capacity((BANDS3 - 1) * SUB + 1);
    for k in 0..BANDS3 as isize - 1 {
        let (p0, p1, p2, p3) = (at(k - 1), at(k), at(k + 1), at(k + 2));
        for j in 0..SUB {
            let t = j as f32 / SUB as f32;
            let (t2, t3) = (t * t, t * t * t);
            let y = 0.5 * (2.0 * p1 + (p2 - p0) * t + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 + (3.0 * p1 - p0 - 3.0 * p2 + p3) * t3);
            out.push(y.clamp(0.0, 1.0));
        }
    }
    out.push(v[BANDS3 - 1]);
    out
}

impl Terrain {
    /// Takes the aligned input and output taps (L, R each) every editor tick; adds a row each
    /// 1 / `ROW_HZ` s. True when a row was added (the terrain then needs drawing again).
    pub fn feed(&mut self, taps: &[Vec<f32>; 4], sr: f32, dt: f32) -> bool {
        self.acc += dt.max(0.0);
        if self.acc < 1.0 / ROW_HZ {
            return false;
        }
        self.acc = (self.acc - 1.0 / ROW_HZ).min(1.0 / ROW_HZ);
        let [a, b, c, d] = taps;
        self.spec[0].update(a, Some(b), sr, dt);
        self.spec[1].update(c, Some(d), sr, dt);
        // Gently smoothed across bands (1-4-6-4-1), the way the ear groups them.
        let smooth = |db: &[f32]| -> [f32; BANDS3] {
            std::array::from_fn(|k| {
                let at = |d: isize| db[(k as isize + d).clamp(0, BANDS3 as isize - 1) as usize];
                norm((at(-2) + 4.0 * at(-1) + 6.0 * at(0) + 4.0 * at(1) + at(2)) / 16.0)
            })
        };
        let row = (smooth(&self.spec[0].db), smooth(&self.spec[1].db));
        self.rows.pop_front();
        self.rows.push_back(row);
        self.written += 1;
        true
    }
}

/// The terrain drawn once into an image of the display at device resolution, redrawn only
/// when a row arrives or the theme or scale changes; every frame then just copies it. The
/// terrain costs tens of ms to draw at 2x and changes 10 times a second, the editor ticks 30.
#[derive(Default)]
pub struct TerrainImage {
    made_for: Option<(u64, u32, usize)>,
    fb: Option<goodlookindri_shell::raster::Framebuffer>,
    pub img: Option<std::sync::Arc<openplugin::dri::goodlookindri::scene::Image>>,
}

impl DynEditor {
    /// Brings the terrain image up to date for this frame (called from `render`).
    pub(super) fn update_terrain_image(&mut self, scale: f32) {
        let want = (self.terrain.written, scale.to_bits(), self.theme as usize | (self.tab << 8) | (self.panel.map_or(0, |p| p + 1) << 12));
        if self.terrain_img.made_for == Some(want) && self.terrain_img.img.is_some() {
            return;
        }
        let (w, h) = ((super::WIDTH as f32 * scale).round() as usize, (super::HEIGHT as f32 * scale).round() as usize);
        let (x0, y0) = ((DISPLAY.0 * scale).round() as usize, (DISPLAY.1 * scale).round() as usize);
        let (dw, dh) = ((DISPLAY.2 * scale).round() as usize, (DISPLAY.3 * scale).round() as usize);
        let mut fb = match self.terrain_img.fb.take() {
            Some(fb) if fb.width == w && fb.height == h => fb,
            _ => goodlookindri_shell::raster::Framebuffer::new(w, h),
        };
        // Everything the display shows in 3D: its frame, ground and labels (cached layer) and
        // the terrain, on the page colour so the display's rounded corners stay right.
        fb.clear(self.t().panel);
        openplugin::dri::goodlookindri::hardware::set_device_scale(scale);
        let mut sc = Scene::new();
        self.paint_3d_layer(&mut sc);
        self.paint_3d_terrain(&mut sc);
        fb.replay(&sc, scale);
        let mut px = Vec::with_capacity(dw * dh);
        for row in fb.pixels[y0 * w..].chunks(w).take(dh) {
            px.extend(row[x0..x0 + dw].iter().map(|p| p | 0xff00_0000));
        }
        self.terrain_img.img = Some(std::sync::Arc::new(openplugin::dri::goodlookindri::scene::Image::new(dw as u32, dh as u32, px)));
        self.terrain_img.fb = Some(fb);
        self.terrain_img.made_for = Some(want);
    }

    /// The terrain: the cached image when there is one, else drawn directly.
    pub(super) fn paint_3d_terrain_cached(&self, s: &mut Scene) {
        match &self.terrain_img.img {
            Some(img) if self.terrain_img.made_for.is_some_and(|m| m.0 == self.terrain.written) => {
                s.image(img, openplugin::dri::goodlookindri::scene::Rect::new(DISPLAY.0, DISPLAY.1, DISPLAY.2, DISPLAY.3), 1.0);
            }
            _ => self.paint_3d_terrain(s),
        }
    }

    /// The cached part: ground plane, second lines, frequency labels, the small history's plot.
    pub(super) fn paint_3d_layer(&self, s: &mut Scene) {
        let t = self.t();
        let wl = well(t);
        kit::plot(s, t, DISPLAY);
        s.fill_round(openplugin::dri::goodlookindri::scene::Rect::new(DISPLAY.0, DISPLAY.1, DISPLAY.2, DISPLAY.3), 4.0, wl.bg, wl.bg, true);
        let (fl, fw, fb, _) = row_geom(0.0);
        let (bl, bw, bb, _) = row_geom(1.0);
        let ground = wl.grid.with_alpha(0.6);
        s.fill_poly(vec![Point::new(fl, fb), Point::new(fl + fw, fb), Point::new(bl + bw, bb), Point::new(bl, bb)], ground, ground);
        for sec in 1..SECONDS3 {
            let (l, w, b, _) = row_geom(sec as f32 / SECONDS3 as f32);
            s.line(Point::new(l, b), Point::new(l + w, b), wl.grid, 1.0);
        }
        s.line(Point::new(fl, fb), Point::new(fl + fw, fb), wl.grid, 1.0);
        for (f, label) in F_LABELS {
            let x = x_of(f).min(DISPLAY.0 + DISPLAY.2 - 10.0);
            s.line(Point::new(x, fb), Point::new(x, fb + 4.0), wl.grid, 1.0);
            s.text(Point::new(x, fb + 12.0), label, wl.text, 8.0, true);
        }
        kit::text_right(s, DISPLAY.0 + DISPLAY.2 - 6.0, DISPLAY.1 + 11.0, "Hz  ·  NOW AT THE FRONT", wl.text, 8.0);
        s.text(Point::new(DISPLAY.0 + 8.0, DISPLAY.1 + 11.0), "3D SPECTRUM  COLOUR = ENERGY", wl.text, 8.0, false);
        // Key: the energy scale, then the gain colours (an open panel covers this corner).
        let (kx, ky) = (DISPLAY.0 + 8.0, DISPLAY.1 + 24.0);
        if self.panel.is_some() {
            return self.paint_3d_lane_layer(s);
        }
        for i in 0..40 {
            let c = wl.heat(i as f32 / 39.0);
            s.fill_rect(openplugin::dri::goodlookindri::scene::Rect::new(kx + i as f32 * 2.0, ky, 2.0, 6.0), c, c, true);
        }
        s.text(Point::new(kx + 86.0, ky + 3.0), "QUIET TO LOUD", wl.text, 8.0, false);
        let lift = lifts(self.tab);
        let (gc, gl) = if lift { (LIFT_COLOUR, "LIFT") } else { (GR_COLOUR, "GAIN REDUCTION") };
        s.fill_rect(openplugin::dri::goodlookindri::scene::Rect::new(kx, ky + 12.0, 12.0, 6.0), gc, gc, true);
        s.text(Point::new(kx + 18.0, ky + 15.0), gl, wl.text, 8.0, false);
        self.paint_3d_lane_layer(s);
    }

    /// The small history's plot, time labels and gain scale.
    fn paint_3d_lane_layer(&self, s: &mut Scene) {
        let t = self.t();
        kit::plot(s, t, LANE);
        for k in 0..=HISTORY_S {
            let x = LANE.0 + LANE.2 * k as f32 / HISTORY_S as f32;
            if k > 0 && k < HISTORY_S {
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
        let (gr_col, sign) = if lifts(self.tab) { (t.ok, "+") } else { (t.over, "-") };
        s.text(Point::new(LANE.0 + LANE.2 + 4.0, LANE.1 + 5.0), "0", gr_col, 8.0, false);
        s.text(Point::new(LANE.0 + LANE.2 + 4.0, LANE.1 + LANE.3 * 0.5), format!("{sign}{GR_DB:.0}"), gr_col, 8.0, false);
    }

    /// The terrain. It changes only when a row is added (10 times a second), so it is drawn
    /// in the cached layer, keyed by the rows written, not on every tick.
    pub(super) fn paint_3d_terrain(&self, s: &mut Scene) {
        let t = self.t();
        let wl = well(t);
        let tr = &self.terrain;
        let n = tr.rows.len();
        let points = (BANDS3 - 1) * SUB + 1;
        let x_at = |l: f32, w: f32, i: usize| l + w * i as f32 / (points - 1) as f32;
        // Each row's ridge, front (age 0) to back, so a row knows the ridge in front of it.
        type Ridge = (f32, f32, f32, f32, Vec<f32>, Vec<f32>);
        let ridges: Vec<Ridge> = (0..n)
            .map(|age| {
                let row = &tr.rows[n - 1 - age];
                let (l, w, base, h) = row_geom(age as f32 / (n - 1) as f32);
                (l, w, base, h, spline(&row.0), spline(&row.1))
            })
            .collect();
        let top_of = |age: usize| -> Vec<Point> {
            let (l, w, base, h, vi, vo) = &ridges[age];
            (0..points).map(|i| Point::new(x_at(*l, *w, i), base - h * vi[i].max(vo[i]))).collect()
        };
        for age in (0..n).rev() {
            let (l, w, base, h, vi, vo) = &ridges[age];
            let (l, w, base, h) = (*l, *w, *base, *h);
            let depth = age as f32 / (n - 1) as f32;
            let shade = 1.0 - 0.6 * depth;
            let y = |v: f32| base - h * v;
            let top = top_of(age);
            let inp: Vec<Point> = (0..points).map(|i| Point::new(x_at(l, w, i), y(vi[i]))).collect();
            let out: Vec<Point> = (0..points).map(|i| Point::new(x_at(l, w, i), y(vo[i]))).collect();
            // Occlusion: the row in front paints over everything under its ridge, so this row
            // only fills down to that ridge (or its own floor). Same pixels, far less filling.
            let floor: Vec<f32> = if age == 0 {
                vec![base; points]
            } else {
                let front = top_of(age - 1);
                let mut j = 0;
                top.iter()
                    .map(|p| {
                        while j + 2 < front.len() && front[j + 1].x < p.x {
                            j += 1;
                        }
                        let (a, b) = (front[j], front[j + 1]);
                        let f = if b.x > a.x { ((p.x - a.x) / (b.x - a.x)).clamp(0.0, 1.0) } else { 0.0 };
                        (a.y + (b.y - a.y) * f).min(base)
                    })
                    .collect()
            };
            let lower_of = |pts: &[Point], other: &[Point]| -> Vec<Point> {
                pts.iter().zip(other).zip(&floor).map(|((p, q), fl)| Point::new(p.x, q.y.min(*fl).max(p.y))).collect()
            };
            let body_floor: Vec<Point> = top.iter().zip(&floor).map(|(p, fl)| Point::new(p.x, fl.max(p.y))).collect();
            // Body: what came out, opaque so it hides the rows behind, coloured by its energy in
            // runs of one step (depth darkens it a little toward the back).
            let step = |i: usize| (vo[i] * HEAT_STEPS).round();
            let mut a = 0;
            while a < points - 1 {
                let q = step(a);
                let mut b = a + 1;
                while b < points - 1 && step(b) == q {
                    b += 1;
                }
                kit::between(s, &top[a..=b], &body_floor[a..=b], mix(wl.bg, wl.heat(q / HEAT_STEPS), 0.45 + 0.55 * shade));
                a = b;
            }
            // Taken out (input above output) red; added (output above input) green.
            // Taken out: from the input down to the output (input above), never under the floor.
            let cut_lower = lower_of(&inp, &out);
            if inp.iter().zip(&cut_lower).any(|(p, q)| q.y - p.y > 0.5) {
                kit::between(s, &inp, &cut_lower, GR_COLOUR);
            }
            let add_lower = lower_of(&out, &inp);
            if out.iter().zip(&add_lower).any(|(p, q)| q.y - p.y > 0.5) {
                kit::between(s, &out, &add_lower, LIFT_COLOUR);
            }
            let ridge = if age == 0 { wl.ridge } else { mix(wl.bg, wl.ridge, 0.08 + 0.2 * shade) };
            kit::curve(s, top, ridge, if age == 0 { 1.4 } else { 0.8 });
        }
        // Second markers at the right edge, on plates over the terrain.
        for sec in 1..SECONDS3 {
            let (_, _, b, _) = row_geom(sec as f32 / SECONDS3 as f32);
            let tag = format!("-{sec} s");
            let w = goodlookindri_shell::text::width(&tag, 8.0);
            let right = DISPLAY.0 + DISPLAY.2 - 6.0;
            s.fill_round(openplugin::dri::goodlookindri::scene::Rect::new(right - w - 5.0, b - 6.5, w + 10.0, 13.0), 3.0, wl.bg, wl.bg, true);
            kit::text_right(s, right, b, &tag, wl.text, 8.0);
        }
    }

    /// The small history under the terrain (moves every tick).
    pub(super) fn paint_3d_live(&self, s: &mut Scene) {
        let t = self.t();
        // Small history: output level (fill), input level (line), gain band from the top.
        let tab = self.tab;
        let gr_col = if lifts(tab) { t.ok } else { t.over };
        let x_at = |k: usize| LANE.0 + LANE.2 * k as f32 / (TRACE_LEN - 1) as f32;
        let ly = |db: f32| LANE.1 + LANE.3 - ((db - LO) / (HI - LO)).clamp(0.0, 1.0) * LANE.3;
        let input: Vec<Point> = self.level[tab].iter().enumerate().map(|(k, l)| Point::new(x_at(k), ly(*l))).collect();
        let output: Vec<Point> = self.level[tab].iter().zip(&self.trace[tab]).enumerate().map(|(k, (l, g))| Point::new(x_at(k), ly(l + g))).collect();
        let band: Vec<Point> = self.trace[tab].iter().enumerate().map(|(k, g)| Point::new(x_at(k), LANE.1 + (g.abs() / GR_DB).min(1.0) * LANE.3 * 0.5)).collect();
        kit::area(s, &output, LANE.1 + LANE.3, t.out_fill.with_alpha(t.out_fill_alpha));
        kit::curve(s, input, t.in_line, 1.0);
        kit::area(s, &band, LANE.1, gr_col.with_alpha(0.25));
        kit::curve(s, band, gr_col, 1.2);
        let cap = "HISTORY";
        let w = goodlookindri_shell::text::width(cap, 8.0);
        s.fill_round(openplugin::dri::goodlookindri::scene::Rect::new(LANE.0 + 4.0, LANE.1 + 3.0, w + 10.0, 12.0), 3.0, t.plot_bg, t.plot_bg, true);
        s.text(Point::new(LANE.0 + 9.0, LANE.1 + 9.0), cap, t.dim, 8.0, false);
    }
}
