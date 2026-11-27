# Scope ledger: conformance kit fixes, 2026-11-27

Uncommitted changes on `main` (after cfb017e) from the openplugin-dev test list (`/openplugin-dev:test-plugin`) and
the GUI coverage pass of 2026-11-26, built against OpenPluginSDK `ce6454f` (PR #57, the kit's latency settle, and
PR #56, VST3 edits right after setProcessing(true)). The shared SDK folder was at `19fac89`, so the build and the test list
ran from a copy of the working tree next to a checkout of `ce6454f` (session scratchpad). During the run another
session removed the shared folder (`/Volumes/Developer 2/Dependencies/OpenPluginSDK-main`); SDK `main` is now `1f608fe`
(PR #60, VST3 controller saves its values again; PR #59, fonts). macOS universal build, all five formats, installed to
the user plug-in folders (VST3, AU, CLAP, LV2; the 2026-11-26 bundles were kept in the scratchpad, which was wiped at 09:01 on 2026-11-27, so they are gone; AAX signed and installed to the system AAX folder, the 2026-11-24 test copy kept in `protools/installed-backup-1124`).

| # | Item | Code | GUI | Test evidence | Status |
|---|---|---|---|---|---|
| C1 | Zero Latency switches when its 5 ms dip ends, also inside a large host block (was a whole block of silence) | `process_buses`: the chunk ends where the dip reaches silence | - | `zero_latency_switch_completes_inside_a_large_block` (fails without the fix); kit `latency` over 24 + 200 seeds; REAPER: CLAP reports 487 -> 0 -> 487 samples as Zero Latency goes on and off, after one render each; VST3 follows one render later, the same with the 2026-11-26 build (`reaper-latency*`, see below) | verified |
| C2 | No allocation or file access on the audio thread (non-finite and silence diagnosis kept in a fixed buffer, written in `prepare` and on drop) | `note`, `flush_diag`, `diag` | - | kit `no_alloc`; T-LINT; `audio_thread_diagnosis_is_kept_until_prepare`; host runs: no diagnosis line was needed or written | verified |
| C3 | A denormal carried through a delay does not reach the output | flush at the end of `run` | - | kit `no_denormals` (strict, T-FUZZ) | verified |
| C4 | Hosts list the gate's parameters under "Gate & Expander"; parameter names unchanged | `info_for` group | - | kit `param_fuzz` `check_group`; `docs/PARAMETERS.md`; no scripted host here shows parameter groups (SKIPPED in a host) | verified (kit) |
| C5 | Conformance, soak, strict fuzz, host behaviour and bench tests in the crate | `tests/`, `examples/bench.rs`, `bench/baseline.json` | - | full tier: 6 of 6 rows pass on SDK `ce6454f` (T-LINT, T-CLIPPY, T-TEST, T-SOAK 24 seeds, T-FUZZ, T-BENCH under load 83-103) | verified |
| C6 | Test runs do not write to the owner's plugin log | `log_line`, `.cargo/config.toml` | - | `panic.log`: unchanged (5 lines, 2026-11-26 18:41) through three test tiers and the lib tests; host runs still write to it | verified |
| C7 | Every control writes only the parameter its label names | - | every tab and panel | `every_control_drives_its_own_parameter` (352 controls) | verified |
| C8 | Clippy clean | - | - | T-CLIPPY | verified |
| X1 | Default sound unchanged (renders null against the previous build) | - | - | REAPER base render against `qa/2026-11-25-auto-loudness-ara/reaper-*/base.wav`: VST3 and CLAP `base.wav` and `base2.wav` null at -240 dB | verified |
| X2 | GUI unchanged (clippy-only edits in `ui/`) | - | - | screenshots against 2026-11-26: 220 screenshots (1x and 2x, every tab, theme, panel, 3D) byte-identical | verified |
| X3 | Every parameter still changes the sound, shows in the GUI, survives save and reopen (VST3, CLAP, AU, LV2) | - | - | REAPER suite, OpenGrid9 suite: REAPER VST3 all PASS (112 parameters), REAPER CLAP all PASS (111), OpenGrid9 AU all PASS (108), OpenGrid9 LV2 all PASS but the repeat render (known host behaviour, below) | verified |
| X4 | Validators | - | - | auval, pluginval 10, clap-validator, AAX Validator: auval SUCCEEDED (2 warnings, below); pluginval 10 VST3 and AU SUCCESS; clap-validator 34 passed, 0 failed, 10 skipped; AAX Validator 6 of 6 (`aax2/`, not for publication) | verified |
| X5 | Pro Tools (AAX): loads, processes, reports the Zero Latency change, keeps it through save and reopen | - | - | Pro Tools 2025.10, AAX Native, PACE-signed test copy (wrap GUID of the profile), installed binary SHA-256 7599b5e8...; the 2026-11-24 test session reopened with this build: settings restored (stereo Threshold -18 dB GR 9 dB, mono -30 dB GR 7 dB), levels as on 2026-11-24 (-16.0, -14.7, -28.5 dBFS). Session system delay 804 samples; Zero Latency on in both instances: 317 (487 less), at once, no render; off and on again follows each time. Saved, closed, reopened: 317 at load, Zero Latency still on, same levels. Bypass reads the source (-6.0). Editor drags reach the audio live (second run, after a report of stepped parameters playing a wrong value live in another plug-in): Threshold -18 to -30 dB (stepped): GR 18, track -25.0; Make-Up +4: -21.0; Output Gain -5 (49 steps): -26.0, each exactly as expected. No stuck-plugin or engine error in the log (`protools/`, local only) | verified |

Fixed during this pass: the doc comment of `log_line` had ended up above `note` (`src/lib.rs`); the diagnosis buffer
had no test of its own (`audio_thread_diagnosis_is_kept_until_prepare`).

## Findings

- **VST3 latency in REAPER follows one render late (pre-existing).** Zero Latency on, render: REAPER still reports
  487 samples, also after idle time; after a second render it reports 0. Off again: 0 after one render, 487 after two.
  The 2026-11-26 build (SDK `19fac89`) does the same; the CLAP follows after one render. The shell sends
  `restartComponent(kLatencyChanged)` from its main-thread timer (SDK `native/vst3/changes.cpp` proves it reaches a
  host); REAPER seems to apply it at the next activation. Not traced inside REAPER; live playback not tested.
- **auval: Compressor Make-Up "did not retain default value" (pre-existing).** Set 0 dB, read -0.000000: a float round
  trip on a -12..24 dB parameter without a step. Seen in 2 of 5 auval runs (also `2026-11-24-wiki-qa`). The other
  warning is the known Tail Time one.
- **OpenGrid9 LV2 repeat render (pre-existing host behaviour).** The two bounces differ only in the first 0.62 s and
  match to -118 dB after it: Ardour does not re-activate an LV2 plugin between bounces (`2026-11-24-wiki-qa`).
- **The plugin log fills with ARA selection lines.** 71 lines of "ARA selection: n clip(s)" from today's REAPER runs,
  one per selection change; from the ARA work, not from this change. Main thread, not the audio thread.
- **Zero Latency switch: a short level overshoot after it (pre-existing, by design of the switch).** In Pro Tools the
  peak hold read -8.2 dBFS on the stereo track (steady -16.0) and -24.0 on the mono track (steady -28.5) after switching;
  steady levels are right in both modes and the limiter ceiling held. `apply_zero_latency` (`src/lib.rs:597`) resets every
  section, so the compressor starts with no gain reduction and its attack (10 ms) pulls the level down again after the 5 ms
  fade-in. Not changed today (`git diff cfb017e` touches only the chunk length there). Keeping the detector state across the
  switch would remove it; not done without the owner.
- **Harness:** OpenGrid9's AU lookup (`auval -a`, 300 s limit) timed out twice at load 110-150. The AU was indexed
  with `ardour-au-scanner -f aufx ODyn Ogrd` instead, then the suite passed. The AAX Validator's parameter test did not
  run once because another session held its port; the rerun passed 6 of 6.

## Not done

- Studio One: on-screen driving needs the owner's OK. Pro Tools: automation, presets, offline bounce, other sample rates
  and buffer sizes not run (X5 covers load, processing, latency, recall, bypass).
- GUI performance and meter ballistics: no painting, meter or readout change.
- Windows and Linux: built by the workflow after a push.
- Live playback latency change in REAPER (only offline renders were scripted).
