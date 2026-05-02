# Session 3: Functions, Expressions, and Basic I/O

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

## What You'll Learn

How to break code into reusable **functions**, the difference between **expressions** and **statements** (a Rust quirk that's actually wonderful once you see it), and how to read input from the keyboard.

## The Big Idea

Most things in Rust are **expressions** — they produce a value. `5`, `5 + 3`, `if x > 0 { 1 } else { -1 }`, even an entire block `{ let a = 1; let b = 2; a + b }` are all expressions. The few things that aren't (like `let` bindings) are **statements** and end in a semicolon. This is why Rust functions don't usually need a `return` keyword: the **last expression in the function body** is the return value, full stop.

## Concepts Covered

- `fn name(param: Type) -> ReturnType { ... }`
- Expressions vs statements
- Implicit return (no semicolon on the last line)
- `std::io::stdin()` for keyboard input
- `.parse()` and `.expect()` for converting strings to numbers
- Building a useful program from small functions

## Building Towards `music-theory-cli`

The mini-project is a CLI tool that reads input and prints results. We'll start by writing a function that takes a MIDI note number and returns the corresponding frequency in Hz — your keyboard sends exactly these numbers when you press a key, so this is the literal foundation of the final-month synthesiser too.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-1/session-03/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-1/session-03/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(2, 3);
    println!("2 + 3 = {}", sum);
}
```

Look at `add`:

- `fn add` — declare a function called `add`
- `(a: i32, b: i32)` — two parameters, both 32-bit signed integers (you must annotate parameter types)
- `-> i32` — it returns an `i32`
- `a + b` — the function body. **Note:** no semicolon, no `return` keyword. The last expression is the value.

If you wrote `a + b;` (with a semicolon), the compiler would yell at you, because that turns the expression into a statement, and statements have no value:

```
error[E0308]: mismatched types
expected `i32`, found `()`
```

`()` is the **unit type**, the equivalent of "nothing". A function that doesn't return anything implicitly returns `()`.

### 2. Why no `return`?

You *can* use `return`:

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

It works. But idiomatic Rust uses implicit return for the final expression because it composes much better in larger expressions:

```rust
fn classify(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    }
}
```

The whole `if`/`else if`/`else` chain is one expression. Each branch is one expression. The function body is one expression. No `return` anywhere.

### 3. The frequency formula

The frequency of MIDI note number `n` is:

$$ f(n) = 440 \times 2^{(n - 69)/12} $$

In Rust:

```rust
fn midi_to_frequency(note: u8) -> f64 {
    440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0)
}
```

A few things to notice:

- The parameter `note: u8` accepts any value 0–255 (and MIDI notes are 0–127, so it fits).
- `note as f64` casts the byte to a float so the maths works.
- `2.0_f64.powf(...)` says "raise 2.0 (as `f64`) to the power of ...".

Sanity check: for `note = 69` (A4) we expect 440. For `note = 60` (middle C) we expect ~261.63.

### 4. Reading input

```rust
use std::io;

fn main() {
    println!("Enter a MIDI note number (0-127):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let note: u8 = input.trim().parse().expect("Please enter a number");
    let freq = midi_to_frequency(note);
    println!("MIDI {} = {:.2} Hz", note, freq);
}

fn midi_to_frequency(note: u8) -> f64 {
    440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0)
}
```

Step by step:

- `use std::io;` — pulls the `io` module into scope so we can write `io::stdin()` instead of `std::io::stdin()`.
- `String::new()` — an empty, growable string. We're going to append the user's typed line to it.
- `read_line(&mut input)` — reads one line and writes it into `input`. The `&mut` means "I want to mutate `input` through this reference". (The borrow checker stuff is coming in Month 2.)
- `.expect("...")` — the read might fail (rare, but possible). `.expect()` says "if it fails, crash with this message". For real programs we'd handle the error properly; for now it's fine.
- `input.trim()` — strips the trailing newline.
- `.parse()` — converts a `&str` to a number. The type Rust converts to is determined by the target type (`u8` here, on the left).

### 5. Putting it together

The complete example is in `examples/midi_to_freq/`. Try a few values:

```
$ cargo run
Enter a MIDI note number (0-127):
69
MIDI 69 = 440.00 Hz

$ cargo run
Enter a MIDI note number (0-127):
60
MIDI 60 = 261.63 Hz
```

That second one is **middle C**. You just wrote a real piece of audio engineering.

---

## Common Mistakes

### ❌ Forgetting `mut` on the input buffer

```rust
let input = String::new();
io::stdin().read_line(&mut input).expect("oops");   // 💥
```

```
error[E0596]: cannot borrow `input` as mutable, as it is not declared as mutable
```

**Fix:** `let mut input = String::new();`. `read_line` writes into the string, so it needs a mutable reference.

### ❌ Adding a semicolon to the return expression

```rust
fn double(x: i32) -> i32 {
    x * 2;       // 💥 returns () instead of i32
}
```

**Fix:** drop the semicolon. (Or write `return x * 2;`.)

### ❌ Forgetting to `.trim()` before parsing

```rust
let n: u8 = input.parse().expect("...");   // 💥 includes "\n"
```

The line read from stdin includes the trailing newline. **Fix:** `input.trim().parse()`.

### ❌ Calling `parse()` without telling Rust what type

```rust
let n = input.trim().parse().expect("...");   // 💥
```

```
error[E0282]: type annotations needed
```

`parse()` is generic — it can produce many types. **Fix:** annotate the variable: `let n: u8 = ...` or use the turbofish: `input.trim().parse::<u8>().expect(...)`.

---

## Session Challenge

Extend `examples/midi_to_freq` so that it:

1. Prompts repeatedly until the user types `quit`.
2. Validates that the number is between 0 and 127, and prints a friendly error otherwise (don't crash).
3. Bonus: print the **note name** alongside (`MIDI 60 = C4 = 261.63 Hz`). You'll need to map `note % 12` to a name. Try writing this with a `match` — that's exactly what we'll cover in Session 5.

---

## Quick Reference

| Concept | Syntax |
|---|---|
| Define function | `fn name(p: T) -> R { body }` |
| Implicit return | last expression, no `;` |
| Explicit return | `return value;` |
| No return value | `fn name() { ... }` (returns `()`) |
| Power | `x.powf(y)` |
| Cast | `x as T` |
| Read line | `io::stdin().read_line(&mut s)` |
| Parse | `s.trim().parse::<T>()` |

---

## Further Reading

Curated extra material on the topics covered in this session (Functions, Expressions, I/O). All free; all current as of writing.

- [**The Rust Book** — *How Functions Work* (3.3)](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — Includes the all-important *expressions vs statements* distinction.
- [**The Rust Book** — *Programming a Guessing Game* (chapter 2)](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) — Builds the same kind of stdin-reading toy you wrote today, and adds error handling — well worth a look.
- [**`std::io` documentation**](https://doc.rust-lang.org/std/io/) — The standard library's I/O primitives. Skim the *Read* and *BufRead* traits.
- [**Rust by Example** — *Expressions*](https://doc.rust-lang.org/rust-by-example/expression.html) — Short and sharp; great refresher.

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

> 📝 Session 3 done. Five minutes in [`dfe/session-log.md`](../../dfe/session-log.md). What's the most surprising thing about expressions vs statements?
