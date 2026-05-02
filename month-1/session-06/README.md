# Session 6: Enums and Strings

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

## What You'll Learn

How to define your own types with `enum`, the difference between `String` and `&str` (and why Rust has both), and a quick preview of `Vec<T>` so you can return a list of notes.

## The Big Idea

Enums in Rust are far more powerful than enums in Java or C — they're the same idea you'd find in Haskell or OCaml. Each variant can carry its own data, and `match` works on them beautifully. Today we'll only use the simple "C-like" enums; in Session 10 we'll see enums that carry data per variant. Together with `match`, enums are the foundation of how Rust models a domain.

## Concepts Covered

- Defining an `enum` with named variants
- Using enums in `match`
- `String` (owned, growable) vs `&str` (borrowed, fixed)
- Common string methods: `.to_uppercase()`, `.contains()`, `.split()`, `.trim()`
- `format!` for building strings
- `Vec<T>` — quick preview as a return type

## Building Towards `music-theory-cli`

Today we lay every brick we still need for the Session 7/8 project:

- A `NoteName` enum for the 12 chromatic notes
- A `ScaleType` enum for major / minor / pentatonic
- A `scale_notes` function that takes a root and a scale type and returns a `Vec<NoteName>`

After this session, the rest of the mini-project is just gluing these together with input parsing and output formatting.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-1/session-06/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-1/session-06/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. Defining a simple enum

```rust
enum ScaleType {
    Major,
    NaturalMinor,
    PentatonicMajor,
}
```

Three variants, no data. Just like a Java enum or C's `enum`.

You construct values with `ScaleType::Major`. The double-colon `::` is the path separator — it says "the `Major` that lives inside `ScaleType`".

### 2. Matching on an enum

```rust
fn semitone_pattern(scale: &ScaleType) -> &'static [u8] {
    match scale {
        ScaleType::Major           => &[2, 2, 1, 2, 2, 2, 1],
        ScaleType::NaturalMinor    => &[2, 1, 2, 2, 1, 2, 2],
        ScaleType::PentatonicMajor => &[2, 2, 3, 2, 3],
    }
}
```

A few new things:

- We take `scale` by reference (`&ScaleType`) rather than by value. `ScaleType` doesn't derive `Copy`, so passing it by value would *move* it into the function and the caller couldn't use it again. Borrowing is cheap and lets the caller keep ownership.
- The return type `&'static [u8]` is a borrowed slice of bytes that lives for the entire program (the `'static` lifetime). For a hardcoded literal like `&[2, 2, 1, 2, 2, 2, 1]`, this is exactly right.
- The `match` is exhaustive over the three variants — try removing one and see what the compiler says.

> Note these are **musical interval patterns**: the number of semitones between consecutive notes of the scale. `2,2,1,2,2,2,1` is the major scale (whole-whole-half-whole-whole-whole-half).

### 3. The `NoteName` enum

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum NoteName {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}
```

The `#[derive(...)]` line asks the compiler to write some boilerplate impls for us:

- `Debug` — lets us print with `{:?}`
- `Clone` — lets us call `.clone()`
- `Copy` — makes the type behave like an `i32` (cheap to copy by value)
- `PartialEq` — lets us compare with `==`

Twelve variants, one per chromatic note. Note we use `CSharp` rather than `C#` because `#` isn't allowed in Rust identifiers.

### 4. Going from `NoteName` to a string

```rust
impl NoteName {
    fn as_str(self) -> &'static str {
        match self {
            NoteName::C       => "C",
            NoteName::CSharp  => "C#",
            NoteName::D       => "D",
            NoteName::DSharp  => "D#",
            NoteName::E       => "E",
            NoteName::F       => "F",
            NoteName::FSharp  => "F#",
            NoteName::G       => "G",
            NoteName::GSharp  => "G#",
            NoteName::A       => "A",
            NoteName::ASharp  => "A#",
            NoteName::B       => "B",
        }
    }
}
```

You'll meet `impl` properly in Session 9. For now: it's how you attach methods to a type.

### 5. `String` vs `&str`

This is the topic that confuses every Rust beginner. The two things to know:

- **`&str`** is a *borrowed* view into a string that lives somewhere else. String literals in Rust source code (`"hello"`) have type `&'static str`. Functions usually take `&str` as a parameter — it's flexible (it accepts both literal strings and slices of `String`s).
- **`String`** is an *owned*, *growable* heap-allocated string. You build one when you need to construct a string at runtime.

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alex");                      // ✅ &'static str

    let owned: String = String::from("Alex");
    greet(&owned);                      // ✅ &String coerces to &str

    let upper = owned.to_uppercase();   // returns a new String
    greet(&upper);
}
```

**Rule of thumb:** parameters take `&str`, return values that you build at runtime are `String`.

### 6. The `format!` macro

```rust
let name = "Alex";
let n = 5;
let msg = format!("{} has done {} sessions", name, n);
println!("{}", msg);
```

`format!` works like `println!` but returns a `String` instead of printing.

### 7. Quick `Vec<T>` preview

```rust
let mut notes: Vec<NoteName> = Vec::new();
notes.push(NoteName::C);
notes.push(NoteName::E);
notes.push(NoteName::G);

