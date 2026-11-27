# Scope ledger: OpenPluginSDK d93d926, 2026-11-27

OpenDynamics at 55fd05f (the conformance fixes, QA'd on SDK ce6454f in `qa/2026-11-27-conformance-fixes`), rebuilt
against OpenPluginSDK main d93d926: PR #59 (GoodLookin DRI fonts, bundled and from the system), PR #60 (VST3
controller saves its values again), PR #61 (macOS shells export only their entry points and dead-strip the rest).
macOS universal build, all five formats, installed to the user plug-in folders (VST3, AU, CLAP, LV2; the 55fd05f
bundles kept in `scratch/installed-backup-ce6454f`) and, PACE-signed, to the system AAX folder for Pro Tools.

| # | Item | Code | GUI | Test evidence | Status |
|---|---|---|---|---|---|
| S1 | Builds on d93d926: VST3 (with ARA), AU, CLAP, LV2, AAX, arm64 and x86_64, macOS 11.0 | `scripts/build.sh` | - | build log; `lipo -archs` and `otool -l` minos 11.0 on every binary; `Cargo.lock` unchanged | verified |
| S2 | The native formats export only their entry points and still load everywhere (PR #61) | SDK `native/CMakeLists.txt` | - | `nm -gU`: VST3 `GetPluginFactory`, `bundleEntry`, `bundleExit`; AU `OpenPluginAUFactory` (the Info.plist `factoryFunction`); AAX the seven `ACF*` entry points; AAX binary 3.5 MB (9.5 MB on ce6454f). Loads in auval, pluginval, AAX Validator, REAPER, OpenGrid9 and Pro Tools (below) | verified |
| S3 | The editor draws as before after the fonts change (PR #59) | SDK `crates/goodlookindri` | every tab, theme, panel, 3D | 220 screenshots (1x and 2x) on d93d926 byte-identical to the same tests on a ce6454f export at 55fd05f; text audit in T-TEST | verified |
| S4 | VST3 state survives save and reopen with the controller change (PR #60) | SDK `native/vst3` | - | REAPER VST3 suite: state survives save and reopen PASS, 112 parameters | verified |
| S5 | Default sound unchanged | - | - | base renders against `qa/2026-11-27-conformance-fixes`: REAPER VST3 and CLAP `base.wav`, `base2.wav` and OpenGrid9 AU null at -240 dB; OpenGrid9 LV2 `base2.wav` null, `base.wav` -42.7 dB (the host's first-bounce carry-over, below) | verified |
| S6 | Test list on d93d926 | `tests/`, `examples/bench.rs` | - | full tier 6 of 6 (T-LINT, T-CLIPPY, T-TEST, T-SOAK, T-FUZZ, T-BENCH); T-LINT first failed on the ce6454f SDK copy this pass had put under `qa/` for the screenshot baseline (9 hits, all in that copy, none in OpenDynamics), passed once the copy was moved out | verified |
| S7 | 55fd05f's fixes still hold on d93d926 (C1-C8 of the conformance ledger) | - | - | T-TEST, T-SOAK, T-FUZZ; Pro Tools: Zero Latency on both instances takes the session delay from 804 to 317 samples at once and back to 804 | verified |
| S8 | Validators and hosts | - | - | auval SUCCEEDED (the 2 known warnings); pluginval 10 VST3 and AU SUCCESS; clap-validator 34 passed, 0 failed, 10 skipped; AAX Validator 6 of 6 (`aax/`, `aax/rerun/`, not for publication; 2 tests waited for another session's validator); REAPER VST3 all PASS; REAPER CLAP run 2 all PASS (111), run 1 FAIL (below); OpenGrid9 AU all PASS (108); OpenGrid9 LV2 all PASS but the repeat render (known host behaviour) | verified |
| S9 | Pro Tools (AAX): loads, keeps saved settings, a stepped knob reaches the audio live | - | - | Pro Tools 2025.10, AAX Native, PACE-signed test copy; the 2026-11-24 session: stereo -16.0, bus -14.7, mono -28.5 dBFS as before; mono Threshold dragged to -39 dB: GR 14, track -35.3 (expected -20 - 14.25 - 1 = -35.25); no engine error in the log (`protools/`, local only) | verified |

## Findings

- **REAPER CLAP, run 1: every render silent from the 79th parameter on (unexplained, did not repeat).** From the
  context render of Decompressor Look-Ahead (before that control moved) to the end of the run, every render was digital
  zero, also with the Decompressor off and in the five random states. The editor still followed parameter changes, but
  its input meter was empty, so no audio reached the plug-in's processing (or it was not called). The plug-in wrote no
  diagnosis line: its silence check fires only when the input has signal. A full rerun of the same build in a new
  REAPER instance passed every check (`reaper-CLAP-2`), and a run of the parameters around the stop passed
  (`reaper-CLAP-repro`). The SDK update does not touch the CLAP audio path: outside the DRI fonts it changes only the
  native VST3, AU and AAX code. Load was 120-160 and another session's REAPER test instance was running. This is the
  signature of the open finding of `qa/2026-11-24-qa-final` (intermittent silence, then seen in VST3 only), now in CLAP.
  New in this capture: the input meter is fed from `process` (`src/lib.rs:438`), so the plug-in saw no input signal or
  was not called. Note for the next capture: the silence diagnosis now waits in a buffer until the plug-in is prepared
  again or dropped, and the REAPER harness kills REAPER 2 s after asking it to quit, so a diagnosis can be lost on a
  slow quit. Not traced further; watch for a repeat.
- **OpenGrid9 LV2 repeat render (pre-existing host behaviour).** The two bounces differ from the start, falling to -125
  dB by 0.6 s and exact from 1.7 s, as on 2026-11-24 and this morning: the host does not re-activate an LV2 plug-in
  between bounces.
- Carried from `qa/2026-11-27-conformance-fixes`, unchanged by this update: VST3 latency in REAPER follows one render
  late; auval's Compressor Make-Up "-0.000000" warning; ARA selection lines in the plug-in log; a short level jump after
  a Zero Latency switch.
- Pro Tools' settings had been reset before this run (another Pro Tools, started 09:48, not this session's). It asked
  for the audio device (Pro Tools Aggregate I/O, as before) and about usage data: answered No, as on the first reset today.

## Not done

- Windows and Linux: built by the workflow after a push.
- Editor speed on screen: the fonts change alters no drawing here (screenshots identical); no on-screen measurement.
- Studio One (needs the owner's OK); live playback latency in REAPER.
