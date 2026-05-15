<p align="center">
  <img src="assets/banner.svg" alt="MonoCycle — single-cycle waveform designer for hardware samplers" width="100%"/>
</p>

**A single-cycle waveform designer for hardware samplers.** Build precise, perfectly-tuned single-cycle WAVs in a focused workbench, then drop them onto your Roland SP-404mkII, Elektron Digitakt, or any sampler that handles short loops — and your sampler is suddenly a synthesizer.

> Status: pre-release. Linux / macOS / Windows builds via Tauri 2. Open source under GPL v3.

---

## What is a single-cycle waveform — and why should you care?

A single-cycle waveform is one period of a periodic tone: one sine, one saw, one custom shape you drew. A handful to a few thousand samples long. The trick is that **a sampler looping a single-cycle file at different pitches becomes a synthesizer.** The waveform you put in the file is the timbre. Pitch comes from how fast the sampler plays it back.

It's the cheat code behind half the iconic chiptune, lo-fi, and synthwave records of the last decade: a small library of tuned single-cycle WAVs sitting on an SD card, turning a sampler box into a stack of monosynths.

The catch: the file has to be **tuned correctly**. The cycle's length in samples (at the sample rate) determines its fundamental frequency. Get the math wrong by a few samples and your sampler plays everything sharp or flat. That math is what MonoCycle does, so you can focus on the part that actually matters — sculpting the tone.

## Who this is for

- **Hardware sampler players** — SP-404mkII, Digitakt I/II, MPC, Octatrack, Volca Sample, anything that loops a short WAV — who want their box to do synth duty without lugging a laptop on stage.
- **Sample pack makers** designing single-cycle banks for the gear above.
- **Curious musicians and students** who want to *see* what FM, wavefolding, ring modulation, or bit-crushing actually does to a wave in real time. MonoCycle is a real-time visual textbook that happens to export its results.
- **Hardware synth nerds** who like the feel of dialing in a tone and want the same workflow for their sampler.

If you're already in a DAW with Vital, Serum, or Pigments — you have everything MonoCycle does and more. This isn't for you. This is for the people who picked hardware on purpose.

## Features

- **Two oscillators**, each independently choosing from 9 wave types:
  sine · triangle · saw · ramp · square · staircase · sample-and-hold · noise · drawable wavetable
- **Per-oscillator controls**: phase, harmonic limit (band-limited or full-bright/aliased), symmetry / pulse width, step count, RNG seed, level
- **10 combine modes** for the two oscillators:
  blend · ring · AM · FM (true phase modulation) · min · max · XOR · AND · OR · wavefold
- **Four shape stages**, in order: phase distortion → wavefold → drive (tanh saturation) → bit crush (down to 2-bit)
- **Filter section**: lowpass / highpass / bandpass / notch + cutoff drive + LFO modulation
- **Drawable wavetable editor** — paint a 256-point custom shape, use it as a source on either oscillator
- **16 factory presets** covering the classic single-cycle territories (clean sine, bright saw, ring bell, FM brass, wavefolded sine, narrow pulse, bit-crushed triangle, super saw, …)
- **Live preview at the export pitch** — what you hear is exactly what the WAV will sound like when triggered at native rate on your sampler. No surprises.
- **48 kHz mono WAV export**, 16- or 24-bit PCM, 1 / 2 / 4 / 8-cycle output, cent-accurate detuning readout
- **Filename auto-encoding** — exported files are named for their settings (`SAW_FM_SINE_C2_x1_16b_2026-05-15T14-23-15.wav`) so you can keep them straight on the card

## The workflow

1. Pick or design a tone. Two oscillators, mix mode, shape, filter — whatever sounds right.
2. Set the **target note** (the MIDI pitch the file will play at when triggered at native rate on the sampler).
3. Hit **Play**. The preview pitches the loop to that target note, so what you hear is the file.
4. Hit **Export WAV**. Drop the file onto your sampler's SD card.
5. Trigger it. Pitch up and down it with your sampler's pitch / chromatic mode. You now have a synthesizer voice.

## Educational use

Synthesis is one of those topics where the math reads dry but the audio is obvious the moment you hear it. MonoCycle is built around that gap: every parameter changes the scope's drawing in real time, and the four scope readouts (PEAK, RMS, ZCR, DC) make the math show up in the picture.

- Drag the FM amount on two sines and watch the spectrum bloom.
- Wavefold a triangle until it looks like an interference pattern.
- Crush a saw to 3-bit and see the staircase appear.
- Draw a wavetable by hand and use it as a source.

It's a synthesis tutor with a WAV export button.

## Install

Pre-built binaries for Linux, macOS, and Windows will ship with the first stable release. Until then, build from source — it takes ~3 minutes the first time and seconds after.

## Build from source

**Prerequisites:**

- [Rust](https://rustup.rs/) (stable, 1.80+)
- [Node](https://nodejs.org/) 18+ and [pnpm](https://pnpm.io/)
- Tauri 2 platform deps — see [Tauri prerequisites](https://tauri.app/start/prerequisites/)

Arch / EndeavourOS example:

```sh
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator librsvg gtk3 patchelf
```

**Run the dev build:**

```sh
cd desktop
pnpm install
pnpm tauri dev
```

The first run pulls and compiles ~500 Rust crates (a few minutes). After that, dev launches in 1–2 seconds and hot-reloads HTML/CSS/JS changes live.

**Build a release bundle:**

```sh
pnpm tauri build
```

Installers / AppImages / `.dmg` / `.msi` land in `desktop/src-tauri/target/release/bundle/`.

## Keyboard shortcuts

- `Space` — play / stop preview
- `W` — open / close the wavetable editor
- `Ctrl+E` (`⌘+E`) — export WAV
- `Ctrl+R` (`⌘+R`) — reset all parameters
- `Esc` — close any open dialog / dropdown

## Roadmap

- **User presets** — save / recall / share named parameter sets (next up)
- **Post-processing waveform visualization** — see the output waveform after filter / volume / pitch alongside the source waveform, so changes are visible as well as audible
- **Drag-and-drop export** — drop a single-cycle directly onto a sampler folder, no Save dialog
- **Stable release builds** for Linux (AppImage + tarball), macOS (universal `.dmg`), Windows (`.msi`)

## License

[GNU GPL v3](LICENSE) © 2026 Andrew O'Shei / [Shei Designs](https://github.com/AOShei).

Free software in the proper sense — read it, modify it, ship it, distribute it — but keep it open.

## Author

**Andrew O'Shei** — electronic musician, hardware-sampler enthusiast, and the builder behind [Shei Designs](https://github.com/AOShei). MonoCycle is a tool I built while making sample packs for my Elektron Digitakt and Roland SP-404mkii. I'm releasing it because I think it will be useful for other musicians who want to turn their sampler into a powerful monosynth.

If MonoCycle ends up on a track you make, I'd genuinely love to hear it. Please leave a star on github so more people can discover this app !
