# Editor performance and readability, 2026-11-24

Rules: plugin-gui-performance skill (OpenLimiter 1.0.1 worked example).

## Drawing rules

| Rule | State | Where |
|---|---|---|
| Curves in pieces of 8 segments | already met | `kit::curve` (copied from OpenLimiter 1.0.1) for transfer curve and gain trace |
| Curve areas as pixel columns | already met | `kit::area` for the gain trace fill |
| One union damage rect for the canvas | met (whole window, one rect) | `tick` calls `damage.add_all()`; see "Not done" |
| Column fill matches the polygon | not ported | helper is OpenLimiter's, tested there (`column_fill_matches_the_polygon`) |

## Frame benchmark (2x, 30 frames, live signal, Auto Gain on)

`cargo test --release --lib frame_cost -- --ignored --nocapture`, Apple Silicon, load average 6.0 (after).

| Tab | Before ms/frame (load 12.2) | After ms/frame (load 6.0) | Dirty |
|---|---|---|---|
| Gate / Expander | 6.52 | 6.74 | 100 % |
| Compressor | 7.87 | 8.12 | 100 % |
| De-Expander | 7.69 | 8.11 | 100 % |
| Decompressor | 10.42 | 11.01 | 100 % |
| Limiter | 7.19 | 8.10 | 100 % |

All well inside the 33 ms tick. No drawing change was needed; differences are noise.

## Readability

| Rule | Before | After | Check |
|---|---|---|---|
| Nothing under 8 pt | meter scale 7.5 pt | 8 pt | `text_is_whole_apart_and_readable` (static + live layers) |
| Moving gain readouts whole, no -0 | trace readout `-0.0 dB`, Auto Gain `-2.6 dB` | `0 dB`, `-3 dB` (`whole_db`) | same test (fails on "-0.0") and screenshots |
| No clipped or overlapping text | - | none found | same test |
| Knob value boxes keep tenths | static values set by the user, not moving | unchanged | - |

Owner question open: are any readouts exempt from whole numbers?

## Not done / SKIPPED

- On-screen measurement in a host (3b): SKIPPED. Needs a visible host instance, which needs the owner's request; would
  cover host CPU, WindowServer, GPU per tab and the loaded binary (`lsof`).
- Retina 2x on screen: SKIPPED (benchmark covers 2x pixel cost in process).
- Per-part damage: whole window repainted per tick (`shortcut:` comment in `tick`). Cheap enough at 6.7-11 ms.
- History plot: the gain trace pushes one block value per tick at one x position each (no binning), so peaks do not
  change shape while scrolling; no fall rate added.

Screenshots: `screenshots` test (`SHOTS=<dir> SCALE=1|2 cargo test --release --lib screenshots -- --ignored`).

## GUI redesign (2026-11-24)

Owner: "rethink the entire GUI. Look at Harmonic Compressor. A lot of these parameters could be sliders in a
submenu that you open up. The main controls should be the main controls, the user needs focus and the curve should
be in the History."

Reference studied: Harmonic Compressor editor (manual pictures `Docs/harmonic-compressor-modules.png`,
`-autoscale.png`): one big display (6 s of pre/post level history, gain reduction from the top), the transfer curve
drawn over the older part of the same display, module chips (Compression, Harmonics, Sidechain, Transient) that open
floating slider panels, four big knobs in a bar under the display, Auto Gain beside them.

### Layout per tab

| Tab | Main controls (big knobs) | Top bar | Settings panels (sliders, keys, switches) |
|---|---|---|---|
| Gate / Expander | Threshold, Ratio, Range, Attack, Hold, Release | Mode (Gate / Expander) | SHAPE: Knee, Hysteresis, Link. TIMING: Look-Ahead, Auto Attack, Auto Release. SIDECHAIN: Detector, RMS Window, Key, Key HP, Key LP |
| Compressor | Threshold, Ratio, Attack, Release, Make-Up, Mix | - | SHAPE: Knee, Link. TIMING: Look-Ahead, Auto Attack, Auto Release. SIDECHAIN: as Gate |
| De-Expander | Threshold, Ratio, Range, Attack, Release, Mix | - | SHAPE: Knee, Floor, Link. TIMING: Hold, Look-Ahead, Auto Attack, Auto Release. SIDECHAIN: as Gate |
| Decompressor | Threshold, Ratio, Range, Attack, Release, Mix | - | SHAPE: Knee, Trim, Link. TIMING: Look-Ahead, Auto Attack, Auto Release. SIDECHAIN: Detector, Crest, RMS Window, Key, Key HP, Key LP. CEILING: Ceiling and Match switches, Ceiling, Type, Match Crest |
| Limiter | Threshold, Ceiling, Attack, Release | Clean | SHAPE: Link. TIMING: Bass Hold, Auto Attack, Auto Release |

