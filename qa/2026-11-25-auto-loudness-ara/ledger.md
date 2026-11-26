# Scope ledger: Auto Loudness mode and ARA, 2026-11-25

Branch `feature/auto-loudness-ara` (OpenDynamics), PR into `main` when it proves good. ARA host plumbing goes to an
OpenPluginSDK branch and its own PR. ARA SDK 2.3.0 (Celemony, Apache 2.0) at `/Volumes/Developer/Dependencies/ARA_SDK`: ARA_API, ARA_Library and
ARA_Examples (with `TestHost`, an ARA command-line host for unattended tests); main has nothing past 2.3.0.
Reference only (AGPLv3 / commercial, no code copied into MIT repositories): Celemony's JUCE_ARA at
`/Volumes/Developer/Dependencies/JUCE_ARA` (828314b): `juce_audio_plugin_client_ARA.cpp`, the VST3 and AU wrappers,
`juce_ARAAudioReaders`. Its AAX wrapper has no ARA either.

Licences: the ARA SDK (API, Library, Examples) is Apache 2.0, so ARA_Library code may go into OpenPluginSDK (MIT)
and the binaries, with the Apache licence and Celemony's NOTICE shipped, their headers kept and changed files marked;
THIRD_PARTY_LICENSES.md and the installers get the entry when ARA ships.

Owner's request: "an integrated auto loudness mode based on LUFS and the signal the user feeds the plugin where it
auto decompresses, expands, compresses, de-expands and limits the content, and it should be ARA."
Owner's answers: feature branch + PR; target LUFS + true-peak ceiling; full ARA editing; ARA in VST3, AU, CLAP and AAX.

