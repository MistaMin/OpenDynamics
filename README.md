# OpenDynamics

An open-source dynamics processor: five sections on five tabs, each a pure gain stage with a real
static curve, so a steady tone passes without added harmonics.

**Status: 1.0.0-beta in development. Not released yet.**

## Sections (in signal order)

| Section | What it does |
|---|---|
| Decompressor | Upward expansion above a threshold: restores transients that earlier compression flattened. Optional sample- or true-peak ceiling and crest matching |
| Gate / Expander | Gate with range, hysteresis and hold, or a downward expander with ratio and knee |
| Compressor | Feed-forward, soft knee centred on the threshold, exact ratio, make-up and mix |
| De-Expander | Upward compression of quiet passages below a threshold, above a floor |
| Limiter | Simple peak limiter, no look-ahead, zero latency. Clean mode holds the reduction over bass cycles and sets the release from the programme's crest factor |

Every section has look-ahead (the gain glides into place and lands on the transient's first
sample), auto attack and auto release, and a sidechain filter: internal or external key through a
high-pass, a low-pass and a bell band at once.

Global: input and output gain, Zero Latency (drops every look-ahead), Auto Gain (matches the output
loudness to the input, a listening aid) and Delta (plays only what the sections changed).

The editor shows the static curve inside a four-second level history, a spectrograph of the energy
each band lost or gained, and a 3D spectrum view over the full band. Seven colour themes.

## Formats

macOS (universal): VST3, AU, CLAP, LV2, AAX. Windows and Linux (x86_64): VST3, CLAP, LV2.

## Building

Rust (stable). The project builds against OpenPluginSDK and GoodLookin DRI checked out next to it
(`../../Dependencies/OpenPluginSDK`, `../../Dependencies/GoodLookinDri`).

```bash
cargo test --release --workspace   # DSP, plugin and editor tests
./scripts/build.sh                 # macOS bundles into dist/ (needs the VST3, AU and AAX SDKs)
```

Windows and Linux builds run in GitHub Actions (`.github/workflows/build.yml`).

## License

MIT, see [LICENSE](LICENSE). Copyright (c) 2026 Marcos Quirino Deida.
