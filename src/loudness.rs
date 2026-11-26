//! Loudness of a programme as BS.1770-4 and EBU R128 measure it: gated integrated loudness,
//! loudness range (EBU Tech 3342), momentary and short-term loudness, sample and true peak,
//! and how loud the quiet parts are. The Auto Loudness mode plans the sections from it.
//!
//! Gating works on histograms of block loudness in 0.1 LU steps (as libebur128's histogram
//! mode), so the meter runs for any length without allocating after `new`.

use crate::autogain::{k_weighting, Biquad};

const BIN_DB: f64 = 0.1;
const LOW_DB: f64 = -70.0;
const BINS: usize = 800; // -70 .. +10 LUFS
const SUB_S: f64 = 0.1; // blocks start every 100 ms
const MOMENTARY: usize = 4; // 400 ms
const SHORT: usize = 30; // 3 s
const TP_PHASES: usize = 4;
const TP_TAPS: usize = 16; // per phase

fn loudness(power: f64) -> f64 {
    -0.691 + 10.0 * power.max(1e-30).log10()
}

fn bin(lufs: f64) -> Option<usize> {
    (lufs >= LOW_DB).then(|| (((lufs - LOW_DB) / BIN_DB) as usize).min(BINS - 1))
}

fn bin_lufs(i: usize) -> f64 {
    LOW_DB + (i as f64 + 0.5) * BIN_DB
}

/// What the meter has measured so far. Values are LUFS, LU or dB; `NEG_INFINITY` when there was
/// not enough signal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stats {
    pub integrated: f64,
    pub range: f64,
    pub momentary: f64,
    pub short_term: f64,
    pub sample_peak_db: f64,
    pub true_peak_db: f64,
    /// The 10th percentile of momentary loudness above the absolute gate: the quiet passages.
    pub quiet: f64,
    /// The 95th percentile of short-term loudness: the loud passages.
    pub loud: f64,
    pub seconds: f64,
}

impl Stats {
    /// Peak to loudness ratio (true peak above integrated loudness).
    pub fn plr(&self) -> f64 {
        self.true_peak_db - self.integrated
    }

    pub fn has_programme(&self) -> bool {
        self.integrated.is_finite()
    }
}

pub struct Meter {
    k: [[Biquad; 2]; 2],
    sub_len: usize,
    sub_fill: usize,
    sub_acc: f64,
    subs: [f64; SHORT],
    sub_count: usize,
    m_count: [u32; BINS],
    m_power: [f64; BINS],
    s_count: [u32; BINS],
    s_power: [f64; BINS],
    momentary: f64,
    short_term: f64,
    sample_peak: f32,
    true_peak: f32,
    tp_h: [[f32; TP_TAPS]; TP_PHASES],
    tp_hist: [[f32; TP_TAPS]; 2],
    tp_pos: usize,
    samples: u64,
    sr: f64,
}

impl Meter {
    pub fn new(sample_rate: f32) -> Box<Self> {
        let sr = f64::from(sample_rate.max(1.0));
        let (s1, s2) = k_weighting(sr);
        let f = [Biquad { c: s1, z: [0.0; 2] }, Biquad { c: s2, z: [0.0; 2] }];
        // True peak: 4x interpolation by a Kaiser-windowed sinc, as BS.1770-4 Annex 2 allows.
        let mut tp_h = [[0.0f32; TP_TAPS]; TP_PHASES];
        let n = TP_PHASES * TP_TAPS;
        let mid = (n - 1) as f64 / 2.0;
        let beta = 8.0;
        let i0 = |x: f64| (0..30).fold((1.0, 1.0), |(s, t), k| { let t = t * (x / 2.0).powi(2) / ((k + 1) as f64).powi(2); (s + t, t) }).0;
        for i in 0..n {
            let x = (i as f64 - mid) / TP_PHASES as f64;
            let sinc = if x == 0.0 { 1.0 } else { (std::f64::consts::PI * x).sin() / (std::f64::consts::PI * x) };
            let r = (i as f64 - mid) / mid;
            let w = i0(beta * (1.0 - r * r).max(0.0).sqrt()) / i0(beta);
            tp_h[i % TP_PHASES][i / TP_PHASES] = (sinc * w) as f32;
        }
        Box::new(Self {
            k: [f, f],
            sub_len: (sr * SUB_S).round().max(1.0) as usize,
            sub_fill: 0,
            sub_acc: 0.0,
            subs: [0.0; SHORT],
            sub_count: 0,
            m_count: [0; BINS],
            m_power: [0.0; BINS],
            s_count: [0; BINS],
            s_power: [0.0; BINS],
            momentary: f64::NEG_INFINITY,
            short_term: f64::NEG_INFINITY,
            sample_peak: 0.0,
            true_peak: 0.0,
            tp_h,
            tp_hist: [[0.0; TP_TAPS]; 2],
            tp_pos: 0,
            samples: 0,
            sr,
        })
    }

