# Plugin QA: CLAP: OpenDynamics (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenDynamics (OpenGrid), 111 parameters |
| finite output | PASS | peak -7.7 dBFS |
| not silent | PASS | rms -22.1 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1040x773, 1.7% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 78 | Decompressor Transient Crest | Decompressor On=1, Decompressor Detector=1 | ok (-13.2 dB) | ok (6.198%) |
| 79 | Decompressor Look-Ahead | Decompressor On=1 | ok (0.4 dB) | ok (7.531%) |
| 80 | Decompressor Stereo Link | Decompressor On=1 | ok (-2.8 dB) | ok (6.991%) |
| 84 | Decompressor Mix | Decompressor On=1 | ok (-12.2 dB) | ok (8.165%) |
| 96 | Limiter On |  | ok (-10.6 dB) | ok (6.032%) |
| 97 | Limiter Threshold |  | ok (-2.5 dB) | ok (4.756%) |
