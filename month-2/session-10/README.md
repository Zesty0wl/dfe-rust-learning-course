# Session 10: Enums with Data and `Option<T>`

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

## What You'll Learn

How to create enums where each variant carries different data — Rust's killer feature for modelling things that can be one of several distinct shapes. And then `Option<T>`, which is how Rust handles "maybe there's a value, maybe not" without ever using `null`.

## The Big Idea

In Session 6 you saw simple enums:

```rust
enum Note { C, D, E, F, G, A, B }
```

That's powerful, but Rust enums can do much more — each variant can carry **different data of different shapes**. This makes them a way to say "this value is exactly one of these, with this exact data attached":

```rust
enum Tile {
    Ocean { depth: u8 },
    Plains,
    Forest { density: u8 },
    Mountain { height: u16 },
    Desert,
}
```

A single `Tile` value is one of those five things. If it's an `Ocean`, the depth is *part of the type*; the compiler guarantees you can't access depth on a Plains tile, because Plains doesn't have one. Try it and the program won't compile. This is a class of bug — accessing fields that don't exist — that Rust eliminates entirely.

Then there's `Option<T>`. In most languages, "no value here" is `null` / `None` / `nil`, and you find out it was missing by crashing at runtime. Rust doesn't have null. Instead it has:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

A function that *might* return a value returns `Option<T>`. The caller is forced by the compiler to handle both `Some` and `None`. The famous "billion-dollar mistake" (Tony Hoare's words about inventing null) — Rust just sidesteps it.

## Concepts Covered

- Enum variants with **named fields** (struct-like) and **tuple data**
- `match` on enum variants, destructuring the inner data
- `Option<T>`, `Some(value)`, `None`
- `.unwrap()`, `.unwrap_or(default)`, `.is_some()`, `.is_none()`
- Pattern matching as the canonical way to use `Option`
- Why Rust doesn't have `null`

## Building Towards `world-generator`

The world is going to be a `Vec<Vec<Tile>>` and `Tile` will be exactly the enum above. Plus, "find the first mountain in the world" returns `Option<(usize, usize)>` — `Some((x, y))` if we find one, `None` if the world has no mountains.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-2/session-10/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-2/session-10/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. The `Tile` enum

`examples/tile_enum/src/main.rs`:

```rust
#[derive(Debug)]
enum Tile {
    Ocean { depth: u8 },
    Plains,
    Forest { density: u8 },
    Mountain { height: u16 },
    Desert,
}
```

Variants come in three shapes:
- **Unit** like `Plains` and `Desert` — no data
- **Struct-like** like `Ocean { depth: u8 }` — named fields
- **Tuple-like** like `Mountain(u16)` — anonymous fields, accessed by `.0`, `.1`, etc.

We're using struct-like throughout because the field names make code more readable.

### 2. Make some tiles

```rust
let a = Tile::Ocean { depth: 12 };
let b = Tile::Plains;
let c = Tile::Mountain { height: 1500 };
```

Note `Tile::` prefix when constructing — same as `Block::new` in Session 9. Variants live "inside" the enum's namespace.

### 3. Match and destructure

```rust
fn describe(t: &Tile) -> String {
    match t {
        Tile::Ocean { depth }    => format!("Ocean ({}m deep)", depth),
        Tile::Plains             => String::from("Grassy plains"),
        Tile::Forest { density } => format!("Forest ({}% trees)", density),
        Tile::Mountain { height }=> format!("Mountain ({}m tall)", height),
        Tile::Desert             => String::from("Hot, sandy desert"),
    }
}
```

`{ depth }` inside `match` is **pattern-matching with destructuring**. It pulls the `depth` field out of the variant and binds it to a local variable. If the field name in the pattern matches the field name in the variant definition, you don't need to write `depth: depth`.

### 4. The compiler forces you to be exhaustive

Comment out the `Tile::Desert` arm and try to compile:

```text
error[E0004]: non-exhaustive patterns: `&Tile::Desert` not covered
```

The compiler will not let you ship code that doesn't handle every possible variant. You can't *forget* a case. (You can use `_ =>` as a catch-all, but then it's a deliberate choice.)

### 5. Now `Option<T>` — the search function

Imagine a tiny world (just a `Vec<Vec<Tile>>` for now) and you want to find the first mountain:

```rust
fn find_mountain(world: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    for (y, row) in world.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Tile::Mountain { .. } = tile {
                return Some((x, y));
            }
        }
    }
    None
}
```

Two new bits:

- **`Option<(usize, usize)>` as the return type.** "Maybe a coordinate pair, maybe nothing."
- **`if let Tile::Mountain { .. } = tile`** — a shorthand for `match` when you only care about *one* variant. The `..` means "any fields, don't care".

### 6. Using the result

```rust
match find_mountain(&world) {
    Some((x, y)) => println!("Mountain at ({}, {})!", x, y),
    None         => println!("No mountains in this world."),
}
```

You **cannot** accidentally use the coordinates as if they were always there. The compiler insists you handle the `None` case.

If you're 100% sure the value is `Some` (or you're prototyping), `.unwrap()` exists:

```rust
let (x, y) = find_mountain(&world).unwrap();   // panics if None
```

But `.unwrap()` is a code smell in production — it's how you crash. Prefer `.unwrap_or(default)` for a sensible fallback, or proper `match`.

---

## Common Mistakes

- **Comparing enum variants with `==`** — only works if you `#[derive(PartialEq)]`. Otherwise use `match` or `if let`.
- **Trying to access fields without destructuring** — `tile.depth` doesn't compile, because the compiler can't prove this `tile` *has* a depth. You have to `match` it.
- **Calling `.unwrap()` everywhere** — works until it doesn't. The whole point of `Option` is to *handle* the None case. Use `match` or `.unwrap_or(...)`.
- **Forgetting `Tile::` prefix** — `Ocean { depth: 12 }` alone won't compile; needs to be `Tile::Ocean { depth: 12 }`.

---

## Session Challenge

Add a method `is_passable(&self) -> bool` to `Tile` (returns `true` for everything except `Mountain` with `height > 1000` and `Ocean` with `depth > 5`).

Then write a function `safest_route_start(world: &Vec<Vec<Tile>>) -> Option<(usize, usize)>` that returns the coordinates of the first **passable** tile in the world. Return `None` if nothing is passable.

Bonus: add `Tile::River { width: u8 }`. Watch the compiler tell you exactly which `match` blocks need updating.

---

## Quick Reference

```rust
enum Shape {
    Circle { radius: f64 },
    Square(f64),                // tuple-like; access with .0
    Point,
}

let s = Shape::Circle { radius: 1.5 };

let area = match s {
    Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
    Shape::Square(side)      => side * side,
    Shape::Point             => 0.0,
};

// Option
fn first_letter(s: &str) -> Option<char> {
    s.chars().next()
}

if let Some(c) = first_letter("hi") {
    println!("First char: {}", c);
}

let x = first_letter("").unwrap_or('?');     // '?' because string is empty
```

---

## Further Reading

Curated extra material on the topics covered in this session (Enums with data and `Option<T>`). All free; all current as of writing.

- [**The Rust Book** — *Enums and Pattern Matching* (chapter 6)](https://doc.rust-lang.org/book/ch06-00-enums.html) — Re-read with fresh eyes now that you've seen them in action.
- [**`Option<T>` documentation**](https://doc.rust-lang.org/std/option/) — Skim every method. `map`, `and_then`, `unwrap_or`, `ok_or` — all the glue you'll use forever.
- [**Null References: The Billion Dollar Mistake** — Tony Hoare (talk)](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/) — The man who invented `null` apologising for it. Explains why `Option<T>` matters.

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

Row 10 of [`session-log.md`](../../dfe/session-log.md). Don't break the chain.
