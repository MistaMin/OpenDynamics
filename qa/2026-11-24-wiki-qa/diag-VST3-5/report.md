# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.5% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.734%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.817%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (6.238%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (6.578%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.712%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.499%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.744%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.356%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.752%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.633%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (6.632%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.001%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.594%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.367%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.353%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.836%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (5.188%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (6.274%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (6.322%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.530%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.488%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.525%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (5.950%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.136%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.126%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.993%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.426%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.861%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.966%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.865%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.189%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.122%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.582%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (4.288%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.145%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.510%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.117%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.412%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.946%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.476%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.263%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.422%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.041%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.548%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.869%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.231%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.448%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (5.710%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (5.954%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.731%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.479%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.811%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.486%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.352%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.837%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (7.008%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.533%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.539%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.078%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (6.731%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.895%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.637%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.806%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.907%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.782%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.991%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.856%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.599%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.456%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.358%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.659%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (7.399%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (5.020%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.246%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (6.748%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (7.405%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.746%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.244%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.404%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.730%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.768%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.784%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.638%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.019%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.794%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (8.269%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (6.933%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.487%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (5.958%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (5.702%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.364%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.513%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.087%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.630%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.248%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.668%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.438%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.825%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.800%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.207%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (6.667%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (5.996%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.462%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (4.671%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.891%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (6.362%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (5.816%) |
| 107 | Bypass |  | ok (-9.0 dB) | ok (4.233%) |
| 108 | Wet |  | ok (-9.0 dB) | ok (6.092%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (4.833%) |
