# Release build QA, 2026-11-25: OpenDynamics 1.0.0 Beta

Source: be140d1 (macOS build), OpenPluginSDK c6fa6a0 (PR #50 AU snapping, PR #51 VST3 edit path), GoodLookin DRI
tree 3fe0426. macOS built on the owner's Mac (Apple silicon, macOS 15.7.5): universal x86_64 + arm64, minimum 11.0,
all five formats. Windows x64 and Linux x86_64: GitHub Actions run 45520040272 at be140d1.

| Check | Result |
|---|---|
| `cargo test --release --workspace`: 21 suites, 95 passed | PASS |
| Build: five formats, both slices, minos 11.0, codesign | PASS |
| auval | PASS |
| AAX: PACE signed (Signing Only, 3C6EF8C0-...), wraptool verify; AAX Validator 6/6 (log local only) | PASS |
| REAPER VST3, AU, CLAP (`reaper-*`, contexts in `exceptions.txt`): every check, 0 silent, diagnostic log empty | ALL PASS |
| Installer: Developer ID signed, notarized, stapled, `spctl` "Notarized Developer ID"; every component expanded with no `._` file | PASS |
| Manual PDF: 12 pages, looked at | PASS |
| CI Windows x64 (tests, build, Inno Setup installer) and Linux x86_64 (tests, build, archive) | PASS |
| Checksums of the CI files match their `.sha256` | PASS |
| Pro Tools on this build | SKIPPED: installing the signed AAX to /Library needs the owner's password (dialog cancelled) |
| Windows and Linux in a host | not tested |

Release files (staged in `dist/release/`, not yet in Release Binaries):

| File | Size | SHA-256 |
|---|---|---|
| OpenDynamics-1.0.0-Beta-macos.pkg | 15111128 | ba59cc69813d8700a69accd04e515aeacf22ba9dfce9b7efc53da27246e406f1 |
| OpenDynamics-1.0.0-Beta-windows-x64-setup.exe | 2821898 | see `.sha256` (42fad1593348...) |
| OpenDynamics-1.0.0-Beta-linux-x86_64.tar.gz | 3851297 | see `.sha256` (155e898dcb59...) |

Installer script fix: `pkgbuild` lists extended attributes (`com.apple.provenance`, which this Mac puts on every new
file and which cannot be removed) as `._` entries in the payload. The installer folds them back into attributes;
expanding each component as the installer does gives no `._` file. The script now checks that expansion.

Open: the intermittent silence (1 of 3 VST3 runs on SDK a8124f9, 1 of 7 CLAP runs on 34420a4; 0 in 12 host runs since
the diagnostic build). Stated in the release notes as a known limit.
