# Session 24 — Polyphony, `--chord` Mode, and a Retrospective

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

> *"Final session. Today we polish the CLI, add chord mode, make sure polyphony works properly, and reflect on the road from `Hello, world!` to a working synthesiser."*

> ### 🅰️🅱️ Tracks converge here
>
> If you took **Track A** in sessions 22 and 23 (parsed `.mid` files, built live MIDI), `--chord` mode is the natural finishing touch — a fourth, simpler mode that completes the CLI.
>
> If you took **Track B** (typed melodies and chord progressions to WAV), today's `--chord` mode is the natural extension of yesterday's `--progression` example — same `chord_intervals`, same additive mixing, just one chord instead of many. You can either work in the project's `starter/` or extend yesterday's `track_b_progression` example. Both count.
>
> The **What You've Built** retrospective at the end is written for both tracks.

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

> 💡 **Where to work today.** This is a project session, so you'll be inside the project folder, not the session folder. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-3/project/midi-synth/starter        # your work-in-progress
> cargo run -- <args>
> ```
>
> The reference implementation lives in `month-3/project/midi-synth/solution/` — peek only when you're properly stuck. All `cargo run` commands shown below assume you're inside `month-3/project/midi-synth/starter/`.

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

Look back. In **session 1** your program said `Hello, world!`. In **session 24** you have a working software synthesiser that:

- Synthesises four waveforms from first principles.
- Applies an ADSR envelope you understand inch-by-inch.
- Writes valid WAV files.
- Renders multi-note music to audio (a `.mid` file if you took Track A; a typed melody and chord progression if you took Track B).
- Exposes everything through a tidy `clap` CLI.

If you took Track A in session 23, your synth *also* listens to a real MIDI keyboard and plays through real speakers — handling threading, audio callbacks, and OS APIs cleanly. That's a non-trivial systems-programming win.

You also built two earlier projects (`music-theory-cli`, `world-generator`), so you have **three** real Rust programs on your laptop. None of those are toys. People sell software like this.

## What's Next

You've outgrown the beginner phase of Rust. Pick whatever sounds fun:

- **Web / APIs** — [`axum`](https://docs.rs/axum) is the easiest entry point. Build a web app.
- **Game dev** — [`bevy`](https://bevyengine.org/) is a fully-featured ECS engine, all in Rust. Make a game.
- **Embedded** — get a £4 Raspberry Pi Pico and run Rust on it via [`embassy`](https://embassy.dev/).
- **WASM in the browser** — wrap your synth in [`wasm-bindgen`](https://rustwasm.github.io/) and put it on a website.
- **Open source** — `rustup component add clippy` then go pick an issue tagged `good-first-issue` on a project you use.

Pick one of those. Make a small thing. Show your supervisor. Keep your DofE log going for as long as you keep coding — you'll be glad you did.

## Further Reading

Curated extra material on the topics covered in this session (Polyphony, `--chord` mode, retrospective). All free; all current as of writing.

- [**Wikipedia — *Polyphony and monophony in instruments***](https://en.wikipedia.org/wiki/Polyphony_and_monophony_in_instruments) — Background on the polyphony/voice-stealing problem we're tackling.
- [**The Rust community — *Where to go from here***](https://www.rust-lang.org/community) — Forums, Discord, user groups, conferences. The community is one of Rust's best features.
- [**This Week in Rust** — weekly newsletter](https://this-week-in-rust.org) — The single best way to keep current. Subscribe.
- [**Are We Game Yet? / Are We Web Yet? / Are We Learning Yet?** — domain-progress trackers](https://wiki.mozilla.org/Areweyet) — Once you've finished this course, these tell you which domains in Rust are ready and which are still maturing — handy for picking your next project.
- [**Rustlings** — interactive practice problems](https://github.com/rust-lang/rustlings) — Official exercise set. Run through the chapters that match areas you found hard in this course; it's the best drill.

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

Open `dfe/session-log.md`, find row 24, and write a longer entry today (5–10 sentences). This is your **final reflection**:

- What was the hardest concept across the 24 sessions?
- What's something you genuinely surprised yourself by building?
- Show one of the WAVs to a parent / friend / supervisor — what did they say?
- What will you build next?

Then complete [`dfe/milestone-3-reflection.md`](../../dfe/milestone-3-reflection.md) and write your final personal statement using [`dfe/participant-statement-template.md`](../../dfe/participant-statement-template.md). Print everything, hand the binder to your assessor, and you're done. Well done. 🎚️