| # | Item | Code | GUI | Test evidence | Status |
|---|---|---|---|---|---|
| A1 | Auto mode switch, Target (LUFS) and Ceiling (dBTP) parameters | `autoTarget`, `autoCeiling` after the sections | LOUDNESS button in the section bar opens the AUTO LOUDNESS panel (Target and Ceiling sliders) | `every_parameter_has_a_control`, text and layout audits over the panel, `loudness_button_keeps_clear_of_the_section_bar`, screenshots `*-auto-*` | verified |
| A2 | Measure the input: integrated loudness (BS.1770, gated), loudness range, crest/peak, noise floor | `src/loudness.rs` | - | `loudness::tests`: EBU Tech 3341 cases 1 and 3 (-23.0 LUFS +-0.1), Tech 3342 LRA 10 LU, true peak between samples, silence | wired (`Meter` in `run` while learning, published to `Shared`) |
| A3 | Set all five sections from the measurement to reach the target (decompress, expand, compress, de-expand, limit) | `src/auto.rs` `plan` | - | `auto::tests`: ids and ranges, quiet/wide, loud/squashed, measured correction, silence | code done, not wired |
| A4 | Without ARA: a Learn pass over the played audio, then hold the settings | `Shared::set_learning`, meters in `run` | LEARN / APPLY buttons, input and output readouts | end-to-end test `auto_loudness_reaches_the_target` | verified (offline); host Learn not run by hand |
| A5 | Result hits the target within a tolerance on test material (offline render, integrated LUFS and true peak measured) | `auto_loudness_reaches_the_target` | - | 30 s quiet, wide programme (-37.8 LUFS): -14/-1: -14.87 LUFS, -1.43 dBTP in 4 passes; -23/-2: -23.00 in 1; -16/-1: -16.28, -1.41 dBTP in 3 | PASS (within 1 LU, true peak at or under the ceiling) |
| A7 | Very loud targets under a tight true-peak ceiling (-9 LUFS, -0.5 dBTP) | - | - | ignored test `auto_loudness_loud_target_tight_ceiling`: stuck near -12 LUFS: hard limiting of bright material puts true peaks 2.75 dB over the limiter's sample peaks | OPEN: needs a true-peak (oversampled) limiter mode |
| A6 | The sections' own controls show what Auto set; the user can still change them | `apply_plan` writes every setting through `EditorHost::set` | the controls move | `loudness_panel_applies_an_ara_clip` | verified |
| R1 | SDK: ARA factory and document controller over a C interface, VST3 shell (ARAVST3.h) | OpenPluginSDK `ara-vst3`: `native/vst3/ara_glue.{h,cpp}`, `OpenPluginVST3.cpp`, `--ara-sdk` | - | ARA TestHost 2.3.0: 12 test cases, 0 API violations (`testhost/all-final.log`) | verified |
| R2 | SDK: read audio sources ahead of playback (ARA audio reader) | `Source::read` on sample access, `op_ara_analyze` | - | TestHost and REAPER: analysis lines in the plugin log (clip.wav -25.8 LUFS; REAPER testsignal.wav -11.1 LUFS) | verified |
| R3 | SDK: playback renderer / editor renderer roles for the plugin instance | `Renderer::render`; renderer-only instances play clips unprocessed, editor instances process | - | TestHost playback renders the clip (RMS 0.026210 = source 0.026206); REAPER binds two instances (roles 1 and 6) and the default render nulls with 1.0.0 Beta at -240 dB | verified |
| R4 | Plugin: whole-clip analysis per audio source feeds A2/A3 (true integrated LUFS of the clip) | `ara::analyze`, `fit_clip` (renders the whole clip through the sections, corrects up to six passes) | panel shows the ARA clip; APPLY fits it in the background | `fit_clip_reaches_the_target` (-14.22 LUFS / -1.83 dBTP; -23.08), `loudness_panel_applies_an_ara_clip` | verified |
| R5 | Plugin: full ARA editing: per region results shown and editable in the editor | `View::doNotifySelection` -> `op_ara_select`; `ara::current` | the LOUDNESS panel shows and fits the selected clip (newest when none) | TestHost editor-view test: 3 selections arrive in VST3, AU and CLAP; `ara::tests::analyzes_sources_by_id` (selection picks the clip) | partly: selection-driven clip editing done; settings per region in the ARA model are not built (settings stay the track instance's parameters, so automation and per-track settings work) |
| R6 | ARA in Audio Unit (ARAAudioUnit.h) | `OpenPluginAU.cpp`: ARAFactory and BindingWithRoles properties, ARA component tag, rendering | - | TestHost (AU): 12 cases, 0 violations, render = clip; auval PASS | verified |
| R7 | ARA in CLAP (ARACLAP.h) | `native/ara/ara_c.cpp` library in the .clap, loaded by `clap.rs`; factory and plugin extension IDs `org.ara-audio.ara.*/2` (and draft) | - | TestHost (CLAP): 12 cases, 0 violations, render = clip; clap-validator 34 passed, 0 failed; REAPER CLAP suite all PASS, default render nulls with 1.0.0 Beta (-240 dB) | verified (macOS) |
| R8a | Pro Tools: clip drag-and-drop into the window (the AAX SDK's documented route to a clip; Pro Tools 2024.10+) | `Editor::files_dropped` reads WAV / AIFF (`src/audiofile.rs`), measures it and selects it | the LOUDNESS panel opens on the dropped clip; APPLY fits it | `audiofile::tests` (WAV 16/24/float/extensible, AIFF, AIFC sowt/fl32, refusals), `a_dropped_clip_becomes_the_current_clip` | built; not yet tried in Pro Tools (needs the signed AAX installed) |
| R8 | ARA in AAX | | | | BLOCKED: neither AAX SDK 2.8.1 nor JUCE_ARA's AAX wrapper (bundled AAX SDK 2.9) has an ARA interface; needs Avid's AAX ARA extension |
| R9 | Host tests: an ARA host (REAPER ARA, Studio One, Logic, Pro Tools) loads the ARA plugin and analyses clips | - | - | ARA TestHost (VST3) and REAPER 7.80 (VST3, ARA bound, clips analysed) | partly: Studio One, Logic, Cubase not run |
| R10 | ARA on Windows and Linux (VST3) | OpenPluginSDK `native/ara/ara.cmake`, `build_plugin_linux.sh --ara-sdk`, `build_plugin.ps1 -AraSdk`; workflow checks out Celemony/ARA_SDK releases/2.3.0 | - | GitHub Actions run 45734231804: ARA SDK TestHost (Debug) on the new Windows x64 and Linux x86_64 VST3: 12 test cases, 0 ARA API violations each | verified |
| X1 | Released behaviour unchanged with Auto off (renders null against 1.0.0 Beta) | - | - | REAPER default render: non-ARA build -240 dB; ARA build -240 dB (after the role fix; before it -20 dB: processed twice) | verified |

Stage order: A1-A6 (no SDK change) -> R1-R4 (VST3) -> R5 -> R6, R7 -> R8 when the AAX extension is available.

## Findings while making ARA work

- ARA TestHost on macOS logs to the unified log (`/usr/bin/log show`), not the terminal; zsh has its own `log`.
- The ICST WAV reader in the ARA examples loops forever on a file without an iXML chunk (it seeks past the end
  looking for one); test clips carry an iXML chunk.
- REAPER splits the roles: a hidden renderer instance (role 1) per clip and the track instance (roles 2 + 4). With
  both processing, clips were processed twice. Now the renderer-only instance plays the clips unprocessed and the
  track instance processes them with its own parameters.
- ARA needs a non-empty information URL: the plugin exports `op_ara_url`.
- REAPER suite on the ARA build (`reaper-VST3`): every check PASS, 0 silent renders, 112 parameters. It rated Auto
  Target "changes the sound" only because the first render after the previous parameter's context was reset still
  had that context (Limiter Clean off, Threshold -12 dB): `p106_0` differs from `base`, `p106_1` equals it at
  -240 dB. The release run has the same one-render lag (parameters 7, 22, 29), so it is not from ARA. Auto Target and
  Auto Ceiling change no sound by design: `exceptions.txt` here excuses them with that reason.
- TestHost's AU loader describes non-interleaved formats with channels x 4 bytes per frame; the Audio Unit refuses that
  (Apple: one sample per frame per buffer). Its CLAP loader asserts mono ports; OpenDynamics is stereo. Both were
  fixed in a scratch copy of the examples for the tests, not in the ARA SDK checkout.
- The first CLAP build guessed the ARA IDs (`com.celemony.*`); ARACLAP.h names `org.ara-audio.ara.factory/2` and
  `org.ara-audio.ara.pluginextension/2`. clap-validator caught `ara:supported` in the factory descriptor but not in
  the instance's; both list it now.
- ARA forwards selections only to an open editor view; the view counts as open from the start.
- Hosts: Studio One 7 is installed but drives only through its window (no scripting): not run. Logic and Cubase are
  not installed.
- CI: TestHost on Windows and Linux loads the module file inside the bundle (`Contents/x86_64-win/*.vst3`,
  `Contents/x86_64-linux/*.so`), not the bundle; with GCC it needs `<limits>` pre-included (ARAContentLogger.h uses
  std::numeric_limits without it); on Windows a failed check opens an error box, so the step is limited to 15 minutes.
