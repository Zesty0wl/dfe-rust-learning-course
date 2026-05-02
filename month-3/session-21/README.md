# Session 21 — Audio Engine: Waveforms, ADSR, and your first WAV with `hound`

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

> *"This week we replace our hand-rolled WAV writer with a real crate, then build the heart of a synthesiser: an oscillator with an envelope."*

## What You'll Learn

- Pulling in a third-party crate (`hound`) and reading its docs.
- Designing a small audio engine (Waveform → Oscillator → Envelope → samples).
- What **ADSR** means and why every synth ever made has it.
- Using `clap` for ergonomic CLI parsing.

## The Big Idea

In session 18 you wrote a WAV file by hand: 44 bytes of header, then 16-bit PCM samples. That was important — you now know what's in the file. From here on we'll use [`hound`](https://docs.rs/hound), the de-facto Rust WAV crate, because re-implementing format headers isn't a great use of your time.

The interesting bit this week is the **engine** — a small set of types that turn "MIDI note 69 for 2 seconds, sine wave" into a `Vec<f32>` of audio samples. Specifically:

```
Waveform (enum) ─┐
                 ├→ Oscillator ─→ ADSR multiplier ─→ Vec<f32>
midi_to_freq ────┘
```

Every synth in the world is built on this exact pattern.

## Concepts Covered

- Adding crate dependencies in `Cargo.toml`.
- Modules across files (`mod synth; mod wav;`) — a recap of session 17.
- The **ADSR** envelope: Attack, Decay, Sustain, Release.
- `clap`'s `#[derive(Parser)]` macro for CLI args.

## Building Towards `midi-synth`

This session creates `synth.rs`, `wav.rs`, and a `main.rs` that handles **single-note** mode only. After today:

```bash
cargo run -- --note 69 --duration 2 --waveform sine --out a4.wav
```

…produces a real `.wav` file you can play.

You'll add `--midi-file` (session 22), `--live` (session 23), and `--chord` (session 24) over the next three weeks.

> 💡 **Where to work today.** This is a project session, so you'll be inside the project folder, not the session folder. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-3/project/midi-synth/starter        # your work-in-progress
> cargo run -- <args>
> ```
>
> The reference implementation lives in `month-3/project/midi-synth/solution/` — peek only when you're properly stuck. All `cargo run` commands shown below assume you're inside `month-3/project/midi-synth/starter/`.

## Step-by-Step Walkthrough

### 1. Open the `starter/` project

```bash
cd month-3/project/midi-synth/starter
cargo build
```

The first build pulls in ~80 crates — that's normal. You'll see warnings about unused TODOs. That's the goal: turn warnings into working code.

### 2. Implement `synth.rs`

The skeleton has function signatures and TODOs. Fill them in:

- `Waveform::parse(s: &str)` — match `"sine" | "square" | "saw" | "triangle"`.
- `midi_to_freq(note: u8)` — `440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)`.
- `Oscillator::sample()` — track `sample_index`, compute `t = idx / sample_rate`, then the waveform.
- `Adsr::amplitude(...)` — return a value in `[0.0, 1.0]` based on time and whether the note has been released.
- `render_note(...)` — loop `total_samples` times, multiply oscillator by envelope by velocity.

### 3. ADSR explained

```
Amplitude
  1.0 │   /\
      │  /  \____________   ← sustain level
      │ /    sustain     \
      │/                  \
  0.0 └──────────────────────→ time
       A   D     S          R
```

- **Attack**: time to ramp from 0 → 1 when key pressed.
- **Decay**: time to fall from 1 → sustain level.
- **Sustain**: level held while key is held down.
- **Release**: time to fall from sustain → 0 when key is released.

For an offline render, "key released" means "near the end of the note's duration".

### 4. Run it

```bash
cargo run -- --note 69 --duration 1 --waveform sine --out a4.wav
file a4.wav   # → RIFF (little-endian) data, WAVE audio, ... mono 44100 Hz
```

Open `a4.wav` in any audio player. You should hear a clean A note with a soft attack and release (no clicks!).

## Common Mistakes

- **Clicks at start/end**: your ADSR isn't ramping. The attack and release stop the speaker membrane jumping suddenly.
- **`f32` vs `f64`**: `hound` accepts `i16` samples. Multiply by `i16::MAX` and clamp to `[-1.0, 1.0]` first.
- **Note 69 ≠ 440 Hz**: check `midi_to_freq` — `(69 - 69) / 12 = 0`, `2^0 = 1`, `440 * 1 = 440`. ✓
- **Forgetting `mod synth;`** in `main.rs` — same gotcha as session 17.

## Session Challenge

1. Add a 5th waveform — `--waveform noise` — that returns random `f32` in `[-1.0, 1.0]` (use `fastrand`).
2. Make `--duration` accept fractions (`--duration 0.5`). It already does in `clap` — verify.
3. Render the same note with all four waveforms and compare them in your audio player. Which sounds the warmest? Which is harshest? Write down why in your DofE log.

## Quick Reference

```rust
// From a MIDI note number to frequency in Hz:
fn midi_to_freq(note: u8) -> f32 {
    440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
}

// Writing a WAV with hound:
let spec = hound::WavSpec {
    channels: 1, sample_rate: 44_100,
    bits_per_sample: 16, sample_format: hound::SampleFormat::Int,
};
let mut writer = hound::WavWriter::create("out.wav", spec)?;
for s in samples { writer.write_sample((s * i16::MAX as f32) as i16)?; }
writer.finalize()?;
```

## Further Reading

Curated extra material on the topics covered in this session (Audio Engine — Waveforms, ADSR, `hound`). All free; all current as of writing.

- [**WAV file format spec** — sapp.org](http://soundfile.sapp.org/doc/WaveFormat/) — The canonical one-page reference for the RIFF/WAVE container we're writing into.
- [**Wikipedia — *Envelope (music)***](https://en.wikipedia.org/wiki/Envelope_(music)) — Background on ADSR (Attack–Decay–Sustain–Release) envelopes.
- [**`hound` crate documentation**](https://docs.rs/hound/latest/hound/) — The crate we're using to write WAV files. Tiny API; worth reading top-to-bottom.
- [**3Blue1Brown — *But what is the Fourier Transform? A visual introduction***](https://www.youtube.com/watch?v=spUNpyF58BY) — Why a sawtooth sounds bright and a sine sounds pure. Not required for the project; absolutely worth watching.
- [**Sound on Sound — *Synth Secrets* archive**](https://www.soundonsound.com/series/synth-secrets-1996-2004) — 63-part legendary series covering every aspect of subtractive synthesis. The first ~10 articles are the foundation.

---

## Stuck?

You're not the first. Three places that work when you're properly stuck:

- [**Rust Discord** — `#beginners`](https://discord.gg/rust-lang-community) (fastest; people are friendly)
- [**`/r/learnrust`**](https://www.reddit.com/r/learnrust/) (paste your code + the error; usually answered within hours)
- [**`users.rust-lang.org`**](https://users.rust-lang.org/) (slower; thorough; answers stay searchable for years)

When the compiler error is the thing confusing you, [`resources/compiler-errors.md`](../../resources/compiler-errors.md) translates the most common ones into plain English.

Asking for help isn't cheating — real Rust developers do it daily. Search first; if no luck, post a [minimal reproducible example](https://stackoverflow.com/help/minimal-reproducible-example).

---
## DofE Log Reminder

Open `dfe/session-log.md`, find row 21, and write 1–3 sentences about what you built today. Mention: which waveform sounds the most "synth-y" to you and why you think that is.
