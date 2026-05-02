# Session 12: Iterators and Closures (Introduction)

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

## What You'll Learn

Iterators are the difference between writing Rust like an old C programmer and writing it like someone who actually likes the language. They're concise, fast (zero-cost — the compiler unrolls them to the same machine code as a hand-written `for` loop), and chainable. You'll also meet **closures** — anonymous functions you can pass around as values.

## The Big Idea

You already saw `for x in &v`. That's iterators in disguise. The full power kicks in when you start chaining adapters:

```rust
let big_evens: Vec<i32> = (1..=100)
    .filter(|n| n % 2 == 0)
    .map(|n| n * n)
    .filter(|n| *n > 1000)
    .collect();
```

Read that left-to-right: take 1 to 100, keep evens, square them, keep the ones over 1000, gather into a `Vec`. One line. No loop body. No accumulator variable.

The thing inside `|n| ...` is a **closure** — an inline function. `|n|` declares the parameter, the rest is the body. Closures can also capture variables from their surroundings, which makes them more flexible than plain functions.

## Concepts Covered

- The iterator concept: `.iter()`, `.iter_mut()`, `.into_iter()`
- Adapters that transform iterators: `.map()`, `.filter()`, `.enumerate()`, `.zip()`, `.take()`, `.skip()`, `.rev()`
- Consumers that produce a final value: `.collect()`, `.sum()`, `.count()`, `.for_each()`, `.find()`, `.any()`, `.all()`
- Closure syntax: `|x| x + 1`, `|x, y| x * y`, `|| 42`
- Why iterators are zero-cost (compiler optimisation)

## Building Towards `world-generator`

In Sessions 15 and 16 we'll use iterators to:
- Render the world: `for (y, row) in world.iter().enumerate() { ... }`
- Count biomes: `world.iter().flatten().filter(|t| matches!(t, Tile::Ocean { .. })).count()`
- Map a row of `Tile`s into a row of display chars before printing

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-2/session-12/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-2/session-12/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. `.iter()` and `.collect()`

`examples/grid_processing/src/main.rs`:

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("{:?}", doubled);   // [2, 4, 6, 8, 10]
```

`nums.iter()` produces an iterator of `&i32` (borrows). `.map(|n| n * 2)` runs the closure on each. `.collect()` packages them up — into a `Vec` here, but the type annotation drives that decision (you can collect into `HashSet`, `String`, etc.).

### 2. `.filter()` keeps what you want

```rust
    let evens: Vec<i32> = nums.iter().copied().filter(|n| n % 2 == 0).collect();
    println!("{:?}", evens);    // [2, 4]
```

`.copied()` turns an iterator of `&i32` into an iterator of `i32` (cheap copy). Filter keeps the items where the closure returns `true`.

### 3. Chain them — the magic

```rust
    let total: i32 = (1..=10)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("Sum of squares of evens 1..=10 = {}", total); // 220
```

This compiles to a single loop with no allocations. **Zero-cost abstraction**: write it nicely, get the same machine code as if you'd hand-rolled the loop.

### 4. `.enumerate()` for index + value

```rust
    let names = vec!["a", "b", "c"];
    for (i, name) in names.iter().enumerate() {
        println!("{}: {}", i, name);
    }
```

This is the Rust-idiomatic version of `for i in 0..names.len()` followed by `names[i]`. Always prefer `enumerate()`.

### 5. Process a 2D grid

This is exactly the pattern Sessions 15/16 will need:

```rust
    let world: Vec<Vec<&str>> = vec![
        vec!["~", "~", "~", "▒", "▒"],
        vec!["~", "~", "▒", "▓", "▓"],
        vec!["▒", "▒", "▓", "▲", "▓"],
    ];

    let ocean_count = world
        .iter()
        .flatten()              // Vec<Vec<&str>> → iter of all &&str
        .filter(|c| **c == "~") // double-deref past the borrows
        .count();
    println!("Ocean tiles: {}", ocean_count);
```

`.flatten()` is brilliant for 2D structures — it turns "iterator of iterators" into "iterator of items".

### 6. Closures that capture

A closure can refer to variables from its surrounding scope. This is impossible with regular functions:

```rust
    let threshold = 3;
    let big = nums.iter().copied().filter(|n| *n > threshold).count();
    println!("Numbers > {} = {}", threshold, big);
}
```

`threshold` was defined outside the closure, but inside the closure body it just works. The compiler captures it automatically.

---

## Common Mistakes

- **`.iter()` gives `&T`, not `T`.** That's why filter closures often need `**c == ...` or you call `.copied()` / `.cloned()` to get owned values.
- **Forgetting `.collect()`.** Adapters are *lazy*. `nums.iter().map(...)` does nothing on its own — it just builds an iterator pipeline. You need a consumer (`.collect()`, `.sum()`, `.for_each()`, ...) to actually run it.
- **Type annotation on `.collect()` is usually required.** Rust needs to know what you're collecting *into*. Either `let v: Vec<i32> = ...collect();` or `...collect::<Vec<i32>>()` (turbofish syntax).
- **Calling `.iter()` then trying to push to the original Vec.** The iterator borrows; can't mutate while iterating. Collect first, then mutate.

---

## Session Challenge

Given:

```rust
let world: Vec<Vec<i32>> = vec![
    vec![1, 0, 0, 2],
    vec![0, 3, 2, 0],
    vec![0, 0, 1, 4],
];
```

Use iterators (no manual `for` loops in the bodies) to compute:

1. The total count of non-zero tiles.
2. A `Vec<i32>` containing every value squared, flattened (so `[1,0,0,2,0,9,4,0,0,0,1,16]`).
3. The largest value in the world (use `.flatten().max()`).
4. The (x, y) coordinates of every tile equal to 0 (use `.enumerate()` twice and `.collect::<Vec<_>>()`).

---

## Quick Reference

```rust
let v = vec![1, 2, 3, 4, 5];

// Adapters (lazy)
v.iter().map(|x| x + 1)                  // [2, 3, 4, 5, 6]
v.iter().filter(|x| **x > 2)             // 3, 4, 5
v.iter().enumerate()                     // (0,&1), (1,&2), ...
v.iter().take(3)                         // first 3
v.iter().skip(2)                         // skip first 2
v.iter().rev()                           // reversed

// Consumers (eager)
v.iter().sum::<i32>()                    // 15
v.iter().count()                         // 5
v.iter().max()                           // Some(&5)
v.iter().find(|x| **x > 2)               // Some(&3)
v.iter().any(|x| *x == 3)                // true
v.iter().all(|x| *x > 0)                 // true
v.iter().for_each(|x| println!("{}", x));

// Closures
|x| x + 1
|x, y| x + y
|| "no args, returns this string"
move |x| outer_var + x       // 'move' takes ownership of captured vars
```

---

## Further Reading

Curated extra material on the topics covered in this session (Iterators and Closures (intro)). All free; all current as of writing.

- [**The Rust Book** — *Functional Language Features* (chapter 13)](https://doc.rust-lang.org/book/ch13-00-functional-features.html) — Closures and iterators in one chapter. Worth reading end-to-end.
- [**`Iterator` trait documentation**](https://doc.rust-lang.org/std/iter/trait.Iterator.html) — Every adapter (`map`, `filter`, `take`, `chain`, ...) listed alphabetically. Skim it; you'll come back.
- [**Rust by Example** — *Closures*](https://doc.rust-lang.org/rust-by-example/fn/closures.html) — All three closure traits (`Fn`, `FnMut`, `FnOnce`) demonstrated in tiny snippets.

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

Row 12. Two-thirds of the way through Month 2.