    /// Starts over (the filters keep their state: no click in the measurement).
    pub fn clear(&mut self) {
        self.sub_fill = 0;
        self.sub_acc = 0.0;
        self.subs = [0.0; SHORT];
        self.sub_count = 0;
        self.m_count = [0; BINS];
        self.m_power = [0.0; BINS];
        self.s_count = [0; BINS];
        self.s_power = [0.0; BINS];
        self.momentary = f64::NEG_INFINITY;
        self.short_term = f64::NEG_INFINITY;
        self.sample_peak = 0.0;
        self.true_peak = 0.0;
        self.samples = 0;
    }

    pub fn feed(&mut self, left: &[f32], right: &[f32]) {
        for (&l, &r) in left.iter().zip(right) {
            let mut sum = 0.0;
            for (c, x) in [l, r].into_iter().enumerate() {
                let [a, b] = &mut self.k[c];
                let y = b.run(a.run(f64::from(x)));
                sum += y * y;
            }
            self.sub_acc += sum;
            self.peaks(l, r);
            self.samples += 1;
            self.sub_fill += 1;
            if self.sub_fill == self.sub_len {
                self.end_sub_block();
            }
        }
    }

    fn peaks(&mut self, l: f32, r: f32) {
        let pos = self.tp_pos;
        for (c, x) in [l, r].into_iter().enumerate() {
            if !x.is_finite() {
                continue;
            }
            self.sample_peak = self.sample_peak.max(x.abs());
            let hist = &mut self.tp_hist[c];
            hist[pos] = x;
            for phase in &self.tp_h {
                let mut y = 0.0;
                for (k, h) in phase.iter().enumerate() {
                    y += h * hist[(pos + TP_TAPS - k) % TP_TAPS];
                }
                self.true_peak = self.true_peak.max(y.abs());
            }
        }
        self.tp_pos = (pos + 1) % TP_TAPS;
    }

    fn end_sub_block(&mut self) {
        let power = self.sub_acc / self.sub_len as f64;
        self.sub_acc = 0.0;
        self.sub_fill = 0;
        self.subs.rotate_right(1);
        self.subs[0] = power;
        self.sub_count += 1;
        if self.sub_count >= MOMENTARY {
            let p = self.subs[..MOMENTARY].iter().sum::<f64>() / MOMENTARY as f64;
            self.momentary = loudness(p);
            if let Some(b) = bin(self.momentary) {
                self.m_count[b] += 1;
                self.m_power[b] += p;
            }
        }
        if self.sub_count >= SHORT {
            let p = self.subs.iter().sum::<f64>() / SHORT as f64;
            self.short_term = loudness(p);
            if let Some(b) = bin(self.short_term) {
                self.s_count[b] += 1;
                self.s_power[b] += p;
            }
        }
    }

    /// Mean loudness of the blocks at or above `gate` (LUFS).
    fn gated_mean(count: &[u32; BINS], power: &[f64; BINS], gate: f64) -> Option<f64> {
        let from = bin(gate).unwrap_or(0);
        let (n, p) = (from..BINS).fold((0u64, 0.0), |(n, p), i| (n + u64::from(count[i]), p + power[i]));
        (n > 0).then(|| loudness(p / n as f64))
    }

    /// The loudness below which `q` (0..1) of the blocks at or above `gate` lie.
    fn percentile(count: &[u32; BINS], gate: f64, q: f64) -> Option<f64> {
        let from = bin(gate).unwrap_or(0);
        let total: u64 = count[from..].iter().map(|&c| u64::from(c)).sum();
        if total == 0 {
            return None;
        }
        let want = (q * total as f64).ceil().max(1.0) as u64;
        let mut seen = 0;
        for (i, &c) in count.iter().enumerate().skip(from) {
            seen += u64::from(c);
            if seen >= want {
                return Some(bin_lufs(i));
            }
        }
        None
    }

