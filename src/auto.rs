//! Auto Loudness: from the measured programme, settings for all five sections that bring it to
//! a target integrated loudness under a true-peak ceiling.
//!
//! The plan is written into the sections' own parameters (by the editor, so the host records
//! them), so the user sees and can change every value. It is an estimate: the result is
//! measured again on the output, and the next plan corrects the limiter's drive by the error.
//!
//! What each section is asked to do:
//! - Decompressor: a programme squashed below the crest the target allows gets its transients
//!   back (Match, to the target crest).
//! - Gate / Expander: a noise floor far below the music is pulled down (expander, gentle).
//! - Compressor: a loudness range wider than the target allows is narrowed above the programme
//!   loudness (RMS detector, soft knee, auto timing).
//! - De-Expander: quiet passages far under the programme are lifted, above the noise floor.
//! - Limiter: the level goes to the target through the limiter's drive; peaks stay under the
//!   ceiling. A programme louder than the target is turned down at the output instead.

use crate::loudness::Stats;

/// One parameter value, by host parameter id.
pub type Setting = (&'static str, f32);

/// The loudness range a target leaves room for: loud targets (streaming, -14) want a narrow
/// range, quiet ones (broadcast, -23) a wide one.
pub fn target_range(target: f64) -> f64 {
    (7.5 + (-target - 14.0) * 0.5).clamp(5.0, 15.0)
}

/// Results of a plan, for the editor to show beside the settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plan {
    pub settings: [Setting; 24],
    /// The gain into the limiter, dB (positive: louder).
    pub drive: f64,
    /// Output gain, dB (zero or negative).
    pub output: f64,
}

