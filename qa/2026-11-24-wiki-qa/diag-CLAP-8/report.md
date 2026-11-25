# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 109 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -21.7 dBFS |
| deterministic render | FAIL | repeat differs by -8.4 dB |
| editor opens | PASS | 1040x773, 3.5% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | skipped (render not deterministic)  | ok (0.194%) |
| 1 | Output Gain |  | skipped (render not deterministic)  | ok (0.268%) |
| 2 | Zero Latency |  | skipped (render not deterministic)  | ok (0.155%) |
| 3 | Auto Gain |  | skipped (render not deterministic)  | ok (0.155%) |
| 4 | Delta |  | skipped (render not deterministic)  | ok (0.155%) |
| 5 | Gate / Expander On |  | skipped (render not deterministic)  | ok (0.155%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | skipped (render not deterministic)  | ok (0.155%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | skipped (render not deterministic)  | ok (0.155%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 14 | Gate / Expander Release | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | skipped (render not deterministic)  | ok (0.155%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 19 | Gate / Expander Key Source |  | skipped (render not deterministic)  | ok (0.155%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 27 | Compressor On |  | skipped (render not deterministic)  | ok (0.155%) |
| 28 | Compressor Threshold |  | skipped (render not deterministic)  | ok (0.155%) |
| 29 | Compressor Ratio |  | skipped (render not deterministic)  | ok (0.155%) |
| 30 | Compressor Knee |  | skipped (render not deterministic)  | ok (0.155%) |
| 31 | Compressor Attack |  | skipped (render not deterministic)  | ok (0.155%) |
| 32 | Compressor Release |  | skipped (render not deterministic)  | ok (0.155%) |
| 33 | Compressor Detector |  | skipped (render not deterministic)  | ok (0.155%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | skipped (render not deterministic)  | ok (0.155%) |
| 35 | Compressor Look-Ahead |  | skipped (render not deterministic)  | ok (0.155%) |
| 36 | Compressor Stereo Link |  | skipped (render not deterministic)  | ok (0.155%) |
| 37 | Compressor Key Source |  | skipped (render not deterministic)  | ok (0.155%) |
| 38 | Compressor Key High-Pass |  | skipped (render not deterministic)  | ok (0.155%) |
| 39 | Compressor Key Low-Pass |  | skipped (render not deterministic)  | ok (0.155%) |
| 40 | Compressor Make-Up |  | skipped (render not deterministic)  | ok (0.155%) |
| 41 | Compressor Mix |  | skipped (render not deterministic)  | ok (0.155%) |
| 42 | Compressor Auto Attack |  | skipped (render not deterministic)  | ok (0.155%) |
| 43 | Compressor Auto Release |  | skipped (render not deterministic)  | ok (0.155%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 45 | Compressor Key Boost |  | skipped (render not deterministic)  | ok (0.155%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 47 | De-Expander On |  | skipped (render not deterministic)  | ok (0.155%) |
| 48 | De-Expander Threshold | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 49 | De-Expander Ratio | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 50 | De-Expander Range | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 51 | De-Expander Knee | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 52 | De-Expander Floor | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 53 | De-Expander Attack | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 54 | De-Expander Hold | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 55 | De-Expander Release | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 56 | De-Expander Detector | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | skipped (render not deterministic)  | ok (0.155%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 60 | De-Expander Key Source |  | skipped (render not deterministic)  | ok (0.155%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 63 | De-Expander Mix | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 64 | De-Expander Auto Attack | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 65 | De-Expander Auto Release | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 69 | Decompressor On |  | skipped (render not deterministic)  | ok (0.155%) |
| 70 | Decompressor Threshold | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 71 | Decompressor Ratio | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | skipped (render not deterministic)  | ok (0.155%) |
| 73 | Decompressor Knee | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 74 | Decompressor Attack | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 75 | Decompressor Release | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 76 | Decompressor Detector | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | skipped (render not deterministic)  | ok (0.155%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | skipped (render not deterministic)  | ok (0.155%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 81 | Decompressor Key Source |  | skipped (render not deterministic)  | ok (0.155%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 84 | Decompressor Mix | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | skipped (render not deterministic)  | ok (0.155%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | skipped (render not deterministic)  | ok (0.155%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 89 | Decompressor Match | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | skipped (render not deterministic)  | ok (0.155%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | skipped (render not deterministic)  | ok (0.155%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | skipped (render not deterministic)  | ok (0.155%) |
| 96 | Limiter On |  | skipped (render not deterministic)  | ok (0.155%) |
| 97 | Limiter Threshold |  | skipped (render not deterministic)  | ok (0.155%) |
| 98 | Limiter Ceiling |  | skipped (render not deterministic)  | ok (0.155%) |
| 99 | Limiter Attack |  | skipped (render not deterministic)  | ok (0.155%) |
| 100 | Limiter Release | Limiter Clean=0 | skipped (render not deterministic)  | ok (0.155%) |
| 101 | Limiter Stereo Link |  | skipped (render not deterministic)  | ok (0.155%) |
| 102 | Limiter Clean |  | skipped (render not deterministic)  | ok (0.155%) |
| 103 | Limiter Bass Hold |  | skipped (render not deterministic)  | ok (0.155%) |
| 104 | Limiter Auto Attack |  | skipped (render not deterministic)  | ok (0.155%) |
| 105 | Limiter Auto Release | Limiter Clean=0 | skipped (render not deterministic)  | ok (0.155%) |
| 106 | Bypass |  | skipped (render not deterministic)  | ok (0.159%) |
| 107 | Wet |  | skipped (render not deterministic)  | ok (0.155%) |
| 108 | Delta |  | skipped (render not deterministic)  | ok (0.155%) |
