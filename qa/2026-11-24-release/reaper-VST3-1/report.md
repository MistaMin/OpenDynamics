# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 12.4% animates by itself |
| every parameter changes the sound | FAIL | no sound at either end of the sweep, so these were never heard (the audio went silent mid-run, or the parameter mutes at both ends and needs an audio: exception): De-Expander Range, De-Expander Knee, De-Expander Floor, De-Expander Attack, De-Expander Hold, De-Expander Release, De-Expander Detector, De-Expander RMS Window, De-Expander Look-Ahead, De-Expander Stereo Link, De-Expander Key High-Pass, De-Expander Key Low-Pass, De-Expander Mix, De-Expander Auto Attack, De-Expander Auto Release, De-Expander Key Boost Freq, De-Expander Key Boost, De-Expander Key Boost Q, Decompressor On, Decompressor Threshold, Decompressor Ratio, Decompressor Range, Decompressor Knee, Decompressor Attack, Decompressor Release, Decompressor Detector, Decompressor RMS Window, Decompressor Transient Crest, Decompressor Look-Ahead, Decompressor Stereo Link, Decompressor Key High-Pass, Decompressor Key Low-Pass, Decompressor Mix, Decompressor Ceiling, Decompressor Ceiling Level, Decompressor Ceiling Type, Decompressor Output Trim, Decompressor Match, Decompressor Match Target Crest, Decompressor Auto Attack, Decompressor Auto Release, Decompressor Key Boost Freq, Decompressor Key Boost, Decompressor Key Boost Q, Limiter On, Limiter Threshold, Limiter Ceiling, Limiter Attack, Limiter Release, Limiter Stereo Link, Limiter Clean, Limiter Bass Hold, Limiter Auto Attack, Limiter Auto Release, Bypass, Bypass, Wet, Delta |
| every parameter is shown in the GUI | FAIL | De-Expander Detector, De-Expander RMS Window, De-Expander Look-Ahead, De-Expander Key Source, De-Expander Key High-Pass, De-Expander Key Low-Pass, De-Expander Mix, De-Expander Auto Attack, De-Expander Auto Release, De-Expander Key Boost Freq, De-Expander Key Boost, De-Expander Key Boost Q, Decompressor On, Decompressor Threshold, Decompressor Ratio, Decompressor Range, Decompressor Knee, Decompressor Attack, Decompressor Release, Decompressor Detector, Decompressor RMS Window, Decompressor Transient Crest, Decompressor Look-Ahead, Decompressor Stereo Link, Decompressor Key Source, Decompressor Key High-Pass, Decompressor Key Low-Pass, Decompressor Mix, Decompressor Ceiling, Decompressor Ceiling Level, Decompressor Ceiling Type, Decompressor Output Trim, Decompressor Match, Decompressor Match Target Crest, Decompressor Auto Attack, Decompressor Auto Release, Decompressor Key Boost Freq, Decompressor Key Boost, Decompressor Key Boost Q, Limiter On, Limiter Threshold, Limiter Ceiling, Limiter Attack, Limiter Release, Limiter Stereo Link, Limiter Clean, Limiter Bass Hold, Limiter Auto Attack, Limiter Auto Release, Bypass, Wet |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.739%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.148%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.608%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (4.802%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.578%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (4.783%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.469%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.210%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.985%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.443%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.4 dB) | ok (7.550%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (8.580%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (8.235%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (8.198%) |
| 14 | Gate / Expander Release | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (8.371%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (7.897%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (7.707%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (8.018%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (8.906%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (6.504%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (6.555%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.593%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (7.327%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.944%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1 | skipped (no audible random state) (-240.0 dB) | ok (7.450%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (7.918%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1 | skipped (no audible random state) (-240.0 dB) | ok (8.324%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.966%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.425%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.757%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (6.297%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.837%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.504%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (4.983%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.540%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.444%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (6.189%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.547%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.972%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.093%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.869%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.825%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.541%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.510%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.006%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.611%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.342%) |
| 47 | De-Expander On |  | skipped (no audible random state) (-240.0 dB) | ok (6.543%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (7.105%) |
| 49 | De-Expander Ratio | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (5.942%) |
| 50 | De-Expander Range | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.492%) |
| 51 | De-Expander Knee | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.067%) |
| 52 | De-Expander Floor | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 53 | De-Expander Attack | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 54 | De-Expander Hold | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 55 | De-Expander Release | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 56 | De-Expander Detector | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.049%) |
| 60 | De-Expander Key Source |  | excused (-7.7 dB) | NOT SHOWN (0.000%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 63 | De-Expander Mix | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 64 | De-Expander Auto Attack | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 65 | De-Expander Auto Release | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 69 | Decompressor On |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 70 | Decompressor Threshold | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 71 | Decompressor Ratio | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 73 | Decompressor Knee | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 74 | Decompressor Attack | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 75 | Decompressor Release | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 76 | Decompressor Detector | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 81 | Decompressor Key Source |  | excused (-7.7 dB) | NOT SHOWN (0.000%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 84 | Decompressor Mix | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 89 | Decompressor Match | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 96 | Limiter On |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 97 | Limiter Threshold |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 98 | Limiter Ceiling |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 99 | Limiter Attack |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 100 | Limiter Release | Limiter Clean=0 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 101 | Limiter Stereo Link |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 102 | Limiter Clean |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 103 | Limiter Bass Hold |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 104 | Limiter Auto Attack |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 105 | Limiter Auto Release | Limiter Clean=0 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 106 | Bypass |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 107 | Bypass |  | SILENT (-7.7 dB) | ok (0.267%) |
| 108 | Wet |  | SILENT (-7.7 dB) | NOT SHOWN (0.002%) |
| 109 | Delta |  | SILENT (-7.7 dB) | ok (0.030%) |
