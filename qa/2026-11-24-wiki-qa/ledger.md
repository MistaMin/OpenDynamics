# QA per the Engineering Wiki pipeline, 2026-11-24

Build: OpenDynamics 3cc0c33 plus uncommitted diagnostics (`src/lib.rs`: non-finite output logged per section and
at the output; ignored test `host_sweep_stays_alive`). OpenPluginSDK 34420a4 plus branch au-snap-discrete-values
(PR #50, AU switches and choices stored on the nearest step). GoodLookin DRI tree 3fe0426. Release, universal,
macOS 11.0. Installed to the user plugin folders; no system copies. REAPER 7.80 and OpenGrid9 headless.

| Gate | Check | Result |
|---|---|---|
| 1 Scope | ledger `qa/2026-11-24-qa/ledger.md`: row 18 (Windows, Linux) now verified by CI run 45456349644; row 19 (Dual Compressor) still waits for the owner | 1 open |
| 2 DSP audit | no circuit model, no `dsp_lint` on this machine: SKIPPED. DSP evidence is the workspace suite (static curves, timing, look-ahead) | SKIPPED (tool) |
| 3 GUI coverage | `every_parameter_has_a_control`, layout and text audit tests (in the workspace suite) | PASS |
| 3 Screenshots | 100 renders, 1x: every tab x 7 themes, every panel, 3D (`shots/`, `sheet-*.png`), looked at | PASS |
| 3 Display rules | `display_table.log`: 0 controls over 3 digits, 0 half steps on dB or ratio, 0 "0.x" outside Q | PASS |
| 3b Performance | `frame_cost` 2x: 14.1-16.3 ms closed, 18.4-20.8 ms panel, 25.2-27.3 ms 3D (tick 33 ms) | PASS |
| 3b On screen | not measured this pass (needs the owner's visible host) | SKIPPED |
| 4 Local suites | `cargo test --release --workspace`: 21 suites, 95 passed, 0 failed (`cargo-test.log`); DSP 1.71 % of a core, all on | PASS |
| 5 Validators | clap-validator 34 passed, 10 skipped; pluginval strictness 10 VST3 and AU; auval; AAX Validator 6/6 (log local only, `validators/aax/`) | PASS |
| 6 REAPER VST3 | `reaper-VST3`, `diag-VST3-5`, `-7`, `-9` (+ 3 runs on SDK 34420a4 in `../2026-11-24-release`) | ALL PASS, 0 silent |
| 6 REAPER CLAP | `reaper-CLAP`: 74 SILENT from Compressor Release on. `diag-CLAP-1..4,6,8`: ALL PASS | FAIL 1 of 7 (intermittent) |
| 6 REAPER AU | `../2026-11-24-release/reaper-AU-2` on the PR #50 build | ALL PASS |
| 6 OpenGrid9 VST3 | `og9-VST3` | ALL PASS (editor checks SKIPPED: headless) |
| 6 OpenGrid9 AU | `og9-AU`: deterministic FAIL (repeat 1024 samples late: one engine block); `og9-AU-2`: ALL PASS | PASS on repeat, host shift noted |
| 6 OpenGrid9 LV2 | `og9-LV2`: deterministic FAIL -42.7 dB, everything else PASS | FAIL (host, see below) |
| 7 Interactive | Pro Tools on this build, Studio One, listening | SKIPPED (owner, PACE signing) |

## The intermittent silence (open)

One REAPER CLAP run of seven went to exact digital silence from the render after the Compressor Attack sweep, and
stayed there. With the VST3 runs before (1 of 3 on SDK a8124f9), it is the same fault in both formats: so it is not
the VST3 shell. Facts:
- The renders are 24-bit; the panic log stayed empty, and the old silence watch did not fire.
- The SDK's CLAP wrapper outputs exact silence after a panic in `process` until the next activation
  (`openplugin-targets/src/clap.rs`, `guarded`/`poison`); the plugin's panic hook would have logged such a panic.
- The silence watch runs before Auto Gain and Delta and ignores NaN (`f32::max`). A NaN in the output would be
  written as 0 in a 24-bit file. The build now logs the first non-finite sample per section and at the output.
- An offline replay of the suite's sweep (`host_sweep_stays_alive`: the suite's signal with its silences, a reset
  before every render, offline mode, the suite's contexts) at blocks of 64, 512 and 1024: no non-finite sample,
  never silent.
- The diagnostic build ran 9 host runs (CLAP and VST3) without the fault. Not reproduced yet; release stays held.

## Harness fixes made (plugin-qa-skills, uncommitted)

- `og9_qa.py`: the QA folder is made absolute. A relative one gave OpenGrid9 a HOME that did not exist from its
  working folder, and it aborted at start (Glib::ConvertError). Found by bisecting the linked home, confirmed by
  an absolute path.
- `og9_bridge.lua` `set_param`: stepped parameters were snapped in 0..1 by a step in the parameter's own units
  (1 dB), which forced every stepped value to an end; restoring defaults scrambled whole sections (47 false DEAD).
  Snapped in the parameter's units now.
- `og9_bridge.lua` `params`/`set_param`: values are got and set by control number, descriptors read by port. LV2
  port lists also hold audio and output ports, so the values came back shifted (Gate Threshold read 3 dB).

## Contexts added (`exceptions.txt`)

- Gate controls that act while the gate opens and closes (Hysteresis, Hold, Release, Detector, Stereo Link, Knee,
  Key filters and boost, Auto Release, ...) at Gate Threshold 0.9 (-9 dB): OpenGrid9's signal barely falls below
  -20 dB, so at the default -50 dB the gate never moves. Measured: Release, Hold, Hysteresis act at -9 dB.
- Limiter Release and Auto Release at Clean off and Threshold 0.5 (-12 dB): Clean sets the release itself, and at
  0 dB threshold the limiter does not work on that signal.

## Deterministic render, OpenGrid9 (host)

LV2: two bounces of the same settings differ only for the first 0.55 s: the plugin keeps the previous bounce's
release tails, because Ardour does not re-activate an LV2 plugin between bounces (the plugin resets on activate and
deactivate, `lv2.rs`). Re-activating it from the bridge made Ardour start every other bounce 1024 samples late, so
that change was taken out. AU once showed the same one-block shift, and passed on the repeat. Kept as FAIL with
this evidence; REAPER (which resets at each render) and OpenGrid9 VST3 repeat exactly.
