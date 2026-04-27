# Project: `midi-synth` — Final project (Month 3)

> A real audio synthesiser. Plays single notes, chords, full MIDI files, **and** a connected MIDI keyboard live.

This is the capstone of the course. By the end you'll have written a multi-mode CLI that:

1. Renders a single note → `.wav` file (`--note`)
2. Renders a chord → `.wav` file (`--chord`)
3. Parses a `.mid` file and renders the whole performance → `.wav` (`--midi-file`)
4. Listens to a live MIDI keyboard and plays it through your speakers in real time (`--live`)

Mode 4 is *the* moment. You press a key on a USB keyboard, your Rust program does FFT-free additive synthesis, and the speaker makes a sound. That's a real synth.

---

## How this project is structured across sessions

The code is split into small files. Each session adds one of them.

| Session | File added                  | What it does                               |
|---------|-----------------------------|--------------------------------------------|
| **21**  | `synth.rs`, `wav.rs`, `main.rs` | Engine, ADSR, single-note → WAV (`hound`)  |
| **22**  | `midi_file.rs`              | Parse `.mid` with `midly`, mix voices      |
| **23**  | `live.rs`                   | `midir` + `cpal` + `mpsc` for live mode    |
| **24**  | `cli.rs` polish             | `clap` flags, polyphony, `--chord`         |

You won't be writing all this from scratch in one go. Each session you'll add one file (or one feature) and you'll see it working before you move on.

---

## Folder layout

```
project/midi-synth/
├── README.md            ← you are here
├── starter/             ← skeleton with TODOs — start here at session 21
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── synth.rs
│       ├── wav.rs
│       ├── midi_file.rs
│       └── live.rs
└── solution/            ← complete reference implementation
    ├── Cargo.toml
    ├── scale.mid        ← sample MIDI file (C major scale)
    └── src/
        ├── main.rs
        ├── synth.rs
        ├── wav.rs
        ├── midi_file.rs
        └── live.rs
```

---

## Dependencies

`Cargo.toml`:

```toml
[dependencies]
hound = "3.5"            # WAV reader/writer
midly = "0.5"            # MIDI file parser (no_std-friendly)
midir = "0.10"           # cross-platform live MIDI input
cpal = "0.15"            # cross-platform audio output
clap = { version = "4", features = ["derive"] }   # CLI parsing
```

### OS-specific notes

- **macOS / Windows** — works out of the box.
- **Linux** — install ALSA dev headers first:
  ```bash
  sudo apt install libasound2-dev pkg-config
  ```
  See `SETUP.md` at the repo root for full setup notes.

The first `cargo build` will pull ~80 crates and take a couple of minutes. Don't worry — the second build is seconds.

---

## Try the solution

```bash
cd solution

# Single note: A4 (MIDI 69) for 1 second, sine wave → a4.wav
cargo run -- --note 69 --duration 1 --waveform sine --out a4.wav

# C major chord, triangle wave, 2 seconds → c-maj.wav
cargo run -- --chord C --chord-quality major --duration 2 --waveform triangle --out c-maj.wav

# Render the included sample MIDI file → scale.wav
cargo run -- --midi-file scale.mid --waveform sine --out scale.wav

# Live mode — needs a USB MIDI keyboard
cargo run -- --live --waveform sawtooth
```

Open the `.wav` files in any player (QuickTime / Windows Media Player / VLC).

---

## What you'll have learned by the end

- How sound is just numbers at 44 100 samples per second.
- How `Iterator`, generics, traits, and `mpsc` channels combine into something real.
- How to wire up multiple OS-level subsystems (audio out, MIDI in) safely from Rust.
- How a real synth's polyphony, envelope, and mixing actually work.

That last one — you'll be able to walk into a music shop and *understand* what a Moog or a MicroFreak is doing internally. Which is a thing not many 15-year-olds can do.

---

## Stuck?

Each session's README has a **Common Mistakes** section. If a session's TODO has you completely stuck, peek at the matching file in `solution/`. That's why it's there. Don't copy-paste — read it, close the file, then write it from memory in `starter/`. That's how you actually learn it.
