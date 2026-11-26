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

/// The three gains a plan sets, dB: Input Gain (before the sections), the limiter's drive
/// (Ceiling - Threshold) and Output Gain (after the limiter, only ever down).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Gains {
    pub input: f64,
    pub drive: f64,
    pub output: f64,
}

impl Gains {
    pub fn total(&self) -> f64 {
        self.input + self.drive + self.output
    }

    /// Spreads a total gain: up through the limiter's drive first (it holds the ceiling), then
    /// the input; down at the output first, then the input. The drive reaches at most
    /// `max_drive` (the limiter's threshold stops at -24 dB, so it is its ceiling + 24).
    fn split(total: f64, max_drive: f64) -> Self {
        if total >= 0.0 {
            let drive = total.min(max_drive.max(0.0));
            Self { input: (total - drive).min(24.0), drive, output: 0.0 }
        } else {
            let output = total.max(-24.0);
            Self { input: (total - output).max(-24.0), drive: 0.0, output }
        }
    }
}

/// Results of a plan, for the editor to show beside the settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plan {
    pub settings: [Setting; 25],
    pub gains: Gains,
    /// How far under the ceiling the limiter is set, so true peaks stay under it, dB.
    pub margin: f64,
    /// The last measured point: a total gain and the output loudness it gave. With the next
    /// measurement it gives how much loudness a dB of gain buys (less as the limiter works harder).
    pub point: Option<(f64, f64)>,
}

