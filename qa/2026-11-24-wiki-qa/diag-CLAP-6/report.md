# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 109 parameters |
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
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.526%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.952%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (4.767%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.093%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.936%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.165%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (3.0 dB) | ok (6.143%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (4.887%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.295%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.536%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.5 dB) | ok (6.928%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.493%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.572%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.924%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.443%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.179%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (7.172%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (6.882%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (5.710%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.379%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (7.151%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (5.986%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (6.138%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.449%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.359%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.296%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.914%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.923%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.107%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.681%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (6.313%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (3.669%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (4.866%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (4.914%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (6.343%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.967%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (4.861%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (5.092%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.364%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.160%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.539%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.605%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.560%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.213%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.758%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.521%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (7.121%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (5.772%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (7.253%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.905%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (7.041%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (5.814%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.874%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.632%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.683%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.755%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.217%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.602%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-4.4 dB) | ok (6.777%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (7.021%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.369%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.917%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.937%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.097%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.312%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.461%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.140%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (7.558%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.544%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.433%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.120%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.287%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.231%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (6.805%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.245%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.801%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.703%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.665%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.217%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.310%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.395%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.513%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.230%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.583%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.382%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.955%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (6.771%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.577%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (7.144%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (7.100%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.228%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.985%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.749%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.570%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.423%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (7.245%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (4.909%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.946%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.728%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.212%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (6.549%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.2 dB) | ok (5.086%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.734%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (5.804%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.057%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (5.354%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (3.832%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (4.413%) |
| 108 | Delta |  | ok (-3.5 dB) | ok (5.375%) |
