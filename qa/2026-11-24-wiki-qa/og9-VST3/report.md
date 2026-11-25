# Plugin QA: VST3: OpenDynamics

| Check | Status | Detail |
|---|---|---|
| loads | PASS | OpenDynamics, 107 parameters |
| finite output | PASS | peak -7.9 dBFS |
| not silent | PASS | rms -21.7 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | SKIPPED | visual checks disabled |
| every parameter changes the sound | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | SKIPPED | headless: OpenGrid9 runs without an editor window |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-2.5 dB) | -  |
| 1 | Output Gain |  | ok (15.6 dB) | -  |
| 2 | Zero Latency |  | ok (-7.9 dB) | -  |
| 3 | Auto Gain |  | ok (-10.2 dB) | -  |
| 4 | Delta |  | ok (-5.0 dB) | -  |
| 5 | Gate / Expander On |  | ok (-12.1 dB) | -  |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 5 | CONDITIONAL (-3.9 dB) | -  |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.2 dB) | -  |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-12.1 dB) | -  |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-12.1 dB) | -  |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-33.5 dB) | -  |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-16.6 dB) | -  |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.1 dB) | -  |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-16.7 dB) | -  |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.4 dB) | -  |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.9 dB) | -  |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-12.7 dB) | -  |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-13.4 dB) | -  |
| 19 | Gate / Expander Key Source |  | excused (-240.0 dB) | -  |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.4 dB) | -  |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | -  |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-26.6 dB) | -  |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.0 dB) | -  |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | -  |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | -  |
| 27 | Compressor On |  | ok (-12.9 dB) | -  |
| 28 | Compressor Threshold |  | ok (-11.5 dB) | -  |
| 29 | Compressor Ratio |  | ok (-12.9 dB) | -  |
| 30 | Compressor Knee |  | ok (-36.7 dB) | -  |
| 31 | Compressor Attack |  | ok (-12.2 dB) | -  |
| 32 | Compressor Release |  | ok (-20.8 dB) | -  |
| 33 | Compressor Detector |  | ok (-18.2 dB) | -  |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-20.8 dB) | -  |
| 35 | Compressor Look-Ahead |  | ok (-20.1 dB) | -  |
| 36 | Compressor Stereo Link |  | ok (-30.9 dB) | -  |
| 37 | Compressor Key Source |  | excused (-72.0 dB) | -  |
| 38 | Compressor Key High-Pass |  | ok (-14.3 dB) | -  |
| 39 | Compressor Key Low-Pass |  | ok (-14.2 dB) | -  |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | -  |
| 41 | Compressor Mix |  | ok (-12.9 dB) | -  |
| 42 | Compressor Auto Attack |  | ok (-19.5 dB) | -  |
| 43 | Compressor Auto Release |  | ok (-35.0 dB) | -  |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.8 dB) | -  |
| 45 | Compressor Key Boost |  | ok (-14.4 dB) | -  |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.1 dB) | -  |
| 47 | De-Expander On |  | ok (-72.0 dB) | -  |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-5.5 dB) | -  |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (-14.0 dB) | -  |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (-14.0 dB) | -  |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (-60.8 dB) | -  |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (-9.1 dB) | -  |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.7 dB) | -  |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (-43.3 dB) | -  |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (-25.7 dB) | -  |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (-12.8 dB) | -  |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-29.7 dB) | -  |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-6.0 dB) | -  |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-8.8 dB) | -  |
| 60 | De-Expander Key Source |  | excused (-72.0 dB) | -  |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | -  |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-17.0 dB) | -  |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (-14.0 dB) | -  |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (-34.8 dB) | -  |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (-43.2 dB) | -  |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (-61.8 dB) | -  |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-45.0 dB) | -  |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (-25.7 dB) | -  |
| 69 | Decompressor On |  | ok (-9.1 dB) | -  |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-9.1 dB) | -  |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-21.6 dB) | -  |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-9.1 dB) | -  |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-60.4 dB) | -  |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-9.1 dB) | -  |
| 75 | Decompressor Release | Decompressor On=1 | ok (-60.4 dB) | -  |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.8 dB) | -  |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.2 dB) | -  |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-9.2 dB) | -  |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (-8.9 dB) | -  |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-20.1 dB) | -  |
| 81 | Decompressor Key Source |  | excused (-72.0 dB) | -  |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-20.8 dB) | -  |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-9.1 dB) | -  |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-9.1 dB) | -  |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-18.4 dB) | -  |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.7 dB) | -  |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-34.0 dB) | -  |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-3.2 dB) | -  |
| 89 | Decompressor Match | Decompressor On=1 | ok (-9.1 dB) | -  |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-8.7 dB) | -  |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.5 dB) | -  |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.4 dB) | -  |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-60.6 dB) | -  |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-15.6 dB) | -  |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.1 dB) | -  |
| 96 | Limiter On |  | ok (-26.1 dB) | -  |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | -  |
| 98 | Limiter Ceiling |  | ok (-8.5 dB) | -  |
| 99 | Limiter Attack |  | ok (-72.0 dB) | -  |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-11.9 dB) | -  |
| 101 | Limiter Stereo Link |  | ok (-72.0 dB) | -  |
| 102 | Limiter Clean |  | ok (-72.0 dB) | -  |
| 103 | Limiter Bass Hold |  | ok (-72.0 dB) | -  |
| 104 | Limiter Auto Attack |  | ok (-72.0 dB) | -  |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-24.5 dB) | -  |
| 106 | Bypass |  | ok (-10.8 dB) | -  |
