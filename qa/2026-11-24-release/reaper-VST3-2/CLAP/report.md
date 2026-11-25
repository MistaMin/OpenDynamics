# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 109 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.9% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.989%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.643%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.916%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (6.132%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.322%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.804%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (4.879%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (3.407%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (4.591%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (4.677%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (4.403%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.313%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (4.702%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.590%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.653%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.398%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (4.263%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (4.669%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (5.022%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.633%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (5.652%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (4.871%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (4.605%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.005%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.381%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.368%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.699%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (2.938%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (4.517%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.470%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.348%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.217%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (4.434%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.862%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (3.687%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.779%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.418%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (5.303%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (3.954%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.443%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.207%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (4.049%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (3.942%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.134%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (5.090%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.940%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (5.637%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (5.989%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (5.678%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.870%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.677%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.932%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.832%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (4.974%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (4.282%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (5.351%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.492%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (4.802%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (4.760%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (4.186%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.647%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (5.159%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (4.363%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (3.306%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.770%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.245%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.819%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (4.394%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.286%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.651%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (5.640%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (4.427%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (4.931%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (3.799%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (5.532%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (4.890%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (4.768%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (4.888%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (5.231%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (5.155%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (5.052%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.500%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (4.404%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (3.943%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (5.131%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (4.983%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (4.978%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (6.177%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (4.603%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (4.968%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (4.607%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (4.282%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (5.239%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (5.146%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (4.628%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (3.968%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (3.623%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.762%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.608%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.487%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (4.339%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (4.625%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (4.959%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (4.881%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.301%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (4.485%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (3.022%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (3.855%) |
| 108 | Delta |  | ok (-9.0 dB) | ok (4.324%) |
