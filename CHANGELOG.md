# Changelog

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
