# Session 17: The Ecosystem — Modules, Crates, and Cargo

## What You'll Learn

How real Rust projects are organised. Up to now everything has lived in `src/main.rs`. That's fine for one-file demos but not for anything serious. Today: split code across files with **modules**, control visibility with **`pub`**, bring names into scope with **`use`**, and pull in third-party libraries (**crates**) from `crates.io`.

## The Big Idea

Three terms, used constantly, often confusingly mixed up:

- **Crate** — a *Rust project*. Either a library or a binary. Each has a single `Cargo.toml`. Things on `crates.io` are crates.
- **Module** — a *namespace inside a crate*. Created with `mod foo;` (looks for `foo.rs` or `foo/mod.rs`). Helps you organise code into logical sections.
- **Package** — a folder with a `Cargo.toml` containing one or more crates (usually one).

The hierarchy: a **package** contains one or more **crates**. A **crate** contains a tree of **modules**.

By default everything in Rust is **private**. To expose something across module boundaries, mark it `pub`. To use a name without typing the full path, `use` it.

```rust
// src/main.rs
mod notes;          // declares: there's a module called notes
mod scales;

use notes::Note;    // bring Note into scope so we can write `Note::C` instead of `notes::Note::C`

fn main() {
    let n = Note::C;
    let scale = scales::major(n);
    println!("{:?}", scale);
}
```

## Concepts Covered

- `mod foo;` — declare a module (file `foo.rs` or folder `foo/mod.rs`)
- `pub` — mark items public to outer scopes
- `use` — bring names into scope; `use foo::{a, b, c};` for multiple
- The crate root: `src/main.rs` for binaries, `src/lib.rs` for libraries
- `Cargo.toml` `[dependencies]` — versions, semantic versioning, `cargo add`
- `[dev-dependencies]` — only used for tests/benchmarks
- Reading docs on `docs.rs`, finding crates on `crates.io`

## Building Towards `midi-synth`

The synthesiser will have multiple files: `oscillator.rs`, `envelope.rs`, `wav.rs`, `midi_file.rs`, `live.rs`, `cli.rs`, plus `main.rs`. You couldn't reasonably keep that in one file. Today's example refactors something familiar — your Month 1 `music-theory-cli` — into proper modules. Same code, cleaner structure.

---

## Step-by-Step Walkthrough

### 1. The example layout

`examples/music_theory_modular/` looks like this:

```text
music_theory_modular/
├── Cargo.toml
└── src/
    ├── main.rs        # the binary's entry point
    ├── notes.rs       # Note enum, MIDI conversion
    └── scales.rs      # scale patterns, scale generation
```

That's all you need. Each `.rs` file at `src/` level becomes a module of the same name.

### 2. `notes.rs`

```rust
#[derive(Debug, Clone, Copy)]
pub enum Note {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}

impl Note {
    pub fn semitone(self) -> u8 {
        self as u8
    }

    pub fn from_semitone(n: u8) -> Self {
        let names = [
            Note::C, Note::CSharp, Note::D, Note::DSharp, Note::E,
            Note::F, Note::FSharp, Note::G, Note::GSharp,
            Note::A, Note::ASharp, Note::B,
        ];
        names[(n % 12) as usize]
    }

    pub fn name(self) -> &'static str {
        match self {
            Note::C => "C",   Note::CSharp => "C#",
            Note::D => "D",   Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",   Note::FSharp => "F#",
            Note::G => "G",   Note::GSharp => "G#",
            Note::A => "A",   Note::ASharp => "A#",
            Note::B => "B",
        }
    }
}
```

Note the `pub` everywhere — without it, this enum and its methods would be invisible to `main.rs`.

### 3. `scales.rs`

```rust
use crate::notes::Note;

pub const MAJOR_PATTERN:        [u8; 7] = [2, 2, 1, 2, 2, 2, 1];
pub const NATURAL_MINOR_PATTERN:[u8; 7] = [2, 1, 2, 2, 1, 2, 2];

pub fn build(root: Note, pattern: &[u8]) -> Vec<Note> {
    let mut out = vec![root];
    let mut current = root.semitone();
    for step in pattern {
        current = (current + step) % 12;
        out.push(Note::from_semitone(current));
    }
    out
}

pub fn major(root: Note) -> Vec<Note> {
    let mut s = build(root, &MAJOR_PATTERN);
    s.pop();   // 8th note is the root again — drop it for a 7-note display
    s
}

pub fn natural_minor(root: Note) -> Vec<Note> {
    let mut s = build(root, &NATURAL_MINOR_PATTERN);
    s.pop();
    s
}
```

