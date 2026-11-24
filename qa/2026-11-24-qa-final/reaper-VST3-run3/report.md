# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.3% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.544%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.297%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.566%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.689%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.132%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.078%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.936%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.467%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.208%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.083%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (6.114%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.848%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.870%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.761%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.019%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.864%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (7.142%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (6.670%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (6.352%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.131%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.692%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.652%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (6.563%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.842%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.019%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.507%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.176%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (4.402%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.093%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.949%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.183%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.267%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (6.020%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.236%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (6.041%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.612%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.493%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.129%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.884%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.568%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.375%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.248%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.378%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.085%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.986%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.865%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.941%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (5.774%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (5.946%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.958%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.820%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.474%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.637%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.906%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.067%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.621%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.248%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.131%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.157%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (5.580%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.442%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (7.235%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (7.203%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (7.180%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.157%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.247%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.367%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (5.929%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.475%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.260%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.801%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.628%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (7.579%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (6.195%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.921%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.414%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.545%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.084%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.031%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.035%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (7.130%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.860%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.794%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (8.167%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (7.245%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (8.721%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.446%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.134%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.590%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.202%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.074%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.277%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.628%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (7.118%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.388%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (7.416%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (4.910%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.604%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.490%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (4.697%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (4.661%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (4.580%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (4.154%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.241%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (6.045%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (7.672%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (5.228%) |
| 107 | Bypass |  | ok (-9.0 dB) | ok (3.712%) |
| 108 | Wet |  | ok (-9.0 dB) | ok (4.706%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (4.929%) |
