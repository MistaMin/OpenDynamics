# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -0.1 dBFS |
| not silent | PASS | rms -11.8 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 9.5% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-6.0 dB) | ok (3.655%) |
| 1 | Output Gain |  | ok (-1.4 dB) | ok (4.072%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (4.174%) |
| 3 | Auto Gain |  | ok (-10.1 dB) | ok (4.785%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.116%) |
| 5 | Gate / Expander On |  | ok (-11.6 dB) | ok (5.044%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1 | ok (-6.3 dB) | ok (5.452%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.265%) |
| 8 | Gate / Expander Ratio | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-8.5 dB) | ok (5.764%) |
| 9 | Gate / Expander Range | Gate / Expander On=1 | ok (-9.1 dB) | ok (5.756%) |
| 10 | Gate / Expander Knee | Gate / Expander On=1, Gate / Expander Mode=1 | ok (-9.2 dB) | ok (6.159%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (5.692%) |
