# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 112 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 0.0% animates by itself |
| every parameter changes the sound | FAIL | no sound at either end of the sweep, so these were never heard (the audio went silent mid-run, or the parameter mutes at both ends and needs an audio: exception): Compressor Ratio |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.684%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.306%) |
| 2 | Zero Latency |  | ok (-9.3 dB) | ok (5.414%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.116%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (4.747%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (4.936%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.310%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.672%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.426%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.906%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (6.155%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.229%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.625%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.972%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (6.574%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (4.743%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (4.346%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (5.789%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (5.994%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (4.565%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (6.419%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.731%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (5.993%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (5.495%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (6.130%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (5.832%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (6.955%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.358%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.386%) |
| 29 | Compressor Ratio |  | SILENT (-7.7 dB) | ok (0.189%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.478%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.071%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (3.899%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (4.382%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.851%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.957%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (3.799%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (3.439%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.695%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.766%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.110%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (6.386%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.838%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.118%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (5.847%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (4.872%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (7.813%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (6.494%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (9.384%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (7.188%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (8.184%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (8.210%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.245%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.644%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (7.746%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (8.449%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.069%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (7.341%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (7.763%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (8.117%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.962%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (8.170%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (7.816%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (7.787%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.976%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.607%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.497%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (7.514%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (9.085%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (6.197%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (9.351%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (9.019%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (8.302%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.568%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.683%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (8.022%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (8.979%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (8.412%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.937%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.664%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.740%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.134%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (8.420%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (8.725%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (8.437%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (8.332%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (9.010%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (9.947%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (7.714%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (8.150%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (8.151%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (8.051%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (9.006%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (7.523%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (7.429%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (8.838%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (6.549%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.989%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (6.665%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.637%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (6.589%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (5.148%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.7 dB) | ok (4.917%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.551%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.583%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (7.387%) |
| 106 | Auto Target |  | excused (-21.8 dB) | ok (5.595%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (4.813%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (5.232%) |
| 109 | Bypass |  | ok (-9.0 dB) | ok (4.238%) |
| 110 | Wet |  | ok (-9.0 dB) | ok (4.651%) |
| 111 | Delta |  | ok (-3.5 dB) | ok (6.171%) |
