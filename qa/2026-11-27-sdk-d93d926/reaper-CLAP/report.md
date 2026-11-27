# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 111 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.8% animates by itself |
| every parameter changes the sound | FAIL | no sound at either end of the sweep, so these were never heard (the audio went silent mid-run, or the parameter mutes at both ends and needs an audio: exception): Decompressor Look-Ahead, Decompressor Stereo Link, Decompressor Key High-Pass, Decompressor Key Low-Pass, Decompressor Mix, Decompressor Ceiling, Decompressor Ceiling Level, Decompressor Ceiling Type, Decompressor Output Trim, Decompressor Match, Decompressor Match Target Crest, Decompressor Auto Attack, Decompressor Auto Release, Decompressor Key Boost Freq, Decompressor Key Boost, Decompressor Key Boost Q, Limiter On, Limiter Threshold, Limiter Ceiling, Limiter Attack, Limiter Release, Limiter Stereo Link, Limiter Clean, Limiter Bass Hold, Limiter Auto Attack, Limiter Auto Release, Bypass, Wet, Delta |
| every parameter is shown in the GUI | FAIL | Decompressor Key Source, Limiter On, Limiter Threshold, Limiter Ceiling, Limiter Attack, Limiter Release, Limiter Stereo Link, Limiter Clean, Limiter Bass Hold, Limiter Auto Attack, Limiter Auto Release, Auto Target, Auto Ceiling, Wet, Delta |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.475%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (7.660%) |
| 2 | Zero Latency |  | ok (-8.1 dB) | ok (6.790%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (7.327%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.990%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.942%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.150%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.731%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.768%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.631%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (7.193%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.419%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.699%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.970%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (6.886%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.044%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (4.581%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (6.993%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (6.768%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.579%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (7.377%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.765%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (6.297%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (5.701%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (6.186%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (6.046%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (6.300%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.424%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.150%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.761%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.664%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (4.691%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (6.233%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.671%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.799%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.129%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (4.461%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (5.162%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.657%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.966%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.466%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.987%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (6.553%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.820%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.470%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.204%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (6.897%) |
| 47 | De-Expander On |  | skipped (no audible random state) (-240.0 dB) | ok (6.029%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.492%) |
| 49 | De-Expander Ratio | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.626%) |
| 50 | De-Expander Range | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (7.200%) |
| 51 | De-Expander Knee | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.400%) |
| 52 | De-Expander Floor | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.879%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.040%) |
| 54 | De-Expander Hold | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.395%) |
| 55 | De-Expander Release | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.488%) |
| 56 | De-Expander Detector | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.098%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (6.202%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.703%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (7.197%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (6.032%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.846%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (6.537%) |
| 63 | De-Expander Mix | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.194%) |
| 64 | De-Expander Auto Attack | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (7.191%) |
| 65 | De-Expander Auto Release | De-Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.927%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1 | skipped (no audible random state) (-240.0 dB) | ok (7.012%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.581%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1 | skipped (no audible random state) (-240.0 dB) | ok (6.471%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.413%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.039%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (8.389%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (7.019%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (6.830%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (7.215%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (7.249%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (8.075%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.327%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-9.0 dB) | ok (8.756%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.695%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 81 | Decompressor Key Source |  | excused (-7.7 dB) | NOT SHOWN (0.000%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 84 | Decompressor Mix | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 89 | Decompressor Match | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.011%) |
| 96 | Limiter On |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 97 | Limiter Threshold |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 98 | Limiter Ceiling |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 99 | Limiter Attack |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 101 | Limiter Stereo Link |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 102 | Limiter Clean |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 103 | Limiter Bass Hold |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 104 | Limiter Auto Attack |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | SILENT (-240.0 dB) | NOT SHOWN (0.000%) |
| 106 | Auto Target |  | excused (-7.7 dB) | NOT SHOWN (0.000%) |
| 107 | Auto Ceiling |  | excused (-7.7 dB) | NOT SHOWN (0.000%) |
| 108 | Bypass |  | SILENT (-7.7 dB) | ok (0.274%) |
| 109 | Wet |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
| 110 | Delta |  | SILENT (-7.7 dB) | NOT SHOWN (0.000%) |
