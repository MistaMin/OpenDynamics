# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.5% animates by itself |
| every parameter changes the sound | FAIL | no sound at either end of the sweep, so these were never heard (the audio went silent mid-run, or the parameter mutes at both ends and needs an audio: exception): Gate / Expander Mode, Gate / Expander Threshold, Gate / Expander Ratio, Gate / Expander Range, Gate / Expander Knee, Gate / Expander Hysteresis, Gate / Expander Attack, Gate / Expander Hold, Gate / Expander Release, Gate / Expander Detector, Gate / Expander RMS Window, Gate / Expander Look-Ahead, Gate / Expander Stereo Link, Gate / Expander Key High-Pass, Gate / Expander Key Low-Pass, Gate / Expander Auto Attack, Gate / Expander Auto Release, Gate / Expander Key Boost Freq, Gate / Expander Key Boost, Gate / Expander Key Boost Q, Compressor On, Compressor Threshold, Compressor Ratio, Compressor Knee, Compressor Attack, Compressor Release, Compressor Detector, Compressor RMS Window, Compressor Look-Ahead, Compressor Stereo Link, Compressor Key High-Pass, Compressor Key Low-Pass, Compressor Make-Up, Compressor Mix, Compressor Auto Attack, Compressor Auto Release, Compressor Key Boost Freq, Compressor Key Boost, Compressor Key Boost Q, De-Expander On, De-Expander Threshold, De-Expander Ratio, De-Expander Range, De-Expander Knee, De-Expander Floor, De-Expander Attack, De-Expander Hold, De-Expander Release, De-Expander Detector, De-Expander RMS Window, De-Expander Look-Ahead, De-Expander Stereo Link, De-Expander Key High-Pass, De-Expander Key Low-Pass, De-Expander Mix, De-Expander Auto Attack, De-Expander Auto Release, De-Expander Key Boost Freq, De-Expander Key Boost, De-Expander Key Boost Q, Decompressor On, Decompressor Threshold, Decompressor Ratio, Decompressor Range, Decompressor Knee, Decompressor Attack, Decompressor Release, Decompressor Detector, Decompressor RMS Window, Decompressor Transient Crest, Decompressor Look-Ahead, Decompressor Stereo Link, Decompressor Key High-Pass, Decompressor Key Low-Pass, Decompressor Mix, Decompressor Ceiling, Decompressor Ceiling Level, Decompressor Ceiling Type, Decompressor Output Trim, Decompressor Match, Decompressor Match Target Crest, Decompressor Auto Attack, Decompressor Auto Release, Decompressor Key Boost Freq, Decompressor Key Boost, Decompressor Key Boost Q, Limiter On, Limiter Threshold, Limiter Ceiling, Limiter Attack, Limiter Release, Limiter Stereo Link, Limiter Clean, Limiter Bass Hold, Limiter Auto Attack, Limiter Auto Release, Bypass, Bypass, Wet, Delta |
| every parameter is shown in the GUI | FAIL | Gate / Expander Key Source, Compressor Detector, Compressor RMS Window, Compressor Look-Ahead, Compressor Stereo Link, Compressor Key Source, Compressor Key High-Pass, Compressor Key Low-Pass, Compressor Auto Attack, Compressor Auto Release, Compressor Key Boost Freq, Compressor Key Boost, Compressor Key Boost Q, De-Expander Key Source, Decompressor Key Source, Limiter Threshold, Limiter Ceiling, Limiter Attack, Limiter Release, Limiter Stereo Link, Limiter Clean, Limiter Bass Hold, Limiter Auto Attack, Limiter Auto Release, Bypass |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.584%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.664%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.135%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.934%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.914%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (6.296%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.550%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 14 | Gate / Expander Release | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 19 | Gate / Expander Key Source |  | excused (-7.7 dB) | NOT SHOWN (0.004%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 27 | Compressor On |  | SILENT (-7.7 dB) | ok (0.391%) |
| 28 | Compressor Threshold |  | SILENT (-7.7 dB) | ok (0.584%) |
| 29 | Compressor Ratio |  | SILENT (-7.7 dB) | ok (0.174%) |
| 30 | Compressor Knee |  | SILENT (-7.7 dB) | ok (0.079%) |
| 31 | Compressor Attack |  | SILENT (-7.7 dB) | ok (0.121%) |
| 32 | Compressor Release |  | SILENT (-7.7 dB) | ok (0.106%) |
| 33 | Compressor Detector |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | SILENT (-240.0 dB) | NOT SHOWN (0.004%) |
| 35 | Compressor Look-Ahead |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 36 | Compressor Stereo Link |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 37 | Compressor Key Source |  | excused (-7.7 dB) | NOT SHOWN (0.004%) |
| 38 | Compressor Key High-Pass |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 39 | Compressor Key Low-Pass |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 40 | Compressor Make-Up |  | SILENT (-7.7 dB) | ok (0.345%) |
| 41 | Compressor Mix |  | SILENT (-7.7 dB) | ok (0.222%) |
| 42 | Compressor Auto Attack |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 43 | Compressor Auto Release |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | SILENT (-240.0 dB) | NOT SHOWN (0.004%) |
| 45 | Compressor Key Boost |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | SILENT (-240.0 dB) | NOT SHOWN (0.004%) |
| 47 | De-Expander On |  | SILENT (-7.7 dB) | ok (0.015%) |
| 48 | De-Expander Threshold | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 49 | De-Expander Ratio | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 50 | De-Expander Range | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 51 | De-Expander Knee | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 52 | De-Expander Floor | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 53 | De-Expander Attack | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 54 | De-Expander Hold | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 55 | De-Expander Release | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 56 | De-Expander Detector | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 60 | De-Expander Key Source |  | excused (-7.7 dB) | NOT SHOWN (0.004%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 63 | De-Expander Mix | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 64 | De-Expander Auto Attack | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 65 | De-Expander Auto Release | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 69 | Decompressor On |  | SILENT (-7.7 dB) | ok (0.015%) |
| 70 | Decompressor Threshold | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 71 | Decompressor Ratio | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | SILENT (-240.0 dB) | ok (0.015%) |
| 73 | Decompressor Knee | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 74 | Decompressor Attack | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 75 | Decompressor Release | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 76 | Decompressor Detector | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | SILENT (-240.0 dB) | ok (0.015%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 81 | Decompressor Key Source |  | excused (-7.7 dB) | NOT SHOWN (0.004%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 84 | Decompressor Mix | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 89 | Decompressor Match | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | SILENT (-240.0 dB) | ok (0.015%) |
| 96 | Limiter On |  | SILENT (-7.7 dB) | ok (0.015%) |
| 97 | Limiter Threshold |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 98 | Limiter Ceiling |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 99 | Limiter Attack |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 100 | Limiter Release | Limiter Clean=0 | SILENT (-240.0 dB) | NOT SHOWN (0.004%) |
| 101 | Limiter Stereo Link |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 102 | Limiter Clean |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 103 | Limiter Bass Hold |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 104 | Limiter Auto Attack |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 105 | Limiter Auto Release | Limiter Clean=0 | SILENT (-240.0 dB) | NOT SHOWN (0.004%) |
| 106 | Bypass |  | SILENT (-7.7 dB) | NOT SHOWN (0.004%) |
| 107 | Bypass |  | SILENT (-7.7 dB) | ok (0.271%) |
| 108 | Wet |  | SILENT (-7.7 dB) | ok (0.006%) |
| 109 | Delta |  | SILENT (-7.7 dB) | ok (0.034%) |
