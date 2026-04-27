# Month 3 — Advanced Concepts and Final Project

**Theme:** The ecosystem, advanced patterns, and building something real.

**Project:** [`midi-synth`](./project/midi-synth/) — a real MIDI synthesiser in Rust. Generate WAV files from MIDI files, or plug in a MIDI keyboard and hear notes synthesised live by your own code.

```text
$ cargo run -- --note 69 --duration 2 --waveform sine --out a4.wav
Wrote 88200 samples to a4.wav (2.0s @ 44100Hz)

$ cargo run -- --midi-file twinkle.mid --out twinkle.wav
Parsed twinkle.mid: 14 note events, total 5.2s
Wrote 229320 samples to twinkle.wav

$ cargo run -- --live
Listening on MIDI port 'Akai LPK25'… (Ctrl-C to stop)
```

By the end of Month 3 you will have built a **working synthesiser** that:
- Generates four waveforms (sine, square, sawtooth, triangle) with proper ADSR envelopes
- Renders MIDI files to WAV
- Plays live from a connected MIDI keyboard
- Supports polyphony and chord mode

---

## Sessions

| # | Title | Concepts |
|---|---|---|
| 17 | [The Ecosystem — Modules, Crates, and Cargo](./session-17/) | `mod`, `pub`, `use`, multi-file, `Cargo.toml` deps |
| 18 | [File I/O and Binary Data](./session-18/) | `std::fs`, `BufReader/Writer`, raw bytes, WAV header |
| 19 | [Closures and Iterators (Deep Dive)](./session-19/) | `Fn`/`FnMut`/`FnOnce`, `move`, custom iterators |
| 20 | [Generics and Advanced Traits](./session-20/) | Generic fns/structs, `impl Trait`, `Box<dyn Trait>`, lifetimes intro |
| 21 | [Final Project — WAV Synthesis Engine](./session-21/) | Waveforms, ADSR, write WAV with `hound` |
| 22 | [Final Project — MIDI File Parsing](./session-22/) | `midly`, ticks, tempo, mixing |
| 23 | [Final Project — Live MIDI Input](./session-23/) | `midir`, `cpal`, `mpsc` channels, threading |
| 24 | [Final Project — Polish, Showcase, What Next](./session-24/) | `clap` CLI, polyphony, retrospective |

---

## What's New in Month 3

Months 1 and 2 stayed inside the standard library. Month 3 is where Rust opens up:

- **The crate ecosystem.** Searching `crates.io`, adding dependencies, reading docs on `docs.rs`. This is how real Rust code gets written.
- **Real-world I/O.** Binary file formats. Bytes and offsets. MIDI parsing. WAV writing. No more printing-to-the-terminal-only.
- **Audio.** You will write code that generates sound. Real sound, real samples, real waveforms — same maths used in Ableton and FL Studio.
- **Threading.** Live MIDI requires audio in one thread and MIDI events in another, talking through a channel. This is industrial-strength stuff.

The final project is the hardest thing in the course. Take your time with it. The four sessions are deliberately spaced so you can work on one piece at a time. You've got this.

---

## DofE Reminder

Eight more sessions of evidence in [`session-log.md`](../dfe/session-log.md). After Session 24, fill in:
- [`milestone-3-reflection.md`](../dfe/milestone-3-reflection.md)
- [`participant-statement-template.md`](../dfe/participant-statement-template.md) — your full DofE Skill summary

Once those are done, your DofE Skill section evidence pack is complete.
