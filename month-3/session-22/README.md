# Session 22 — Parsing MIDI Files with `midly` and Mixing Voices

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

> *"Today we teach the synth to play music — actual songs, parsed from real `.mid` files."*

> ### 🅰️🅱️ Choose your track for today
>
> The goal of this session is the same on both tracks: **a `.wav` file containing a multi-note tune you wrote.** Pick the path that matches your confidence with binary file formats.
>
> **Track A — parse a `.mid` file (the original path).**
> Use the [`midly`](https://docs.rs/midly) crate to walk a real MIDI file's events, track time in ticks, mix multiple voices, and render to WAV. Real-world skill, more code, more debugging. Fill in `midi_file.rs` in the project. Continue reading below.
>
> **Track B — type a melody on the command line.**
> Skip the binary parser. Type your tune as `"C4:0.5 D4:0.5 E4:1.0"` and render that. Same end result; same synth concepts (oscillator, envelope, mixing); about half the code. The example is in [`examples/track_b_melody/`](./examples/track_b_melody/) and it's a working program — read it, run it, modify the melody. **If you take this track, the `midly` walkthrough below doesn't apply to you — jump straight to "Track B walkthrough" further down.**
>
> Both tracks count for the same DofE evidence. Track B is **not the easy way out** — composing a melody by hand is just a different kind of work. Note your choice (A or B) in your session log.

## What You'll Learn

- What's actually inside a `.mid` file.
- Parsing binary formats safely with [`midly`](https://docs.rs/midly).
- Tracking time in **ticks**, **tempo**, and converting both to seconds.
- Mixing multiple notes (additive synthesis) into a single buffer.

## The Big Idea

A `.mid` file is *not* audio. It's a list of timestamped events:

> `at tick 0, note 60 (middle C) on, velocity 90`
> `at tick 240, note 60 off`
> `at tick 240, note 62 on, velocity 90`
> `…`

Your job is to walk that list and, for every NoteOff, render that note's duration as audio samples and **mix** it into the output buffer at the correct position.

Mixing is just addition: `dst[i] += src[i]`. Two notes playing at the same time = their samples summed.

## Concepts Covered

- The MIDI file format at a high level (header → tracks → events).
- `delta` ticks vs absolute ticks.
- Tempo as **microseconds per quarter note** (default 500 000 = 120 BPM).
- The `midly::Smf::parse(&bytes)` API.
- Pattern matching on `TrackEventKind` and `MidiMessage` enums.

## Building Towards `midi-synth`

This session you'll fill in `midi_file.rs` and add a `--midi-file` flag to `main.rs`. After today:

```bash
cargo run -- --midi-file scale.mid --waveform triangle --out scale.wav
```

…renders the bundled C-major scale to a `.wav` you can listen to.

> 💡 **Where to work today.** This is a project session, so you'll be inside the project folder, not the session folder. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-3/project/midi-synth/starter        # your work-in-progress
> cargo run -- <args>
> ```
>
> The reference implementation lives in `month-3/project/midi-synth/solution/` — peek only when you're properly stuck. All `cargo run` commands shown below assume you're inside `month-3/project/midi-synth/starter/`.

## Step-by-Step Walkthrough

### 1. Inspect the sample file

`scale.mid` is included in `solution/`. It's 98 bytes — open it in a hex editor if you're curious. The header `MThd` and track header `MTrk` are obvious in ASCII view.

### 2. Parse with `midly`

```rust
use midly::{Smf, Timing, MidiMessage, TrackEventKind, MetaMessage};

let bytes = std::fs::read("scale.mid")?;
let smf = Smf::parse(&bytes)?;
let tpqn = match smf.header.timing {
    Timing::Metrical(t) => t.as_int() as u32,
    _ => return Err("SMPTE timing not supported".into()),
};
```

`tpqn` = ticks per quarter note. For our file it's 240.

### 3. Walk events, track time

For each track:

```rust
let mut tick = 0u64;
for ev in track {
    tick += ev.delta.as_int() as u64;
    let seconds = (tick as f32 / tpqn as f32) * (tempo as f32 / 1_000_000.0);
    // …handle ev.kind
}
```

### 4. Track active notes

Push onto a `Vec<ActiveNote>` on NoteOn. On NoteOff, find that note's entry, compute its duration in samples, and call `render_note(...)`. Then `mix_into(&mut buffer, &note_samples, start_sample as usize)`.

> ⚠️ **Gotcha**: a NoteOn with velocity 0 is treated as NoteOff (a quirk of the MIDI spec — running status optimisation). Handle that.

### 5. Normalise and write

After mixing, peak amplitude can exceed 1.0 if many notes overlap. Scale the whole buffer down so the max absolute value is ≈ 0.99 (`normalise()` in `synth.rs`).

### 6. Run

```bash
cargo run -- --midi-file scale.mid --out scale.wav
file scale.wav   # → ... 44100 Hz
```

Listen — you should hear a C-major scale played with your chosen waveform.

## Track B walkthrough — type a melody on the command line

Skip this section if you took Track A. If you took Track B, this is the whole session.

### 1. Run the example as-is

```bash
cd month-3/session-22/examples/track_b_melody
cargo run -- "C4:0.5 D4:0.5 E4:0.5 F4:0.5 G4:1.0" scale.wav
```

You'll get `scale.wav` — a five-note C-major scale fragment. Open it in any audio player. Same end product as Track A's `--midi-file scale.mid`, just expressed differently.

### 2. Read the code (about 150 lines, single file)

Open `examples/track_b_melody/src/main.rs`. The five sections are signposted with comment dividers:

1. **Note parsing** — `"C#4"` → MIDI 61. Pure string handling.
2. **`midi_to_freq` and `render_note`** — exactly the same maths as Track A's project. Sine wave, attack/release envelope to stop clicks.
3. **`render_song`** — concatenate notes end-to-end. (Track A also mixes overlapping notes; Track B doesn't need to because melodies are sequential.)
4. **`write_wav`** — uses the [`hound`](https://docs.rs/hound) crate to write 16-bit mono PCM.
5. **`main`** — parse args, render, write.

### 3. Run the tests

```bash
cargo test
```

There are five — they cover note parsing edge cases. Make sure they all pass before you start modifying.

### 4. Modify it: write your own melody

Pick a tune you can hum. Easy ones:

- **Twinkle Twinkle:** `"C4:0.5 C4:0.5 G4:0.5 G4:0.5 A4:0.5 A4:0.5 G4:1.0 F4:0.5 F4:0.5 E4:0.5 E4:0.5 D4:0.5 D4:0.5 C4:1.0"`
- **Frère Jacques (first phrase):** `"C4:0.5 D4:0.5 E4:0.5 C4:0.5 C4:0.5 D4:0.5 E4:0.5 C4:0.5"`
- **Happy Birthday opening:** `"G4:0.375 G4:0.125 A4:0.5 G4:0.5 C5:0.5 B4:1.0"`

Render at least one of these. Listen to it. Notice that without expression — no dynamics, no humanisation — even a familiar tune sounds robotic. That's a real insight about why music software is harder than it looks.

### 5. (Optional) Bring in your synth project

The melody-rendering example is intentionally self-contained: it duplicates the oscillator and ADSR code rather than depending on the `midi-synth` project. If you want to bridge to Track A's reference implementation, the `midi-synth` solution already supports `--note` for single-note rendering — you can build a simple shell loop that calls it once per token of your melody and concatenates the WAVs with `sox` or `ffmpeg`. That's an entirely optional stretch goal; the core session is complete after step 4.

## Common Mistakes

- **Multi-track confusion**: Format-1 MIDI has multiple tracks all playing at once. Iterate every track *independently* (each has its own delta-tick clock starting at 0) and mix into the same buffer.
- **Tempo changes mid-song**: real songs change tempo. Update `tempo_us_per_qn` whenever you see a `MetaMessage::Tempo`.
- **Forgetting clamp/normalise**: 4 simultaneous voices at velocity 100 will absolutely blow your speakers if you skip normalisation.

## Session Challenge

**Track A:**

1. Find a free `.mid` file online (look for "free MIDI file site" — Bach inventions are great test material). Render it with each waveform.
2. Print the song's total duration before rendering. Bonus points for showing the BPM.
3. Add a `--gain 0.5` flag to attenuate the output.

**Track B:**

1. Render a melody you actually like (more than 8 notes). It can be from a real song, a video-game theme, or one you made up.
2. Add a CLI flag `--waveform sine|square|triangle|sawtooth` and switch the oscillator on it. (Hint: copy `Waveform` enum and `parse_note` from the project's `synth.rs`.)
3. Add a `--bpm` flag. Treat the duration in each token as **beats** rather than seconds, and convert beats → seconds using `60.0 / bpm`.

## Quick Reference

```rust
// Convert ticks to seconds:
fn ticks_to_seconds(ticks: u64, tpqn: u32, tempo_us_per_qn: u32) -> f32 {
    (ticks as f64 / tpqn as f64) as f32 * (tempo_us_per_qn as f32 / 1_000_000.0)
}

// MIDI status nibbles:
// 0x90 = NoteOn   (vel 0 → treat as NoteOff)
// 0x80 = NoteOff
// 0xB0 = ControlChange
// 0xC0 = ProgramChange
```

## Further Reading

Curated extra material on the topics covered in this session (Parsing MIDI files with `midly`). All free; all current as of writing.

- [**`midly` crate documentation**](https://docs.rs/midly/latest/midly/) — The crate we're using. Read the top-level docs and the *Usage* section.
- [**MIDI Association — *MIDI 1.0 specification overview***](https://www.midi.org/specifications/midi1-specifications) — The actual spec, free with a sign-up. The 'Standard MIDI Files' (SMF) document is what we're parsing.
- [**Wikipedia — *MIDI*** (good intro)](https://en.wikipedia.org/wiki/MIDI) — Plain-English overview of what MIDI is, why it's old, and why it's still everywhere.
- [**`midi.teragonaudio.com`** — SMF format explained byte-by-byte](http://midi.teragonaudio.com/tech/midifile.htm) — The most-readable per-byte breakdown of the file format on the web.

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

Open `dfe/session-log.md`, find row 22, and write 1–3 sentences. **Note which track you took (A or B)** and why. Mention: how many notes were in your output and what surprised you (Track A: about the file format; Track B: about how a tune sounds without expression).
