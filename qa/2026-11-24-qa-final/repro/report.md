# Plugin QA: VST3: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenDynamics (OpenGrid), 110 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 4.3% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-3.5 dB) | ok (4.346%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (6.235%) |
| 2 | Zero Latency |  | ok (-7.7 dB) | ok (4.913%) |
| 3 | Auto Gain |  | ok (-8.2 dB) | ok (5.160%) |
| 4 | Delta |  | ok (-3.5 dB) | ok (5.366%) |
| 5 | Gate / Expander On |  | ok (-7.5 dB) | ok (5.104%) |
| 6 | Gate / Expander Mode | Gate / Expander On=1random state 1 | CONDITIONAL (2.5 dB) | ok (3.720%) |
| 7 | Gate / Expander Threshold | Gate / Expander On=1 | ok (-9.1 dB) | ok (2.745%) |
| 109 | Delta |  | ok (-3.5 dB) | ok (4.434%) |
