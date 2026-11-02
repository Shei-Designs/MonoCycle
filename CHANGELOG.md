# Changelog

All notable changes to MonoCycle are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **The whole interface now always fits the window, with no scrolling.** When
  the window is smaller than the layout's design size (1200 × ~960), the UI
  scales down uniformly instead of overflowing. The layout itself no longer
  reflows into two or one columns on narrow windows. Dialogs scale with it.
- On first launch the window shrinks to fit the screen if the default
  1480 × 970 would not fit (e.g. 1366 × 768 laptops), and opens centred.
- **New app icon**: the Shei Designs MonoCycle tile (a disc cut by one sine
  cycle, Cycle pink) replaces the default Tauri icon on every platform. The
  source and the regeneration script live in `assets/brand/`.

### Fixed

- **The wavetable editor was sluggish to draw in, especially on Linux.** Every
  pointer event redrew the editor *and* the main scope hidden behind the
  dialog, both with expensive canvas glow, and WebKitGTK delivers pointer
  events faster than the screen refreshes. Drawing now updates at most once per
  frame, and the covered scope is only repainted when the dialog closes.
- Dragging a wavetable stroke past the edge of the dialog and releasing over
  the backdrop closed the editor. It now only closes when the click starts on
  the backdrop. Strokes also keep tracking outside the canvas (clamped to the
  edges) instead of stopping at its border.
- The wavetable editor's header text ran under the close button.
- PLAY could stay silent after the system slept or the audio device changed:
  it only resumed the audio engine from the `suspended` state, not WebKit's
  `interrupted` state.
- Moving the window to a monitor with a different scale factor left the scope
  and wavetable canvases rendered at the old pixel density (soft).

## [1.0.0] — 2026-09-21

