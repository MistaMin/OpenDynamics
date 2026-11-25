# Plugin QA: AU: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: OpenDynamics (OpenGrid), 109 parameters |
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
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.612%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (4.602%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (4.465%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (4.544%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (4.314%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (4.914%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.991%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (4.704%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.329%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (5.374%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.5 dB) | ok (6.815%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.835%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (7.389%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.377%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.742%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.185%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (6.227%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (5.885%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (6.206%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (4.146%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.037%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.363%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (5.695%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.178%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.086%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.555%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.246%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.500%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.355%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.591%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.790%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (4.863%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (4.406%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (3.239%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.754%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.626%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.412%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (5.204%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.192%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.007%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.628%) |
| 41 | Compressor Mix |  | ok (-10.6 dB) | ok (5.617%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.361%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.616%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.989%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (4.235%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.471%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (5.164%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (5.818%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (7.975%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (7.618%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (8.223%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (8.254%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.287%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.993%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.387%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.359%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (5.787%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.487%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (5.906%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.282%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (7.307%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (7.773%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.572%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.377%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.019%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.532%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.811%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.028%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.214%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.687%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-19.0 dB) | ok (7.692%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.655%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (7.906%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.027%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (7.359%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (7.398%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.388%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.728%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (6.907%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.834%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.888%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (7.815%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.182%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.167%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-12.2 dB) | ok (6.739%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.270%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.753%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.789%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-6.6 dB) | ok (7.518%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.719%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.325%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.288%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (7.272%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.299%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.801%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (4.098%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.095%) |
| 98 | Limiter Ceiling |  | ok (-8.9 dB) | ok (4.135%) |
| 99 | Limiter Attack |  | ok (-18.4 dB) | ok (3.996%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (4.690%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.2 dB) | ok (3.959%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (3.970%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (4.009%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.135%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (4.978%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (3.057%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (3.793%) |
| 108 | Delta |  | ok (-3.5 dB) | ok (3.863%) |