`use crate::notes::Note;` — `crate::` means "the root of this crate" (i.e. `src/`). So `crate::notes::Note` is the `Note` enum from `notes.rs`.

### 4. `main.rs`

```rust
mod notes;
mod scales;

use notes::Note;

fn main() {
    let scale = scales::major(Note::C);
    let names: Vec<&str> = scale.iter().map(|n| n.name()).collect();
    println!("C Major: {}", names.join(" "));

    let minor = scales::natural_minor(Note::A);
    let mnames: Vec<&str> = minor.iter().map(|n| n.name()).collect();
    println!("A Minor: {}", mnames.join(" "));
}
```

Two `mod` declarations tell the compiler "these modules exist". Then `use notes::Note;` is just for convenience.

### 5. `pub`, demystified

Visibility rules in Rust are strict but logical:

- Default: **private**, only the current module can see it.
- `pub` — visible to any code that knows the path.
- `pub(crate)` — visible anywhere in this crate, but not to outside crates that depend on yours.
- `pub(super)` — visible to the parent module only.

For `main.rs`-only programs (binaries), `pub` and `pub(crate)` behave identically because there's no "outside crate" to worry about. For libraries, the distinction matters — `pub` is your public API.

### 6. Folder modules

If `notes` grows large, you can promote it from `notes.rs` to a folder:

```text
src/
├── notes/
│   ├── mod.rs        # the module's "main file"
│   └── chord.rs      # a sub-module: notes::chord
└── main.rs
```

Then in `notes/mod.rs`: `pub mod chord;` exposes `notes::chord`. Works exactly the same.

### 7. Adding crates

Suppose you want coloured terminal output. Open `Cargo.toml` and:

```toml
[dependencies]
colored = "2.1"
```

…or just from the terminal:

```bash
cargo add colored
```

Both do the same. Then in your code:

```rust
use colored::Colorize;
println!("{}", "C Major".green().bold());
```

`cargo build` downloads the crate from `crates.io`, compiles it, links it. Done. The downloaded source lives in `~/.cargo/registry/` so subsequent builds are instant.

### 8. The `Cargo.toml` zoo

```toml
[package]
name = "my-thing"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
clap  = { version = "4", features = ["derive"] }
some-crate = { git = "https://github.com/foo/bar" }      # straight from git
local-helper = { path = "../helper" }                     # local path

[dev-dependencies]
proptest = "1"        # only built for tests / benches

[features]
fast = []             # define your own conditional-compilation features
```

Versions follow **semver**: `"2.1"` means "any 2.x release at or above 2.1, but not 3.x". Cargo will happily upgrade you to 2.5, 2.99, etc. — but never 3.0 unless you explicitly bump.

---

## Common Mistakes

- **Forgetting `pub`** — error: "function `foo` is private". Fix: `pub fn foo`.
- **`mod` declared twice in different files** — only the crate root (`main.rs` or `lib.rs`) and the parent module's `mod.rs` should `mod foo;` it. If you `mod foo;` from two places you get a duplicate definition error.
- **`use crate::...` vs `use super::...` vs no prefix** — `crate::` from the root, `super::` from one level up, no prefix uses the current module's children.
- **Forgetting to commit `Cargo.lock`** — for binary projects, *do* commit `Cargo.lock` (it pins exact versions). For library crates, traditionally don't (let consumers pick). Our setup is binary-only, so commit it.

---

## Session Challenge

Take your Month 1 `music-theory-cli` solution. Refactor it from a single `main.rs` into:

- `src/notes.rs` — `Note` and helpers
- `src/scales.rs` — `Scale` enum and pattern lookup
- `src/chords.rs` — chord-quality logic
- `src/main.rs` — argument parsing and orchestration

Confirm the refactored version produces identical output to the original. Commit before and after, so you can see the diff is *just* moving code, not changing behaviour.

---

## Quick Reference

```rust
// Crate root: src/main.rs
mod notes;            // load notes.rs
mod scales;           // load scales.rs

use notes::Note;      // bring symbol into scope
use scales::{major, natural_minor};   // multiple at once

fn main() {
    let s = major(Note::C);
}
```

```toml
# Cargo.toml
[dependencies]
colored = "2.1"
clap = { version = "4", features = ["derive"] }
```

```bash
cargo add hound        # add to dependencies
cargo add --dev proptest   # add to dev-dependencies
cargo tree             # see your dep graph
cargo doc --open       # build & view docs for every dep you use
```

---

## DofE Log Reminder

Row 17. New month, fresh chain. Keep going.
