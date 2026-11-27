# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 111 parameters |
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
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.559%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.248%) |
| 2 | Zero Latency |  | ok (-27.2 dB) | ok (4.565%) |
| 3 | Auto Gain |  | ok (-5.7 dB) | ok (5.926%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (3.622%) |
| 5 | Gate / Expander On |  | ok (-11.6 dB) | ok (4.762%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.038%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.584%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.847%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.429%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (6.348%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (4.857%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.401%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (8.393%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (6.884%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.981%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (5.092%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-12.4 dB) | ok (5.699%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-20.9 dB) | ok (5.043%) |
| 19 | Gate / Expander Key Source |  | excused (-240.0 dB) | ok (4.165%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (5.810%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.879%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (6.649%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (5.484%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (5.127%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (5.594%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (4.795%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.155%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.752%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (6.532%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.187%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (4.476%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (4.916%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (3.875%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.328%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.118%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (4.528%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (5.014%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.176%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.040%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (4.686%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (3.823%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.012%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (3.993%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (5.694%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (4.821%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.818%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (5.503%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (5.709%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.136%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (5.550%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (4.707%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.148%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.747%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.816%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.583%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.597%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (4.812%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-4.1 dB) | ok (6.099%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (6.496%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (4.325%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.416%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (4.530%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.421%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (5.590%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.029%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (8.361%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (5.094%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.411%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (6.653%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (8.077%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (5.346%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (7.576%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (8.711%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.455%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.957%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (8.767%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (4.845%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.272%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (8.508%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (5.238%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.176%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.356%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (5.472%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (5.298%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (4.982%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (5.487%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (6.523%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (5.308%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.022%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.094%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (7.343%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (8.100%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.747%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (7.247%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.387%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.108%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.368%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (3.790%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.031%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (6.161%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (4.164%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (4.028%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (4.190%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.843%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (8.101%) |
| 106 | Auto Target |  | excused (-21.8 dB) | ok (5.464%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (6.610%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (3.996%) |
| 109 | Wet |  | ok (-9.0 dB) | ok (5.379%) |
| 110 | Delta |  | ok (-3.5 dB) | ok (5.088%) |
