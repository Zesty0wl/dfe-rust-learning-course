# Progress Summary

A static map of the entire course. Use this to give the assessor (or yourself) a quick overview of what each session covered and what evidence it produced.

| Session | Title | Concept(s) introduced | Evidence in this repo |
|---|---|---|---|
| 1 | Why Rust? History, Setup, and the Speed Demo | `cargo new`, `cargo run`, `fn main`, `println!` | `month-1/session-01/examples/pi_python.py`, `pi_rust/`, `hello_world/` + Pi benchmark output in session log |
| 2 | Variables, Types, and Mutability | `let`, `let mut`, scalar types, type inference, shadowing, integer overflow | `month-1/session-02/examples/` (chromatic frequency calculator) |
| 3 | Functions, Expressions, and Basic I/O | `fn`, parameters, return types, expression-vs-statement, `std::io`, `.parse()` | `month-1/session-03/examples/midi_to_freq/` |
| 4 | Control Flow | `if`/`else`, `loop`, `while`, `for`, `break`, `continue`, value-returning loops | `month-1/session-04/examples/piano_keyboard/` |
| 5 | Pattern Matching and `match` | `match` expressions, exhaustive matching, guards, `_` wildcard | `month-1/session-05/examples/midi_note_names/` |
| 6 | Enums and Strings | `enum`, `String` vs `&str`, `format!`, brief `Vec` preview | `month-1/session-06/examples/scales_intro/` |
| 7 | Mini-Project Build Part 1 — Scale Generator | Project work — applying Sessions 1–6 | `month-1/project/music-theory-cli/` (in-progress build) |
| 8 | Mini-Project Build Part 2 — Chord Progressions and Polish | `Cargo.toml` deps preview, `eprintln!`, `process::exit`, `colored` | `month-1/project/music-theory-cli/solution/` (complete) + Milestone 1 reflection |
| 9 | Structs and Methods | `struct`, `impl`, `&self`, `&mut self`, associated functions, `#[derive(Debug)]` | `month-2/session-09/examples/block_struct/` |
| 10 | Enums with Data and `Option<T>` | Variants with data, `Option`, `Some`/`None`, pattern-matching `Option` | `month-2/session-10/examples/tile_enum/` |
| 11 | Collections — `Vec` and `HashMap` | `Vec` in depth, `HashMap`, when to use which | `month-2/session-11/examples/world_grid/` |
| 12 | Iterators and Closures (Introduction) | `.iter()`, `.map()`, `.filter()`, `.collect()`, basic closures | `month-2/session-12/examples/grid_processing/` |
| 13 | Error Handling | `Result<T, E>`, `?`, custom error enums, `Result` from `main` | `month-2/session-13/examples/parse_seed/` |
| 14 | Traits | Defining/implementing traits, `Display`, `Debug`, `Clone`, `Copy`, trait bounds | `month-2/session-14/examples/describable/` |
| 15 | Mini-Project Build Part 1 — World Core | Project work — applying Sessions 9–14 | `month-2/project/world-generator/` (in-progress build) |
| 16 | Mini-Project Build Part 2 — Render and Polish | Render, biomes, statistics, optional `colored` | `month-2/project/world-generator/solution/` (complete) + Milestone 2 reflection |
| 17 | The Ecosystem — Modules, Crates, and Cargo | `mod`, `pub`, `use`, multi-file projects, `Cargo.toml` deps, `cargo add` | `month-3/session-17/examples/music_theory_modular/` |
| 18 | File I/O and Binary Data | `std::fs`, `BufReader`/`BufWriter`, raw byte writing, WAV header | `month-3/session-18/examples/wav_writer/` (hand-rolled WAV) |
| 19 | Closures and Iterators (Deep Dive) | `Fn`/`FnMut`/`FnOnce`, `move`, `.zip`/`.chain`/`.take`, custom iterators | `month-3/session-19/examples/oscillator_iter/` |
| 20 | Generics and Advanced Traits | Generic fns/structs, trait bounds, `impl Trait`, `Box<dyn Trait>`, lifetimes intro | `month-3/session-20/examples/generic_oscillator/` |
| 21 | Final Project Session 1 — WAV Synthesis Engine | Waveforms, ADSR envelope, writing WAV with `hound` | `month-3/project/midi-synth/` (engine complete) |
| 22 | Final Project Session 2 — MIDI File Parsing | `midly`, MIDI ticks, tempo, mixing buffers | `month-3/project/midi-synth/` (file → WAV) |
| 23 | Final Project Session 3 — Live MIDI Input | `midir`, `cpal`, `mpsc` channels, real-time threading | `month-3/project/midi-synth/` (live keyboard mode) |
| 24 | Final Project Session 4 — Polish, Showcase, What Next | `clap` CLI, polyphony, retrospective | `month-3/project/midi-synth/solution/` (complete) + Milestone 3 reflection + participant statement |

---

**Total:** 24 sessions • 3 milestone projects • ~36 working code samples • 1 complete DofE evidence pack.
