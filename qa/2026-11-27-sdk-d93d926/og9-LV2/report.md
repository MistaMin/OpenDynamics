# Plugin QA: LV2: OpenDynamics

| Check | Status | Detail |
|---|---|---|
| loads | PASS | OpenDynamics, 109 parameters |
| finite output | PASS | peak -11.3 dBFS |
| not silent | PASS | rms -21.7 dBFS |
| deterministic render | FAIL | repeat differs by -44.0 dB |
| editor opens | SKIPPED | visual checks disabled |
| every parameter changes the sound | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | SKIPPED | headless: OpenGrid9 runs without an editor window |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | skipped (render not deterministic)  | -  |
| 1 | Output Gain |  | skipped (render not deterministic)  | -  |
| 2 | Zero Latency |  | skipped (render not deterministic)  | -  |
| 3 | Auto Gain |  | skipped (render not deterministic)  | -  |
| 4 | Delta |  | skipped (render not deterministic)  | -  |
| 5 | Gate / Expander On |  | skipped (render not deterministic)  | -  |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | skipped (render not deterministic)  | -  |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | skipped (render not deterministic)  | -  |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | skipped (render not deterministic)  | -  |
| 9 | Gate / Expander Range | Gate / Expander On=1 | skipped (render not deterministic)  | -  |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 19 | Gate / Expander Key Source |  | skipped (render not deterministic)  | -  |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | skipped (render not deterministic)  | -  |
| 27 | Compressor On |  | skipped (render not deterministic)  | -  |
| 28 | Compressor Threshold |  | skipped (render not deterministic)  | -  |
| 29 | Compressor Ratio |  | skipped (render not deterministic)  | -  |
| 30 | Compressor Knee |  | skipped (render not deterministic)  | -  |
| 31 | Compressor Attack |  | skipped (render not deterministic)  | -  |
| 32 | Compressor Release |  | skipped (render not deterministic)  | -  |
| 33 | Compressor Detector |  | skipped (render not deterministic)  | -  |
| 34 | Compressor RMS Window | Compressor Detector=1 | skipped (render not deterministic)  | -  |
| 35 | Compressor Look-Ahead |  | skipped (render not deterministic)  | -  |
| 36 | Compressor Stereo Link |  | skipped (render not deterministic)  | -  |
| 37 | Compressor Key Source |  | skipped (render not deterministic)  | -  |
| 38 | Compressor Key High-Pass |  | skipped (render not deterministic)  | -  |
| 39 | Compressor Key Low-Pass |  | skipped (render not deterministic)  | -  |
| 40 | Compressor Make-Up |  | skipped (render not deterministic)  | -  |
| 41 | Compressor Mix |  | skipped (render not deterministic)  | -  |
| 42 | Compressor Auto Attack |  | skipped (render not deterministic)  | -  |
| 43 | Compressor Auto Release |  | skipped (render not deterministic)  | -  |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | skipped (render not deterministic)  | -  |
| 45 | Compressor Key Boost |  | skipped (render not deterministic)  | -  |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | skipped (render not deterministic)  | -  |
| 47 | De-Expander On |  | skipped (render not deterministic)  | -  |
| 48 | De-Expander Threshold | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 49 | De-Expander Ratio | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 50 | De-Expander Range | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 51 | De-Expander Knee | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 52 | De-Expander Floor | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 53 | De-Expander Attack | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 54 | De-Expander Hold | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 55 | De-Expander Release | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 56 | De-Expander Detector | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | skipped (render not deterministic)  | -  |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 59 | De-Expander Stereo Link | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 60 | De-Expander Key Source |  | skipped (render not deterministic)  | -  |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 63 | De-Expander Mix | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 64 | De-Expander Auto Attack | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 65 | De-Expander Auto Release | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1 | skipped (render not deterministic)  | -  |
| 67 | De-Expander Key Boost | De-Expander On=1 | skipped (render not deterministic)  | -  |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1 | skipped (render not deterministic)  | -  |
| 69 | Decompressor On |  | skipped (render not deterministic)  | -  |
| 70 | Decompressor Threshold | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 71 | Decompressor Ratio | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | skipped (render not deterministic)  | -  |
| 73 | Decompressor Knee | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 74 | Decompressor Attack | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 75 | Decompressor Release | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 76 | Decompressor Detector | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | skipped (render not deterministic)  | -  |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | skipped (render not deterministic)  | -  |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 80 | Decompressor Stereo Link | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 81 | Decompressor Key Source |  | skipped (render not deterministic)  | -  |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 84 | Decompressor Mix | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 85 | Decompressor Ceiling | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | skipped (render not deterministic)  | -  |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | skipped (render not deterministic)  | -  |
| 88 | Decompressor Output Trim | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 89 | Decompressor Match | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | skipped (render not deterministic)  | -  |
| 91 | Decompressor Auto Attack | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 92 | Decompressor Auto Release | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | skipped (render not deterministic)  | -  |
| 94 | Decompressor Key Boost | Decompressor On=1 | skipped (render not deterministic)  | -  |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | skipped (render not deterministic)  | -  |
| 96 | Limiter On |  | skipped (render not deterministic)  | -  |
| 97 | Limiter Threshold |  | skipped (render not deterministic)  | -  |
| 98 | Limiter Ceiling |  | skipped (render not deterministic)  | -  |
| 99 | Limiter Attack |  | skipped (render not deterministic)  | -  |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | skipped (render not deterministic)  | -  |
| 101 | Limiter Stereo Link |  | skipped (render not deterministic)  | -  |
| 102 | Limiter Clean |  | skipped (render not deterministic)  | -  |
| 103 | Limiter Bass Hold |  | skipped (render not deterministic)  | -  |
| 104 | Limiter Auto Attack |  | skipped (render not deterministic)  | -  |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | skipped (render not deterministic)  | -  |
| 106 | Auto Target |  | skipped (render not deterministic)  | -  |
| 107 | Auto Ceiling |  | skipped (render not deterministic)  | -  |
| 108 | Freewheel |  | skipped (render not deterministic)  | -  |
