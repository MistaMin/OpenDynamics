# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 13.3% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.538%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.556%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (4.978%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.620%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (4.911%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.043%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (4.630%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (3.508%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (4.111%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (4.677%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (4.215%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.462%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (4.526%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.874%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.140%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.472%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (4.481%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (4.269%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (3.931%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.128%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (5.075%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (3.432%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (5.126%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.207%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.362%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.488%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.540%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (2.776%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (4.379%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.243%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (1.571%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (3.099%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (3.591%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (3.174%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (2.710%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (3.268%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (4.279%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (3.492%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.144%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (3.962%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (4.011%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (4.708%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.905%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (3.939%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (5.942%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (4.275%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (5.121%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (5.422%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (4.858%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.176%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.629%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.427%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.535%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (5.201%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (4.826%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (4.873%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.983%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (4.324%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (4.255%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (5.231%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.306%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (4.800%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (4.619%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.583%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.482%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.185%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.086%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (4.910%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.945%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.316%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (4.858%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (4.450%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (4.733%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (4.188%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (4.020%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (4.212%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (4.774%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (4.118%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (4.322%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (4.114%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (4.972%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.044%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (5.099%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (4.747%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (4.801%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (5.362%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (4.426%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (5.002%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (4.066%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (5.011%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (4.658%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (4.738%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (4.286%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (4.127%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (4.097%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (4.059%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (4.248%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.789%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.888%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (4.994%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (5.802%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (6.088%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.140%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.446%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.183%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (4.817%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (5.131%) |
| 107 | Bypass |  | ok (-9.0 dB) | ok (2.882%) |
| 108 | Wet |  | ok (-9.0 dB) | ok (3.559%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (4.629%) |