for n in &notes {
    print!("{} ", n.as_str());
}
println!();
```

`Vec<T>` is a growable list. We'll cover it properly in Session 11. Today we just need it as a return type.

### 8. Putting it together: `scale_notes`

```rust
fn scale_notes(root: NoteName, scale: &ScaleType) -> Vec<NoteName> {
    let pattern = semitone_pattern(scale);
    let chromatic = [
        NoteName::C, NoteName::CSharp, NoteName::D, NoteName::DSharp,
        NoteName::E, NoteName::F, NoteName::FSharp, NoteName::G,
        NoteName::GSharp, NoteName::A, NoteName::ASharp, NoteName::B,
    ];

    let mut index = chromatic.iter().position(|&n| n == root).unwrap();
    let mut result = vec![root];
    for &step in pattern {
        index = (index + step as usize) % 12;
        result.push(chromatic[index]);
    }
    result
}
```

`root` is `NoteName` by value (cheap — it's `Copy`), but `scale` is `&ScaleType` for the same reason as before.

Don't worry about every line yet (`.position()` and `|&n| n == root` are iterator methods we'll cover later). The shape of it is the important bit:

1. Find the index of the root note in the chromatic scale.
2. Walk the interval pattern, each step jumping `step` semitones.
3. Collect the notes into a `Vec` and return it.

Run the example (`examples/scales_intro/`) to see it in action:

```text
C Major: C D E F G A B C
A Natural Minor: A B C D E F G A
G Pentatonic Major: G A B D E G
```

---

## Common Mistakes

### ❌ Trying to use `==` without `PartialEq`

```rust
enum Foo { A, B }
let x = Foo::A;
if x == Foo::A {}      // 💥 unless you derive PartialEq
```

**Fix:** add `#[derive(PartialEq)]` above the enum.

### ❌ Forgetting that `String` and `&str` are different types

```rust
let s: String = "hi".to_string();
greet(s);              // 💥 if greet takes &str
```

**Fix:** `greet(&s);`. The `&` borrows the `String` as a `&str`.

### ❌ Building strings with `+` in a loop

```rust
let mut s = String::new();
for word in words {
    s = s + word + " ";    // works but inefficient — copies repeatedly
}
```

**Fix:** use `s.push_str(word); s.push(' ');` — or build via `.collect::<String>()` later.

### ❌ Using `unwrap()` on something that might be `None`

We'll cover this in Session 10. For now, just be aware that `.unwrap()` will crash if there's nothing to unwrap.

---

## Session Challenge

Add a third scale type to `scale_notes`: `Blues` (semitone pattern `3, 2, 1, 1, 3, 2`). Test it with `A` as the root — you should get `A C D D# E G A`. Then think about: what other scale types might be fun? **Lydian**? **Phrygian dominant**? Add one of your own choosing.

---

## Quick Reference

| Concept | Syntax |
|---|---|
| Define enum | `enum E { A, B, C }` |
| Use variant | `E::A` |
| Derive | `#[derive(Debug, Clone, Copy, PartialEq)]` |
| Method block | `impl E { fn foo(self) -> ... { ... } }` |
| Owned string | `String::from("...")` or `"...".to_string()` |
| Borrow as `&str` | `&owned_string` |
| Build string | `format!("{}-{}", a, b)` |
| Make a `Vec` | `vec![1, 2, 3]` |
| Empty `Vec` | `Vec::new()` |
| Push | `v.push(x);` |

---

## Further Reading

Curated extra material on the topics covered in this session (Enums and Strings). All free; all current as of writing.

- [**The Rust Book** — *Defining an Enum* (6.1)](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) — Why enums in Rust are far more powerful than in most languages.
- [**The Rust Book** — *Storing UTF-8 Encoded Text with Strings* (8.2)](https://doc.rust-lang.org/book/ch08-02-strings.html) — Why `String` and `&str` exist as separate types and when to use each.
- [**It's Not Wrong That `"🤦🏼‍♂️".length == 7`** — Henri Sivonen](https://hsivonen.fi/string-length/) — Why string length is a complicated question, and how Rust's choice differs from JavaScript and Python.
- [**`std::string` and `std::str` API docs**](https://doc.rust-lang.org/std/string/struct.String.html) — The full API for `String`. Most of the methods you'll ever want are listed at the top.

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

> 📝 Session 6 done. The next two sessions are project work — no new concepts. Capture this one in [`dfe/session-log.md`](../../dfe/session-log.md). What was the most surprising thing about `String` vs `&str`?
