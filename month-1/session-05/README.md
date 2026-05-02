# Session 5: Pattern Matching and `match`

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

## What You'll Learn

How to use `match` — Rust's most powerful control-flow construct. By the end of today you'll have written the function that maps any MIDI note number to its note name, which is a literal building block of the mini-project.

## The Big Idea

`match` is like a `switch` from C or Java, but with three superpowers:

1. **It's an expression** — like `if`, it produces a value.
2. **It's exhaustive** — the compiler refuses to compile if you've missed a possible case. No more "I forgot to handle that" bugs.
3. **It's pattern-based** — you can match on shapes, ranges, multiple values, and apply guard conditions, not just equality.

It's the feature that, once you have it, you miss in every other language.

## Concepts Covered

- The `match` expression
- Exhaustiveness checking
- Match arms with `=>`
- The `_` wildcard
- Multiple patterns with `|`
- Range patterns: `0..=9`
- Guards: `n if n > 5`
- `match` as an expression

## Building Towards `music-theory-cli`

The mini-project takes a root note (`C`, `D#`, `Bb`, etc.) and produces a scale. We need a function that maps a note name to a number, and another that maps a number back to a name. Both are textbook `match` problems.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-1/session-05/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-1/session-05/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. The simplest `match`

```rust
fn main() {
    let n = 3;
    let word = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    println!("{} → {}", n, word);
}
```

Things to spot:

- Each **arm** has the form `pattern => value,`
- The **`_`** is the catch-all. Without it (or another arm covering all remaining values) the compiler refuses to compile, because `match` must be exhaustive.
- The whole `match` is an expression, so we can `let word = match n { ... };`.

### 2. Multiple values in one arm

```rust
let in_octave = 6;
let kind = match in_octave {
    0 | 2 | 4 | 5 | 7 | 9 | 11 => "white key",
    1 | 3 | 6 | 8 | 10         => "black key",
    _                          => "out of range",
};
println!("{}", kind);
```

The `|` (pipe) means "or". This is much cleaner than a long `if`/`else if` chain.

### 3. Range patterns

```rust
let velocity = 92;
let dynamic = match velocity {
    0..=20    => "ppp (very quiet)",
    21..=40   => "p (quiet)",
    41..=70   => "mf (medium)",
    71..=100  => "f (loud)",
    101..=127 => "ff (very loud)",
    _         => "out of MIDI range",
};
println!("velocity {} → {}", velocity, dynamic);
```

Range patterns use **inclusive** ranges (`..=`). You can't use `..` (exclusive) in patterns — that's a separate construct for slices.

### 4. Match guards

```rust
let temperature = 22;
let mood = match temperature {
    t if t < 0   => "freezing",
    t if t < 15  => "cold",
    t if t < 25  => "pleasant",
    _            => "hot",
};
```

A guard is a boolean condition added to an arm with `if`. Useful when the pattern alone can't express what you want.

### 5. The MIDI-to-name function

The 12 semitones in an octave have names:

| `note % 12` | Name |
|---|---|
| 0 | C |
| 1 | C# |
| 2 | D |
| 3 | D# |
| 4 | E |
| 5 | F |
| 6 | F# |
| 7 | G |
| 8 | G# |
| 9 | A |
| 10 | A# |
| 11 | B |

In Rust:

```rust
fn note_name(midi: u8) -> &'static str {
    match midi % 12 {
        0  => "C",
        1  => "C#",
        2  => "D",
        3  => "D#",
        4  => "E",
        5  => "F",
        6  => "F#",
        7  => "G",
        8  => "G#",
        9  => "A",
        10 => "A#",
        11 => "B",
        _  => unreachable!(),   // u8 % 12 is always 0..=11
    }
}
```

> **Why the `_`?** `match` is exhaustive over the *type* of the matched value. `u8 % 12` is always 0..=11 mathematically — but the compiler only knows it's a `u8` (range 0..=255). So we add `_ => unreachable!()` to satisfy exhaustiveness. `unreachable!()` is a macro that panics if it ever runs — perfect for "this should never happen".

