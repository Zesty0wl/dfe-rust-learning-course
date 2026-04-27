# `music-theory-cli` — Month 1 Project

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

A command-line tool that takes a musical root note and a scale type, and prints the notes, intervals, and diatonic chord progression for that scale.

## Usage

```bash
$ cargo run -- --root C --scale major
Scale: C Major
Notes:     C    D    E    F    G    A    B
Intervals: W    W    H    W    W    W    H
Chords:    Cmaj Dmin Emin Fmaj Gmaj Amin Bdim
```

```bash
$ cargo run -- --root F# --scale minor
Scale: F# NaturalMinor
Notes:     F#   G#   A    B    C#   D    E
Intervals: W    H    W    W    H    W    W
Chords:    F#min G#dim Amaj Bmin C#min Dmaj Emaj
```

## Supported scales

- `major` (alias: `maj`)
- `minor` (aliases: `min`, `natural-minor`)
- `pentatonic` (alias: `pentatonic-major`)

## Supported root notes

Any of `C`, `C#`, `Db`, `D`, `D#`, `Eb`, `E`, `F`, `F#`, `Gb`, `G`, `G#`, `Ab`, `A`, `A#`, `Bb`, `B`. Case-insensitive.

## Folders

- [`starter/`](./starter/) — skeleton with `TODO`s for you to fill in during Session 7.
- [`solution/`](./solution/) — fully working reference solution. Don't peek before you've tried.

## Build and run from the solution

From the **repo root**:

```bash
cd month-1/project/music-theory-cli/solution
cargo run -- --root C --scale major
```

## Concepts demonstrated

- Enums and `match` (Sessions 5–6)
- Functions and expressions (Session 3)
- String parsing (Sessions 3, 6)
- `Vec<T>` (preview, Session 6; full in Session 11)
- CLI args via `std::env::args` (Session 7)
- `eprintln!` and process exit codes (Session 8)
- Optional: external crates via `colored` (Session 8 stretch)
