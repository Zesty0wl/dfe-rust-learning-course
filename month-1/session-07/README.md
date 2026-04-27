# Session 7: Mini-Project Build Part 1 — Scale Generator

This is a **project session**, not a teaching session. There's no new material — today is about putting together everything from Sessions 1–6 into a working tool.

## Goal

By the end of today, `month-1/project/music-theory-cli/` should:

- Accept command-line arguments for the **root note** and **scale type** (major, minor, or pentatonic-major).
- Parse those arguments into your `NoteName` and `ScaleType` enums.
- Print the notes of the scale.

Target output:

```
$ cd month-1/project/music-theory-cli
$ cargo run -- --root C --scale major
Scale: C Major
Notes: C  D  E  F  G  A  B  C
```

(The trailing C — the octave — is optional. We'll add intervals and chord progressions tomorrow.)

---

## Approach

A `starter/` skeleton is provided in [`../../project/music-theory-cli/starter/`](../../project/music-theory-cli/starter/). Open it, work through the TODOs, and don't peek at the [`solution/`](../../project/music-theory-cli/solution/) until you've genuinely tried.

### Suggested project structure

For Month 1, a single `main.rs` is fine. We'll restructure into modules in Session 17.

### Parsing CLI arguments without external crates

For now, use `std::env::args()`. It returns an iterator over the program's argument strings (the first one is the program name itself).

```rust
let args: Vec<String> = std::env::args().collect();
// args[0] is the program path
// args[1..] is what the user passed
```

The user will pass arguments like `--root C --scale major`. The simplest approach: walk the arguments two at a time and pull out the values you recognise. A more elegant approach would use the `clap` crate, but we deliberately defer that to Month 3 — for now the goal is to feel the value `clap` provides by writing the manual version yourself.

### Mapping a string to a `NoteName`

This is the spot where a `match` is essential. Be sure to handle:

- Both `#` notation (`C#`) and `b` notation (`Db`) — they refer to the same pitch.
- Case insensitivity if you can manage it (`c`, `C`, and `c#` all valid).

### Mapping a string to a `ScaleType`

```rust
fn parse_scale(s: &str) -> Option<ScaleType> {
    match s.to_lowercase().as_str() {
        "major" | "maj"             => Some(ScaleType::Major),
        "minor" | "min" | "natural-minor" => Some(ScaleType::NaturalMinor),
        "pentatonic" | "pentatonic-major" => Some(ScaleType::PentatonicMajor),
        _ => None,
    }
}
```

We're using `Option` here as a "could fail" return — full coverage in Session 10.

### Reusing your scales code

Copy the `scale_notes`, `semitone_pattern`, `NoteName`, and `ScaleType` from `examples/scales_intro` in Session 6. That gives you the engine; today is mostly wiring it up.

---

## Definition of done

- [ ] `cargo run -- --root C --scale major` prints the C major scale.
- [ ] `cargo run -- --root A --scale minor` prints the A natural minor scale.
- [ ] `cargo run -- --root G --scale pentatonic` prints the G pentatonic-major scale.
- [ ] If the user passes a missing or malformed argument, the program prints a friendly usage message and exits.
- [ ] The output is reasonably formatted (consistent column widths).

If all six checkboxes are ticked, you're done for today.

---

## If you get stuck

- Look at the [`solution/`](../../project/music-theory-cli/solution/) — but try for at least 30 minutes first.
- Re-read Session 6 — particularly the `scale_notes` walkthrough.
- Print debug info with `dbg!(value)` — it's like `println!("{:?}", value)` but more useful.

---

## DofE Log Reminder

> 📝 Project sessions are when you really see your skills land. In your [`dfe/session-log.md`](../../dfe/session-log.md), record what worked, what didn't, what you had to look up. The "what was hard" field is especially valuable on project days — these are the moments your future self (and your assessor) will most appreciate seeing.
