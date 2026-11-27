# Plugin QA: AU: OpenDynamics

| Check | Status | Detail |
|---|---|---|
| loads | PASS | OpenDynamics, 108 parameters |
| finite output | PASS | peak -7.9 dBFS |
| not silent | PASS | rms -21.7 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | SKIPPED | visual checks disabled |
| every parameter changes the sound | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | SKIPPED | headless: OpenGrid9 runs without an editor window |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-5.0 dB) | -  |
| 1 | Output Gain |  | ok (15.6 dB) | -  |
| 2 | Zero Latency |  | ok (-8.2 dB) | -  |
| 3 | Auto Gain |  | ok (-10.2 dB) | -  |
| 4 | Delta |  | ok (-5.0 dB) | -  |
| 5 | Gate / Expander On |  | ok (-16.4 dB) | -  |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 5 | CONDITIONAL (-1.3 dB) | -  |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-11.5 dB) | -  |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-16.4 dB) | -  |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-16.4 dB) | -  |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-35.7 dB) | -  |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-16.6 dB) | -  |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.1 dB) | -  |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-16.6 dB) | -  |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.0 dB) | -  |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-12.3 dB) | -  |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-5.4 dB) | -  |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.9 dB) | -  |
| 19 | Gate / Expander Key Source |  | excused (-240.0 dB) | -  |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.6 dB) | -  |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.1 dB) | -  |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-28.1 dB) | -  |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-31.9 dB) | -  |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.1 dB) | -  |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 27 | Compressor On |  | ok (-12.9 dB) | -  |
| 28 | Compressor Threshold |  | ok (-11.5 dB) | -  |
| 29 | Compressor Ratio |  | ok (-12.9 dB) | -  |
| 30 | Compressor Knee |  | ok (-36.7 dB) | -  |
| 31 | Compressor Attack |  | ok (-12.2 dB) | -  |
| 32 | Compressor Release |  | ok (-20.8 dB) | -  |
| 33 | Compressor Detector |  | ok (-18.2 dB) | -  |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-20.8 dB) | -  |
| 35 | Compressor Look-Ahead |  | ok (-5.1 dB) | -  |
| 36 | Compressor Stereo Link |  | ok (-30.9 dB) | -  |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | -  |
| 38 | Compressor Key High-Pass |  | ok (-14.3 dB) | -  |
| 39 | Compressor Key Low-Pass |  | ok (-14.2 dB) | -  |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | -  |
| 41 | Compressor Mix |  | ok (-12.9 dB) | -  |
| 42 | Compressor Auto Attack |  | ok (-19.5 dB) | -  |
| 43 | Compressor Auto Release |  | ok (-35.2 dB) | -  |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.8 dB) | -  |
| 45 | Compressor Key Boost |  | ok (-14.5 dB) | -  |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-16.9 dB) | -  |
| 47 | De-Expander On | random state 1 | CONDITIONAL (-11.8 dB) | -  |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-5.5 dB) | -  |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (-11.8 dB) | -  |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (-11.8 dB) | -  |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (-37.8 dB) | -  |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (-8.1 dB) | -  |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.7 dB) | -  |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (-31.3 dB) | -  |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (-7.9 dB) | -  |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (-5.5 dB) | -  |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | -  |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-6.0 dB) | -  |
| 59 | De-Expander Stereo Link | De-Expander On=1random state 1 | CONDITIONAL (-10.0 dB) | -  |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | -  |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | -  |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.6 dB) | -  |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (-11.8 dB) | -  |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (-44.8 dB) | -  |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (-24.1 dB) | -  |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (-20.7 dB) | -  |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-45.0 dB) | -  |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (-17.6 dB) | -  |
| 69 | Decompressor On |  | ok (-15.5 dB) | -  |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-15.5 dB) | -  |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-15.5 dB) | -  |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-10.6 dB) | -  |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-60.4 dB) | -  |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-15.5 dB) | -  |
| 75 | Decompressor Release | Decompressor On=1 | ok (-60.4 dB) | -  |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-22.6 dB) | -  |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-15.2 dB) | -  |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-15.5 dB) | -  |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (-2.5 dB) | -  |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-27.6 dB) | -  |
| 81 | Decompressor Key Source |  | excused (-240.0 dB) | -  |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-20.7 dB) | -  |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-16.6 dB) | -  |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-15.5 dB) | -  |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-23.5 dB) | -  |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-11.4 dB) | -  |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-36.1 dB) | -  |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-9.7 dB) | -  |
| 89 | Decompressor Match | Decompressor On=1 | ok (-15.5 dB) | -  |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-26.2 dB) | -  |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.4 dB) | -  |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.4 dB) | -  |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-60.6 dB) | -  |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-15.6 dB) | -  |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.3 dB) | -  |
| 96 | Limiter On |  | ok (-26.1 dB) | -  |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | -  |
| 98 | Limiter Ceiling |  | ok (-8.2 dB) | -  |
| 99 | Limiter Attack | random state 1 | CONDITIONAL (-9.3 dB) | -  |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-14.7 dB) | -  |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (-9.2 dB) | -  |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (-14.4 dB) | -  |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (-15.1 dB) | -  |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (-4.8 dB) | -  |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-26.7 dB) | -  |
| 106 | Auto Target |  | excused (-240.0 dB) | -  |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | -  |
