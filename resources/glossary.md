# Glossary

A friendly dictionary of every important Rust term used in the course.

---

**ADSR envelope** — Attack, Decay, Sustain, Release. A four-stage curve that shapes a note's loudness over time so it sounds natural and doesn't click.

**Associated function** — A function attached to a type via `impl` but that doesn't take `self`. Often used as a constructor: `String::new()`, `Vec::with_capacity(10)`.

**Borrow checker** — The part of the Rust compiler that tracks which code "owns" each piece of data and which code is "borrowing" it. Catches whole categories of memory bugs at compile time.

**Cargo** — Rust's official build tool and package manager. You'll use it constantly.

**Closure** — An anonymous function that can capture variables from the surrounding scope: `|x| x + n`.

**Compile time** — When the compiler runs (`cargo build`). Errors caught here happen *before* your program runs. Rust catches a lot at compile time on purpose.

**Crate** — A package of Rust code. Either a binary (an app) or a library (used by other crates). `crates.io` is the public registry.

**`derive`** — A shortcut that asks the compiler to generate trait implementations for you. `#[derive(Debug, Clone)]` saves you writing them by hand.

**Enum** — A type that can be one of several named variants. Each variant can carry its own data. `enum Tile { Ocean(u8), Mountain }`.

**Expression vs statement** — An *expression* produces a value. A *statement* doesn't (ends with `;`). The last expression in a block (without a semicolon) is the block's value — that's how Rust returns things.

**`Fn`, `FnMut`, `FnOnce`** — Three traits closures can implement, depending on whether they read, mutate, or consume what they capture.

**Generic** — Code that works with many types via type parameters: `fn first<T>(v: &[T]) -> &T`.

**Hash function** — A function that takes input and returns a fixed-size pseudo-random-looking number. Used in `HashMap`, in our world generator's noise function, and in cryptography.

**`HashMap<K, V>`** — A key→value lookup table. `O(1)` average insert and lookup.

**`impl` block** — Where you attach methods (and associated functions) to a type.

**Iterator** — Anything that produces a sequence of values one at a time. The `Iterator` trait powers `for` loops and the `.map().filter().collect()` style.

**Lifetime** — A label (like `'a`) that tells the compiler how long a reference is valid. Most of the time the compiler figures these out automatically.

**`match`** — Rust's pattern-matching expression. Like a supercharged `switch`. The compiler insists you handle every case.

**Method** — A function defined inside an `impl` block that takes `self`, `&self`, or `&mut self` as its first parameter.

**MIDI** — Musical Instrument Digital Interface. A protocol for representing musical notes (note on, note off, velocity, channel, etc.) without any actual audio. Your keyboard sends MIDI; the synth turns it into audio.

**Module (`mod`)** — A namespace for organising code. A module can be a sub-folder or a sub-file.

**Monte Carlo method** — Estimating a result by sampling lots of random points. We use it to estimate Pi in Session 1.

**Mutability (`mut`)** — Whether a binding can be changed after creation. Variables and references are immutable by default; opt in with `mut`.

**Noise function** — A function that takes coordinates and returns a deterministic pseudo-random value. Used in procedural generation (terrain, textures, music).

**`Option<T>`** — An enum representing "either a value of type `T` (`Some(x)`) or nothing (`None`)". Rust's answer to null without the bugs.

**Ownership** — The single rule at the heart of Rust: every value has exactly one owner. When the owner goes out of scope, the value is dropped.

**Pattern** — The thing on the left side of `=>` in a `match` arm, or in a `let` destructure: `let (a, b) = pair;`.

**Polyphony** — The ability of a synthesiser to play more than one note at the same time.

**Procedural generation** — Creating content (terrain, music, levels) algorithmically rather than by hand. What Minecraft does for its worlds.

**Reference (`&` and `&mut`)** — A "borrow" of a value. `&T` is a shared/read-only reference; `&mut T` is exclusive/mutable.

**`Result<T, E>`** — An enum representing either success (`Ok(value)`) or failure (`Err(error)`). Rust uses this everywhere instead of exceptions.

**Runtime** — When your program is actually running. The opposite of compile time.

**`rustup`** — The tool that installs and updates Rust itself.

**Sample (audio)** — A single number representing the amplitude of an audio signal at one moment in time. CD-quality audio uses 44,100 samples per second.

**Sample rate** — How many audio samples per second. Standard values are 44,100 Hz (CD) and 48,000 Hz (most pro audio).

**Semitone** — The smallest interval in standard Western music. Twelve semitones make an octave. The frequency ratio between adjacent semitones is the twelfth root of two.

**Shadowing** — Re-declaring a variable with the same name. The new binding replaces the old, possibly with a different type.

**Slice (`&[T]`)** — A reference to a contiguous run of elements. `&str` is a string slice.

**`String` vs `&str`** — `String` owns its data and is heap-allocated. `&str` is a borrowed view into a string. Most function parameters take `&str`; you store `String`.

**Struct** — A type that bundles named fields together. Like a record or class without methods (methods go in `impl`).

**Trait** — Rust's version of an interface. Lists the methods a type must provide. You can implement traits for types you don't own.

**Trait bound** — A constraint that says "this type parameter `T` must implement this trait": `fn print<T: Display>(x: T)`.

**Type inference** — Rust figures out a variable's type from context, so you don't usually need to write it.

**`unwrap()`** — Pull the value out of an `Option` or `Result`, or crash if there isn't one. Fine for examples; lazy in real code.

**Variant** — One of the cases an enum can take. `Some` and `None` are variants of `Option`.

**`Vec<T>`** — A growable, heap-allocated array of `T`s. The default list type in Rust.

**WAV** — A simple audio file format. A short header followed by raw audio samples. We build one from scratch in Session 18.

**Waveform** — The shape of an audio signal over time. Sine, square, sawtooth, and triangle are the classical four.
