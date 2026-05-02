# Session 23 — Live MIDI: `midir`, `cpal`, and Threads with `mpsc`

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

> *"This week the program plays in real time. Press a key on a real MIDI keyboard, hear it through real speakers. Welcome to systems programming."*

> ### 🅰️🅱️ Choose your track for today
>
> This session is the most demanding in Month 3 — multi-threading, real-time audio, cross-platform USB device code. **You may not have a MIDI keyboard, or your platform's audio stack may make `cpal` painful.** That's normal; here are two routes through.
>
> **Track A — live MIDI (the original path).**
> Real keyboard or virtual keyboard → `midir` → channel → `cpal` audio thread → speakers. Continue reading below. *Pre-flight:* make sure `cargo run -- --note 69 --duration 1 --waveform sine` produces a `.wav` you can hear (i.e. the basic project still builds).
>
> **Track B — render a chord progression to WAV.**
> Skip the threading, skip the hardware. Type a progression on the command line: `"Cmaj:2 Fmaj:2 Gmaj:2 Cmaj:2"`. Each chord is multiple notes sounding *simultaneously* — additive mixing — so you still meet the core "many voices, one buffer" idea behind a polyphonic synth, just without the real-time constraints. The example is in [`examples/track_b_progression/`](./examples/track_b_progression/) and it's a working program. **If you take this track, the `midir`/`cpal`/threads walkthrough below doesn't apply to you — jump straight to "Track B walkthrough" further down.**
>
> Both tracks count for the same DofE evidence. Session 24's `--chord` mode works on both tracks because session 24 stays offline. Note your choice (A or B) in your session log.

## What You'll Learn