/// `input`: the programme as it enters the plugin. `output`: the plugin's output measured with
/// the settings of the previous plan, if there was one (`prev_drive` and `prev_output` were its
/// gains). `target` in LUFS, `ceiling` in dBTP.
pub fn plan(input: &Stats, output: Option<(&Stats, f64, f64)>, target: f64, ceiling: f64) -> Option<Plan> {
    if !input.has_programme() {
        return None;
    }
    let i = input.integrated;
    let range_t = target_range(target);
    let crest_t = (ceiling - target).clamp(6.0, 24.0);

    // Decompressor: give back transients that a squashed master lost.
    let decomp = input.plr() < crest_t - 3.0;
    // Gate / Expander: a noise floor well under the music.
    let floor = input.quiet;
    let expand = floor.is_finite() && input.loud - floor > 30.0;
    let exp_threshold = (floor + 6.0).clamp(-90.0, -20.0);
    // Compressor: narrow a wide loudness range.
    let compress = input.range > range_t + 1.0;
    let ratio = (input.range / range_t).clamp(1.5, 4.0).round().max(2.0);
    let comp_threshold = (i - 2.0).clamp(-60.0, 0.0).round();
    // De-expander: lift quiet passages to the bottom of the range the target allows.
    let lift_to = i - range_t;
    let deexpand = floor.is_finite() && floor < lift_to - 6.0;
    let deexp_floor = if expand { exp_threshold } else { (floor - 12.0).clamp(-120.0, -20.0) };

    // Level: compressing above `i` lowers the loud parts by about (1 - 1/ratio) of their
    // excursion over the threshold; the rest goes into the limiter.
    let comp_loss = if compress { (input.loud - comp_threshold).max(0.0) * (1.0 - 1.0 / ratio) * 0.5 } else { 0.0 };
    let estimate = i - comp_loss;
    let mut gain = target - estimate;
    if let Some((out, prev_drive, prev_output)) = output {
        if out.has_programme() {
            // Measured: correct the previous gains by what was missed.
            gain = prev_drive + prev_output + (target - out.integrated);
        }
    }
    let drive = gain.clamp(0.0, 24.0);
    let output_gain = gain.min(0.0).max(-24.0);
    // The limiter raises by ceiling - threshold.
    let lim_threshold = (ceiling - drive).clamp(-24.0, 0.0);

    let b = |on: bool| if on { 1.0 } else { 0.0 };
    let r = |x: f64| x.round() as f32;
    Some(Plan {
        settings: [
            ("decomp.on", b(decomp)),
            ("decomp.match", b(decomp)),
            ("decomp.matchTarget", r(crest_t.min(14.0))),
            ("decomp.ceilingOn", 1.0),
            ("decomp.ceiling", r(ceiling)),
            ("gate.on", b(expand)),
            ("gate.mode", 1.0),
            ("gate.threshold", r(exp_threshold)),
            ("gate.ratio", 2.0),
            ("gate.range", 12.0),
            ("comp.on", b(compress)),
            ("comp.threshold", r(comp_threshold)),
            ("comp.ratio", ratio as f32),
            ("comp.detector", 1.0),
            ("comp.autoAttack", 1.0),
            ("comp.autoRelease", 1.0),
            ("deexp.on", b(deexpand)),
            ("deexp.threshold", r(lift_to.clamp(-80.0, 0.0))),
            ("deexp.range", r((lift_to - floor).clamp(0.0, 12.0))),
            ("deexp.floor", r(deexp_floor)),
            ("limit.on", 1.0),
            ("limit.ceiling", r(ceiling)),
            ("limit.threshold", lim_threshold as f32),
            ("output", r(output_gain)),
        ],
        drive,
        output: output_gain,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stats(integrated: f64, range: f64, quiet: f64, loud: f64, tp: f64) -> Stats {
        Stats { integrated, range, momentary: integrated, short_term: integrated, sample_peak_db: tp, true_peak_db: tp, quiet, loud, seconds: 60.0 }
    }

    fn get(p: &Plan, id: &str) -> f32 {
        p.settings.iter().find(|s| s.0 == id).unwrap().1
    }

    /// Every id the plan writes is a real host parameter, and every value is in its range.
    #[test]
    fn writes_real_parameters_in_range() {
        let p = plan(&stats(-30.0, 20.0, -70.0, -20.0, -10.0), None, -14.0, -1.0).unwrap();
        for (id, v) in p.settings {
            let info = crate::INFO.params.iter().find(|q| q.id == id).unwrap_or_else(|| panic!("{id}"));
            assert!(v >= info.min && v <= info.max, "{id} = {v} outside {}..{}", info.min, info.max);
        }
    }

    /// A quiet, wide, noisy recording: every section works, and the level is pushed up.
    #[test]
    fn quiet_wide_programme_uses_every_section() {
        let p = plan(&stats(-30.0, 20.0, -70.0, -20.0, -10.0), None, -14.0, -1.0).unwrap();
        assert_eq!(get(&p, "comp.on"), 1.0);
        assert_eq!(get(&p, "deexp.on"), 1.0);
        assert_eq!(get(&p, "gate.on"), 1.0);
        assert!(p.drive > 10.0 && p.output == 0.0, "{p:?}");
    }

    /// A squashed master louder than the target: transients back, turned down, nothing pushed.
    #[test]
    fn loud_squashed_master_is_opened_and_turned_down() {
        let p = plan(&stats(-8.0, 3.0, -14.0, -6.0, -0.5), None, -14.0, -1.0).unwrap();
        assert_eq!(get(&p, "decomp.on"), 1.0);
        assert_eq!(get(&p, "comp.on"), 0.0);
        assert!(p.output < -5.0 && p.drive == 0.0, "{p:?}");
    }

    /// A measured output corrects the gain by what the estimate missed.
    #[test]
    fn measured_output_corrects_the_drive() {
        let input = stats(-30.0, 6.0, -40.0, -26.0, -12.0);
        let first = plan(&input, None, -14.0, -1.0).unwrap();
        let out = stats(-16.5, 6.0, -26.0, -12.0, -1.0);
        let second = plan(&input, Some((&out, first.drive, first.output)), -14.0, -1.0).unwrap();
        assert!((second.drive - (first.drive + 2.5)).abs() < 1e-9, "{first:?} {second:?}");
    }

    #[test]
    fn silence_has_no_plan() {
        let s = Stats { integrated: f64::NEG_INFINITY, ..stats(0.0, 0.0, 0.0, 0.0, 0.0) };
        assert!(plan(&s, None, -14.0, -1.0).is_none());
    }
}
