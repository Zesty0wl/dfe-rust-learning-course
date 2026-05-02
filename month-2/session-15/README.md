# Session 15: Mini-Project Build Part 1 — World Core

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

## What You'll Build

The generation engine for `world-generator`. By the end of this session you'll have:

- A working `Tile` enum and `World` struct
- A deterministic hash function turning `(seed, x, y)` into a value in `[0, 1)`
- A `World::generate(seed, width, height)` that fills a 2D grid

You won't have rendering yet. That's Session 16.

## Where to Work

Open [`../project/world-generator/starter/`](../project/world-generator/starter/). The skeleton is laid out with `TODO` comments for Sessions 15 *and* 16. Today you tackle the Session 15 TODOs.

A complete reference is in [`../project/world-generator/solution/`](../project/world-generator/solution/) — try first, peek if stuck.

---

> 💡 **Where to work today.** This is a project session, so you'll be inside the project folder, not the session folder. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-2/project/world-generator/starter        # your work-in-progress
> cargo run -- <args>
> ```
>
> The reference implementation lives in `month-2/project/world-generator/solution/` — peek only when you're properly stuck. All `cargo run` commands shown below assume you're inside `month-2/project/world-generator/starter/`.

## Step-by-Step Walkthrough

### 1. The `Tile` enum

Open `starter/src/main.rs`. Replace the placeholder `Tile`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ocean,
    Plains,
    Forest,
    Mountain,
    Desert,
}
```

Why those derives?

- `Debug` — handy for `{:?}` while we're developing
- `Clone, Copy` — `Tile` is just a tag with no data, so it's tiny and safe to copy implicitly
- `PartialEq, Eq` — needed for `==` and to be a `HashMap` key
- `Hash` — needed to be a `HashMap` key

### 2. The `World` struct

```rust
struct World {
    seed: u64,
    width: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
}
```

`Vec<Vec<Tile>>` — outer vec is rows (`y`), inner vec is columns (`x`). When we render we'll iterate rows top-to-bottom.

### 3. The hash function — *understand* it, don't just copy

It's already in your starter file, but let's read it:

```rust
fn hash(seed: u64, x: usize, y: usize) -> f64 {
    let mut h = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(x as u64)
        .wrapping_mul(2891336453)
        .wrapping_add(y as u64);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    (h as f64) / (u64::MAX as f64)
}
```

What's going on?

1. **Mix the inputs together** — combine `seed`, `x`, `y` into a single `u64` `h`. The big constants are large primes that ensure tiny changes ripple through all 64 bits. We use **`wrapping_mul`/`wrapping_add`** because we *want* overflow — it's part of how the mixing works (without `wrapping_*` Rust would panic in debug mode on integer overflow, which is normally a great safety feature but here we'd bypass it).
2. **Avalanche** — `h ^= h >> 33;` shifts the top bits down and XORs them, mixing high and low halves. Repeat with another multiply to scramble further. This is the [SplitMix64](https://en.wikipedia.org/wiki/Xorshift) finaliser.
3. **Convert to `[0, 1)`** — divide by `u64::MAX` as `f64`.

The result: same input → same output every time (determinism), but tiny input changes produce wildly different outputs (good for noise). For *visual* generation this is fine. For cryptography it's not — never use this for passwords, security tokens, or anything that needs unpredictability.

> **Maths moment:** This is essentially a hash function. A cryptographic hash (SHA-256) is more rigorous; this is a tiny, fast non-cryptographic one. The algorithm in your starter is from Sebastiano Vigna's *xoshiro* family; it's used in real game engines.

### 4. The biome thresholds

```rust
fn tile_for(n: f64) -> Tile {
    if n < 0.30      { Tile::Ocean }
    else if n < 0.55 { Tile::Plains }
    else if n < 0.78 { Tile::Forest }
    else if n < 0.90 { Tile::Mountain }
    else             { Tile::Desert }
}
```

If `hash` is uniformly distributed in `[0, 1)` (which it basically is), this gives you:
- 30% Ocean
- 25% Plains
- 23% Forest
- 12% Mountain
- 10% Desert

Tweak the thresholds to taste — fewer mountains, more deserts, etc. Your call.

### 5. `World::generate`

```rust
impl World {
    fn generate(seed: u64, width: usize, height: usize) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::with_capacity(height);
        for y in 0..height {
            let mut row: Vec<Tile> = Vec::with_capacity(width);
            for x in 0..width {
                let n = hash(seed, x, y);
                row.push(tile_for(n));
            }
            grid.push(row);
        }
        Self { seed, width, height, grid }
    }
}
```

Two nested loops, fills the grid. The `Vec::with_capacity` calls are a small optimisation — they pre-allocate so push doesn't have to keep resizing.

### 6. Smoke test in `main`

For now, just generate a small world and print debug output:

```rust
fn main() {
    let world = World::generate(42, 8, 5);
    for row in &world.grid {
        println!("{:?}", row);
    }
}
```

You should see a grid of `Ocean / Plains / Forest / Mountain / Desert` debug output. **Run it twice** — same output. **Change the seed** — totally different output. **Same seed but bigger world** — first part is identical, new tiles fill the new area. Determinism in action.

---

## Common Mistakes

- **Forgetting `wrapping_mul`** — `u64 * u64` can overflow; without `wrapping_*` you get a panic in debug mode.
- **Mixing up `x` and `y`** — `grid[y][x]` not `grid[x][y]`. Outer vec = rows.
- **Building the grid then trying to mutate `Tile` later** — `Tile` is `Copy`, so you can replace whole tiles freely; but if you forgot `Copy`, the borrow checker will complain.
- **Calling `World::generate` with `width = 0`** — the result is technically valid but useless. We'll add range checks in Session 16.

---

## Session Challenge

Once your generator works:

1. Print the same world twice. Confirm identical output.
2. Now print seeds 1, 2, 3 side by side. Confirm they look totally different.
3. Try `width = 200, height = 60`. Notice it's still instantaneous — Rust is fast.

---

## Further Reading

Curated extra material on the topics covered in this session (Project — World Core (part 1)). All free; all current as of writing.

- [**Red Blob Games — *Making maps with noise functions***](https://www.redblobgames.com/maps/terrain-from-noise/) — The single best web tutorial on procedural terrain. Pictures everywhere; language-agnostic.
- [**RogueBasin — *Cellular Automata Method for Generating Random Cave-Like Levels***](https://roguebasin.com/index.php/Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels) — The classic 'random + smoothing passes = caves' algorithm we're inspired by.
- [**`rand` crate documentation**](https://docs.rs/rand/latest/rand/) — Rust's standard random library. Read the *Quick Start* page first.
- [**Notch's blog — *Terrain generation, part 1***](https://notch.tumblr.com/post/3746989361/terrain-generation-part-1) — Minecraft's creator describing how Minecraft's terrain works. Exactly the lineage of what you're building.

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

Row 15. Project session — note in your log what you got working ("World generation engine, deterministic, ASCII-debug output").
