# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 109 parameters |
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
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.105%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.853%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.823%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.996%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (4.839%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.935%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.965%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.825%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.038%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (5.966%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.5 dB) | ok (6.526%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.621%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.961%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.526%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.836%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.835%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (6.476%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (6.701%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (5.835%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.684%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.647%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (5.808%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (6.513%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.129%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.272%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.681%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.952%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.715%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.436%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.245%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.628%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.593%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.730%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.331%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.496%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.923%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.308%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (5.166%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.857%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.609%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.852%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.070%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.357%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.081%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (7.046%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.315%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.620%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (5.249%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.854%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.663%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.861%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.706%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.585%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.178%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (7.632%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.641%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.023%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.092%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (5.928%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (5.968%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.636%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.871%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (6.808%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.523%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.589%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.790%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.631%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.822%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.113%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.855%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.219%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.238%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.098%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (6.180%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (5.874%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.595%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (5.891%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.257%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.337%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (6.120%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.069%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.093%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.591%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (6.019%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.736%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (7.005%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.453%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.800%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.609%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.648%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.020%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.250%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.315%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.341%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (7.202%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (7.087%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.735%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.874%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.914%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.623%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (5.922%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.2 dB) | ok (5.762%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.768%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (5.483%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.264%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (5.101%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (3.621%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (4.944%) |
| 108 | Delta |  | ok (-3.5 dB) | ok (5.372%) |
