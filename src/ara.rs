//! ARA: an ARA host (VST3 built with OpenPluginSDK's `--ara-sdk`) hands the plugin every clip's
//! whole audio source once. Each is measured here (loudness, range, true peak, quiet and loud
//! passages), so Auto Loudness can plan from the whole clip without a Learn pass.
//!
//! The results are kept by the source's persistent ID for the whole process: an ARA document
//! is shared by every instance bound to it, and the editor of any of them may read them.

use std::ffi::{c_char, CStr};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::sync::{LazyLock, Mutex};

use crate::loudness::{Meter, Stats};

/// One analysed audio source.
#[derive(Clone, Debug)]
pub struct Source {
    pub id: String,
    pub stats: Stats,
}

static SOURCES: LazyLock<Mutex<Vec<Source>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static GENERATION: AtomicU64 = AtomicU64::new(0);

/// Measures a whole source and keeps the result under `id` (a newer one replaces it).
pub fn analyze(id: &str, sample_rate: f64, channels: &[&[f32]]) -> Option<Stats> {
    let (Some(&left), Some(&right)) = (channels.first(), channels.get(1).or(channels.first())) else { return None };
    if !(8_000.0..=768_000.0).contains(&sample_rate) || left.len() != right.len() {
        return None;
    }
    let mut meter = Meter::new(sample_rate as f32);
    for (l, r) in left.chunks(8192).zip(right.chunks(8192)) {
        meter.feed(l, r);
    }
    let stats = meter.stats();
    crate::log_line(&format!(
        "ARA analysis of {id}: {:.1} s, {:.1} LUFS, range {:.1} LU, true peak {:.1} dBTP, quiet {:.1}, loud {:.1}",
        stats.seconds, stats.integrated, stats.range, stats.true_peak_db, stats.quiet, stats.loud
    ));
    let mut sources = SOURCES.lock().ok()?;
    sources.retain(|s| s.id != id);
    sources.push(Source { id: id.to_string(), stats });
    GENERATION.fetch_add(1, Relaxed);
    Some(stats)
}

/// The analysed sources, oldest first, and a number that changes whenever they do.
pub fn sources() -> (u64, Vec<Source>) {
    (GENERATION.load(Relaxed), SOURCES.lock().map(|s| s.clone()).unwrap_or_default())
}

/// Called by the VST3 shell's ARA document controller (OpenPluginSDK `op_shell.h`).
///
/// # Safety
/// `id` is a NUL-terminated string and `data` holds `channels` pointers to `frames` samples each,
/// all valid for the call (the shell guarantees it).
#[no_mangle]
pub unsafe extern "C" fn op_ara_analyze(id: *const c_char, channels: u32, frames: u64, sample_rate: f64, data: *const *const f32) {
    let _ = std::panic::catch_unwind(|| {
        if id.is_null() || data.is_null() || channels == 0 || frames == 0 || frames > usize::MAX as u64 {
            return;
        }
        let id = unsafe { CStr::from_ptr(id) }.to_string_lossy();
        let ptrs = unsafe { std::slice::from_raw_parts(data, channels as usize) };
        if ptrs.iter().any(|p| p.is_null()) {
            return;
        }
        let chans: Vec<&[f32]> = ptrs.iter().take(2).map(|&p| unsafe { std::slice::from_raw_parts(p, frames as usize) }).collect();
        analyze(&id, sample_rate, &chans);
    });
}

/// The plugin's web page, for the ARA factory (OpenPluginSDK `op_ara_url`).
#[no_mangle]
pub extern "C" fn op_ara_url() -> *const c_char {
    c"https://github.com/MistaMin/OpenDynamics".as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The C entry measures a source and keeps it by ID; a second call with the same ID
    /// replaces it.
    #[test]
    fn analyzes_sources_by_id() {
        let n = 48_000 * 10;
        let a = 10f32.powf(-23.0 / 20.0);
        let x: Vec<f32> = (0..n).map(|i| a * (std::f32::consts::TAU * 1000.0 * i as f32 / 48_000.0).sin()).collect();
        let ptrs = [x.as_ptr(), x.as_ptr()];
        let id = c"test.source.1";
        let (g0, _) = sources();
        unsafe { op_ara_analyze(id.as_ptr(), 2, n as u64, 48_000.0, ptrs.as_ptr()) };
        unsafe { op_ara_analyze(id.as_ptr(), 2, n as u64, 48_000.0, ptrs.as_ptr()) };
        let (g1, list) = sources();
        assert!(g1 >= g0 + 2);
        let hits: Vec<_> = list.iter().filter(|s| s.id == "test.source.1").collect();
        assert_eq!(hits.len(), 1);
        assert!((hits[0].stats.integrated + 23.0).abs() < 0.1, "{:?}", hits[0].stats);
        // Bad calls are ignored, never a crash.
        unsafe { op_ara_analyze(std::ptr::null(), 2, 10, 48_000.0, ptrs.as_ptr()) };
        unsafe { op_ara_analyze(id.as_ptr(), 2, 10, 48_000.0, std::ptr::null()) };
    }
}
