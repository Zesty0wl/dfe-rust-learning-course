# Session 22 — Parsing MIDI Files with `midly` and Mixing Voices

> *"Today we teach the synth to play music — actual songs, parsed from real `.mid` files."*

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

## Common Mistakes

- **Multi-track confusion**: Format-1 MIDI has multiple tracks all playing at once. Iterate every track *independently* (each has its own delta-tick clock starting at 0) and mix into the same buffer.
- **Tempo changes mid-song**: real songs change tempo. Update `tempo_us_per_qn` whenever you see a `MetaMessage::Tempo`.
- **Forgetting clamp/normalise**: 4 simultaneous voices at velocity 100 will absolutely blow your speakers if you skip normalisation.

## Session Challenge

1. Find a free `.mid` file online (look for "free MIDI file site" — Bach inventions are great test material). Render it with each waveform.
2. Print the song's total duration before rendering. Bonus points for showing the BPM.
3. Add a `--gain 0.5` flag to attenuate the output.

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

## DofE Log Reminder

Open `dfe/session-log.md`, find row 22, and write 1–3 sentences. Mention: how many notes were in your file and what surprised you about the file format.
