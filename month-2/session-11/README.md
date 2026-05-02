# Session 11: Collections — `Vec` and `HashMap`

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

## What You'll Learn

How to store *many* values, not just one. `Vec<T>` is Rust's growable list — what `list` is in Python or `ArrayList` is in Java. `HashMap<K, V>` is the dictionary / hash table. You'll use both constantly for the rest of the course.

## The Big Idea

A program with one `Tile` is no fun. A world has hundreds of them in a 2D grid. You need a container.

Rust's main two general-purpose containers are:

- **`Vec<T>`** — an *ordered* sequence of `T`. Index by position (`v[0]`). Push to the end. Iterate front to back.
- **`HashMap<K, V>`** — pairs of `K` keys mapped to `V` values. Lookup by key. No order. Like Python's `dict`.

Both are **generic** — `Vec<u8>`, `Vec<String>`, `Vec<Tile>`, `HashMap<String, u32>`, `HashMap<(i32, i32), Tile>`. Anything goes.

The key question is **which one when?**

- "I have an ordered list of things; I'll usually iterate them in order" → `Vec`
- "I have an arbitrary key (string, coordinate, ID) and I want to look up its value fast" → `HashMap`

For a game world specifically, both happen — the *grid* is a `Vec<Vec<Tile>>`, the *biome statistics* (how many of each biome) is a `HashMap<&str, u32>`.

## Concepts Covered

- Creating `Vec`s: `vec![]` macro, `Vec::new()`, `Vec::with_capacity()`
- `.push()`, `.pop()`, indexing `v[i]`, `v.get(i)` (returns `Option`!), `.len()`, `.is_empty()`
- Iterating with `for x in &v` (borrow) vs `for x in v` (consume)
- Slicing: `&v[1..3]`
- `Vec<Vec<T>>` — 2D grids
- `HashMap`: `.insert()`, `.get()`, `.contains_key()`, `.entry().or_insert()`
- The `*` count pattern: `*counter.entry(key).or_insert(0) += 1;`

## Building Towards `world-generator`

The world's grid will literally be `Vec<Vec<Tile>>`. The "stats" line at the bottom of the rendered map (`Ocean 412  Plains 318 ...`) will be built by counting tiles into a `HashMap<&str, u32>`. Both directly from this session.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-2/session-11/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-2/session-11/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. `Vec` basics

`examples/world_grid/src/main.rs`:

```rust
use std::collections::HashMap;

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    nums.push(10);
    nums.push(20);
    nums.push(30);
    println!("Vec: {:?}, len={}", nums, nums.len());

    // The vec! macro is a shortcut
    let names: Vec<&str> = vec!["Alice", "Bob", "Carol"];
    for name in &names {
        println!("hello {}", name);
    }
```

`for name in &names` borrows the vec — you can use `names` after the loop. `for name in names` would *consume* it (move it into the loop), and you couldn't use it again. Rust's borrow checker enforces this.

### 2. `v.get(i)` returns an `Option` — bounds-safe indexing

```rust
    println!("names[1] = {}", names[1]);              // "Bob" — panics if out of range
    println!("names.get(99) = {:?}", names.get(99));  // None — never panics
```

If you don't know whether the index is valid, prefer `.get(i)`. `v[i]` is the convenience version that panics if `i` is out of range.

### 3. 2D grids: `Vec<Vec<T>>`

This is how we build a world. Each inner `Vec` is one row.

```rust
    let width = 5;
    let height = 3;
    let mut grid: Vec<Vec<&str>> = Vec::with_capacity(height);
    for _y in 0..height {
        let row = vec!["~"; width];     // 5 copies of "~"
        grid.push(row);
    }
    grid[1][2] = "▲";
    grid[1][3] = "▲";

    for row in &grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
```

`vec!["~"; width]` creates a `Vec` of `width` copies of `"~"`. Very handy for grid initialisation.

### 4. `HashMap` — counting biomes

