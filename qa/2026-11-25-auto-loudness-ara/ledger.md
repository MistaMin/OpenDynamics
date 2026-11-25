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
| A1 | Auto mode switch, Target (LUFS) and Ceiling (dBTP) parameters | `autoTarget` -30..-6 LUFS (default -14), `autoCeiling` -3.0..0.0 dBTP (default -1.0), appended after the sections | not yet | `text` tests (LUFS whole, dBTP tenths) | params done, GUI open |
| A2 | Measure the input: integrated loudness (BS.1770, gated), loudness range, crest/peak, noise floor | `src/loudness.rs` | - | `loudness::tests`: EBU Tech 3341 cases 1 and 3 (-23.0 LUFS +-0.1), Tech 3342 LRA 10 LU, true peak between samples, silence | wired (`Meter` in `run` while learning, published to `Shared`) |
| A3 | Set all five sections from the measurement to reach the target (decompress, expand, compress, de-expand, limit) | `src/auto.rs` `plan` | - | `auto::tests`: ids and ranges, quiet/wide, loud/squashed, measured correction, silence | code done, not wired |
| A4 | Without ARA: a Learn pass over the played audio, then hold the settings | `Shared::set_learning`, meters in `run` | not yet | end-to-end test drives it | audio side done, GUI open (`every_parameter_has_a_control` fails until the Auto panel exists: correct) |
| A5 | Result hits the target within a tolerance on test material (offline render, integrated LUFS and true peak measured) | `auto_loudness_reaches_the_target` | - | 30 s quiet, wide programme (-37.8 LUFS): -14/-1: -14.87 LUFS, -1.43 dBTP in 4 passes; -23/-2: -23.00 in 1; -16/-1: -16.28, -1.41 dBTP in 3 | PASS (within 1 LU, true peak at or under the ceiling) |
| A7 | Very loud targets under a tight true-peak ceiling (-9 LUFS, -0.5 dBTP) | - | - | ignored test `auto_loudness_loud_target_tight_ceiling`: stuck near -12 LUFS: hard limiting of bright material puts true peaks 2.75 dB over the limiter's sample peaks | OPEN: needs a true-peak (oversampled) limiter mode |
| A6 | The sections' own controls show what Auto set; the user can still change them | | | | open |
| R1 | SDK: ARA factory and document controller over a C interface, VST3 shell (ARAVST3.h) | | | | open |
| R2 | SDK: read audio sources ahead of playback (ARA audio reader) | | | | open |
| R3 | SDK: playback renderer / editor renderer roles for the plugin instance | | | | open |
| R4 | Plugin: whole-clip analysis per audio source feeds A2/A3 (true integrated LUFS of the clip) | | | | open |
| R5 | Plugin: full ARA editing: per region results shown and editable in the editor | | | | open |
| R6 | ARA in Audio Unit (ARAAudioUnit.h) | | | | open |
| R7 | ARA in CLAP (ARACLAP.h) | | | | open |
| R8 | ARA in AAX | | | | BLOCKED: neither AAX SDK 2.8.1 nor JUCE_ARA's AAX wrapper (bundled AAX SDK 2.9) has an ARA interface; needs Avid's AAX ARA extension |
| R9 | Host tests: an ARA host (REAPER ARA, Studio One, Logic, Pro Tools) loads the ARA plugin and analyses clips | | | | open |
| X1 | Released behaviour unchanged with Auto off (renders null against 1.0.0 Beta) | | | | open |

Stage order: A1-A6 (no SDK change) -> R1-R4 (VST3) -> R5 -> R6, R7 -> R8 when the AAX extension is available.
