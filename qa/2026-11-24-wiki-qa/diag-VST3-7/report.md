# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 2.9% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.601%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.994%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (6.538%) |
| 3 | Auto Gain |  | ok (-9.6 dB) | ok (6.856%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.880%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (6.364%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.076%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.245%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.607%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.554%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (7.123%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.952%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.749%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.074%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.501%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.815%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (7.033%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (7.286%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (7.086%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (4.988%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (7.365%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (7.364%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (7.117%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.673%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.037%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.764%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.345%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.828%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.333%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (7.024%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (6.731%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (6.407%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (7.098%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.347%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.313%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.601%) |
| 36 | Compressor Stereo Link |  | ok (-31.0 dB) | ok (5.601%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (5.206%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.108%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.252%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.848%) |
| 41 | Compressor Mix |  | ok (-10.6 dB) | ok (5.975%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.555%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.807%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.390%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.947%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (7.388%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (7.578%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.419%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.750%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (7.025%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.665%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.018%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.147%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (7.014%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.943%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.582%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (7.212%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (7.124%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (7.274%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.721%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.970%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (6.743%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.729%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.244%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.519%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.697%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.793%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.041%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.988%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.597%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.908%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (7.237%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.153%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.462%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (7.606%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.984%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.657%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.963%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.745%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.557%) |
| 81 | Decompressor Key Source |  | excused (-240.0 dB) | ok (5.167%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (7.525%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.685%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.778%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.429%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-6.8 dB) | ok (6.822%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (8.054%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (7.248%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (7.017%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (7.187%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.980%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.249%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (8.537%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (7.406%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.887%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.145%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (7.449%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (7.203%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (6.701%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (6.483%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (5.637%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (7.332%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.758%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.855%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (6.026%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (5.127%) |
| 107 | Bypass |  | ok (-9.0 dB) | ok (5.148%) |
| 108 | Wet |  | ok (-9.0 dB) | ok (5.916%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (6.277%) |
