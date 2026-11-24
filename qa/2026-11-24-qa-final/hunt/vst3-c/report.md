# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
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
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.258%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.340%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.805%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (6.188%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.392%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (6.341%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.561%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.226%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (8.832%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (7.095%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (6.429%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.187%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.271%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.624%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.679%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.669%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (7.474%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (6.805%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (5.412%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.153%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.219%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (7.541%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (7.570%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.262%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.829%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.521%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.989%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.902%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (4.538%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.612%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (6.238%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.466%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.284%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.148%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.809%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.632%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (6.428%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.141%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.755%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.347%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (6.379%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.770%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.711%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.791%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.527%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.886%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.034%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (5.991%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (7.112%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.477%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.293%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.144%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.732%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.741%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.247%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.806%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.915%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.212%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.667%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (6.603%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.587%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (7.107%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (6.782%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.562%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.435%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.918%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.233%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.316%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.433%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.232%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.495%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (8.616%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (8.684%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.386%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (8.852%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (8.296%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.887%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.732%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.008%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.309%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (7.234%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (6.094%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.301%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.451%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (5.347%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.819%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.569%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.649%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (5.986%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.617%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (8.149%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (9.013%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (8.050%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (8.920%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (8.245%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (5.745%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (4.508%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.523%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.222%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (4.732%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (8.265%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (7.058%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.911%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.980%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (6.760%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (6.751%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (4.903%) |
| 107 | Bypass |  | ok (-9.0 dB) | ok (3.686%) |
| 108 | Wet |  | ok (-9.0 dB) | ok (6.445%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (6.306%) |