Why: the main row is what is turned while listening (level, amount, speed, blend); the gate's Hold is part of how it
sounds on every hit, so it stays main; its Mode changes the whole section, so it sits in the top bar, as does the
Limiter's Clean. Knee, link, look-ahead, detector and key filters are set once per source and went to panels. Global
Auto Gain and Zero Latency stay on every tab (top right).

### Curve in the history

One display, one dBFS scale (-60..+6) on the left axis for both. The level history scrolls across the whole width
(input level light, output level line and fill, the gap between them is what the section changed); the gain hangs from
the top edge as a band (red cut, green lift, 0..24 dB scale on the right). The static curve sits in the display's
left square (input dBFS along its bottom), on a plate over the oldest part of the history (history there as faint
lines). Because output level is on the same scale, the live dot on the curve is at the height the newest output level
reaches at the right edge: one place to look. Threshold: dashed across the history with a plated tag, and vertical
in the curve square.

### Settings panels

Chips above the display (with a chevron) open one panel at a time over the display; a second click or a click on the
display outside the panel closes it. Closed by default and on every tab change. Panel controls are the kit's
horizontal sliders (FaderH, label column now fixed at 96 pt so tracks line up), keys and pills. Accessibility: each
chip is listed as a switch node ("sidechain settings", open/closed, Press toggles), so a closed panel stays reachable.

### Tests

`cargo test --release --workspace`: 21 suites pass. UI tests now walk every tab with the panel closed and each panel
open: every parameter has a control (panel controls count); controls inside the page, panel inside the display and its
controls inside the panel, main controls clear of the display and chips, no overlaps; renders in every theme;
`text_is_whole_apart_and_readable` (static + live layers, panels open).

### Frame benchmark (2x, 30 frames, Auto Gain on)

| Tab | Old page | Redesign, first cut (load 17) | Redesign, final, closed / panel open (load 7.8) |
|---|---|---|---|
| Gate / Expander | 6.74 | 13.8 / 18.3 | 6.05 / 10.36 |
| Compressor | 8.12 | 21.8 / 26.0 | 6.20 / 10.24 |
| De-Expander | 8.11 | 19.2 / 23.6 | 6.04 / 10.13 |
| Decompressor | 11.01 | 23.4 / 26.8 | 6.41 / 9.68 |
| Limiter | 8.10 | 23.0 / 24.7 | 7.95 / 10.22 |

What it took (`live_layer_cost_by_command`, an ignored per-command timing test): big-knob value arcs drawn as one
stroke cost up to 6 ms each at 2x, so `kit::arc` draws them in 0.3 rad pieces (opaque only; translucent arcs of faded
knobs stay one stroke, pieces showed dots at the joins); the curve square's translucent plate (5 ms) and border
(1.6 ms) moved to the cached layer; history fills only right of the square, and the input fill is only the gap to the
output.

### Screenshots

the session's scratch folder (not kept):
every tab in every theme closed, every panel open in Dark and Light, 1x and 2x (100 files).

### Not done

- No legend for the history lines (tooltip explains them); no curve on/off switch like the reference.
- On-screen host measurement: SKIPPED as before.

## Larger history, spectrograph lane (2026-11-24, owner request)

- Display: level history 352 px plus a 56 px spectrograph lane on the same 4 s axis (was 364 px total); knobs moved
  down 46 px (KNOB_Y 532 -> 578) into the empty space under them.
