# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 111 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.4% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.882%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.820%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (6.580%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (6.302%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.482%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (7.097%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (3.0 dB) | ok (6.877%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.256%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.091%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (5.230%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (5.582%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.408%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.739%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (5.877%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (5.510%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.525%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (4.409%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (5.392%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (6.756%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (4.386%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (5.617%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.731%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (5.939%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (6.018%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (5.682%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (6.331%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (7.364%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.860%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.885%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.001%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.188%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (6.058%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (6.166%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.129%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.459%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.719%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.783%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (4.468%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.871%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.314%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (6.880%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.742%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.314%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.547%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (5.867%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.141%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.903%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.7 dB) | ok (6.637%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (5.982%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (7.153%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (7.325%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.920%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.433%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.272%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (7.067%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.118%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.215%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (5.302%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.354%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (7.145%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (6.165%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (7.056%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (6.412%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.682%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.407%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.489%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.546%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.351%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.508%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.672%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.422%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.792%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (7.115%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.022%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.098%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.837%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (8.061%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (5.975%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.225%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.536%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (7.760%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.936%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (7.018%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.561%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (7.121%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (7.328%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.403%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.960%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.855%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (7.318%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (7.300%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (7.611%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.279%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (7.273%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.674%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (7.426%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.415%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.360%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.602%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (4.164%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (5.041%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (4.207%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.087%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (5.122%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.148%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (5.261%) |
| 106 | Auto Target |  | excused (-21.8 dB) | ok (4.378%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (5.128%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (3.855%) |
| 109 | Wet |  | ok (-9.0 dB) | ok (5.003%) |
| 110 | Delta |  | ok (-3.5 dB) | ok (5.273%) |
