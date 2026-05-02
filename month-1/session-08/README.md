# Session 8: Mini-Project Build Part 2 — Chord Progressions and Polish

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

Second project session. Today you complete `music-theory-cli` and earn **Milestone 1** of your DofE evidence pack.

## Goal

By the end of today, the tool should:

- Print the notes (from yesterday).
- Print the **interval pattern** (W = whole step, H = half step).
- Print the **diatonic chord progression** (the seven chords built on each scale degree).
- Print a friendly error if the user passes a bad input.
- (Optional, stretch) Use coloured output via the `colored` crate.

Target output:

```
$ cargo run -- --root C --scale major
Scale: C Major
Notes:     C    D    E    F    G    A    B
Intervals: W    W    H    W    W    W    H
Chords:    Cmaj Dmin Emin Fmaj Gmaj Amin Bdim
```

---

> 💡 **Where to work today.** This is a project session, so you'll be inside the project folder, not the session folder. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-1/project/music-theory-cli/starter        # your work-in-progress
> cargo run -- <args>
> ```
>
> The reference implementation lives in `month-1/project/music-theory-cli/solution/` — peek only when you're properly stuck. All `cargo run` commands shown below assume you're inside `month-1/project/music-theory-cli/starter/`.

## Approach

### 1. Intervals

A whole step is 2 semitones, a half step is 1. So you can map the **semitone pattern** straight into a sequence of `W`s and `H`s:

```rust
fn intervals_for(scale: &ScaleType) -> Vec<&'static str> {
    semitone_pattern(scale)
        .iter()
        .map(|&n| if n == 1 { "H" } else { "W" })
        .collect()
}
```

(Quick `iter()` / `map()` / `collect()` preview — full coverage in Session 12.)

### 2. Diatonic chord qualities

The chords built on each scale degree of a **major** scale follow this pattern:

| Degree | I | II | III | IV | V | VI | VII |
|---|---|---|---|---|---|---|---|
| Quality | maj | min | min | maj | maj | min | dim |

For a **natural minor** scale:

| Degree | i | ii | III | iv | v | VI | VII |
|---|---|---|---|---|---|---|---|
| Quality | min | dim | maj | min | min | maj | maj |

For a pentatonic-major scale, classical diatonic chord theory doesn't strictly apply (only 5 notes), so just print the major triad on each note as a reasonable simplification.

```rust
fn chord_qualities(scale: &ScaleType) -> &'static [&'static str] {
    match scale {
        ScaleType::Major           => &["maj", "min", "min", "maj", "maj", "min", "dim"],
        ScaleType::NaturalMinor    => &["min", "dim", "maj", "min", "min", "maj", "maj"],
        ScaleType::PentatonicMajor => &["maj", "maj", "maj", "maj", "maj"],
    }
}
```

### 3. Error handling

For now, if a parsing function returns `None`, print a usage message and exit:

```rust
fn print_usage_and_exit() -> ! {
    eprintln!("Usage: music-theory-cli --root <NOTE> --scale <major|minor|pentatonic>");
    eprintln!("Examples:");
    eprintln!("  music-theory-cli --root C --scale major");
    eprintln!("  music-theory-cli --root F# --scale minor");
    std::process::exit(1);
}
```

Two new things:

- `eprintln!` writes to **stderr** instead of stdout. That's the convention for error messages.
- `std::process::exit(1)` exits with a non-zero status code, signalling failure to whatever shell or script ran us.
- `-> !` is the **never type**. It tells the compiler "this function never returns". The compiler can then accept it in expression positions where any other type is needed (because it never produces *any* value, it can stand in for any type).

### 4. (Stretch) Coloured output

This is your first taste of using an external crate. Add to `Cargo.toml`:

```toml
[dependencies]
colored = "2.1"
```

Then in `main.rs`:

```rust
use colored::Colorize;

println!("Scale: {} {}", root_str.bold().cyan(), scale_str.bold().yellow());
```

`Cargo.toml` dependencies and `cargo add` get full coverage in Session 17 — this is just a peek.

---

## Definition of done

- [ ] All Session 7 criteria still pass.
- [ ] Intervals row is printed.
- [ ] Chord row is printed using the correct qualities for major and minor.
- [ ] Bad input shows a usage message instead of crashing.
- [ ] The whole solution lives in `month-1/project/music-theory-cli/` with a `Cargo.toml` and a `README.md` describing how to run it.

If all five checkboxes are ticked: **Milestone 1 reached. 🎉**

---

## After you finish

1. Make sure the working code is committed (if you're using git).
2. Open [`dfe/milestone-1-reflection.md`](../../dfe/milestone-1-reflection.md) and complete it.
3. Take a break. You just shipped a working program.

---

## Further Reading

Curated extra material on the topics covered in this session (Project — Chord Progressions and Polish (part 2)). All free; all current as of writing.

- [**Wikipedia — Diatonic functional harmony**](https://en.wikipedia.org/wiki/Diatonic_function) — The theory behind 'I-vi-IV-V' and friends — the chord-quality rules we're encoding.
- [**`colored` crate documentation**](https://docs.rs/colored/latest/colored/) — ANSI terminal colours with a friendly API. Used for the chord highlighting.
- [**Cargo Book — Specifying dependencies**](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) — Everything you can put in `Cargo.toml` for a dependency: versions, features, git, path, dev-dependencies.
- [**`crates.io`** — search the Rust package registry](https://crates.io) — Where every crate lives. Download counts and last-updated dates are useful when picking between options.

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

> 📝 Session 8 done — and **Milestone 1 reached**. Two things to do in `dfe/`: fill in your Session 8 log entry in [`dfe/session-log.md`](../../dfe/session-log.md), and complete [`dfe/milestone-1-reflection.md`](../../dfe/milestone-1-reflection.md). The milestone reflection is the more substantial one — block out 15 minutes for it.
