# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 12.5% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.991%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.167%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (5.404%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.650%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.523%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.566%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.525%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.987%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-11.6 dB) | ok (6.210%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-11.6 dB) | ok (6.146%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1, Gate / Expander Threshold=0.9 | ok (-31.0 dB) | ok (5.714%) |
| 11 | Gate / Expander Hysteresis | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (5.838%) |
| 12 | Gate / Expander Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.088%) |
| 13 | Gate / Expander Hold | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-17.0 dB) | ok (6.045%) |
| 14 | Gate / Expander Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-18.1 dB) | ok (5.563%) |
| 15 | Gate / Expander Detector | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (4.809%) |
| 16 | Gate / Expander RMS Window | Gate / Expander On=1, Gate / Expander Detector=1, Gate / Expander Threshold=0.9 | ok (-10.5 dB) | ok (4.134%) |
| 17 | Gate / Expander Look-Ahead | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-4.3 dB) | ok (5.863%) |
| 18 | Gate / Expander Stereo Link | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.4 dB) | ok (6.303%) |
| 19 | Gate / Expander Key Source |  | excused (-20.1 dB) | ok (5.554%) |
| 20 | Gate / Expander Key High-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-8.5 dB) | ok (6.317%) |
| 21 | Gate / Expander Key Low-Pass | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-7.7 dB) | ok (6.202%) |
| 22 | Gate / Expander Auto Attack | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-24.9 dB) | ok (5.694%) |
| 23 | Gate / Expander Auto Release | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-32.5 dB) | ok (5.998%) |
| 24 | Gate / Expander Key Boost Freq | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (5.912%) |
| 25 | Gate / Expander Key Boost | Gate / Expander On=1, Gate / Expander Threshold=0.9 | ok (-10.6 dB) | ok (5.839%) |
| 26 | Gate / Expander Key Boost Q | Gate / Expander On=1, Gate / Expander Key Boost=1, Gate / Expander Threshold=0.9 | ok (-17.7 dB) | ok (6.003%) |
| 27 | Compressor On |  | ok (-10.6 dB) | ok (3.630%) |
| 28 | Compressor Threshold |  | ok (-10.1 dB) | ok (5.493%) |
| 29 | Compressor Ratio |  | ok (-10.6 dB) | ok (5.428%) |
| 30 | Compressor Knee |  | ok (-35.1 dB) | ok (5.226%) |
| 31 | Compressor Attack |  | ok (-11.8 dB) | ok (4.823%) |
| 32 | Compressor Release |  | ok (-18.5 dB) | ok (4.815%) |
| 33 | Compressor Detector |  | ok (-15.4 dB) | ok (5.088%) |
| 34 | Compressor RMS Window | Compressor Detector=1 | ok (-16.1 dB) | ok (5.581%) |
| 35 | Compressor Look-Ahead |  | ok (-4.7 dB) | ok (5.127%) |
| 36 | Compressor Stereo Link |  | ok (-7.1 dB) | ok (5.381%) |
| 37 | Compressor Key Source |  | excused (-10.6 dB) | ok (4.456%) |
| 38 | Compressor Key High-Pass |  | ok (-12.3 dB) | ok (5.087%) |
| 39 | Compressor Key Low-Pass |  | ok (-12.6 dB) | ok (4.632%) |
| 40 | Compressor Make-Up |  | ok (-2.5 dB) | ok (5.269%) |
| 41 | Compressor Mix |  | ok (-3.6 dB) | ok (4.504%) |
| 42 | Compressor Auto Attack |  | ok (-19.4 dB) | ok (4.664%) |
| 43 | Compressor Auto Release |  | ok (-32.0 dB) | ok (5.135%) |
| 44 | Compressor Key Boost Freq | Compressor Key Boost=1 | ok (-14.1 dB) | ok (6.646%) |
| 45 | Compressor Key Boost |  | ok (-13.8 dB) | ok (6.023%) |
| 46 | Compressor Key Boost Q | Compressor Key Boost=1 | ok (-17.4 dB) | ok (5.913%) |
| 47 | De-Expander On | random state 1 | CONDITIONAL (2.6 dB) | ok (5.424%) |
| 48 | De-Expander Threshold | De-Expander On=1 | ok (-4.7 dB) | ok (6.663%) |
| 49 | De-Expander Ratio | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (5.754%) |
| 50 | De-Expander Range | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.274%) |
| 51 | De-Expander Knee | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.936%) |
| 52 | De-Expander Floor | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (5.920%) |
| 53 | De-Expander Attack | De-Expander On=1 | ok (-5.9 dB) | ok (6.304%) |
| 54 | De-Expander Hold | De-Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (6.262%) |
| 55 | De-Expander Release | De-Expander On=1random state 1 | CONDITIONAL (2.3 dB) | ok (5.817%) |
| 56 | De-Expander Detector | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (5.562%) |
| 57 | De-Expander RMS Window | De-Expander On=1, De-Expander Detector=1 | ok (-25.7 dB) | ok (5.991%) |
| 58 | De-Expander Look-Ahead | De-Expander On=1 | ok (-5.6 dB) | ok (6.232%) |
| 59 | De-Expander Stereo Link | De-Expander On=1 | ok (-7.1 dB) | ok (6.322%) |
| 60 | De-Expander Key Source |  | excused (-240.0 dB) | ok (5.187%) |
| 61 | De-Expander Key High-Pass | De-Expander On=1 | ok (-9.9 dB) | ok (6.377%) |
| 62 | De-Expander Key Low-Pass | De-Expander On=1 | ok (-16.3 dB) | ok (5.891%) |
| 63 | De-Expander Mix | De-Expander On=1random state 1 | CONDITIONAL (2.6 dB) | ok (6.185%) |
| 64 | De-Expander Auto Attack | De-Expander On=1random state 1 | CONDITIONAL (2.2 dB) | ok (6.470%) |
| 65 | De-Expander Auto Release | De-Expander On=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.466%) |
| 66 | De-Expander Key Boost Freq | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.465%) |
| 67 | De-Expander Key Boost | De-Expander On=1 | ok (-44.1 dB) | ok (6.167%) |
| 68 | De-Expander Key Boost Q | De-Expander On=1, De-Expander Key Boost=1random state 1 | CONDITIONAL (2.1 dB) | ok (6.150%) |
| 69 | Decompressor On |  | ok (-15.5 dB) | ok (6.008%) |
| 70 | Decompressor Threshold | Decompressor On=1 | ok (-18.5 dB) | ok (6.339%) |
| 71 | Decompressor Ratio | Decompressor On=1 | ok (-8.6 dB) | ok (6.106%) |
| 72 | Decompressor Range | Decompressor On=1, Decompressor Match=0 | ok (-8.4 dB) | ok (6.789%) |
| 73 | Decompressor Knee | Decompressor On=1 | ok (-13.1 dB) | ok (6.500%) |
| 74 | Decompressor Attack | Decompressor On=1 | ok (-8.6 dB) | ok (6.349%) |
| 75 | Decompressor Release | Decompressor On=1 | ok (-58.8 dB) | ok (6.723%) |
| 76 | Decompressor Detector | Decompressor On=1 | ok (-23.0 dB) | ok (6.328%) |
| 77 | Decompressor RMS Window | Decompressor On=1, Decompressor Detector=0.5 | ok (-11.1 dB) | ok (6.159%) |
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-14.7 dB) | ok (6.346%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.161%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (7.024%) |
| 81 | Decompressor Key Source |  | excused (-13.2 dB) | ok (5.296%) |
| 82 | Decompressor Key High-Pass | Decompressor On=1 | ok (-13.4 dB) | ok (6.867%) |
| 83 | Decompressor Key Low-Pass | Decompressor On=1 | ok (-8.7 dB) | ok (6.398%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (6.087%) |
| 85 | Decompressor Ceiling | Decompressor On=1 | ok (-16.6 dB) | ok (6.244%) |
| 86 | Decompressor Ceiling Level | Decompressor On=1, Decompressor Ceiling=1 | ok (-4.6 dB) | ok (6.882%) |
| 87 | Decompressor Ceiling Type | Decompressor On=1, Decompressor Ceiling=1 | ok (-29.7 dB) | ok (6.852%) |
| 88 | Decompressor Output Trim | Decompressor On=1 | ok (-6.6 dB) | ok (5.786%) |
| 89 | Decompressor Match | Decompressor On=1 | ok (-8.6 dB) | ok (5.735%) |
| 90 | Decompressor Match Target Crest | Decompressor On=1, Decompressor Match=1 | ok (-25.0 dB) | ok (6.688%) |
| 91 | Decompressor Auto Attack | Decompressor On=1 | ok (-79.6 dB) | ok (7.597%) |
| 92 | Decompressor Auto Release | Decompressor On=1 | ok (-63.5 dB) | ok (7.882%) |
| 93 | Decompressor Key Boost Freq | Decompressor On=1, Decompressor Key Boost=1 | ok (-59.0 dB) | ok (6.678%) |
| 94 | Decompressor Key Boost | Decompressor On=1 | ok (-14.6 dB) | ok (7.106%) |
| 95 | Decompressor Key Boost Q | Decompressor On=1, Decompressor Key Boost=1 | ok (-64.5 dB) | ok (6.928%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (5.047%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (5.647%) |
| 98 | Limiter Ceiling |  | ok (-8.6 dB) | ok (5.744%) |
| 99 | Limiter Attack |  | ok (-44.1 dB) | ok (5.450%) |
| 100 | Limiter Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-10.1 dB) | ok (5.978%) |
| 101 | Limiter Stereo Link |  | ok (-21.8 dB) | ok (4.490%) |
| 102 | Limiter Clean | random state 1 | CONDITIONAL (2.7 dB) | ok (4.814%) |
| 103 | Limiter Bass Hold | random state 1 | CONDITIONAL (2.9 dB) | ok (5.516%) |
| 104 | Limiter Auto Attack | random state 1 | CONDITIONAL (2.1 dB) | ok (4.893%) |
| 105 | Limiter Auto Release | Limiter Clean=0, Limiter Threshold=0.5 | ok (-16.7 dB) | ok (6.002%) |
| 106 | Bypass |  | ok (-9.0 dB) | ok (5.437%) |
| 107 | Bypass |  | ok (-9.0 dB) | ok (3.546%) |
| 108 | Wet |  | ok (-9.0 dB) | ok (4.537%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (4.900%) |
