# Plugin QA: AU: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: OpenDynamics (OpenGrid), 109 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 12.9% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | FAIL | Zero Latency (off -> on), Delta (off -> on), Gate / Expander On (off -> on), Gate / Expander Detector (off -> on), Gate / Expander Auto Release (off -> on), Compressor Key Source (off -> on), Compressor Auto Attack (off -> on), De-Expander On (off -> on), De-Expander Detector (off -> on), De-Expander Auto Attack (off -> on), Decompressor On (off -> on), Decompressor Detector (Peak -> RMS), Decompressor Ceiling (off -> on), Decompressor Auto Attack (off -> on), Limiter On (off -> on), Limiter Clean (off -> on), Limiter Auto Attack (off -> on), Limiter Auto Release (off -> on) |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.400%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.820%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (4.987%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.010%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.276%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (4.935%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (5.037%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (3.668%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (4.679%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (4.288%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (3.729%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.884%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (4.596%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.844%) |
| 14 | Gate / Expander Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.842%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.618%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (3.919%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (3.696%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (4.338%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.358%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (4.529%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (4.626%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (4.917%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.409%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.437%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.732%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.437%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.968%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.689%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.174%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.567%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (3.842%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (4.146%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (4.239%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (3.538%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.735%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.392%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.583%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.512%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.284%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.461%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (4.327%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.736%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.508%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (4.544%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.424%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (4.558%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.5 dB) | ok (4.755%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (4.539%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.369%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.447%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (3.831%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (3.997%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (3.852%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (4.746%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (4.449%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (3.910%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (4.371%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (4.257%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (4.549%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.666%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (3.666%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (4.767%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (4.617%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (4.515%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.030%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.086%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (3.750%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (4.285%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (3.260%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (4.326%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (4.180%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (4.439%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (4.305%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (3.986%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (4.878%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (5.018%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (4.005%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (3.919%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (4.214%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (3.886%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.849%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (4.632%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (4.517%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (3.774%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (4.312%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (4.295%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (4.631%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (3.735%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (3.936%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (4.256%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (4.340%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (4.525%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (4.085%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (3.916%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (3.565%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (3.080%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.835%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.726%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (4.700%) |
| 100 | Limiter Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (3.741%) |
| 101 | Limiter Stereo Link | random state 1 | CONDITIONAL (2.1 dB) | ok (3.932%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (4.311%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.034%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.386%) |
| 105 | Limiter Auto Release | Limiter Clean=0random state 1 | CONDITIONAL (2.1 dB) | ok (4.221%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (3.602%) |
| 107 | Wet |  | ok (-9.0 dB) | ok (3.640%) |
| 108 | Delta |  | ok (-9.0 dB) | ok (4.798%) |