    pub fn stats(&self) -> Stats {
        let none = f64::NEG_INFINITY;
        let integrated = Self::gated_mean(&self.m_count, &self.m_power, LOW_DB)
            .and_then(|ungated| Self::gated_mean(&self.m_count, &self.m_power, ungated - 10.0))
            .unwrap_or(none);
        // Loudness range: short-term blocks, relative gate 20 LU, 10th to 95th percentile.
        let s_gate = Self::gated_mean(&self.s_count, &self.s_power, LOW_DB).map(|m| m - 20.0);
        let (lo, hi) = match s_gate {
            Some(g) => (Self::percentile(&self.s_count, g, 0.10), Self::percentile(&self.s_count, g, 0.95)),
            None => (None, None),
        };
        let range = match (lo, hi) {
            (Some(lo), Some(hi)) => (hi - lo).max(0.0),
            _ => none,
        };
        let db = |x: f32| if x > 0.0 { 20.0 * f64::from(x).log10() } else { none };
        Stats {
            integrated,
            range,
            momentary: self.momentary,
            short_term: self.short_term,
            sample_peak_db: db(self.sample_peak),
            true_peak_db: db(self.true_peak.max(self.sample_peak)),
            quiet: Self::percentile(&self.m_count, LOW_DB, 0.10).unwrap_or(none),
            loud: hi.unwrap_or(none),
            seconds: self.samples as f64 / self.sr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    /// A stereo sine of `dbfs` per channel at `hz` for `secs`.
    fn tone(m: &mut Meter, dbfs: f64, hz: f64, secs: f64, phase: &mut f64) {
        let a = 10f64.powf(dbfs / 20.0);
        let n = (secs * f64::from(SR)) as usize;
        let mut buf = vec![0.0f32; 4800];
        let mut done = 0;
        while done < n {
            let len = (n - done).min(buf.len());
            for x in buf.iter_mut().take(len) {
                *x = (a * phase.sin()) as f32;
                *phase += std::f64::consts::TAU * hz / f64::from(SR);
            }
            m.feed(&buf[..len], &buf[..len]);
            done += len;
        }
    }

    /// EBU Tech 3341 case 1: stereo 1 kHz sine at -23 dBFS reads -23.0 LUFS (+-0.1).
    #[test]
    fn reads_minus_23() {
        let mut m = Meter::new(SR);
        let mut ph = 0.0;
        tone(&mut m, -23.0, 1000.0, 20.0, &mut ph);
        let s = m.stats();
        assert!((s.integrated + 23.0).abs() < 0.1, "{s:?}");
        assert!((s.momentary + 23.0).abs() < 0.1 && (s.short_term + 23.0).abs() < 0.1, "{s:?}");
    }

    /// EBU Tech 3341 case 3: 10 s at -36, 60 s at -23, 10 s at -36 dBFS: integrated -23.0 (the
    /// quiet parts fall under the relative gate).
    #[test]
    fn gates_the_quiet_parts() {
        let mut m = Meter::new(SR);
        let mut ph = 0.0;
        tone(&mut m, -36.0, 1000.0, 10.0, &mut ph);
        tone(&mut m, -23.0, 1000.0, 60.0, &mut ph);
        tone(&mut m, -36.0, 1000.0, 10.0, &mut ph);
        let s = m.stats();
        assert!((s.integrated + 23.0).abs() < 0.1, "{s:?}");
        assert!((s.quiet + 36.0).abs() < 0.3, "{s:?}");
    }

    /// EBU Tech 3342 case 1: 20 s at -20 then 20 s at -30 dBFS: loudness range 10 LU (+-1).
    #[test]
    fn loudness_range_of_two_levels() {
        let mut m = Meter::new(SR);
        let mut ph = 0.0;
        tone(&mut m, -20.0, 1000.0, 20.0, &mut ph);
        tone(&mut m, -30.0, 1000.0, 20.0, &mut ph);
        let s = m.stats();
        assert!((s.range - 10.0).abs() < 1.0, "{s:?}");
    }

    /// True peak sees the peak between samples: a full-scale sine at fs/4 sampled at 45 degrees
    /// peaks at 0.707 in the samples and at 1.0 between them.
    #[test]
    fn true_peak_between_samples() {
        let mut m = Meter::new(SR);
        let x: Vec<f32> = (0..4800).map(|i| (std::f64::consts::FRAC_PI_2 * i as f64 + std::f64::consts::FRAC_PI_4).sin() as f32).collect();
        m.feed(&x, &x);
        let s = m.stats();
        assert!((s.sample_peak_db + 3.01).abs() < 0.05, "{s:?}");
        assert!(s.true_peak_db > -0.5 && s.true_peak_db < 0.3, "{s:?}");
    }

    #[test]
    fn silence_has_no_programme() {
        let mut m = Meter::new(SR);
        m.feed(&[0.0; 48_000], &[0.0; 48_000]);
        assert!(!m.stats().has_programme());
    }
}
