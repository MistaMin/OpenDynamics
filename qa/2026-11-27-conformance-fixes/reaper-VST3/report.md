# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 112 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 9.5% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.259%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (4.271%) |
| 2 | Zero Latency |  | ok (-8.1 dB) | ok (3.832%) |
| 3 | Auto Gain |  | ok (-8.8 dB) | ok (3.983%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (3.851%) |
| 5 | Gate / Expander On |  | ok (-11.6 dB) | ok (3.962%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (-1.5 dB) | ok (4.995%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (4.101%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.497%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (4.585%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-8.7 dB) | ok (4.979%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (5.148%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.088%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.4 dB) | ok (5.476%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-2.6 dB) | ok (5.484%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (4.484%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-3.9 dB) | ok (3.087%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-12.4 dB) | ok (5.694%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-11.8 dB) | ok (4.881%) |
| 19 | Gate / Expander Key Source |  | excused (-240.0 dB) | ok (3.610%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (4.529%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-5.1 dB) | ok (5.594%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-9.5 dB) | ok (5.740%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (5.102%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (5.055%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (4.533%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (4.632%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (2.550%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (4.119%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (3.898%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (3.890%) |
| 31 | Compressor Attack |  | ok (-4.1 dB) | ok (4.113%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (3.903%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (4.285%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (4.156%) |
| 35 | Compressor Look-Ahead |  | ok (-9.3 dB) | ok (3.764%) |
| 36 | Compressor Stereo Link |  | ok (-4.5 dB) | ok (4.347%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (2.870%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.642%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (3.620%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (3.985%) |
| 41 | Compressor Mix |  | ok (-10.6 dB) | ok (3.797%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.072%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.136%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (5.413%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (4.327%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.148%) |
| 47 | De-Expander On |  | ok (-6.7 dB) | ok (5.489%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-3.6 dB) | ok (6.795%) |
| 49 | De-Expander Ratio | De-Expander On=1 | ok (-8.5 dB) | ok (4.990%) |
| 50 | De-Expander Range | De-Expander On=1 | ok (-3.7 dB) | ok (5.233%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (-1.5 dB) | ok (6.495%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (-1.4 dB) | ok (6.843%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (5.382%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (-1.4 dB) | ok (6.323%) |
| 55 | De-Expander Release | De-Expander On=1 | ok (-6.6 dB) | ok (6.880%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (-1.8 dB) | ok (5.658%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-8.4 dB) | ok (5.746%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-6.4 dB) | ok (5.466%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-8.4 dB) | ok (4.598%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (3.727%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (5.957%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (6.476%) |
| 63 | De-Expander Mix | De-Expander On=1 | ok (-3.7 dB) | ok (6.079%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (-1.5 dB) | ok (7.558%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (-1.2 dB) | ok (7.272%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (-1.2 dB) | ok (5.689%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-3.7 dB) | ok (6.483%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (-1.5 dB) | ok (6.807%) |
| 69 | Decompressor On |  | ok (-8.6 dB) | ok (5.226%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-5.2 dB) | ok (6.013%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-5.3 dB) | ok (5.393%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.205%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-4.2 dB) | ok (5.937%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (4.665%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (8.233%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (5.092%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-4.4 dB) | ok (6.252%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-9.0 dB) | ok (5.974%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (-8.4 dB) | ok (6.491%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-26.0 dB) | ok (6.742%) |
| 81 | Decompressor Key Source |  | excused (-240.0 dB) | ok (4.569%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-7.8 dB) | ok (5.283%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-7.8 dB) | ok (5.566%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-8.6 dB) | ok (7.883%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-5.2 dB) | ok (5.980%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.2 dB) | ok (5.577%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-7.9 dB) | ok (6.422%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-2.8 dB) | ok (5.743%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-4.2 dB) | ok (6.918%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (5.452%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-3.8 dB) | ok (7.040%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.729%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.824%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-3.8 dB) | ok (6.013%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-3.1 dB) | ok (5.300%) |
| 96 | Limiter On |  | ok (-25.9 dB) | ok (5.558%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.747%) |
| 98 | Limiter Ceiling |  | ok (-8.3 dB) | ok (5.392%) |
| 99 | Limiter Attack | random state 1 | CONDITIONAL (-1.4 dB) | ok (4.375%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-8.9 dB) | ok (5.616%) |
| 101 | Limiter Stereo Link |  | ok (-8.4 dB) | ok (4.480%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (-1.9 dB) | ok (4.316%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (-13.4 dB) | ok (4.365%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (-1.5 dB) | ok (4.103%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-28.6 dB) | ok (5.257%) |
| 106 | Auto Target |  | excused (-240.0 dB) | ok (5.377%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (4.372%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (3.882%) |
| 109 | Bypass |  | ok (-9.0 dB) | ok (2.600%) |
| 110 | Wet |  | ok (-9.0 dB) | ok (3.439%) |
| 111 | Delta |  | ok (-3.5 dB) | ok (4.139%) |