```rust
    let world_tiles = vec![
        "Ocean", "Ocean", "Plains", "Mountain",
        "Plains", "Plains", "Forest", "Ocean",
        "Mountain", "Plains", "Forest", "Ocean",
    ];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for tile in &world_tiles {
        *counts.entry(tile).or_insert(0) += 1;
    }

    println!("\nBiome counts:");
    for (biome, count) in &counts {
        println!("  {:<10} {}", biome, count);
    }
}
```

The line `*counts.entry(tile).or_insert(0) += 1;` is famous and worth understanding character by character:

- `counts.entry(tile)` — gets a mutable handle to the entry for this key (whether it exists or not)
- `.or_insert(0)` — if it doesn't exist yet, insert `0`, then either way return a `&mut V` to the value
- `*... += 1` — dereference the `&mut V` and add 1

So in three terse tokens you implement "if key exists, +1; else, set to 1". It's the canonical Rust counting idiom.

### 5. `HashMap` lookup

```rust
let n = counts.get("Ocean").copied().unwrap_or(0);
```

`.get()` returns `Option<&u32>` — that `&` is a reference, so we use `.copied()` to get an owned `u32`. Then `.unwrap_or(0)` falls back to 0 if the key wasn't there.

---

## Common Mistakes

- **`v[99]` when v has 3 elements** — panic! Use `v.get(99)` for safety.
- **Using a `Vec` after `for x in v`** — moved into the loop. Use `&v` to borrow instead.
- **Mutating a `Vec` while iterating it** — Rust's borrow checker won't even let you compile this. The fix: collect changes into a separate `Vec`, then apply them after.
- **`HashMap` without `use`** — `HashMap` lives in `std::collections::HashMap`, so you need `use std::collections::HashMap;` at the top.
- **Forgetting the `*` in `*counts.entry(k).or_insert(0) += 1;`** — without the dereference you're trying to add 1 to a *reference*, which is a different beast.

---

## Session Challenge

Build a function `flat_world(width: usize, height: usize) -> Vec<Vec<Tile>>` that returns a grid where the bottom half is `Tile::Ocean { depth: 5 }` and the top half is `Tile::Plains`.

Then build `count_biomes(world: &Vec<Vec<Tile>>) -> HashMap<&'static str, u32>` that returns the count of each biome name.

(Use `Tile` from Session 10 — copy the enum across to play.)

---

## Quick Reference

```rust
// Vec
let mut v: Vec<i32> = Vec::new();
v.push(1); v.push(2); v.push(3);
let v2 = vec![10, 20, 30];
v2[0];                    // 10, panics if oob
v2.get(99);               // Option<&i32>: None
for x in &v2 { ... }
for x in v2 { ... }       // consumes v2
v2.iter().sum::<i32>();   // 60
v2.len();                 // 3

// HashMap
use std::collections::HashMap;
let mut m: HashMap<&str, u32> = HashMap::new();
m.insert("a", 1);
m.get("a");               // Some(&1)
m.contains_key("a");      // true
*m.entry("b").or_insert(0) += 1;
for (k, v) in &m { ... }
```

---

## Further Reading

Curated extra material on the topics covered in this session (Collections — `Vec` and `HashMap`). All free; all current as of writing.

- [**The Rust Book** — *Common Collections* (chapter 8)](https://doc.rust-lang.org/book/ch08-00-common-collections.html) — Vec, String, and HashMap explained together.
- [**`Vec<T>` documentation**](https://doc.rust-lang.org/std/vec/struct.Vec.html) — The most-used data structure in Rust. The method list at the top is your menu.
- [**`HashMap<K, V>` documentation**](https://doc.rust-lang.org/std/collections/struct.HashMap.html) — Note the *Performance* and *HashDoS resistance* sections — explains why Rust's default hasher is slower than e.g. Python's.
- [**The Rust Performance Book** — Collections](https://nnethercote.github.io/perf-book/collections.html) — Tips for choosing the right container. Reach for this once your program does anything serious.

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

Row 11. Half a minute of writing earns you a full week of Skill credit. Bargain.
