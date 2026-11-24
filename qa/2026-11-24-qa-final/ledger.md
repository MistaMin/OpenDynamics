# OpenDynamics QA (final build), 2026-11-24

Pipeline: Engineering Wiki "Processes: QA pipeline"; the plugin-qa project profile for OpenDynamics.
Build: release, macOS universal, uncommitted source (no git repository), OpenPluginSDK main fc294e7, GoodLookin DRI 3fe0426.
Installed bundles checked against `dist/` (SHA-1 equal: VST3 56cd8093, AU 46bedf6d, CLAP 7d3dc1f8) before testing.
Owner answers as recorded in `qa/2026-11-24-qa/ledger.md` (installs, visible REAPER, Delta, whole steps).
Scope ledger: `qa/2026-11-24-qa/ledger.md` (19 items; 17 verified, Windows/Linux unverified, Dual Compressor open)
plus since then: decompressor first in the chain, 3D VIEW right-aligned, editor optimisation round.

## Checks

| Gate | Check | Result |
|---|---|---|
| 3 GUI coverage | every parameter has a control; no overlap; panels inside the display; chip-row readouts clear of chips and 3D VIEW at their widest | PASS |
| 3 Text audit | 8 pt minimum, no clipped/overlapping text, no "-0" (static + live, 2D + 3D, every panel) | PASS |
| 3 Display rules | `display-table.txt`: min/mid/max of 75 controls: <= 3 digits, whole dB/ratio, no 0.x | PASS (exception: Key Boost Q, EQ control) |
| 3 Meter ballistics | IN/OUT sample-peak meters: full scale 0 dB, -6 reads -6, 24 dB/s fall at 30 and 60 Hz ticks, louder peak takes over (`meters_read_and_fall_as_specified`) | PASS |
| 3b GUI performance | frame_cost 2x (full history and terrain, 30 Hz): normal 14.4-16.4 ms, panel open 18.5-20.7 ms, 3D 25.2-27.3 ms | PASS (< 33 ms) |
| 3b On screen | REAPER 1x: editor open +4.5 % of a core over closed (was +17 %); 3D with live data SKIPPED (isolated REAPER could not open the audio device) | partly SKIPPED |
| 4 Local suites | `cargo test --release --workspace`: 21 suites, 92 tests (`cargo-test.log`) | PASS |
| 4 Build | release, universal, 0 Rust warnings | PASS |
| 4 DSP cost | 1.7 % of one core, all sections on, 64-sample blocks | recorded |
| 5 clap-validator 0.4.1 | 34 passed, 0 failed, 0 warnings, 10 skipped | PASS |
| 5 auval | `aufx ODyn Ogrd` | PASS (1 warning: Tail Time) |
| 5 pluginval 1.0.4 strictness 10 | VST3 with editor tests; AU with editor tests (new) | PASS, PASS |
| 5 AAX Validator 2024.6 | 6/6 | PASS |
| 6 REAPER CLAP (new) | loads (109 params), finite, not silent, deterministic, editor, every parameter acts, every parameter shown, state, 5 reopen cycles; 0 silent renders | ALL PASS |
| 6 REAPER VST3 | run 1 (`reaper-VST3/`): FAIL, output exact digital silence from the render after Gate On's editor sweep on, 200 silent renders; run 2 (`reaper-VST3-run2/`) and run 3 (`reaper-VST3-run3/`): ALL PASS, 0 silent | intermittent FAIL (see below) |
| 8 Pro Tools | AAX unsigned; PACE GUID now known (3C6EF8C0-E7F7-11F1-AC54-00505692C25A), signing needs the owner's iLok | SKIPPED |
| 7/8 Logic, Studio One, listening | not run | SKIPPED |

## Open finding: intermittent VST3 silence in REAPER

- Seen in 2 of 5 full VST3 suite runs today, never in CLAP (1 run), never in 3 short VST3 replays with the editor sweep, never
  in the in-process extremes test.
- Signature: every render before is normal; from one render on, the output is exact digital zero (-240 dB) for the rest of
  the run, also with the triggering section switched off. Both times right after a parameter changed through REAPER while
  the editor was open (Gate Range max in run 1 of the morning; Gate On's visual step today).
- Not a panic: the panic hook logged nothing.
- Diagnostic added (`watch_silence`): when the input has signal and the output has been exactly zero for 1 s, the state that
  can silence the plugin (section on/fade, Delta and its mix, Zero Latency dip and switch, latency, gains, rate, block) is
  written once to `~/Library/Logs/OpenDynamics/panic.log`. Runs 2 and 3 with it passed, so no capture yet.
- Not fixed. Not a release blocker only if the owner accepts it as unexplained; it should be chased with the log first.