### 6. Adding the octave number

MIDI note numbering puts middle C (`C4`) at 60. Each octave is 12 numbers. So the octave for MIDI note `n` is `(n / 12) - 1`. Putting it together:

```rust
fn full_note_name(midi: u8) -> String {
    let name = note_name(midi);
    let octave = (midi as i32 / 12) - 1;
    format!("{}{}", name, octave)
}

fn main() {
    for midi in [21u8, 60, 69, 108] {
        println!("MIDI {:>3} = {}", midi, full_note_name(midi));
    }
}
```

Output:

```
MIDI  21 = A0
MIDI  60 = C4
MIDI  69 = A4
MIDI 108 = C8
```

Lowest note on a piano (A0), middle C (C4), concert pitch (A4), highest note on a piano (C8). All correct.

The complete example is in `examples/midi_note_names/`.

---

## Common Mistakes

### ❌ Forgetting the catch-all on a non-exhaustive type

```rust
let n: u32 = 5;
let s = match n {
    1 => "one",
    2 => "two",
};   // 💥
```

```
error[E0004]: non-exhaustive patterns: `0_u32`, `3_u32..=u32::MAX` not covered
```

**Fix:** add `_ => "other"`.

### ❌ Using `..` instead of `..=` in a range pattern

```rust
match n {
    0..10 => ...,   // 💥
}
```

**Fix:** `0..=10`. (As of Rust 1.80, `..` exclusive ranges in patterns are stable too — but `..=` has been supported since the beginning and is what you'll see in most code.)

### ❌ Falling through (you can't)

```rust
match n {
    1 => println!("one"),
    2 | 3 => {            // ✅ this is correct: matches 2 or 3
        println!("two or three");
    }
    _ => {}
}
```

There's no fall-through like in C. If you want one arm to handle multiple values, list them with `|`.

### ❌ Comma after the catch-all is *required*

```rust
match n {
    1 => "one",
    _ => "other"
}   // 💥 missing comma somewhere — can be confusing
```

Trailing commas are optional after `}`-bodied arms but required after `value,`-style arms. When in doubt, add one.

---

## Session Challenge

Write a function `parse_note_name(name: &str) -> Option<u8>` that returns the MIDI note number for a given input like `"C4"`, `"A4"`, `"F#5"`, `"Bb3"`. Use `match` extensively. Bonus: support both `#` and `b` (flat) notation. (`Bb3` should return the same value as `A#3`.)

> Hint: split the input into a "letter part" and a "number part". You don't know `Option` properly yet — Session 10 covers it — so for now, you can return 255 for invalid input and accept that's a hack.

---

## Quick Reference

| Concept | Syntax |
|---|---|
| Basic `match` | `match v { p => e, _ => default }` |
| Multiple patterns | `1 \| 2 \| 3 => ...` |
| Range pattern | `0..=9 => ...` |
| Guard | `n if n > 5 => ...` |
| Catch-all | `_ => ...` |
| Match as expression | `let x = match ...;` |
| "Should never happen" | `_ => unreachable!()` |

---

## Further Reading

Curated extra material on the topics covered in this session (Pattern matching and `match`). All free; all current as of writing.

- [**The Rust Book** — *The `match` Control Flow Construct* (6.2)](https://doc.rust-lang.org/book/ch06-02-match.html) — The whole story, with the famous Option example.
- [**The Rust Book** — *Patterns and Matching* (chapter 18)](https://doc.rust-lang.org/book/ch18-00-patterns.html) — Everywhere patterns appear in Rust — `let`, function args, `if let`, `while let`. Read this once and you'll see them everywhere.
- [**Rust by Example** — *match*](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) — Bite-sized variants, including ranges and guards.
- [**The Rust Reference** — *Pattern syntax*](https://doc.rust-lang.org/reference/patterns.html) — The exhaustive grammar. Reach for this when something unusual works (or refuses to).

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

> 📝 Session 5 done. `match` is one of the most loved features of Rust — was it intuitive? What did you build with it? Five minutes in [`dfe/session-log.md`](../../dfe/session-log.md).