- Cross-platform live MIDI input with [`midir`](https://docs.rs/midir).
- Cross-platform audio output with [`cpal`](https://docs.rs/cpal).
- Multi-threading with `std::sync::mpsc` channels — the producer/consumer pattern.
- Why audio callbacks must be **lock-light and allocation-free**.

## The Big Idea

There are three threads in a live synth:

```
   ┌──────────────┐  NoteOn/Off  ┌──────────────┐
   │ MIDI thread  ├─────────────→│  mpsc::tx    │
   └──────────────┘              └─────┬────────┘
                                       │
                                       ▼
   ┌──────────────┐  fill samples ┌──────────────┐
   │ Audio thread │←──────────────│   rx + voices│
   └──────────────┘               └──────────────┘
```

The MIDI thread fires whenever your keyboard sends a byte. The audio thread fires every few milliseconds asking for more samples. They talk through a channel.

If the audio thread blocks for more than a few ms, the speaker glitches. So no `println!`, no `Mutex` contention, no allocation in the hot path. (We *do* take a `Mutex` here — but very briefly. For a real product you'd use a lock-free ring buffer.)

## Concepts Covered

- `midir::MidiInput`, ports, `connect()` callback.
- `cpal::Host → Device → Stream` pattern.
- `std::sync::mpsc::channel()` for thread-safe message passing.
- `Arc<Mutex<Vec<Voice>>>` shared between callbacks (the *only* shared state).
- Why the audio callback **must not** allocate or block (real-time constraint).

## Building Towards `midi-synth`

You'll implement `live.rs` and add a `--live` flag. After today:

```bash
cargo run -- --live --waveform sawtooth
```

…starts listening. Plug in a USB MIDI keyboard (even a cheap £30 one), press keys, hear sound.

> 💡 **No keyboard?** Free virtual MIDI keyboards exist for every OS:
> - macOS: built-in via Audio MIDI Setup → "IAC Driver" + a free app like **VMPK**
> - Windows: **loopMIDI** + **VMPK**
> - Linux: `aconnect` + **vkeybd**

> 💡 **Where to work today.** This is a project session, so you'll be inside the project folder, not the session folder. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-3/project/midi-synth/starter        # your work-in-progress
> cargo run -- <args>
> ```
>
> The reference implementation lives in `month-3/project/midi-synth/solution/` — peek only when you're properly stuck. All `cargo run` commands shown below assume you're inside `month-3/project/midi-synth/starter/`.

## Step-by-Step Walkthrough

### 1. Open MIDI input

```rust
let mut midi_in = MidiInput::new("midi-synth")?;
midi_in.ignore(Ignore::None);
let ports = midi_in.ports();
let port = &ports[0];      // first port — list and pick if multiple
```

### 2. Make a channel

```rust
let (tx, rx) = mpsc::channel::<NoteEvent>();
```

The `tx` is `Send` and `Clone` — perfect for moving into the MIDI callback.

### 3. Connect with a callback

```rust
let _conn = midi_in.connect(port, "midi-synth-port",
    move |_stamp, message, _| {
        if message.len() < 3 { return; }
        let status = message[0] & 0xF0;
        let (note, vel) = (message[1], message[2]);
        match status {
            0x90 if vel > 0 => { let _ = tx.send(NoteEvent::On  { note, velocity: vel }); }
            0x80 | 0x90     => { let _ = tx.send(NoteEvent::Off { note }); }
            _ => {}
        }
    },
    (),
)?;
```

> **Why the underscore on `_conn`?** If we drop the connection, the callback stops. Binding it keeps it alive for the lifetime of `main`. Don't shadow it.

### 4. Open audio output

```rust
let host = cpal::default_host();
let device = host.default_output_device().ok_or("no output device")?;
let config = device.default_output_config()?;
let sample_rate = config.sample_rate().0;
let channels = config.channels() as usize;
```

### 5. Build the stream

The `data` callback runs on the audio thread. It receives a `&mut [f32]` slice of length `frames * channels` and you must fill it.

```rust
let stream = device.build_output_stream(
    &config.into(),
    move |data: &mut [f32], _| {
        // Drain MIDI events into the voice list.
        while let Ok(ev) = rx.try_recv() { /* push or release Voice */ }
        // Fill samples.
        for frame in data.chunks_mut(channels) {
            let mut s = 0.0;
            for v in voices.iter_mut() { s += v.next_sample(); }
            for sample in frame { *sample = s.clamp(-1.0, 1.0); }
        }
        voices.retain(|v| !v.done());
    },
    |err| eprintln!("stream error: {}", err),
    None,
)?;
stream.play()?;
```

### 6. Sleep forever

```rust
loop { std::thread::sleep(Duration::from_secs(60)); }
```

`Ctrl-C` to stop.

## Track B walkthrough — chord progressions to WAV

Skip this section if you took Track A. If you took Track B, this is the whole session.

### 1. Run the example as-is

```bash
cd month-3/session-23/examples/track_b_progression
cargo run -- "Cmaj:2 Fmaj:2 Gmaj:2 Cmaj:2" cadence.wav
```

You'll get `cadence.wav` — the I–IV–V–I progression in C major, played on a sine wave. That's a real cadence; you'll recognise it from countless songs.

### 2. Read the code (about 200 lines, single file)

Open `examples/track_b_progression/src/main.rs`. The six sections are signposted with comment dividers:

1. **Note-letter parsing** — `"C#"` → semitone offset 1.
2. **Chord parsing** — `"Cmaj"` → `[60, 64, 67]` (C, E, G as MIDI numbers).
3. **`midi_to_freq` and `render_note`** — same as Track B for Session 22.
4. **`mix_into` (additive mixing)** — *this is the new bit*. Two notes that sound at the same time get their samples summed.
5. **`render_progression`** — for each chord, render every voice and mix them all into the same time slot. Then advance the cursor and do the next chord.
6. **`normalise`** — when 4 voices play together, the peak amplitude can hit 4.0. We scale the whole buffer down to ±0.99 so it doesn't clip when written as 16-bit PCM.

### 3. Run the tests

```bash
cargo test
```

Six tests — they cover chord-name parsing and additive mixing.

### 4. Modify it: write your own progression

Famous progressions to try (all four chords last 2 seconds):

- **The pop progression (vi–IV–I–V):** `"Am:2 F:2 C:2 G:2"` — Don't Stop Believin', Let It Be, hundreds more.
- **The classic cadence (I–IV–V–I):** `"Cmaj:2 Fmaj:2 Gmaj:2 Cmaj:2"` — every hymn ever.
- **The ii–V–I (jazz):** `"Dm7:2 G7:2 Cmaj7:2"` — about 80% of a jazz standard.
- **12-bar blues in C:** `"Cmaj:2 Cmaj:2 Cmaj:2 Cmaj:2 Fmaj:2 Fmaj:2 Cmaj:2 Cmaj:2 G7:2 Fmaj:2 Cmaj:2 G7:2"`

Render at least two. Listen back and notice how each progression has a different *mood* despite using the same three- or four-note building blocks.

### 5. (Optional) Bring it back into the project

If you finish early, port the `parse_progression` function and a `--progression` flag into the main `midi-synth` project's `main.rs`. The mixing/normalisation infrastructure is already there in `synth.rs`. This is a real, useful, ~30-line addition that demonstrates you can read existing code and extend it — exactly what the assessor's looking for.

## Common Mistakes

**Track A:**

- **No sound, no error**: your default device might be muted, or `cpal` picked the wrong one. Print `device.name()`.
- **Sound stutters / pops**: you're doing too much work in the audio callback. No I/O, no allocation, no `println!`.
- **Stuck notes**: forgot to handle the velocity-0 NoteOn → NoteOff case. Same as session 22.
- **Build fails on Linux**: `sudo apt install libasound2-dev pkg-config`.

**Track B:**

- **Output sounds way too quiet**: you forgot to call `normalise()` after mixing — but the *opposite* (forgetting to normalise so it clips) is also possible. Listen and check.
- **`Cb` parses as a flat instead of a B**: edge case in `parse_note_letter`. The example handles it, but if you write your own parser, watch for it.
- **Chord sounds dissonant**: double-check your interval pattern. `[0, 4, 7]` = major; `[0, 3, 7]` = minor; mixing them up is a one-character mistake with a very audible result.

## Session Challenge

**Track A:**

1. Add a `--port N` flag to pick which MIDI input port to use.
2. Print every received MIDI byte to a *separate* logging thread (use a second channel — never print from the audio callback).
3. Make the synth respond to MIDI Control Change CC74 (filter cutoff) by changing the waveform on-the-fly.

**Track B:**

1. Add a `--bpm N` flag and treat each chord's duration as **beats** rather than seconds.
2. Add a `--waveform` flag and switch between sine, square, triangle, sawtooth.
3. Compose a 16-chord progression that goes somewhere musically interesting (modulate to another key in the middle, perhaps).

## Quick Reference

```rust
// MIDI status bytes:
0x80 = NoteOff (channel low nibble)
0x90 = NoteOn  (vel > 0); NoteOn vel=0 == NoteOff
0xB0 = ControlChange
0xE0 = PitchBend

// cpal sample formats:
SampleFormat::F32 | SampleFormat::I16 | SampleFormat::U16
// You handled F32; add the others if your device requires them.
```

## Further Reading

Curated extra material on the topics covered in this session (Live MIDI — `midir`, `cpal`, threads & `mpsc`). All free; all current as of writing.

- [**The Rust Book** — *Fearless Concurrency* (chapter 16)](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — Threads, channels (`mpsc`), shared state. The session's foundation.
- [**`std::sync::mpsc` documentation**](https://doc.rust-lang.org/std/sync/mpsc/) — The standard-library channel we're using. Note: there's also `crossbeam-channel` if you ever want more performance.
- [**`midir` crate documentation**](https://docs.rs/midir/latest/midir/) — Cross-platform MIDI input library.
- [**`cpal` crate documentation**](https://docs.rs/cpal/latest/cpal/) — Cross-platform audio output library. Read the *Getting Started* section.
- [**Mara Bos — *Rust Atomics and Locks* (free book)**](https://marabos.nl/atomics/) — If concurrency clicks for you, this is the next book. Free online; ~150 pages of dense gold.

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

Open `dfe/session-log.md`, find row 23, and write 1–3 sentences about what you built. **Note which track you took (A or B)**. Track A: what was the hardest part of the threading model? Did you see a "stuck note" bug? How did you fix it? Track B: which progression sounded the most "musical" to you, and why do you think that is?
