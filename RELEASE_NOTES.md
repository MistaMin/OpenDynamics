# OpenDynamics 1.0.0 Beta

OpenDynamics is a dynamics processor by OpenGrid: Decompressor, Gate / Expander, Compressor, De-Expander and Limiter
in one plugin, each a pure gain stage with an exact static curve. This is the first public release, and a beta.

## Formats and requirements

| Download | System | Formats |
|---|---|---|
| `OpenDynamics-1.0.0-Beta-macos.pkg` | macOS 11 or later, Apple silicon and Intel | VST3, Audio Unit, CLAP, LV2, AAX |
| `OpenDynamics-1.0.0-Beta-windows-x64-setup.exe` | Windows 10 or later, x64 | VST3, CLAP, LV2 |
| `OpenDynamics-1.0.0-Beta-linux-x86_64.tar.gz` | Linux x86_64 | VST3, CLAP, LV2 |

No standalone app.

## Install

- **macOS:** open the `.pkg` and choose the formats. It is signed with Developer ID and notarized by Apple; the AAX is
  signed with PACE Eden. Plugins go to `/Library/Audio/Plug-Ins` and the AAX to
  `/Library/Application Support/Avid/Audio/Plug-Ins`.
- **Windows:** run the setup and choose the formats; they go to `Common Files\VST3`, `CLAP` and `LV2`. The installer is
  not code-signed, so SmartScreen may ask before it runs.
- **Linux:** `sudo tar -xzf OpenDynamics-1.0.0-Beta-linux-x86_64.tar.gz -C /` installs to `/usr/lib/vst3`, `clap` and
  `lv2`; or copy the bundles to `~/.vst3`, `~/.clap` and `~/.lv2`.

## Checks this release passed (macOS)

- 95 automated tests: static curves, timing, look-ahead, every parameter's control, text and layout of the editor.
- Validators: auval, pluginval at strictness 10 (VST3 and Audio Unit), clap-validator, AAX Validator 6 of 6.
- Host tests in REAPER (VST3, Audio Unit, CLAP) and OpenGrid9 (VST3, Audio Unit, LV2): load, sound, every parameter
  changes the sound, every parameter is shown in the editor, save and reopen, editor open and close.
- Pro Tools 2025.10 (earlier build of this release): insert, processing, levels, bypass, typed values, save and reopen,
  offline and real-time bounce.
- Windows and Linux: built and tested by GitHub Actions (automated tests only).

## Known limits

- In host tests, one run in several went silent part of the way through and stayed silent until the plugin was
  reloaded (seen once in VST3 and once in CLAP, in REAPER). It has not been reproduced since, offline or in nine more
  host runs; the plugin now logs the first sample that is not a number to `~/Library/Logs/OpenDynamics/panic.log` so
  it can be found. If OpenDynamics goes silent for you, reloading it brings it back; please send that file.
- The Windows and Linux builds have not been tested in a host.
- Logic Pro and other hosts not named above have not been tested.

## Files

- `OpenDynamics-1.0.0-Beta-macos.pkg`, `OpenDynamics-1.0.0-Beta-windows-x64-setup.exe`,
  `OpenDynamics-1.0.0-Beta-linux-x86_64.tar.gz`, each with a `.sha256` file
- `OpenDynamics-User-Manual.pdf`
- `opengrid-manifest.json`

Source: MIT. Official binaries: freeware under `BINARY_LICENSE.txt`. VST is a trademark of Steinberg Media Technologies
GmbH. Audio Units is a trademark of Apple Inc. AAX and Pro Tools are trademarks of Avid Technology, Inc.