/// `input`: the programme as the sections saw it (after Input Gain), measured while Input Gain
/// was `input_gain_then`. `output`: if the current settings came from a previous plan, the
/// output measured with them and that plan. `target` in LUFS, `ceiling` in dBTP.
pub fn plan(input: &Stats, input_gain_then: f64, output: Option<(&Stats, &Plan)>, target: f64, ceiling: f64) -> Option<Plan> {
    if !input.has_programme() {
        return None;
    }
    let range_t = target_range(target);
    let crest_t = (ceiling - target).clamp(6.0, 24.0);
    // The programme before Input Gain.
    let raw = |x: f64| x - input_gain_then;

    // Crest: what the target's peak-to-loudness ratio leaves room for. A programme with more
    // crest than that has its peaks shared between the compressor (peak detector, above the
    // programme loudness) and the limiter, so the limiter is not asked for everything.
    let crest_excess = input.plr() - crest_t;
    let crest_mode = crest_excess > 3.0;
    // Loudness range: a programme wider than the target allows is narrowed (RMS detector).
    let range_mode = input.range > range_t + 1.0;
    let compress = crest_mode || range_mode;
    let ratio = if crest_mode { 4.0 } else { (input.range / range_t).clamp(1.5, 4.0).round().max(2.0) };
    let over = if crest_mode { 4.0 } else { -2.0 };
    // Level estimate: compressing above the threshold lowers the loud parts by about
    // (1 - 1/ratio) of their excursion; the rest of the way is gain.
    let comp_loss = if range_mode { (input.loud - input.integrated - over).max(0.0) * (1.0 - 1.0 / ratio) * 0.5 } else { 0.0 };
    let (gains, margin, point) = match output {
        Some((out, prev)) if out.has_programme() => {
            // Measured: correct what the previous plan missed. With an earlier measurement too,
            // divide the miss by how much loudness a dB bought between the two (a secant step).
            let here = (prev.gains.total(), out.integrated);
            let slope = match prev.point {
                Some((g0, o0)) if (here.0 - g0).abs() > 0.1 => ((here.1 - o0) / (here.0 - g0)).clamp(0.2, 1.0),
                _ => 1.0,
            };
            let step = ((target - out.integrated) / slope).clamp(-12.0, 12.0);
            // True peaks stay under the ceiling by the gap between them and the limiter's peaks.
            let gap = (out.true_peak_db - out.sample_peak_db).max(0.0);
            let margin = (prev.margin + (out.true_peak_db - ceiling).max(0.0)).max(gap).min(3.0);
            (Gains::split(here.0 + step, ceiling - margin + 24.0), margin, Some(here))
        }
        _ => (Gains::split(target - (raw(input.integrated) - comp_loss), ceiling - 0.5 + 24.0), 0.5, None),
    };
    // The sections see the programme raised by the new Input Gain.
    let at = |x: f64| raw(x) + gains.input;
    let i = at(input.integrated);

    let decomp = input.plr() < crest_t - 3.0;
    let floor = at(input.quiet);
    let expand = input.quiet.is_finite() && input.loud - input.quiet > 30.0;
    let exp_threshold = (floor + 6.0).clamp(-90.0, -20.0);
    let comp_threshold = (i + over).clamp(-60.0, 0.0).round();
    let lift_to = i - range_t;
    let deexpand = input.quiet.is_finite() && floor < lift_to - 6.0;
    let deexp_floor = if expand { exp_threshold } else { (floor - 12.0).clamp(-120.0, -20.0) };
    let lim_ceiling = ceiling - margin;
    // The limiter raises by ceiling - threshold.
    let lim_threshold = (lim_ceiling - gains.drive).clamp(-24.0, 0.0);

    let b = |on: bool| if on { 1.0 } else { 0.0 };
    let r = |x: f64| x.round() as f32;
    let tenth = |x: f64| ((x * 10.0).round() / 10.0) as f32;
    Some(Plan {
        settings: [
            ("decomp.on", b(decomp)),
            ("decomp.match", b(decomp)),
            ("decomp.matchTarget", r(crest_t.min(14.0))),
            ("decomp.ceilingOn", 1.0),
            ("decomp.ceiling", r(lim_ceiling.min(0.0))),
            ("gate.on", b(expand)),
            ("gate.mode", 1.0),
            ("gate.threshold", r(exp_threshold)),
            ("gate.ratio", 2.0),
            ("gate.range", 12.0),
            ("comp.on", b(compress)),
            ("comp.threshold", r(comp_threshold)),
            ("comp.ratio", ratio as f32),
            ("comp.detector", if crest_mode { 0.0 } else { 1.0 }),
            ("comp.autoAttack", 1.0),
            ("comp.autoRelease", 1.0),
            ("deexp.on", b(deexpand)),
            ("deexp.threshold", r(lift_to.clamp(-80.0, 0.0))),
            ("deexp.range", r((lift_to - floor).clamp(0.0, 12.0))),
            ("deexp.floor", r(deexp_floor)),
            ("limit.on", 1.0),
            ("limit.ceiling", tenth(lim_ceiling)),
            ("limit.threshold", tenth(lim_threshold)),
            ("input", r(gains.input)),
            ("output", r(gains.output)),
        ],
        gains,
        margin,
        point,
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
        let p = plan(&stats(-30.0, 20.0, -70.0, -20.0, -10.0), 0.0, None, -14.0, -1.0).unwrap();
        for (id, v) in p.settings {
            let info = crate::INFO.params.iter().find(|q| q.id == id).unwrap_or_else(|| panic!("{id}"));
            assert!(v >= info.min && v <= info.max, "{id} = {v} outside {}..{}", info.min, info.max);
        }
    }

    /// A quiet, wide, noisy recording: every section works, and the level is pushed up.
    #[test]
    fn quiet_wide_programme_uses_every_section() {
        let p = plan(&stats(-30.0, 20.0, -70.0, -20.0, -10.0), 0.0, None, -14.0, -1.0).unwrap();
        assert_eq!(get(&p, "comp.on"), 1.0);
        assert_eq!(get(&p, "deexp.on"), 1.0);
        assert_eq!(get(&p, "gate.on"), 1.0);
        assert!(p.gains.drive > 10.0 && p.gains.output == 0.0, "{p:?}");
    }

    /// A squashed master louder than the target: transients back, turned down, nothing pushed.
    #[test]
    fn loud_squashed_master_is_opened_and_turned_down() {
        let p = plan(&stats(-8.0, 3.0, -14.0, -6.0, -0.5), 0.0, None, -14.0, -1.0).unwrap();
        assert_eq!(get(&p, "decomp.on"), 1.0);
        assert_eq!(get(&p, "comp.on"), 0.0);
        assert!(p.gains.output < -5.0 && p.gains.drive == 0.0, "{p:?}");
    }

    /// A measured output corrects the gain by what the estimate missed.
    #[test]
    fn measured_output_corrects_the_drive() {
        let input = stats(-30.0, 6.0, -40.0, -26.0, -12.0);
        let first = plan(&input, 0.0, None, -14.0, -1.0).unwrap();
        let out = stats(-16.5, 6.0, -26.0, -12.0, -1.0);
        let second = plan(&input, 0.0, Some((&out, &first)), -14.0, -1.0).unwrap();
        assert!((second.gains.total() - (first.gains.total() + 2.5)).abs() < 1e-9, "{first:?} {second:?}");
    }

    /// More than the limiter's 24 dB of drive: the rest goes to Input Gain, and the sections'
    /// thresholds move up with it.
    #[test]
    fn gain_past_the_drive_goes_to_the_input() {
        let g = Gains::split(30.0, 24.0);
        assert_eq!((g.input, g.drive, g.output), (6.0, 24.0, 0.0));
        let g = Gains::split(30.0, 22.5);
        assert_eq!((g.input, g.drive, g.output), (7.5, 22.5, 0.0));
        let g = Gains::split(-30.0, 24.0);
        assert_eq!((g.input, g.drive, g.output), (-6.0, 0.0, -24.0));
        let input = stats(-44.0, 12.0, -80.0, -36.0, -26.0);
        let p = plan(&input, 0.0, None, -14.0, -1.0).unwrap();
        assert!(p.gains.input > 0.0);
        assert_eq!(get(&p, "comp.threshold") as f64, (-44.0 + p.gains.input + 4.0).round());
    }

    #[test]
    fn silence_has_no_plan() {
        let s = Stats { integrated: f64::NEG_INFINITY, ..stats(0.0, 0.0, 0.0, 0.0, 0.0) };
        assert!(plan(&s, 0.0, None, -14.0, -1.0).is_none());
    }
}