- Spectrograph: the plugin publishes the input after the input gain, delayed by its latency, and the sections'
  output (`Shared::push_taps`); the editor takes 4096-sample Hann FFTs of both each tick (OpenLimiter's analyser,
  no peak hold), 24 log bands 40 Hz-16 kHz. Shown: out - in (dB), 3-band averaged, weighted by the band's level
  relative to the loudest band (0 at 40 dB under), half-smoothed over ticks. Red = taken out, green = added.
- Test `spectrum_taps_line_up`: with every section off, the aligned input and the output taps are equal sample for
  sample. Text audit passes (the lane's "Hz" tag collided with "10k" and was removed).
- frame_cost (2x, lane fully lit with noise, load 8.0): closed 5.7-7.5 ms, panel open 9.5-9.8 ms.

## 3D VIEW (2026-11-24, owner request "a 3d spectrograph", after OpenEQ's SPEC 3D)

- Button "3D VIEW" in the chip row (editor setting, not saved; accessibility switch; tooltip). On: the display shows a
  terrain over the full band (20 Hz-20 kHz, 96 bands), newest row at the front spanning the width, 3 s receding up
  and back; the history shrinks to the lane (output fill, input line, gain band, same time labels).
- Each row: the input spectrum's height with the output inside it. Body (theme `cool`) = what came out; red = taken
  out; green = added. Rows opaque, back to front; every other stored row drawn, chosen by row id (no shimmer).
- Cost rules: column fills only, one opaque fill per row does shade + occlusion. frame_cost at 2x (load ~5):
  3D 14.3-14.5 ms (Limiter 17.3 ms); 2D unchanged (5.7-9.9 ms). OpenEQ's JUCE terrain was 127-177 ms.
- Text audit and render tests cover 3D on every tab and panel state; screenshots `<tab>-3d-<theme>-1x`.

## Smoother spectrum drawing (2026-11-24, owner: "a bit jagged edged")

- Cause (2x crops): not the edges (anti-aliased) but the data. Bass bands narrower than the 11.7 Hz FFT bins read
  the same bin (flat steps, hard corners); 96 straight segments per ridge; the lane was 24 blocks x 6 shades.
- 3D: 8192-sample FFT (5.9 Hz bins), 1-4-6-4-1 smoothing across bands, Catmull-Rom ridges (3 points per band).
- Lane: one row per pixel interpolated between bands, 12 shades each way.
- frame_cost 2x (load 11.7): 2D closed 5.9-7.7 ms, 3D 15.0-16.5 ms.

## Key boost, 3D energy colours (2026-11-24, owner requests)

- Sidechain: every keyed section (gate, comp, de-exp, decomp) runs its key, internal or external, through HP -> LP ->
  a bell band (`crates/dynamics/src/keyboost.rs`, RBJ peaking, -24..+24 dB, 20 Hz-20 kHz, Q 0.3-10; off at 0 dB),
  all three at once. New params per section: keyBoostFreq, keyBoost, keyBoostQ; sliders in SIDECHAIN, Freq/Q faded
  at 0 dB. Tests: `keyboost::boosts_its_frequency_and_leaves_the_rest` (+12.0 dB at 2 kHz, 100 Hz flat);
  `comp::external_key_through_boost_hp_and_lp` (-28 dBFS 2 kHz key: no GR; +12 dB boost: 6.0 dB GR at 4:1).
- 3D colours: body coloured by its energy (deep blue -> blue -> cyan -> ice -> near white, 12 steps, runs per row);
  gain reduction always one red (lift one green) at every level and depth; dark well in every theme with an on-screen
  key (hidden under an open panel). frame_cost 3D 14.7-16.7 ms at 2x (load 14).
- 3D well follows the theme (2026-11-24): light themes use the theme's plot colour with a pale-blue -> navy energy scale and dark ridges; dark themes keep the dark well. GR red / lift green unchanged.
- Themes (2026-11-24): added Slate (mid grey, sky blue), Neon (night purple, pink, violet, cyan), Rose (soft pink, dusty rose red); seven in the THEME cycle; every test walks all seven.
- Neon reworked: midnight blue base (no dark purple), pink controls, cyan output/meters, violet input. Rose reworked: pink ground with rose red, deep teal output/meters, lavender input (less monochrome).
- Rose: white and neutral grey surfaces (the pink-tinted paper whites removed); rose red, teal, lavender kept.
