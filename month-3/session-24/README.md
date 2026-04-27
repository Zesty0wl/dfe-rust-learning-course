# Session 24 — Polyphony, `--chord` Mode, and a Retrospective

> *"Final session. Today we polish the CLI, add chord mode, make sure polyphony works properly, and reflect on the road from `Hello, world!` to a working synthesiser."*

## What You'll Learn

- Designing a clean CLI with `clap`'s `#[derive(Parser)]`.
- Polyphony: playing multiple notes truly simultaneously.
- A `--chord ROOT --chord-quality maj|min|...` mode.
- Code review: looking at code you wrote 12 weeks ago and improving it.

## The Big Idea

You already have polyphony in two places:
- `midi_file.rs` mixes overlapping notes from a `.mid`.
- `live.rs` mixes voices from a live keyboard.

This session lifts that capability into the offline CLI as **chord mode**. A chord is just N notes started at the same time:

```
C major = root + major-3rd + perfect-5th = MIDI [60, 64, 67]
```

Loop through them, render each, sum into one buffer, normalise, write WAV. That's it.

## Concepts Covered

- `clap`'s `#[command(...)]` and `#[arg(...)]` attributes.
- Chord intervals (a tiny revisit of `music-theory-cli` from Month 1).
- `clamp` + `normalise` for clean audio at any voice count.
- Reading old code with fresh eyes.

## Building Towards `midi-synth` (final)

After today the CLI supports four mutually exclusive modes:

```bash
midi-synth --note 69                              # single note
midi-synth --chord C --chord-quality minor        # chord
midi-synth --midi-file song.mid                   # whole song
midi-synth --live                                 # live keyboard
```

…all sharing the same engine, ADSR, and WAV writer. That's the project complete.

## Step-by-Step Walkthrough

### 1. Add chord-related CLI flags

```rust
#[arg(long)] chord: Option<String>,                       // root note name
#[arg(long, default_value = "major")] chord_quality: String,
```

### 2. Note-name → MIDI

```rust
fn note_name_to_midi(name: &str) -> Option<u8> {
    let (letter, accidental) = /* C, C#, Bb, ... */;
    let base = match letter { 'C'=>0, 'D'=>2, 'E'=>4, 'F'=>5, 'G'=>7, 'A'=>9, 'B'=>11, _=>return None };
    Some((60 + base + semis) as u8)
}
```

You wrote almost this exact function in session 4 of `music-theory-cli`. Lift and adapt.

### 3. Chord intervals

```rust
fn chord_intervals(quality: &str) -> Option<&'static [i32]> {
    Some(match quality.to_lowercase().as_str() {
        "major" => &[0, 4, 7],
        "minor" => &[0, 3, 7],
        "dim"   => &[0, 3, 6],
        "maj7"  => &[0, 4, 7, 11],
        "min7"  => &[0, 3, 7, 10],
        "dom7" | "7" => &[0, 4, 7, 10],
        _ => return None,
    })
}
```

### 4. Render the chord

```rust
let mut buffer: Vec<f32> = Vec::new();
let voices: Vec<u8> = intervals.iter().map(|i| (root as i32 + i) as u8).collect();
for n in &voices {
    let buf = render_note(waveform, *n, duration, sample_rate, 0.7, Adsr::default());
    mix_into(&mut buffer, &buf, 0);
}
normalise(&mut buffer);
wav::write_wav(&out, &buffer, sample_rate)?;
```

### 5. Try it

```bash
cargo run -- --chord C --chord-quality minor --duration 2 --waveform triangle --out c-min.wav
cargo run -- --chord G --chord-quality dom7  --duration 3 --waveform saw      --out g7.wav
```

A `dom7` chord wants to resolve down a fifth — try playing G7 then C major back-to-back.

## Common Mistakes

- **Chord too quiet** after normalisation: that's correct! Four voices summed and then scaled to peak ≈ 1.0 puts each individual voice at ~0.25. To compensate, raise per-voice velocity to ~0.85.
- **Modes silently overlap**: order your `if let Some(...) = ...` branches sensibly and `return Ok(())` from each.

## Session Challenge — capstone

1. **Add `--arpeggio`** that, instead of mixing chord notes simultaneously, plays them in sequence with a configurable note length.
2. **Read a chord progression** from a text file (`Cmaj | Am | F | G7 |`) and render the whole thing.
3. **Write a 30-second backing track**: a simple chord progression + a single melody line over the top, all from one Rust command.

---

## What You've Built

Look back. In **session 1** your program said `Hello, Leo!`. In **session 24**:

- It synthesises four waveforms from first principles.
- It applies an ADSR envelope you understand inch-by-inch.
- It writes valid WAV files.
- It parses arbitrary MIDI files and renders them.
- It listens to a real MIDI keyboard and plays through real speakers — handling threading, audio callbacks, and OS APIs cleanly.
- It exposes all of this through a tidy `clap` CLI.

You also built two earlier projects (`music-theory-cli`, `world-generator`), so you have **three** real Rust programs on your laptop. None of those are toys. People sell software like this.

## What's Next

You've outgrown the beginner phase of Rust. Pick whatever sounds fun:

- **Web / APIs** — [`axum`](https://docs.rs/axum) is the easiest entry point. Build a web app.
- **Game dev** — [`bevy`](https://bevyengine.org/) is a fully-featured ECS engine, all in Rust. Make a game.
- **Embedded** — get a £4 Raspberry Pi Pico and run Rust on it via [`embassy`](https://embassy.dev/).
- **WASM in the browser** — wrap your synth in [`wasm-bindgen`](https://rustwasm.github.io/) and put it on a website.
- **Open source** — `rustup component add clippy` then go pick an issue tagged `good-first-issue` on a project you use.

Pick one of those. Make a small thing. Show your supervisor. Keep your DofE log going for as long as you keep coding — you'll be glad you did.

## DofE Log Reminder

Open `dfe/session-log.md`, find row 24, and write a longer entry today (5–10 sentences). This is your **final reflection**:

- What was the hardest concept across the 24 sessions?
- What's something you genuinely surprised yourself by building?
- Show one of the WAVs to a parent / friend / supervisor — what did they say?
- What will you build next?

Then update `dfe/skill-evidence.md` with the final-week reflection and `dfe/final-presentation.md` if you're ready. Well done. 🎚️
