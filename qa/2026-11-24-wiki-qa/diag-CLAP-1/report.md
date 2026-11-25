# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 109 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 1.3% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.825%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.753%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (6.347%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (7.153%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.507%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (6.677%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.369%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (4.847%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.493%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.596%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.5 dB) | ok (6.583%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.457%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.009%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.259%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.161%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.553%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (5.717%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-5.6 dB) | ok (6.023%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-5.3 dB) | ok (5.751%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.338%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.216%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.082%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (5.913%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.211%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.013%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.901%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.217%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (4.679%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.242%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.687%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.808%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (6.593%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (6.516%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (6.095%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (6.322%) |
| 35 | Compressor Look-Ahead |  | ok (-4.1 dB) | ok (6.045%) |
| 36 | Compressor Stereo Link |  | ok (-4.1 dB) | ok (6.205%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (6.362%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (6.695%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (6.184%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (6.329%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.985%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.998%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.994%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.148%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.447%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.953%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (7.386%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (7.382%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.017%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (5.990%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.117%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.774%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.780%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.691%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.110%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.606%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (5.891%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.848%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (6.570%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.987%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.290%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.534%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.634%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (7.037%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.730%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.975%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.984%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.376%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (6.136%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.121%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (7.805%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (8.421%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.608%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.257%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.406%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.383%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.216%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.398%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.832%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (7.103%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.502%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (7.340%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.451%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (7.023%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.883%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.101%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.344%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.536%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.396%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.277%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.566%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.009%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.437%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.582%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.583%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.225%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.518%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.529%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (4.613%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (6.164%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.2 dB) | ok (5.612%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.856%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (6.316%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (6.610%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (6.293%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (4.389%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (4.894%) |
| 108 | Delta |  | ok (-3.5 dB) | ok (5.313%) |
