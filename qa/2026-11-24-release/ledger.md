# QA, 2026-11-24, release build on OpenPluginSDK main

Build: OpenDynamics 3cc0c33, OpenPluginSDK 34420a4 (includes #44 VST3 block size, #40 VST3 controller values),
GoodLookin DRI tree 3fe0426. Release, universal (x86_64 arm64), macOS 11.0. Installed to the user folders
(VST3 SHA-256 2c9cad6fdb75bb92...); no copies in /Library/Audio/Plug-Ins. REAPER 7.80, hidden test instances.

| Step | Check | Result |
|---|---|---|
| 4 | `cargo test --release`: 22 passed, 7 ignored | PASS |
| 4 | build: five formats, codesign, both slices, no build paths | PASS |
| 5 | auval `aufx ODyn Ogrd` | PASS |
| 5 | clap-validator 0.4.1: 34 passed, 0 failed, 10 skipped (`validators/`) | PASS |
| 5 | pluginval 1.0.4 strictness 10, VST3 and AU | PASS, PASS |
| 5 | AAX Validator | SKIPPED (needs the PACE-signed copy; last run 6/6 on the previous build) |
| 6 | REAPER VST3, three runs (`reaper-VST3-2/vst3`, `reaper-VST3-3`, `reaper-VST3-4`): every check, 0 silent renders | ALL PASS |
| 6 | REAPER CLAP (`reaper-VST3-2/CLAP`) | ALL PASS |
| 6 | REAPER AU (`reaper-VST3-2/AU`): all PASS except the state check | FAIL (fixed, see below) |
| 6 | REAPER AU after the fix (`reaper-AU-2`), SDK branch au-snap-discrete-values (PR #50) | ALL PASS |
| CI | GitHub Actions 45456349644 at 3cc0c33: Windows x64 (installer) and Linux x86_64 (archive) | PASS |

VST3 silence: run 1 of `reaper-VST3-1` on SDK a8124f9 went silent from De-Expander Range on (58 silent rows).
On 34420a4 three runs in a row had none. The earlier fault was intermittent (1 of 3 runs on the build before),
so three clean runs make the fix likely, not proven.

AU state check: 18 switches and choices read "off -> on" (one "Peak -> RMS"). The suite compares REAPER's
displayed text. For AU Boolean and Indexed parameters REAPER makes the text itself and rounds down (switch at
0.6 reads "off", measured with `params`), while the plugin rounds to the nearest step (SDK `openplugin-core`
lib.rs:529, `v.round()`), stores 1 and reads "on" after reopen. The unit kept the 0.6 it was given, so the
host's label and the plugin disagreed. Fix in the SDK AU shell (`native/au/OpenPluginAU.cpp`, `snapDiscrete`): SetParameter
stores discrete values on the nearest step. `au_text_proof` checks it (fails without the change, passes with it; 11 of
11 `au_*` proofs pass). OpenPluginSDK PR #50; OpenDynamics must be re-pinned to its merge commit.

Not run: Pro Tools on this build, Logic, listening, on-screen GPU measurement.
