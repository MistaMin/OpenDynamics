# OpenDynamics QA, 2026-11-24

Pipeline: Engineering Wiki "Processes: QA pipeline" (skills plugin-qa, plugin-gui-coverage, plugin-gui-performance,
plugin-daw-test). Build: release, macOS universal (x86_64 + arm64), uncommitted source (no git repository yet),
OpenPluginSDK main fc294e7, GoodLookin DRI main 3fe0426. Formats: VST3, AU, CLAP, LV2, AAX (unsigned).

## Owner answers (2026-11-24)

| Question | Answer |
|---|---|
| Installs | User plug-in folders (VST3, AU, CLAP, LV2) and AAX in /Library |
| REAPER | Visible instance allowed for on-screen checks |
| Delta | Add it (plugin-wide, next to Auto Gain) |
| Knob steps | Whole steps: dB controls in whole dB, ratios in whole steps, at most 3 digits |

## Scope ledger

Every item the owner asked for in this project, with code, GUI and test evidence.

| # | Item | Code | GUI | Evidence | Status |
|---|---|---|---|---|---|
| 1 | Five sections: Gate/Expander, Compressor, De-Expander, Decompressor, Limiter | `crates/dynamics`, `crates/dyn-*` | five tabs with power dots | 21 suites; REAPER sweep | verified |
| 2 | Real curves, pure gain, "cleaner than clean" | gain multipliers, 20 ms level hold | transfer curve | steady-tone THD -150..-180 dB (comp, limiter Clean, zero latency, auto timing) | verified |
| 3 | Look-ahead for gate/comp/de-exp/decomp, landing on the onset | hold + moving-average ramp | Look-Ahead sliders | `lookahead_lands_the_gain_on_the_onset`, `lookahead_opens_before_the_onset`, timing tests | verified |
| 4 | Simple limiter, no look-ahead | `limiter.rs` | Limiter tab | `no_sample_passes_the_ceiling_with_attack_zero`, latency 0 | verified |
| 5 | Zero Latency switch | `set_zero_latency` | ZERO LATENCY pill | `zero_latency_reports_and_is_zero`, `zero_latency_switch_does_not_click` | verified |
| 6 | Limiter Clean mode (Pure's bass-adaptive post-hold) | `limiter.rs` | CLEAN pill, Bass Hold | `clean_bass_gets_no_harmonics` (-37 -> -175 dB at 40 Hz), `clean_keeps_the_ceiling_too` | verified |
| 7 | Clean: auto release only | crest rule | Release knob faded in Clean | `auto_release_follows_the_crest` (output identical for any Release) | verified |
| 8 | Auto attack and release, every section | `crates/dynamics/src/auto.rs` | TIMING panel switches | `scales_span_the_harmonic_compressor_ranges`, THD with auto on | verified |
| 9 | Auto Gain | `src/autogain.rs` | AUTO GAIN pill + readout | `matches_the_input_loudness` (9.0 dB), `off_is_a_wire` | verified |
| 10 | Delta (QA answer) | `src/lib.rs` | DELTA pill | `delta_is_the_difference` (normal - delta = input delayed, < 1e-5; all off = silence) | verified |
| 11 | External sidechain + key HP, LP and boost at once | `keyboost.rs`, detectors | SIDECHAIN panel | `boosts_its_frequency_and_leaves_the_rest`, `external_key_through_boost_hp_and_lp` (6.0 dB GR) | verified in code; host sidechain routing not tested (no key signal in the REAPER suite) |
| 12 | GUI: main knobs, settings in panels, curve inside the history | `src/ui/mod.rs` | all tabs | coverage + overlap tests, screenshots | verified |
| 13 | History larger, knobs moved down | layout constants | display 352 + lane 56 px | screenshots | verified |
| 14 | Spectrograph lane (energy change by frequency) | taps + analyser | lane | `spectrum_taps_line_up` | verified |
| 15 | 3D spectrum view, full band, energy colours, one GR colour, light well in light themes | `src/ui/view3d.rs` | 3D VIEW button | screenshots all themes; frame_cost 3D 14.5-16.7 ms at 2x | verified |
| 16 | Themes: Slate, Neon (midnight blue), Rose (white) | `theme.rs` | THEME button cycles 7 | render test, text audit, screenshots | verified |
| 17 | Whole steps on dB and ratio (QA answer) | `whole_steps` in `src/lib.rs`, decomp ratio is x of 1:x | whole-number display | `dynamics_controls_move_in_whole_steps`, display table | verified |
| 18 | Mac, Linux, Windows | `scripts/build.sh`, `.github/workflows/build.yml` | - | macOS built; Windows/Linux workflow never run (no git repository) | unverified |
| 19 | Dual Compressor question | - | - | owner has not answered | open |

## Changes made during this QA

| Change | Why | Evidence |
|---|---|---|
| Delta (plugin-wide) | QA answer | `delta_is_the_difference` |
| Whole steps on every dB and ratio control; Limiter Ceiling default -1 dB; Decompressor ratio stored as x of 1:x (1..4) | QA answer | `dynamics_controls_move_in_whole_steps`, display table |
| Times under 1 ms read in microseconds, zero as "0 ms" | display rule 3 ("no 0.x"): Attack minimum showed "0.0 ms" for 0.01 ms | display table |
| Panic log `~/Library/Logs/OpenDynamics/panic.log` | a caught panic left no trace while the first REAPER run went silent | - |
| Decompressor first in the chain, tabs in chain order (`CHAIN`) | owner, during QA | `chain_order`, screenshot |

## Checks

| Gate | Check | Result |
|---|---|---|
| 3 GUI coverage | every parameter has a control (all tabs, panels open and closed) | PASS (`every_parameter_has_a_control`) |
| 3 GUI coverage | no overlap, inside page and display, every theme renders | PASS |
| 3 Text audit | 8 pt minimum, no clipped/overlapping text, no "-0" (static + live, 2D and 3D, every panel) | PASS |
| 3 Display rules | min/mid/max of every control: <= 3 digits, whole dB/ratio, no 0.x | PASS, one exception (below) |
| 3b GUI performance | frame_cost 2x: 2D 5.7-7.7 ms, panel open 9.5-9.9 ms, 3D 14.5-16.7 ms (tick 33 ms) | PASS |
| 3b GUI performance on screen | host CPU / GPU per tab in a visible REAPER | see below |
| 4 Local suites | `cargo test --release --workspace`: 21 suites | PASS |
| 4 Build | release, universal, 0 Rust warnings | PASS |
| 4 DSP cost | all sections on, 48 kHz, 256 blocks: 1.69 % of one core (1.37 % zero latency), load 12.7 | recorded |
| 5 clap-validator 0.4.1 | 44 tests: 34 passed, 0 failed, 0 warnings, 10 skipped (preset discovery, note ports: not implemented) | PASS |
| 5 auval | `aufx ODyn Ogrd` | PASS (1 warning: Tail Time not reported) |
| 5 pluginval 1.0.4 strictness 10 | VST3, AU, VST3 with editor tests | PASS, PASS, PASS |
| 5 AAX Validator 2024.6 | 6/6 E_COMPLETED_PASS on the unsigned dist bundle (warning: gaps between registered fields in the algorithm context, from the SDK's AAX wrapper) | PASS |
| 8 Pro Tools | unsigned AAX; release Pro Tools loads PACE-signed AAX only | SKIPPED (needs iLok signing); /Library install not done for that reason |

## Exceptions

| Exception | Reason |
|---|---|
| Key Boost Q shows 0.30 at its minimum | display rule 3 allows 0.x on an EQ control the user types into; the key boost is a bell EQ on the sidechain |
| REAPER audio sweep: Key Source excused | External needs a sidechain signal the REAPER test does not provide; covered by `comp::external_key_through_boost_hp_and_lp` |
| REAPER visual sweep: parameters in closed panels and other tabs read NOT SHOWN | the sweep captures only the open page; every parameter's control is proven by `every_parameter_has_a_control` over every tab and panel |

## Host suite (REAPER 7.80, VST3, isolated instance)

| Run | Result |
|---|---|
| 1 (`reaper/`, build before Delta-order and chain change) | loads, finite, deterministic, editor, state, reopen PASS. From the render "Gate / Expander Range = max" on, every render was silent (also with the gate off), so 96 parameters were never heard: FAIL as reported, not excused |
| Diagnosis | replayed Gate On + Range max directly (normal -7.7 dBFS), replayed the first 11 parameters through the suite with and without the visual sweep (all heard), in-process test `extremes_never_kill_the_output` (every parameter's min and max, every section on: output finite and alive). Not reproduced. Panic log added; empty in every later run |
| final (`reaper-final/`, chain decompressor first) | ALL PASS: loads (110 params), finite (-7.7 dBFS), not silent, deterministic (-240 dB), editor opens, every parameter changes the sound (78 ok, 28 CONDITIONAL, 4 Key Source excused), every parameter shown, state round trip, 5 editor reopen cycles. 0 silent renders |

Open: run 1's silence is unexplained (one occurrence, three clean reruns). Watch `~/Library/Logs/OpenDynamics/panic.log`.
The final suite ran before the KEY IN readout fix (readout only; `key_in_needs_signal`); validators were rerun after it.

## On screen (gate 3b), REAPER 7.80 VST3, Paper theme, looping test signal, loaded binary checked with lsof (~/Library user folder)

| State | REAPER CPU % of one core | WindowServer % | GPU % |
|---|---|---|---|
| editor closed | 14.4 | 32.3 | 1.6 |
| each tab, normal view | 30.8-31.3 | 34.3-35.0 | 1.6-1.7 |
| Compressor, 3D view | 68.0 | 37.2 | 1.7 |

Load average 6.5 -> 6.6. The 2D editor costs about 17 % of a core (OpenLimiter 1.0.1: about 15 %). The 3D view costs about 54 %:
FINDING, not fixed (proposal: draw the terrain at half the rate and redraw only its rows' band, or fewer rows).
Display scale not recorded (1x or 2x).

## Finding fixed after the host run

| Finding | Fix | Evidence |
|---|---|---|
| KEY IN lit with nothing routed to the sidechain (REAPER hands over the 2/4 pins, silent) | key counts as present only when it carries signal, held 1 s | `key_in_needs_signal`; validators rerun: clap 34/0/0, auval PASS, pluginval VST3 (with editor) and AU PASS, AAX 6/6 |

## Not done / needs the owner

- Pro Tools: needs PACE signing (iLok). AAX not installed to /Library for that reason.
- Logic, Studio One, interactive listening: not run.
- CLAP, AU and LV2 in REAPER: validators only, no REAPER suite run on those formats.
- Windows and Linux builds: workflow written, never run (no git repository).
- Dual Compressor question: unanswered.

## Optimisation round (owner: "the dsp is way too heavy", wiki optimisation gate + GUI performance page)

Profile first (`dsp_profile`, 48 kHz, 64-sample blocks, 20 s): all off 1.8 us/block, one section 5.1-7.6 us, all five
22.6 us = 1.7 % of one core. The load seen in REAPER (31 % editor open, 68 % in 3D, 14 % closed) was the editor.

Exact changes (same pixels or same output):

| Change | Evidence |
|---|---|
| History by elapsed time, 30 entries/s (the SDK's Mac editor ticks at 30 Hz, so 240 entries had shown 8 s labelled 4 s; 3D depth 6 s labelled 3 s) | axis now true; half the history points |
| 3D: each row fills only down to the ridge in front (occlusion) | pixel compare vs no occlusion, all 35 3D shots at 2x: mean diff 0.0009, at most 82 isolated pixels > 8 under ridge lines; terrain fill 59.8 -> 37.2 ms |
| 3D: terrain drawn into an image of the display 10 times a second, copied on other frames | ordinary 3D frames ~10 ms, row frames ~61 ms |
| Tabs and meter tracks in the cached layer | -1.5 ms per frame |
| Cache key is a hash of its parts (theme 4 collided with the 3D bit) | bug fix |
| Rejected: terrain in the cached layer (a cached-layer rebuild costs ~100 ms at 2x); clip around the cached layer (slower) | measured, reverted |

frame_cost at 2x, full history and terrain, 30 Hz (load 6-12): normal 14.4-16.4 ms (16.0 before this round's last
changes), panel open 18.5-20.7, 3D 25.2-27.3 (31 with occlusion alone; the unoptimised full terrain was ~70 ms of
fills alone). On screen: 1x displays; editor-open Compressor 22.1 % vs closed 17.6 % (+4.5 %, was +17 %);
the 3D on-screen run was SKIPPED (the isolated REAPER could not open the audio device, so no live data).
Validators after: clap-validator 34/0/0, pluginval VST3 strictness 10 with editor SUCCESS.

Left (needs the owner): history line strokes are the biggest normal-view cost (DRI tests every segment per pixel;
a DRI change needs the owner's OK); the faint history over the curve square could go (look change); DSP exact
micro-optimisations exist but the DSP is already 1.7 %.
