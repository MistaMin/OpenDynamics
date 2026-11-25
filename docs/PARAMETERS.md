# OpenDynamics parameter reference

Generated from the plugin (`cargo test --release --lib dump_manual_params -- --ignored`). Every parameter, by section (the order hosts list them), with its range and default as hosts and the editor show them.

## Global

| Parameter | Range or choices | Default |
|---|---|---|
| Input Gain | -24 dB to 24 dB | 0 dB |
| Output Gain | -24 dB to 24 dB | 0 dB |
| Zero Latency | Off, On | Off |
| Auto Gain | Off, On | Off |
| Delta | Off, On | Off |

## Gate / Expander

| Parameter | Range or choices | Default |
|---|---|---|
| Gate / Expander On | Off, On | Off |
| Gate / Expander Mode | Gate, Expander | Gate |
| Gate / Expander Threshold | -90 dB to 0 dB | -50 dB |
| Gate / Expander Ratio | 1:1 to 20:1 | 2:1 |
| Gate / Expander Range | 0 dB to 90 dB | 40 dB |
| Gate / Expander Knee | 0 dB to 24 dB | 6 dB |
| Gate / Expander Hysteresis | 0 dB to 12 dB | 3 dB |
| Gate / Expander Attack | 10 µs to 100 ms | 500 µs |
| Gate / Expander Hold | 0 ms to 500 ms | 20.0 ms |
| Gate / Expander Release | 5.00 ms to 5.00 s | 150 ms |
| Gate / Expander Detector | Peak, RMS | Peak |
| Gate / Expander RMS Window | 1.00 ms to 300 ms | 10.0 ms |
| Gate / Expander Look-Ahead | 0 ms to 10.0 ms | 2.00 ms |
| Gate / Expander Stereo Link | 0% to 100% | 100% |
| Gate / Expander Key Source | Internal, External | Internal |
| Gate / Expander Key High-Pass | 10.0 Hz to 2.00 kHz | 10.0 Hz |
| Gate / Expander Key Low-Pass | 500 Hz to 22.0 kHz | 22.0 kHz |
| Gate / Expander Auto Attack | Off, On | Off |
| Gate / Expander Auto Release | Off, On | Off |
| Gate / Expander Key Boost Freq | 20.0 Hz to 20.0 kHz | 1.00 kHz |
| Gate / Expander Key Boost | -24 dB to 24 dB | 0 dB |
| Gate / Expander Key Boost Q | 0.30 to 10.0 | 1.00 |

## Compressor

| Parameter | Range or choices | Default |
|---|---|---|
| Compressor On | Off, On | On |
| Compressor Threshold | -60 dB to 0 dB | -18 dB |
| Compressor Ratio | 1:1 to 30:1 | 4:1 |
| Compressor Knee | 0 dB to 24 dB | 6 dB |
| Compressor Attack | 10 µs to 300 ms | 10.0 ms |
| Compressor Release | 5.00 ms to 5.00 s | 120 ms |
| Compressor Detector | Peak, RMS | Peak |
| Compressor RMS Window | 1.00 ms to 300 ms | 10.0 ms |
| Compressor Look-Ahead | 0 ms to 10.0 ms | 2.00 ms |
| Compressor Stereo Link | 0% to 100% | 100% |
| Compressor Key Source | Internal, External | Internal |
| Compressor Key High-Pass | 10.0 Hz to 2.00 kHz | 10.0 Hz |
| Compressor Key Low-Pass | 500 Hz to 22.0 kHz | 22.0 kHz |
| Compressor Make-Up | -12 dB to 24 dB | 0 dB |
| Compressor Mix | 0% to 100% | 100% |
| Compressor Auto Attack | Off, On | Off |
| Compressor Auto Release | Off, On | Off |
| Compressor Key Boost Freq | 20.0 Hz to 20.0 kHz | 1.00 kHz |
| Compressor Key Boost | -24 dB to 24 dB | 0 dB |
| Compressor Key Boost Q | 0.30 to 10.0 | 1.00 |

## De-Expander

