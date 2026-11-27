# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 111 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 3.2% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (5.434%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.946%) |
| 2 | Zero Latency |  | ok (-8.1 dB) | ok (5.778%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.746%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (6.824%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (6.740%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.353%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.762%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.955%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (7.591%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (7.109%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.591%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.776%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.584%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (6.944%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.093%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (4.887%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (6.576%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (7.072%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (6.366%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (6.132%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.077%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (6.601%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (6.444%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (8.559%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (7.159%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (7.277%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (4.153%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.272%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (4.565%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.559%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (4.940%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (5.129%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.334%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (6.217%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.433%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.550%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (5.542%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.370%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (5.002%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (6.201%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.530%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.799%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (4.922%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.667%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.361%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (8.048%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.7 dB) | ok (5.422%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.414%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.422%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (7.601%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.964%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.478%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (8.740%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (8.314%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.557%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.833%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (7.890%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.2 dB) | ok (7.419%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (7.642%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.293%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (5.425%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.659%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.221%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (5.783%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (8.133%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.388%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.768%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (7.665%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (6.508%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.788%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (7.074%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (8.002%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (8.835%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (8.840%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.592%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.881%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.655%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.286%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (5.852%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.737%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (4.705%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (7.349%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.055%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (7.111%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (7.129%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (8.551%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (8.504%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (6.066%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (5.866%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.012%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.466%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (6.989%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (7.279%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (8.196%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (7.540%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.337%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.864%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (4.454%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.973%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (5.312%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (5.020%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.777%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (6.760%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.882%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (6.274%) |
| 106 | Auto Target |  | excused (-21.8 dB) | ok (4.129%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (4.582%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (4.278%) |
| 109 | Wet |  | ok (-9.0 dB) | ok (4.821%) |
| 110 | Delta |  | ok (-3.5 dB) | ok (5.159%) |
