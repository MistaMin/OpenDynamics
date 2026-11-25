# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 109 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.1% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (6.188%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (8.126%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (7.048%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (7.027%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.172%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (6.083%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.693%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.684%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.445%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.677%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.5 dB) | ok (6.543%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.965%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.684%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.098%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.451%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.344%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (6.049%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (5.595%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (6.256%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.448%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (7.077%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.592%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (5.967%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.141%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.660%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.083%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.826%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.966%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.088%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.551%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (6.508%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (6.155%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (6.251%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (6.056%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.501%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.132%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.783%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (6.627%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (6.374%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.692%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (7.022%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (7.043%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (6.175%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.438%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.908%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (6.211%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.239%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (6.072%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.791%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.514%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.191%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.240%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.224%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (5.677%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.568%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.992%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.593%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (7.228%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.1 dB) | ok (7.335%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (6.493%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.631%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.345%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (7.208%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.821%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.767%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.765%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.101%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.323%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.668%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.893%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.039%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (7.610%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (7.503%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.827%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (6.627%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.331%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (7.755%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.789%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.238%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.336%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.930%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (6.356%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.998%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (6.691%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.714%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (7.458%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.156%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.967%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (7.197%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.885%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (7.257%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (7.461%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.212%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.849%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (7.160%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (7.286%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.254%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.891%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (6.856%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (6.197%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (6.800%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.2 dB) | ok (6.236%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (7.252%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (6.600%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (6.385%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (5.781%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (3.413%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (4.603%) |
| 108 | Delta |  | ok (-3.5 dB) | ok (6.332%) |
