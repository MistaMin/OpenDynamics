# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 112 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 12.4% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.984%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.688%) |
| 2 | Zero Latency |  | ok (-8.1 dB) | ok (5.778%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.884%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.254%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.598%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.332%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.143%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.409%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.451%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (6.333%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.955%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.770%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (5.680%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (6.080%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (4.863%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (3.642%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (6.213%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (5.758%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (4.297%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (5.870%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.233%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (5.374%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (6.052%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (6.803%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (5.856%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (5.836%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.716%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.107%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.463%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.460%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.537%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.321%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.298%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.424%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.372%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (4.788%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (3.506%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.726%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.274%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (4.852%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (4.216%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (3.729%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.689%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.311%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (4.055%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.707%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (4.592%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.498%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.481%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.542%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.248%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.585%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.463%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.204%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.176%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.757%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (5.924%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.183%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (5.411%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.193%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (5.723%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.292%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.603%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.770%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.789%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.867%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (5.658%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.928%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.919%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.035%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (5.773%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.683%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (5.964%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (6.042%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.406%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.492%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.819%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.680%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (6.211%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.113%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.297%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.377%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (6.077%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.002%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.802%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.010%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.309%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (5.881%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.248%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (5.627%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (7.934%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.793%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.785%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.406%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.708%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (4.463%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.307%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.111%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.374%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (5.941%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (4.395%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.7 dB) | ok (5.148%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.721%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (6.254%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (6.721%) |
| 106 | Auto Target |  | excused (-21.8 dB) | ok (4.945%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (5.101%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (5.594%) |
| 109 | Bypass |  | ok (-9.0 dB) | ok (4.721%) |
| 110 | Wet |  | ok (-9.0 dB) | ok (5.754%) |
| 111 | Delta |  | ok (-3.5 dB) | ok (5.871%) |