First public release. Pre-built binaries for Linux, macOS, Windows and
Raspberry Pi are attached to the [release](https://github.com/Shei-Designs/MonoCycle/releases/tag/v1.0.0).

### Added

- Pre-built installers for Linux (AppImage, `.deb`, `.rpm`), Raspberry Pi OS
  64-bit (`arm64.deb`), Windows (`.msi`, `.exe`) and macOS (universal `.dmg`).
- Fonts are now bundled with the application. MonoCycle previously pulled
  JetBrains Mono and VT323 from Google Fonts at runtime, which meant the app
  lost its typography when offline and made an outbound request on every
  launch. It now makes **no network requests at all**.
- The About panel reads its version from the compiled binary rather than a
  hardcoded string, so it can no longer disagree with the release.

### Fixed

- **Changing a parameter during playback clicked and crackled.** The preview
  swapped to the re-rendered waveform by starting a second looping source at
  full gain and hard-stopping the first 20 ms later from a `setTimeout`. Nothing
  faded. Two copies of a near-identical signal therefore summed — up to +6 dB,
  i.e. clipping — and the outgoing one was then cut off mid-cycle. A single
  one-notch slider move produced a 0.58 sample-to-sample step and 7 clipped
  samples; a 1.6 s parameter sweep produced 22 audible clicks and a peak jump of
  1.17 on a signal whose own largest step is 0.02.

  Replaced with a real crossfade. Each voice now owns a gain node, the incoming
  voice starts at the exact point in its loop the outgoing voice is passing
  through, both ramp linearly over 35 ms, and the old voice is stopped on the
  audio clock rather than by a main-thread timer that a busy UI can delay.
  Crossfades are also prevented from overlapping, since two fades in flight do
  not sum to unity. The same sweep now measures a peak jump of 0.033 and **zero**
  click events. Phase alignment is what makes it transparent: crossfading a
  waveform with itself measures 0 error aligned, versus 0.68 RMS unaligned.

  Switching the filter in and out of circuit is ducked over 12 ms for the same
  reason, and starting and stopping playback were already enveloped.
- **A resonant filter could clip the preview output.** A biquad adds gain the
  rest of the chain knows nothing about — a normalized saw through a lowpass at
  Q=1 already exceeded full scale, and the output hard-clipped: 393 railed
  samples in a 0.7 s window. The monitoring path now ends in a compressor
  (threshold 0 dBFS, hard knee) and a backstop shaping curve. Both are inactive
  at or below full scale, so every patch the synth can produce passes through
  untouched — verified sample-for-sample — and only a filter's overshoot is
  reduced. This is the monitoring path only; the filter is preview-only and is
  never part of an exported WAV.
- **The Linux AppImage had no working audio.** Tauri does not bundle GStreamer
  plugins into an AppImage unless `bundle.linux.appimage.bundleMediaFramework`
  is enabled, and WebKitGTK routes the Web Audio API through GStreamer. Without
  it the AppImage shipped GStreamer's core libraries but zero plugins, so no
  audio sink could be created and pressing PLAY failed with "Failed to start
  the audio device". Now enabled and verified: the audio clock runs inside the
  AppImage. It costs ~60 MB of bundle size — worth paying for an instrument that
  has to make sound. The `.deb` and `.rpm` were never affected: they use the
  system GStreamer.
- **RESET now clears the filter.** "Reset all parameters to default" left the
  filter type, cutoff, resonance, LFO and drive exactly as they were, so a
  resonant lowpass survived a reset and coloured everything that followed.
- **The volume readout was wrong by 6 dB.** The fader is square-law — the gain
  node receives `volume²` — but the readout described `volume`, so the 50%
  default reported −6 dB while actually applying −12 dB. The readout now
  describes the gain that is actually applied, and the app no longer disagrees
  with its own startup value.
- **The cutoff slider had two competing handlers.** A leftover linear handler
  ran before the logarithmic one on every drag, briefly writing raw slider
  units (0–1000) into the filter cutoff and pushing them to the BiquadFilter —
  commanding roughly 0 Hz at the bottom of the sweep — before being corrected
  microseconds later.
- **16-bit export truncated where 24-bit rounded.** Samples were converted with
  `| 0`, which truncates toward zero and biases every sample by up to 1 LSB.
  Both depths now round, and both clamp to full scale.
- **OSC B's noise seed was inconsistent at startup.** `state.oscB` shipped
  without a `seed` field while resets and presets assigned one, so OSC B's
  sample-and-hold and noise sources sounded different before the first reset
  than after it.
- The `FOLD` combine mode's fold loop is now bounded, matching the equivalent
  loop in the shaping stage.
- The filter panel and volume fader are now initialised from application state
  at startup instead of relying on values hardcoded in the markup.
- Wavetable editor tip said "click+drag inside the scope"; it meant the
  wavetable canvas.
- In-app links pointed at `AOShei/MonoCycle` and only resolved via GitHub's
  redirect. They now point at the canonical `Shei-Designs/MonoCycle`.

### Changed

- Application identifier is now `com.sheidesigns.monocycle` (was
  `com.monocycle.app`).
- Minimum window size lowered to 800×560 and the default height reduced from
  1060 to 960, so the window fits on a 1080p desktop and on smaller
  Raspberry Pi displays. Maximum width stays at 1480 to keep the layout locked.
- Release builds are optimised for size with LTO and stripped symbols.
- The noise table's resolution is now an explicit constant rather than reusing
  the preview working length. Behaviour is unchanged: noise is generated once
  at a fixed resolution and resampled to the render length, so the preview and
  the exported file share the same underlying random shape.
- Removed the `greet` command left over from the Tauri project template.

### Documentation

- Documented that the filter section is **preview-only** and never baked into
  the exported WAV — the app has always said so in the UI, the README had not.
- Added a tuning-accuracy table. Integer-sample cycle lengths mean pitch error
  grows with the target note: within ~1 cent below B1, ~5 cents to E4, and up
  to ~16 cents above that. The recommendation is to export low and let the
  sampler transpose up.
- Corrected the minimum Rust version from 1.80 to **1.88**, which is what the
  dependency tree actually requires.
- Added install instructions covering the unsigned-binary warnings on Windows
  and macOS, and a Raspberry Pi section (64-bit only; screen size guidance).
- Replaced the stock Tauri template README in `desktop/` with real
  architecture, release and security notes.
- User presets were listed on the roadmap as "next up" despite already
  shipping.

[1.0.0]: https://github.com/Shei-Designs/MonoCycle/releases/tag/v1.0.0
