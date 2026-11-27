# Changelog

## 1.1.0 (2026-11-27)

- Auto Loudness: LOUDNESS in the section bar opens a panel that sets all five sections so the output reaches a loudness
  target (-30 to -6 LUFS) under a true-peak ceiling (-3.0 to 0.0 dBTP). LEARN measures while the song plays; APPLY plans
  the sections and writes their controls. Two new parameters, Auto Target and Auto Ceiling, after the existing ones.
- ARA 2 on macOS (VST3, Audio Unit, CLAP) and in the Windows and Linux VST3: the host hands over each clip's audio, the
  panel shows the selected clip, and APPLY fits the whole clip without playing it.
- A clip dropped on the window (from a host that renders it to a file on drop, or a WAV or AIFF file) is fitted like an
  ARA clip.
- Builds against OpenPluginSDK d93d926, which carries GoodLookin DRI in its own tree; the separate GoodLookin DRI
  checkout is no longer needed. The editor draws exactly as before. The macOS VST3, AU and AAX binaries export only
  their entry points and are smaller (AAX 3.5 MB, was 9.5 MB).
- Zero Latency switches as soon as its 5 ms dip ends, also in the middle of a host block. With large blocks the
  output used to stay silent for the rest of the block (up to about 85 ms at 4096 samples, 48 kHz).
- No allocation or file access on the audio thread: the diagnosis lines (non-finite output, silent output with input) are
  kept in a fixed buffer there and written to the log when the plugin is prepared again or closed.
- A denormal input carried through a delay no longer reaches the output (flushed at the end of each block).
- Hosts list the gate's parameters under "Gate & Expander": "Gate / Expander" was read as two levels with stray spaces.
  The parameter names are unchanged.
- Tests: the SDK's conformance kit (tests/conformance.rs), soak and strict fuzz (tests/soak.rs) and the bench
  (examples/bench.rs, bench/baseline.json). Test runs no longer write to the plugin's log (.cargo/config.toml).
- Test: every control, on every tab and panel, writes only the parameter its label names when it is dragged or clicked.
- Clippy clean with the current toolchain.

## 1.0.0 Beta (2026-11-25)

First public release.

- Five sections in signal order: Decompressor, Gate / Expander, Compressor, De-Expander and Limiter, each a pure gain stage
  with an exact static curve.
- Look-ahead on every section that lands the gain on a transient's first sample; Zero Latency drops it.
- Auto attack and auto release on every section.
- Sidechain on every section with a key: internal or external, through a high-pass, a low-pass and a bell at once.
- Limiter Clean mode: holds the reduction over bass cycles and sets the release from the programme's crest factor.
- Decompressor: Transient detector, sample- or true-peak ceiling, crest matching.
- Auto Gain (BS.1770-weighted loudness match) and Delta listening aids.
- Editor: static curve inside a four-second level history, energy change by frequency, 3D spectrum view, slider panels for
  the less used controls, seven themes. Host value text in whole dB and whole ratios, the same as the editor.
- macOS: VST3, Audio Unit, CLAP, LV2 and AAX, universal. Windows and Linux: VST3, CLAP and LV2.
