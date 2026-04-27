# Session 15: Mini-Project Build Part 1 — World Core

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

## DofE Log Reminder

Row 15. Project session — note in your log what you got working ("World generation engine, deterministic, ASCII-debug output").
