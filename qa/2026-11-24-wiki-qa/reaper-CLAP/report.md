# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 109 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.5% animates by itself |
| every parameter changes the sound | FAIL | no sound at either end of the sweep, so these were never heard (the audio went silent mid-run, or the parameter mutes at both ends and needs an audio: exception): Compressor Release, Compressor Detector, Compressor RMS Window, Compressor Look-Ahead, Compressor Stereo Link, Compressor Key High-Pass, Compressor Key Low-Pass, Compressor Make-Up, Compressor Mix, Compressor Auto Attack, Compressor Auto Release, Compressor Key Boost Freq, Compressor Key Boost, Compressor Key Boost Q, De-Expander On, De-Expander Threshold, De-Expander Ratio, De-Expander Range, De-Expander Knee, De-Expander Floor, De-Expander Attack, De-Expander Hold, De-Expander Release, De-Expander Detector, De-Expander RMS Window, De-Expander Look-Ahead, De-Expander Stereo Link, De-Expander Key High-Pass, De-Expander Key Low-Pass, De-Expander Mix, De-Expander Auto Attack, De-Expander Auto Release, De-Expander Key Boost Freq, De-Expander Key Boost, De-Expander Key Boost Q, Decompressor On, Decompressor Threshold, Decompressor Ratio, Decompressor Range, Decompressor Knee, Decompressor Attack, Decompressor Release, Decompressor Detector, Decompressor RMS Window, Decompressor Transient Crest, Decompressor Look-Ahead, Decompressor Stereo Link, Decompressor Key High-Pass, Decompressor Key Low-Pass, Decompressor Mix, Decompressor Ceiling, Decompressor Ceiling Level, Decompressor Ceiling Type, Decompressor Output Trim, Decompressor Match, Decompressor Match Target Crest, Decompressor Auto Attack, Decompressor Auto Release, Decompressor Key Boost Freq, Decompressor Key Boost, Decompressor Key Boost Q, Limiter On, Limiter Threshold, Limiter Ceiling, Limiter Attack, Limiter Release, Limiter Stereo Link, Limiter Clean, Limiter Bass Hold, Limiter Auto Attack, Limiter Auto Release, Bypass, Wet, Delta |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.595%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.406%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (4.954%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (4.914%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (4.716%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (4.719%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (4.900%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.118%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.466%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.633%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-86.5 dB) | ok (6.456%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.941%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | ok (-9.1 dB) | ok (6.233%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.349%) |
| 14 | Gate / Expander Release | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.150%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.574%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | ok (-44.0 dB) | ok (6.859%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | ok (-6.2 dB) | ok (6.217%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | ok (-7.1 dB) | ok (6.345%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.926%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | ok (-17.7 dB) | ok (7.735%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | ok (-17.8 dB) | ok (6.959%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | ok (-24.9 dB) | ok (7.202%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (7.523%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1 | skipped (no audible random state) (-240.0 dB) | ok (6.973%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1 | skipped (no audible random state) (-240.0 dB) | ok (6.784%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1 | skipped (no audible random state) (-240.0 dB) | ok (6.577%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.454%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.202%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.201%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (4.436%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (4.385%) |
| 32 | Compressor Release |  | SILENT (-7.7 dB) | ok (0.774%) |
| 33 | Compressor Detector |  | SILENT (-7.7 dB) | ok (0.282%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | SILENT (-240.0 dB) | ok (0.029%) |
| 35 | Compressor Look-Ahead |  | SILENT (-7.7 dB) | ok (0.029%) |
| 36 | Compressor Stereo Link |  | SILENT (-7.7 dB) | ok (0.029%) |
| 37 | Compressor Key Source |  | excused (-7.7 dB) | ok (0.029%) |
| 38 | Compressor Key High-Pass |  | SILENT (-7.7 dB) | ok (0.029%) |
| 39 | Compressor Key Low-Pass |  | SILENT (-7.7 dB) | ok (0.029%) |
| 40 | Compressor Make-Up |  | SILENT (-7.7 dB) | ok (0.376%) |
| 41 | Compressor Mix |  | SILENT (-7.7 dB) | ok (0.029%) |
| 42 | Compressor Auto Attack |  | SILENT (-7.7 dB) | ok (0.029%) |
| 43 | Compressor Auto Release |  | SILENT (-7.7 dB) | ok (0.029%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.029%) |
| 45 | Compressor Key Boost |  | SILENT (-7.7 dB) | ok (0.029%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.029%) |
| 47 | De-Expander On |  | SILENT (-7.7 dB) | ok (0.040%) |
| 48 | De-Expander Threshold | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 49 | De-Expander Ratio | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 50 | De-Expander Range | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 51 | De-Expander Knee | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 52 | De-Expander Floor | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 53 | De-Expander Attack | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 54 | De-Expander Hold | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 55 | De-Expander Release | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 56 | De-Expander Detector | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 60 | De-Expander Key Source |  | excused (-7.7 dB) | ok (0.029%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 63 | De-Expander Mix | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 64 | De-Expander Auto Attack | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 65 | De-Expander Auto Release | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 69 | Decompressor On |  | SILENT (-7.7 dB) | ok (0.040%) |
| 70 | Decompressor Threshold | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 71 | Decompressor Ratio | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | SILENT (-240.0 dB) | ok (0.040%) |
| 73 | Decompressor Knee | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 74 | Decompressor Attack | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 75 | Decompressor Release | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 76 | Decompressor Detector | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | SILENT (-240.0 dB) | ok (0.040%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 81 | Decompressor Key Source |  | excused (-7.7 dB) | ok (0.029%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 84 | Decompressor Mix | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 89 | Decompressor Match | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.040%) |
| 96 | Limiter On |  | SILENT (-7.7 dB) | ok (0.029%) |
| 97 | Limiter Threshold |  | SILENT (-7.7 dB) | ok (0.029%) |
| 98 | Limiter Ceiling |  | SILENT (-7.7 dB) | ok (0.029%) |
| 99 | Limiter Attack |  | SILENT (-7.7 dB) | ok (0.029%) |
| 100 | Limiter Release | Limiter Clean=0 | SILENT (-240.0 dB) | ok (0.029%) |
| 101 | Limiter Stereo Link |  | SILENT (-7.7 dB) | ok (0.029%) |
| 102 | Limiter Clean |  | SILENT (-7.7 dB) | ok (0.029%) |
| 103 | Limiter Bass Hold |  | SILENT (-7.7 dB) | ok (0.029%) |
| 104 | Limiter Auto Attack |  | SILENT (-7.7 dB) | ok (0.029%) |
| 105 | Limiter Auto Release | Limiter Clean=0 | SILENT (-240.0 dB) | ok (0.029%) |
| 106 | Bypass |  | SILENT (-7.7 dB) | ok (0.303%) |
| 107 | Wet |  | SILENT (-7.7 dB) | ok (0.031%) |
| 108 | Delta |  | SILENT (-7.7 dB) | ok (0.059%) |
