# OpenDynamics 1.1.0

OpenDynamics is a dynamics processor by OpenGrid: Decompressor, Gate / Expander, Compressor, De-Expander and Limiter
in one plugin, each a pure gain stage with an exact static curve. 1.1.0 adds Auto Loudness and ARA 2, and is no longer
a beta.

## What is new

- **Auto Loudness.** LOUDNESS in the section bar opens a panel that sets all five sections so the output reaches a
  loudness target (-30 to -6 LUFS) under a true-peak ceiling (-3.0 to 0.0 dBTP), and writes the result into the
  sections' own controls. LEARN measures while the song plays; APPLY plans the sections and sets them.
- **ARA 2** (macOS: VST3, Audio Unit and CLAP; Windows and Linux: VST3): the host hands over each clip's audio, the
  panel shows the selected clip, and APPLY fits the whole clip without playing it.
- A clip dropped on the window, from a host that renders it to a file on drop or as a WAV or AIFF file, is fitted the
  same way.
- Zero Latency switches as soon as its 5 ms fade ends, also inside a large host block (large blocks used to stay silent
  for the rest of the block).
- No allocation or file access on the audio thread; a denormal carried through a delay no longer reaches the output.
- Hosts list the gate's parameters under "Gate & Expander". Parameter names and ids are unchanged; sessions from
  1.0.0 Beta open with their settings.

## Formats and requirements

| Download | System | Formats |
|---|---|---|
| `OpenDynamics-1.1.0-macos.pkg` | macOS 11 or later, Apple silicon and Intel | VST3, Audio Unit, CLAP (each with ARA 2), LV2, AAX |
| `OpenDynamics-1.1.0-windows-x64-setup.exe` | Windows 10 or later, x64 | VST3 (with ARA 2), CLAP, LV2 |
| `OpenDynamics-1.1.0-linux-x86_64.tar.gz` | Linux x86_64 | VST3 (with ARA 2), CLAP, LV2 |

No standalone app.

## Install

- **macOS:** open the `.pkg` and choose the formats. It is signed with Developer ID and notarized by Apple; the AAX is
  signed with PACE Eden. Plugins go to `/Library/Audio/Plug-Ins` and the AAX to
  `/Library/Application Support/Avid/Audio/Plug-Ins`.
- **Windows:** run the setup and choose the formats; they go to `Common Files\VST3`, `CLAP` and `LV2`. The installer is
  not code-signed, so SmartScreen may ask before it runs.
- **Linux:** `sudo tar -xzf OpenDynamics-1.1.0-linux-x86_64.tar.gz -C /` installs to `/usr/lib/vst3`, `clap` and
  `lv2`; or copy the bundles to `~/.vst3`, `~/.clap` and `~/.lv2`.

## Checks this release passed (macOS)

- The OpenPluginSDK test list on the release commit: realtime lint, clippy, every test of the crate with the SDK's
  conformance kit, soak over 200 seeds, strict fuzz, and the bench against its baseline.
- Validators: auval, pluginval at strictness 10 (VST3 and Audio Unit), clap-validator, AAX Validator 6 of 6.
- Host tests in REAPER (VST3, CLAP) and OpenGrid9 (Audio Unit, LV2): load, sound, every parameter changes the sound,
  every parameter is shown in the editor, save and reopen, editor open and close.
- Pro Tools 2025.10: a 1.0.0 Beta session opens with its settings and levels, a knob moved in the editor changes the
  sound at once, Zero Latency changes the delay compensation at once, save and reopen.
- Windows and Linux: built and tested by GitHub Actions (automated tests and the ARA SDK test host).

## Known limits

- In host tests, a run now and then goes silent part of the way through and stays silent (seen in REAPER, in VST3 and
  CLAP, one run in several; once in the tests for this release, not on the rerun). In the last capture the plugin
  received no input signal, which points at the host, but the cause is not proven. Reloading the plugin brings the
  sound back.
- In REAPER, a VST3 instance reports a Zero Latency change one render late; CLAP reports it at once.
- Switching Zero Latency during playback restarts the sections, so the level rises for a moment until they catch up.
- Very loud targets under a tight ceiling (for example -9 LUFS at -0.5 dBTP) are not reached on bright material: the
  limiter limits sample peaks, not the peaks between samples.
- The Windows and Linux builds have not been tested in a host. Logic Pro and hosts not named above have not been tested.

## Files

- `OpenDynamics-1.1.0-macos.pkg`, `OpenDynamics-1.1.0-windows-x64-setup.exe`,
  `OpenDynamics-1.1.0-linux-x86_64.tar.gz`, each with a `.sha256` file
- `OpenDynamics-1.1.0-User-Manual.pdf`
- `opengrid-manifest.json`

Source: MIT. Official binaries: freeware under `BINARY_LICENSE.txt`. VST is a trademark of Steinberg Media Technologies
GmbH. Audio Units is a trademark of Apple Inc. AAX and Pro Tools are trademarks of Avid Technology, Inc.
