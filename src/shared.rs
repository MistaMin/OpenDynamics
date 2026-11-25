//! What the engine shows the editor: lock-free, one writer (audio) and one reader (editor).
//! Values merge between editor frames: peaks and levels keep the highest, gains the one
//! furthest from 0 dB.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering, Ordering::Relaxed};
use std::sync::Mutex;

use crate::loudness::Stats;

use crate::SECTIONS;

struct F(AtomicU32);

impl Default for F {
    fn default() -> Self {
        Self(AtomicU32::new(f32::NAN.to_bits()))
    }
}

impl F {
    fn get(&self) -> f32 {
        f32::from_bits(self.0.load(Relaxed))
    }
    fn set(&self, v: f32) {
        self.0.store(v.to_bits(), Relaxed);
    }
    /// Keeps `v` if nothing is stored yet or `keep(v, old)` says so.
    fn merge(&self, v: f32, keep: impl Fn(f32, f32) -> bool) {
        let old = self.get();
        if old.is_nan() || keep(v, old) {
            self.set(v);
        }
    }
    fn take(&self, empty: f32) -> f32 {
        let v = f32::from_bits(self.0.swap(f32::NAN.to_bits(), Relaxed));
        if v.is_nan() {
            empty
        } else {
            v
        }
    }
}

#[derive(Default)]
struct SectionLive {
    gain: [F; 2],
    level: [F; 2],
    active: AtomicBool,
}

/// Samples kept per tap channel, a power of two.
pub const TAP_LEN: usize = 8192;

/// Audio for the editor's spectrum: 0/1 the input after the input gain, delayed by the
/// plugin's latency (so it lines up with the output); 2/3 the output of the sections.
struct Taps {
    buf: Vec<AtomicU32>,
    head: AtomicUsize,
}

impl Default for Taps {
    fn default() -> Self {
        Self { buf: (0..4 * TAP_LEN).map(|_| AtomicU32::new(0)).collect(), head: AtomicUsize::new(0) }
    }
}

#[derive(Default)]
pub struct Shared {
    taps: Taps,
    sections: [SectionLive; SECTIONS],
    peak_in: [F; 2],
    peak_out: [F; 2],
    key: AtomicBool,
    rate: F,
    auto_gain: F,
    learn: AtomicBool,
    learn_reset: AtomicBool,
    /// Bumped each time a measurement is published, so the editor knows it is new.
    loudness_seq: AtomicU64,
    /// Input and output loudness. Written by the audio thread with `try_lock` only, so it
    /// never waits: a frame the editor is reading is simply published on the next tick.
    loudness: Mutex<Option<(Stats, Stats)>>,
}

/// One section as of the last editor frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct Live {
    /// Signed gain per channel, dB.
    pub gain: [f32; 2],
    /// Detector level per channel, dBFS.
    pub level: [f32; 2],
    pub active: bool,
}

impl Shared {
    pub(crate) fn section(&self, i: usize, gain: [f32; 2], level: [f32; 2], active: bool) {
        let Some(s) = self.sections.get(i) else { return };
        for c in 0..2 {
            s.gain[c].merge(gain[c], |v, old| v.abs() > old.abs());
            s.level[c].merge(level[c], |v, old| v > old);
        }
        s.active.store(active, Relaxed);
    }

    pub(crate) fn peaks(&self, input: [f32; 2], output: [f32; 2]) {
        for c in 0..2 {
            self.peak_in[c].merge(input[c], |v, old| v > old);
            self.peak_out[c].merge(output[c], |v, old| v > old);
        }
    }

    pub(crate) fn push_taps(&self, pre: [&[f32]; 2], post: [&[f32]; 2]) {
        let head = self.taps.head.load(Relaxed);
        for (c, src) in [pre[0], pre[1], post[0], post[1]].into_iter().enumerate() {
            let base = c * TAP_LEN;
            for (j, v) in src.iter().enumerate() {
                self.taps.buf[base + ((head + j) & (TAP_LEN - 1))].store(v.to_bits(), Relaxed);
            }
        }
        self.taps.head.store(head.wrapping_add(pre[0].len()), Ordering::Release);
    }

    /// The newest `out.len()` samples (at most `TAP_LEN`) of tap `channel` (0/1 aligned input
    /// L/R, 2/3 output L/R), oldest first.
    pub fn read_tap(&self, channel: usize, out: &mut [f32]) {
        let head = self.taps.head.load(Ordering::Acquire);
        let n = out.len().min(TAP_LEN);
        let base = (channel % 4) * TAP_LEN;
        for (j, o) in out[..n].iter_mut().enumerate() {
            let i = head.wrapping_sub(n).wrapping_add(j) & (TAP_LEN - 1);
            *o = f32::from_bits(self.taps.buf[base + i].load(Relaxed));
        }
    }

    pub(crate) fn set_key_present(&self, on: bool) {
        self.key.store(on, Relaxed);
    }

    pub(crate) fn set_auto_gain(&self, db: f32) {
        self.auto_gain.set(db);
    }

    /// The gain Auto Gain applies now, dB.
    /// Auto Loudness: start (clearing what was measured) or stop measuring.
    pub fn set_learning(&self, on: bool) {
        if on {
            self.learn_reset.store(true, Relaxed);
        }
        self.learn.store(on, Relaxed);
    }

    pub fn learning(&self) -> bool {
        self.learn.load(Relaxed)
    }

    pub(crate) fn take_learn_reset(&self) -> bool {
        self.learn_reset.swap(false, Relaxed)
    }

    pub(crate) fn publish_loudness(&self, input: Stats, output: Stats) {
        if let Ok(mut l) = self.loudness.try_lock() {
            *l = Some((input, output));
            self.loudness_seq.fetch_add(1, Relaxed);
        }
    }

    /// The last measurement of the input and the output, and its sequence number.
    pub fn loudness(&self) -> (u64, Option<(Stats, Stats)>) {
        (self.loudness_seq.load(Relaxed), self.loudness.lock().map(|l| *l).unwrap_or(None))
    }

    pub fn auto_gain_db(&self) -> f32 {
        let v = self.auto_gain.get();
        if v.is_nan() { 0.0 } else { v }
    }

    pub(crate) fn set_sample_rate(&self, sr: f32) {
        self.rate.set(sr);
    }

    pub fn take_section(&self, i: usize) -> Live {
        let Some(s) = self.sections.get(i) else { return Live::default() };
        Live {
            gain: [s.gain[0].take(0.0), s.gain[1].take(0.0)],
            level: [s.level[0].take(-200.0), s.level[1].take(-200.0)],
            active: s.active.load(Relaxed),
        }
    }

    /// `(input, output)` peaks per channel since the last call, linear.
    pub fn take_peaks(&self) -> ([f32; 2], [f32; 2]) {
        ([self.peak_in[0].take(0.0), self.peak_in[1].take(0.0)], [self.peak_out[0].take(0.0), self.peak_out[1].take(0.0)])
    }

    pub fn key_present(&self) -> bool {
        self.key.load(Relaxed)
    }

    pub fn sample_rate(&self) -> f32 {
        let r = self.rate.get();
        if r.is_nan() {
            48_000.0
        } else {
            r
        }
    }
}
