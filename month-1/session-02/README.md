# Session 2: Variables, Types, and Mutability

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

## What You'll Learn

How Rust thinks about data: declaring variables, choosing types, when (and how) values are allowed to change, and what a "12th root of two" has to do with the white key next to A on a piano.

## The Big Idea

Rust gives every variable two properties at birth: a **[type](../../GLOSSARY.md#type--type-system)** (what kind of thing it is) and a **mutability** (whether you're allowed to change it). Most other languages make everything [mutable](../../GLOSSARY.md#mutable) by default. Rust flips that. **[Immutability](../../GLOSSARY.md#immutable) is the default**, and you opt into mutation with `mut`. This sounds restrictive — and that's the point. The compiler can reason much more confidently about code that doesn't randomly change underneath it.

## Concepts Covered

- `let` and `let mut`
- Scalar types: `i32`, `u64`, [`f64`](../../GLOSSARY.md#floating-point-number-f32-f64), `bool`, `char`
- Type inference vs explicit annotations
- **Shadowing** — a Rust-specific trick that looks like mutation but isn't
- [Integer overflow](../../GLOSSARY.md#integer-overflow) in debug vs release builds
- Numeric literals: `1_000_000`, `0xff`, `0b1010`, `3.14_f64`

## Building Towards `music-theory-cli`

The mini-project hinges on storing note names, MIDI numbers, and frequencies. Today we cover the types you'll use to do that — and we'll introduce the **frequency formula for musical notes**, which we use again in Session 3, Session 18, and the entire final project.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-1/session-02/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-1/session-02/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. `let` is a binding, not an assignment

```rust
fn main() {
    let a4_frequency = 440.0;
    println!("A4 is {} Hz", a4_frequency);
}
```

Run it: `cargo run`. You'll see `A4 is 440 Hz`.

What's the type of `a4_frequency`? Rust **[inferred](../../GLOSSARY.md#type-inference)** it from the literal `440.0` — that decimal point makes it an `f64` (a 64-bit floating-point number). If you want to spell it out:

```rust
let a4_frequency: f64 = 440.0;
```

### 2. Variables are immutable by default

Try this:

```rust
fn main() {
    let frequency = 440.0;
    frequency = 880.0;            // 💥
    println!("{}", frequency);
}
```

```
error[E0384]: cannot assign twice to immutable variable `frequency`
  |
2 |     let frequency = 440.0;
  |         --------- first assignment to `frequency`
3 |     frequency = 880.0;
  |     ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
```

The compiler is your friend here. It's telling you exactly what's wrong and pointing at both relevant lines.

### 3. Opt in with `mut`

```rust
fn main() {
    let mut frequency = 440.0;
    println!("Starting at {} Hz", frequency);
    frequency *= 2.0;
    println!("One octave up: {} Hz", frequency);
}
```

Output:

```
Starting at 440 Hz
One octave up: 880 Hz
```

The rule: doubling the frequency raises a note by exactly one octave. That's just how human hearing works — and now you know why a piano has the same key pattern repeating.

### 4. The chromatic scale

In Western music there are **12 semitones in an octave**, and the frequency ratio between adjacent semitones is the **twelfth root of 2**. Let's print every semitone from A4 (440 Hz) up an octave:

```rust
fn main() {
    let a4 = 440.0_f64;
    let ratio = 2.0_f64.powf(1.0 / 12.0);
    let mut frequency = a4;

    let names = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

    for name in names {
        println!("{:>2}: {:>7.2} Hz", name, frequency);
        frequency *= ratio;
    }

    println!("A5: {:>7.2} Hz (should be ~880)", frequency);
}
```

This is a complete, runnable example — see `examples/chromatic_scale/`.

> The maths: `2^(1/12) ≈ 1.05946`. Multiply twelve times and you get exactly 2. Twelve semitones, one octave. Elegant.

### 5. Shadowing — Rust's trick that looks like mutation

What if you want to "change" a variable's type? You can't — but you *can* re-declare it with the same name:

```rust
let midi_note = "69";                  // a &str (text)
let midi_note: u8 = midi_note.parse().unwrap();   // now a u8 (number)
println!("MIDI note: {}", midi_note);
```

This isn't mutation. It's a brand-new variable that happens to share the name. The old `midi_note` is gone after the second `let`. Useful when parsing input or transforming a value through stages.

### 6. Integer types matter

```rust
let small: u8 = 200;
let bigger: u8 = small + 100;     // 💥 overflow!
```

`u8` holds values 0–255. `200 + 100` is 300, which doesn't fit. **In debug builds (`cargo run`)**, Rust panics with a clear error. **In release builds (`cargo run --release`)**, Rust wraps around for performance — `300 % 256 = 44`. That's a foot-gun if you're not aware. Pick types with enough room.

For most general arithmetic, prefer `i64` (signed) or `u64` (unsigned). Use `u8` when you genuinely mean a byte (like a MIDI note number, which ranges 0–127).

### 7. Booleans and characters

```rust
let is_playing: bool = true;
let key_symbol: char = '♪';     // single Unicode scalar, single quotes
println!("{} is_playing? {}", key_symbol, is_playing);
```

`char` in Rust is **4 bytes** and holds any Unicode code point — including emoji and music symbols.

### 8. Numeric literals are flexible

```rust
let million = 1_000_000;          // underscores for readability
let hex = 0xff;                   // 255
let binary = 0b1010;              // 10
let pi = 3.14159_f64;             // type suffix
let velocity: u8 = 127;
```

Underscores are ignored by the compiler — they're just for your eyes. Prefer them for big numbers.

---

## Common Mistakes

### ❌ Forgetting `mut`

```rust
let counter = 0;
counter += 1;   // 💥
```

**Fix:** `let mut counter = 0;`. Or if you only "change" it once (e.g. to convert types), use shadowing: `let counter = counter + 1;`.

### ❌ Confusing shadowing with mutation

```rust
let x = 5;
let x = "hello";       // ✅ shadowing, type can change
// vs
let mut x = 5;
x = "hello";           // 💥 cannot change type via mutation
```

**Rule:** `mut` means "this binding can be reassigned to **a value of the same type**". Shadowing means "I'm declaring a brand-new variable that happens to share a name."

### ❌ Mixing integer types in arithmetic

```rust
let a: i32 = 5;
let b: i64 = 10;
let c = a + b;   // 💥 mismatched types
```

Rust does not silently promote types. **Fix:** `let c = a as i64 + b;` or pick the same type for both.

### ❌ Treating `f64` like `i32`

```rust
let n = 10 / 3;          // i32 division → 3
let n = 10.0 / 3.0;      // f64 division → 3.3333…
let n = 10 / 3.0;        // 💥 mismatched types
```

The literal type matters. `10` is an integer; `10.0` is a float.

---

## Session Challenge

Modify `examples/chromatic_scale` to:

1. Print **two full octaves** of frequencies from A3 (220 Hz) to A5 (880 Hz).
2. Add a column showing the **MIDI note number** for each note. (Hint: A4 is MIDI 69. Each semitone up adds 1.)
3. Highlight the `A` notes (A3, A4, A5) in some way — maybe with a `*` next to them.

> No solution provided for this one — try it yourself. If you get stuck, peek at how `for` loops and `if` work in the cheat sheet ([`resources/cheatsheet.md`](../../resources/cheatsheet.md)).

---

## Quick Reference

| Concept | Syntax |
|---|---|
| Immutable binding | `let x = 5;` |
| Mutable binding | `let mut x = 5;` |
| Type annotation | `let x: i32 = 5;` |
| Shadowing | `let x = x + 1;` |
| Float literal | `1.0`, `3.14_f64` |
| Big integer | `1_000_000` |
| Hex / binary | `0xff`, `0b1010` |
| Power | `2.0_f64.powf(1.0/12.0)` |
| Cast | `let n = 5 as f64;` |

---

## Further Reading

Curated extra material on the topics covered in this session (Variables, Types, Mutability). All free; all current as of writing.

- [**The Rust Book** — *Variables and Mutability* (3.1)](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) — The canonical explanation, with all the edge cases.
- [**The Rust Book** — *Data Types* (3.2)](https://doc.rust-lang.org/book/ch03-02-data-types.html) — Every scalar and compound type Rust ships with.
- [**Rust by Example** — *Variable bindings*](https://doc.rust-lang.org/rust-by-example/variable_bindings.html) — Tiny runnable snippets for every concept, side by side with explanations.
- [**What Every Programmer Should Know About Floating-Point**](https://floating-point-gui.de) — A friendly version of the famous Goldberg paper. Read this before you ever compare two `f64`s with `==`.

---
## DofE Log Reminder

> 📝 Session 2 done. Open [`dfe/session-log.md`](../../dfe/session-log.md), find the Session 2 block, and fill it in. What clicked? What didn't? Five minutes, while it's fresh.
