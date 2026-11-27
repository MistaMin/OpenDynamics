# Release build QA, 2026-11-27: OpenDynamics 1.1.0

Source: f6d3152 (version 1.1.0, documents) built on the owner's Mac (Apple silicon, macOS 15): universal x86_64 + arm64,
minimum 11.0, all five formats, OpenPluginSDK d93d926. 8191b0b changes only the macOS installer script (no build
input of a plugin bundle). Windows x64 and Linux x86_64: GitHub Actions run 46010465148 at f6d3152 (branch
`release-1.1.0`). Code QA of this source: `qa/2026-11-27-conformance-fixes` and `qa/2026-11-27-sdk-d93d926`.

| Check | Result |
|---|---|
| Test list on f6d3152 and on 8191b0b (lint, clippy, tests, soak, strict fuzz, bench) | PASS |
| Soak over 200 seeds on f6d3152 (808 s) | PASS |
| Identity against 1.0.0 Beta: bundle ids, AU `aufx ODyn Ogrd`, LV2 URI, the 106 parameters in place (names and host ids, REAPER VST3 and CLAP), Auto Target and Auto Ceiling appended; no range changed | PASS |
| Version: AU 65792 (1.1.0), bundles 1.1.0, plugin log lines "1.1.0" | PASS |
| Build: five formats, both slices, minos 11.0 | PASS |
| AAX: PACE signed (Signing Only, 3C6EF8C0-...), wraptool verify; AAX Validator 6/6 on the signed bundle (log local only) | PASS |
| Installer: Developer ID signed, notarized, stapled, `spctl` "Notarized Developer ID" (first submission rejected: the CLAP's ARA library was left ad-hoc signed; fixed in 8191b0b) | PASS |
| auval (2 known warnings), pluginval 10 VST3 and AU, clap-validator 34/0/10 on the signed bundles | PASS |
| REAPER CLAP, OpenGrid9 AU and LV2 on the signed bundles: every check | PASS |
| REAPER VST3, run 1: Compressor Ratio's two sweep renders silent, every other check passed; run 2: every check | FAIL, then PASS (below) |
| Default renders against the d93d926 test build: REAPER VST3, CLAP, OpenGrid9 AU null at -240 dB | PASS |
| Manual PDF: 13 pages, looked at; pictures renewed from the plugin's own renders | PASS |
| CI Windows x64 (tests, ARA SDK TestHost, Inno Setup installer) and Linux x86_64 (tests, ARA TestHost, archive) | PASS |
| Checksums of the CI files match their `.sha256`; Linux archive has only `usr/lib/{vst3,clap,lv2}` and `usr/share/doc`, no `._`, `.DS_Store` or `Thumbs.db` | PASS |
| Pro Tools on the signed 1.1.0 AAX | SKIPPED: a Pro Tools not started by this session (14:35) was running; the same code was checked in Pro Tools on the d93d926 test build (`qa/2026-11-27-sdk-d93d926`, S9) |
| Windows and Linux in a host | not tested |

Release files (`dist/release-1.1.0/`):

| File | Size | SHA-256 |
|---|---|---|
| OpenDynamics-1.1.0-macos.pkg | 9619625 | fd3d738e19b65dc4177355d64fa0cc5b43faea8360742421c682d75fdbc13fc5 |
| OpenDynamics-1.1.0-windows-x64-setup.exe | 2888961 | see `.sha256` (95df47924f1a...) |
| OpenDynamics-1.1.0-linux-x86_64.tar.gz | 5690410 | see `.sha256` (40c932bab445...) |

## The intermittent silence

Seen twice today in about nine full REAPER runs of this code: one CLAP run stayed silent from the 79th parameter on
(`qa/2026-11-27-sdk-d93d926/reaper-CLAP`), and release VST3 run 1 had two silent renders (Compressor Ratio at both
ends) with sound before and after. Both times the editor's input meter, which `process` feeds, was empty: the plugin saw
no input signal or was not called. The reruns passed. It is the open finding of 1.0.0 Beta (stated there as a known
limit); the cause is not proven.
