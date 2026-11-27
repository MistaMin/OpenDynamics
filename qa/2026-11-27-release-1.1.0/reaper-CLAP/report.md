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
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.929%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.102%) |
| 2 | Zero Latency |  | ok (-8.1 dB) | ok (5.372%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.484%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (4.785%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.559%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (7.460%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.012%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (5.944%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (7.009%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (6.432%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (5.458%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.233%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (5.998%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (6.154%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (5.306%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (4.650%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (7.164%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (7.339%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.426%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (7.242%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (7.207%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (5.831%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (8.277%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (8.582%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (6.971%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (5.837%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.483%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (6.366%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.117%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (6.026%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (5.895%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (6.227%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.877%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.930%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (4.937%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (4.904%) |
| 37 | Compressor Key Source |  | excused (-240.0 dB) | ok (4.836%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (4.822%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.536%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.944%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (5.228%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (5.251%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (6.915%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (7.610%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (5.923%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (7.725%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (6.378%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (7.326%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (6.161%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.7 dB) | ok (5.788%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.039%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.501%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (7.328%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.070%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.276%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.270%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (7.821%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.133%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (5.918%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.106%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.493%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (6.896%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.508%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (6.678%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.955%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.389%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.966%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.983%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (5.297%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (7.347%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.486%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.468%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (6.816%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (6.805%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (7.555%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.664%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (7.601%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (7.478%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.162%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.397%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.008%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.528%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (7.175%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.253%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (8.012%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (7.382%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (7.583%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (7.030%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (6.578%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (7.141%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (6.933%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.514%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (7.870%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (7.337%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.944%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.580%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.276%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.774%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.644%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (6.161%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (5.436%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.8 dB) | ok (5.812%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.8 dB) | ok (5.721%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (5.421%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (7.217%) |
| 106 | Auto Target |  | excused (-21.8 dB) | ok (6.232%) |
| 107 | Auto Ceiling |  | excused (-240.0 dB) | ok (6.606%) |
| 108 | Bypass |  | ok (-9.0 dB) | ok (4.067%) |
| 109 | Wet |  | ok (-9.0 dB) | ok (5.315%) |
| 110 | Delta |  | ok (-3.5 dB) | ok (6.487%) |
