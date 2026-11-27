# Plugin QA: LV2: OpenDynamics

| Check | Status | Detail |
|---|---|---|
| loads | PASS | OpenDynamics, 109 parameters |
| finite output | PASS | peak -11.3 dBFS |
| not silent | PASS | rms -21.7 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | SKIPPED | visual checks disabled |
| every parameter changes the sound | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | SKIPPED | headless: OpenGrid9 runs without an editor window |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-11.4 dB) | -  |
| 1 | Output Gain |  | ok (12.1 dB) | -  |
| 2 | Zero Latency |  | ok (-25.8 dB) | -  |
| 3 | Auto Gain |  | ok (-26.0 dB) | -  |
| 4 | Delta |  | ok (-5.0 dB) | -  |
| 5 | Gate / Expander On |  | ok (-34.1 dB) | -  |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | ok (-35.5 dB) | -  |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-11.5 dB) | -  |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-24.7 dB) | -  |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-34.8 dB) | -  |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-30.7 dB) | -  |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-16.7 dB) | -  |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.1 dB) | -  |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-16.7 dB) | -  |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.1 dB) | -  |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-12.3 dB) | -  |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-14.0 dB) | -  |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-25.7 dB) | -  |
| 19 | Gate / Expander Key Source |  | excused (-29.8 dB) | -  |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.6 dB) | -  |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.1 dB) | -  |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-27.2 dB) | -  |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-27.2 dB) | -  |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.1 dB) | -  |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 27 | Compressor On |  | ok (-10.8 dB) | -  |
| 28 | Compressor Threshold |  | ok (-11.5 dB) | -  |
| 29 | Compressor Ratio |  | ok (-12.9 dB) | -  |
| 30 | Compressor Knee |  | ok (-36.7 dB) | -  |
| 31 | Compressor Attack |  | ok (-19.2 dB) | -  |
| 32 | Compressor Release |  | ok (-20.8 dB) | -  |
| 33 | Compressor Detector |  | ok (-19.2 dB) | -  |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-25.2 dB) | -  |
| 35 | Compressor Look-Ahead |  | ok (-26.6 dB) | -  |
| 36 | Compressor Stereo Link |  | ok (-33.3 dB) | -  |
| 37 | Compressor Key Source |  | excused (-12.9 dB) | -  |
| 38 | Compressor Key High-Pass |  | ok (-14.3 dB) | -  |
| 39 | Compressor Key Low-Pass |  | ok (-14.2 dB) | -  |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | -  |
| 41 | Compressor Mix |  | ok (-12.9 dB) | -  |
| 42 | Compressor Auto Attack |  | ok (-34.1 dB) | -  |
| 43 | Compressor Auto Release |  | ok (-35.2 dB) | -  |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.8 dB) | -  |
| 45 | Compressor Key Boost |  | ok (-14.5 dB) | -  |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-16.9 dB) | -  |
| 47 | De-Expander On |  | ok (-38.7 dB) | -  |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-5.5 dB) | -  |
| 49 | De-Expander Ratio | De-Expander On=1 | ok (-30.3 dB) | -  |
| 50 | De-Expander Range | De-Expander On=1 | ok (-40.2 dB) | -  |
| 51 | De-Expander Knee | De-Expander On=1 | ok (-31.8 dB) | -  |
| 52 | De-Expander Floor | De-Expander On=1 | ok (-31.8 dB) | -  |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-35.8 dB) | -  |
| 54 | De-Expander Hold | De-Expander On=1 | ok (-40.2 dB) | -  |
| 55 | De-Expander Release | De-Expander On=1 | ok (-35.8 dB) | -  |
| 56 | De-Expander Detector | De-Expander On=1 | ok (-34.2 dB) | -  |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-32.1 dB) | -  |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-30.1 dB) | -  |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-31.5 dB) | -  |
| 60 | De-Expander Key Source |  | excused (-31.8 dB) | -  |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | -  |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.6 dB) | -  |
| 63 | De-Expander Mix | De-Expander On=1 | ok (-32.6 dB) | -  |
| 64 | De-Expander Auto Attack | De-Expander On=1 | ok (-33.3 dB) | -  |
| 65 | De-Expander Auto Release | De-Expander On=1 | ok (-33.3 dB) | -  |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1 | ok (-30.5 dB) | -  |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-29.9 dB) | -  |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1 | ok (-32.6 dB) | -  |
| 69 | Decompressor On |  | ok (-21.6 dB) | -  |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-7.3 dB) | -  |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-16.8 dB) | -  |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-12.2 dB) | -  |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-29.8 dB) | -  |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-30.5 dB) | -  |
| 75 | Decompressor Release | Decompressor On=1 | ok (-28.7 dB) | -  |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-19.8 dB) | -  |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-21.0 dB) | -  |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-20.6 dB) | -  |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (-27.7 dB) | -  |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-33.2 dB) | -  |
| 81 | Decompressor Key Source |  | excused (-33.3 dB) | -  |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-20.7 dB) | -  |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-17.9 dB) | -  |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-16.2 dB) | -  |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-23.5 dB) | -  |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-7.8 dB) | -  |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-36.1 dB) | -  |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-11.7 dB) | -  |
| 89 | Decompressor Match | Decompressor On=1 | ok (-21.6 dB) | -  |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.5 dB) | -  |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-26.7 dB) | -  |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-28.8 dB) | -  |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-29.7 dB) | -  |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-15.6 dB) | -  |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-30.7 dB) | -  |
| 96 | Limiter On |  | ok (-29.6 dB) | -  |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | -  |
| 98 | Limiter Ceiling |  | ok (-11.9 dB) | -  |
| 99 | Limiter Attack |  | ok (-34.1 dB) | -  |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-18.9 dB) | -  |
| 101 | Limiter Stereo Link |  | ok (-35.4 dB) | -  |
| 102 | Limiter Clean |  | ok (-31.8 dB) | -  |
| 103 | Limiter Bass Hold |  | ok (-28.3 dB) | -  |
| 104 | Limiter Auto Attack |  | ok (-31.8 dB) | -  |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.2 dB) | -  |
| 106 | Auto Target |  | excused (-34.2 dB) | -  |
| 107 | Auto Ceiling |  | excused (-35.9 dB) | -  |
| 108 | Freewheel |  | ok (-32.7 dB) | -  |