| Parameter | Range or choices | Default |
|---|---|---|
| De-Expander On | Off, On | Off |
| De-Expander Threshold | -80 dB to 0 dB | -40 dB |
| De-Expander Ratio | 1:1 to 8:1 | 2:1 |
| De-Expander Range | 0 dB to 36 dB | 12 dB |
| De-Expander Knee | 0 dB to 24 dB | 6 dB |
| De-Expander Floor | -120 dB to -20 dB | -72 dB |
| De-Expander Attack | 10 µs to 100 ms | 500 µs |
| De-Expander Hold | 0 ms to 500 ms | 20.0 ms |
| De-Expander Release | 1.00 ms to 5.00 s | 150 ms |
| De-Expander Detector | Peak, RMS | RMS |
| De-Expander RMS Window | 1.00 ms to 500 ms | 10.0 ms |
| De-Expander Look-Ahead | 0 ms to 10.0 ms | 2.00 ms |
| De-Expander Stereo Link | 0% to 100% | 100% |
| De-Expander Key Source | Internal, External | Internal |
| De-Expander Key High-Pass | 10.0 Hz to 2.00 kHz | 10.0 Hz |
| De-Expander Key Low-Pass | 500 Hz to 22.0 kHz | 22.0 kHz |
| De-Expander Mix | 0% to 100% | 100% |
| De-Expander Auto Attack | Off, On | Off |
| De-Expander Auto Release | Off, On | Off |
| De-Expander Key Boost Freq | 20.0 Hz to 20.0 kHz | 1.00 kHz |
| De-Expander Key Boost | -24 dB to 24 dB | 0 dB |
| De-Expander Key Boost Q | 0.30 to 10.0 | 1.00 |

## Decompressor

| Parameter | Range or choices | Default |
|---|---|---|
| Decompressor On | Off, On | Off |
| Decompressor Threshold | -60 dB to 0 dB | -18 dB |
| Decompressor Ratio | 1:1 to 1:4 | 1:2 |
| Decompressor Range | 0 dB to 24 dB | 6 dB |
| Decompressor Knee | 0 dB to 24 dB | 6 dB |
| Decompressor Attack | 10 µs to 100 ms | 1.00 ms |
| Decompressor Release | 1.00 ms to 2.00 s | 80.0 ms |
| Decompressor Detector | Peak, RMS, Transient | Peak |
| Decompressor RMS Window | 1.00 ms to 500 ms | 30.0 ms |
| Decompressor Transient Crest | 0 dB to 24 dB | 6 dB |
| Decompressor Look-Ahead | 0 ms to 10.0 ms | 2.00 ms |
| Decompressor Stereo Link | 0% to 100% | 100% |
| Decompressor Key Source | Internal, External | Internal |
| Decompressor Key High-Pass | 10.0 Hz to 2.00 kHz | 10.0 Hz |
| Decompressor Key Low-Pass | 500 Hz to 22.0 kHz | 22.0 kHz |
| Decompressor Mix | 0% to 100% | 100% |
| Decompressor Ceiling | Off, On | Off |
| Decompressor Ceiling Level | -24 dB to 0 dB | -1 dB |
| Decompressor Ceiling Type | Sample peak, True peak | True peak |
| Decompressor Output Trim | -24 dB to 0 dB | 0 dB |
| Decompressor Match | Off, On | Off |
| Decompressor Match Target Crest | 6 dB to 24 dB | 12 dB |
| Decompressor Auto Attack | Off, On | Off |
| Decompressor Auto Release | Off, On | Off |
| Decompressor Key Boost Freq | 20.0 Hz to 20.0 kHz | 1.00 kHz |
| Decompressor Key Boost | -24 dB to 24 dB | 0 dB |
| Decompressor Key Boost Q | 0.30 to 10.0 | 1.00 |

## Limiter

| Parameter | Range or choices | Default |
|---|---|---|
| Limiter On | Off, On | On |
| Limiter Threshold | -24 dB to 0 dB | 0 dB |
| Limiter Ceiling | -24 dB to 0 dB | -1 dB |
| Limiter Attack | 0 ms to 10.0 ms | 0 ms |
| Limiter Release | 1.00 ms to 2.00 s | 100 ms |
| Limiter Stereo Link | 0% to 100% | 100% |
| Limiter Clean | Off, On | On |
| Limiter Bass Hold | 0 ms to 20.0 ms | 14.0 ms |
| Limiter Auto Attack | Off, On | Off |
| Limiter Auto Release | Off, On | Off |

## Auto

| Parameter | Range or choices | Default |
|---|---|---|
| Auto Target | -30 LUFS to -6 LUFS | -14 LUFS |
| Auto Ceiling | -3.0 dBTP to 0.0 dBTP | -1.0 dBTP |
