# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 112 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 0.7% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.405%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.686%) |
| 2 | Zero Latency |  | ok (-9.3 dB) | ok (5.559%) |
| 3 | Auto Gain |  | ok (-11.5 dB) | ok (6.573%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.433%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.300%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (7.735%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.809%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (7.305%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.586%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (7.966%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.474%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.980%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.549%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (6.231%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.802%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (5.163%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (8.094%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (8.254%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.340%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (6.271%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.651%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (6.703%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (7.286%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (7.232%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (6.092%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (7.350%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (4.611%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.628%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.972%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.965%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.287%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.647%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (6.092%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (6.231%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.760%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.736%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.029%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.173%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.381%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (6.850%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.172%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.784%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.882%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (8.042%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.031%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (7.193%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (6.158%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.589%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.420%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (8.511%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.776%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.140%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.923%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (7.771%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.683%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.726%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.944%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.895%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (7.921%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (6.518%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (8.199%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (7.360%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (7.238%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.083%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.208%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.719%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.088%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.704%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.693%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.372%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.504%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.740%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.950%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.645%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (7.892%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (8.103%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.859%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.013%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (8.423%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.891%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.363%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.011%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (6.260%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (7.435%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (8.047%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (8.213%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (9.186%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (7.617%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.965%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.931%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (7.823%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.743%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (8.194%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (5.746%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (7.885%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (6.998%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (6.583%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (6.301%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.880%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (5.319%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (5.222%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.7 dB) | ok (5.784%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (6.512%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.875%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (6.410%) |
| 106 | Auto Target |  | excused (-21.8 dB) | ok (5.216%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (4.992%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (5.628%) |
| 109 | Bypass |  | ok (-9.0 dB) | ok (4.037%) |
| 110 | Wet |  | ok (-9.0 dB) | ok (5.953%) |
| 111 | Delta |  | ok (-3.5 dB) | ok (6.107%) |
