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
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.716%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.162%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (6.033%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (6.601%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.154%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (6.469%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (3.0 dB) | ok (7.002%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.626%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.808%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (5.975%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.5 dB) | ok (5.940%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.266%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.502%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.455%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.315%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.806%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (6.586%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (6.257%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (7.001%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.997%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (5.964%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.117%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (6.005%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.443%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.786%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.295%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.817%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (4.240%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.756%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.727%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.157%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.158%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.356%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.269%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.914%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.895%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.770%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (5.105%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.279%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.501%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.616%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (4.988%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.236%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.037%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.515%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.586%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.935%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (5.739%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.352%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.763%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.258%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.327%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.157%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.434%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.169%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.924%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.268%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.043%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-3.9 dB) | ok (6.201%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (6.874%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.788%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.444%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.435%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.775%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.593%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.635%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.589%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.148%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.180%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (4.950%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.801%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.839%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.605%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (6.596%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (6.100%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.157%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.858%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.354%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.042%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (6.055%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (7.113%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.267%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.904%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (6.310%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (5.914%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.753%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.368%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (6.732%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.204%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.734%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.505%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (7.113%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.431%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.192%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (6.623%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.747%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.101%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.967%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.817%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.438%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (6.510%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.2 dB) | ok (5.580%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.101%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (5.134%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.295%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.2 dB) | ok (6.248%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (3.891%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (4.614%) |
| 108 | Delta |  | ok (-3.5 dB) | ok (5.938%) |
