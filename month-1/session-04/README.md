# Session 4: Control Flow

## What You'll Learn

How to make decisions (`if`/`else`), how to repeat (`loop`, `while`, `for`), and how Rust loops can return values. We'll print an ASCII piano keyboard to put it all together.

## The Big Idea

Rust's control-flow constructs (`if`, `loop`, `match`) are **expressions**. That means they produce values, which means you can write things like `let x = if cond { 1 } else { 2 };` directly. Most languages either can't do this or have a separate "ternary" syntax for it. Rust just lets you use the same constructs everywhere.

## Concepts Covered

- `if` / `else if` / `else` (and `if` as an expression)
- `loop` — infinite loop with `break` (and `break value` to return)
- `while` — loop while a condition holds
- `for x in iter` — the workhorse loop
- Ranges: `0..10` (exclusive end) and `0..=10` (inclusive end)
- `break` and `continue`

## Building Towards `music-theory-cli`

The project doesn't need infinite loops, but it does need to iterate over the seven notes of a scale and decide what chord quality each one has. That's a `for` loop and an `if`/`match`. We'll practise both today.

---

## Step-by-Step Walkthrough

### 1. `if` is an expression

```rust
fn main() {
    let velocity = 72;
    let dynamic = if velocity > 100 {
        "loud"
    } else if velocity > 60 {
        "medium"
    } else {
        "soft"
    };
    println!("Velocity {} → {}", velocity, dynamic);
}
```

That whole `if`/`else if`/`else` produces a `&'static str`. We bind that value to `dynamic`. Note **no semicolons** on the values inside each branch — those are the expressions being returned by each branch. Every branch must produce the same type.

### 2. `for` over a range

```rust
for i in 0..5 {
    println!("{}", i);   // 0, 1, 2, 3, 4
}

for i in 1..=4 {
    println!("{}", i);   // 1, 2, 3, 4 (note the =)
}
```

`0..5` is a half-open range (excludes 5). `0..=4` is an inclusive range. Pick whichever reads better.

### 3. `for` over a slice

```rust
let names = ["C", "D", "E", "F", "G", "A", "B"];
for name in names {
    print!("{} ", name);
}
println!();
```

`for name in names` calls `.into_iter()` on the array under the hood. We'll meet iterators properly in Session 12.

### 4. `loop` and `break value`

`loop` is an infinite loop. The neat trick: you can `break` out of it with a value, and the whole `loop` becomes an expression that produces that value:

```rust
let mut n = 1;
let first_power_of_two_over_1000 = loop {
    if n > 1000 {
        break n;
    }
    n *= 2;
};
println!("{}", first_power_of_two_over_1000);   // 1024
```

Useful when you're searching for something and the loop body is more complex than a `while` condition can express.

### 5. `while`

```rust
let mut frequency = 27.5_f64;       // A0
while frequency < 5000.0 {
    println!("{:>7.2} Hz", frequency);
    frequency *= 2.0;
}
```

Prints every A from A0 to A7 by repeatedly doubling.

### 6. `continue` and `break`

```rust
for i in 1..=20 {
    if i % 2 == 0 {
        continue;          // skip even numbers
    }
    if i > 15 {
        break;             // stop after 15
    }
    println!("{}", i);     // prints 1, 3, 5, 7, 9, 11, 13, 15
}
```

### 7. The piano keyboard project

The pattern of black and white keys repeats every octave: positions `0, 2, 4, 5, 7, 9, 11` (C, D, E, F, G, A, B) within an octave are white; the rest (`1, 3, 6, 8, 10` — C#, D#, F#, G#, A#) are black.

Goal: print a 2-octave ASCII keyboard, with `W` for white keys and `B` for black keys, and the note name underneath.

```rust
fn main() {
    let names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let octaves = 2;
    let total_semitones = octaves * 12;

    print!("Keys:  ");
    for i in 0..total_semitones {
        let in_octave = i % 12;
        let is_black = matches!(in_octave, 1 | 3 | 6 | 8 | 10);
        if is_black {
            print!("B  ");
        } else {
            print!("W  ");
        }
    }
    println!();

    print!("Notes: ");
    for i in 0..total_semitones {
        let in_octave = i % 12;
        print!("{:<3}", names[in_octave]);
    }
    println!();
}
```

Output:

```
Keys:  W  B  W  B  W  W  B  W  B  W  B  W  W  B  W  B  W  W  B  W  B  W  B  W
Notes: C  C# D  D# E  F  F# G  G# A  A# B  C  C# D  D# E  F  F# G  G# A  A# B
```

Two new things in there:

- **`matches!(value, pattern)`** is a macro that returns `true` if `value` matches the pattern. We'll cover patterns properly next session — this is a sneak peek.
- **`print!`** (without the `ln`) doesn't add a newline. Useful for building up a row of characters before flushing the line.

The complete project is in `examples/piano_keyboard/`.

---

## Common Mistakes

### ❌ Putting parentheses around the `if` condition

```rust
if (x > 0) { ... }    // ❌ allowed but not idiomatic
```

The compiler will warn you. Drop the parentheses: `if x > 0 { ... }`.

### ❌ Mismatched branch types

```rust
let x = if cond { 1 } else { "hello" };   // 💥
```

```
error[E0308]: `if` and `else` have incompatible types
```

Both branches must produce the same type. **Fix:** make them match.

### ❌ Forgetting that `0..10` excludes 10

```rust
for i in 0..10 {
    println!("{}", i);   // 0..9, not 0..10
}
```

If you wanted 0–10 inclusive, write `0..=10`.

### ❌ Infinite loop you can't break out of

```rust
loop {
    println!("forever");
    // no break, no return
}
```

Press `Ctrl+C` to kill it. Then add a `break` condition.

---

## Session Challenge

Extend `examples/piano_keyboard` to:

1. Print the keyboard for **all 88 keys** of a real piano (from A0 to C8).
2. Add a third row showing the **MIDI note number** under each key. (A0 is MIDI 21, C8 is MIDI 108.)
3. Mark **middle C** (MIDI 60) with an arrow or asterisk so it's easy to find.

---

## Quick Reference

| Concept | Syntax |
|---|---|
| `if` expression | `let x = if cond { a } else { b };` |
| Range exclusive | `0..10` |
| Range inclusive | `0..=10` |
| `for` loop | `for x in iter { ... }` |
| `while` loop | `while cond { ... }` |
| Infinite loop | `loop { ... }` |
| Loop with value | `let v = loop { break 42; };` |
| Skip iteration | `continue;` |
| Exit loop | `break;` |
| Pattern check | `matches!(value, pattern)` |

---

## DofE Log Reminder

> 📝 Session 4 done. Open [`dfe/session-log.md`](../../dfe/session-log.md) and capture the session. Did the keyboard render correctly the first time, or did you get the black/white pattern wrong? That's worth noting.
