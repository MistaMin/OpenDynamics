# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 12.7% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.888%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.874%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.738%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.940%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.161%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.119%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.598%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.162%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.919%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.336%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (6.306%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.792%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.804%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.886%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.904%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.001%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (5.886%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (5.300%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (5.550%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (4.409%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.337%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.538%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (6.810%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.719%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.550%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.237%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.098%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.568%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (4.990%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.670%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.806%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.095%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (4.977%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (4.389%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.605%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.328%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (4.888%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.210%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.027%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.481%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.245%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.147%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.048%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.940%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.129%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.099%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.271%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (5.768%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (5.993%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.652%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.636%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.885%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.867%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (5.801%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (5.997%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (5.660%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.917%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.691%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.345%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (5.825%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.179%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.283%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.959%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.178%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.322%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.971%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.896%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (5.563%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.740%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.443%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.151%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.386%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (5.805%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (5.798%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (6.275%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.194%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.532%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.131%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.030%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (6.966%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.547%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.131%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (7.013%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (6.156%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.275%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.808%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.189%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.431%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.192%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.427%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (5.799%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.363%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.553%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.811%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.420%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.258%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (4.853%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.100%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.179%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (4.803%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (5.602%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (5.094%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.635%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.223%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.626%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (5.662%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (4.843%) |
| 107 | Bypass |  | ok (-9.0 dB) | ok (3.327%) |
| 108 | Wet |  | ok (-9.0 dB) | ok (5.235%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (5.024%) |
