# Session 13 — Steam: A State Change

> **Stuck on a word?** Things like *iterator*, *map*, *filter*, *collect*, *phase change*, *condense* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **water boils to steam, steam rises, steam condenses back to water at the top of the world** — a working convection cycle. You'll also have rewritten the temperature-cool pass using **iterators** (Rust's most-loved feature) and seen the per-cell update become shorter and more idiomatic.

---

## What you'll learn

- `.iter()`, `.iter_mut()`, and the iterator protocol
- `.map()`, `.filter()`, `.enumerate()`, `.zip()`, `.fold()`, `.sum()`
- `.collect::<Vec<_>>()` — turning an iterator back into a collection
- Why iterators are usually as fast as hand-written loops (zero-cost abstraction)
- The state-change pattern: temperature crosses a threshold, type changes

---

## The big idea

Iterators are how Rust people *prefer* to write loops. Instead of `for i in 0..v.len() { v[i] = ... }`, you write `v.iter_mut().for_each(|x| { ... })`. Two reasons:

1. **Less bug surface.** Off-by-one errors, accidental skipped iterations, and "what does this index even mean" all melt away. Iterators say what you mean directly.
2. **Composability.** `.map(...).filter(...).take(10).sum::<i32>()` chains five operations into one line. Each operation reads like prose.

Today you'll use iterators for two things: (a) the cool-pass over the whole grid, rewritten as `grid.iter_mut().flat_map(|row| row.iter_mut())`, and (b) counting cells of each type for an on-screen readout.

Then water gets a state change: at 100°C, it becomes steam. Steam is just smoke with different behaviour — rises, then condenses back to water if it gets cold enough. **This is the first complete physics cycle in the project.**

---

## Concepts covered

- `.iter()` / `.iter_mut()` — by-reference iteration
- `.map(closure)` — transform each item
- `.filter(closure)` — keep items matching a predicate
- `.count()`, `.sum()`, `.collect()` — *terminal* operations
- `.flat_map(...)` — flatten nested iterators
- Closures `|x| x * 2` — anonymous mini-functions
- Adding a `Steam` variant; the state-change rule

---

## Building towards `sand-sim`

Iterators are how Session 14's reaction table gets *queried* (you'll fold over `(source, neighbours)` to find the highest-priority reaction). Iterators are how Session 19's recipe system *unlocks* elements (filter discoveries that meet the requirement). The cell-counting code today becomes the element-counter HUD in Session 16 and the codex grid in Session 20. Today is the iterator-fluency session that everything later relies on.

---

## Step-by-step walkthrough

> **Where you should be.** Session 12 finished. Oil, water, fire, oil-fire, smoke, wood, sand, stone all work. Reactions, fire-spread, and the heat field are all live.

### 1. The cool-pass, the old way — 1 minute

Look at your existing `cool_pass` from Session 9:

```rust
fn cool_pass(grid: &mut Vec<Vec<Cell>>) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if !cell.is_empty() && cell.temperature > 20.0 {
                cell.temperature = (cell.temperature - 0.5).max(20.0);
            }
        }
    }
}
```

This already uses `.iter_mut()` — you've been writing iterator code without naming it.

### 2. Refactor with `.flat_map` — 3 minutes

A flatter, more "functional" form:

```rust
fn cool_pass(grid: &mut Vec<Vec<Cell>>) {
    grid.iter_mut()
        .flat_map(|row| row.iter_mut())
        .filter(|c| !c.is_empty() && c.temperature > 20.0)
        .for_each(|c| c.temperature = (c.temperature - 0.5).max(20.0));
}
```

Read it like English: "for the grid, flatten into cells, filter to interesting ones, for each one cool down."

`flat_map` takes a closure that returns an iterator and *flattens* the result into a single stream. So `grid.iter_mut().flat_map(|row| row.iter_mut())` is "every cell in every row, one big stream."

`for_each` is the terminal operation that consumes the iterator side-effectfully. We use it instead of `.collect()` because we're mutating in place, not producing a new collection.

(Both forms compile to identical machine code. Rust iterators are zero-cost abstractions — `cargo run --release` proves it: same FPS as the hand-written loop.)

### 3. Count cells of each type — 5 minutes

This goes in your HUD. Counts how many cells of each type are currently on the grid:

```rust
use std::collections::HashMap;

fn count_cells(grid: &Vec<Vec<Cell>>) -> HashMap<CellType, usize> {
    let mut counts: HashMap<CellType, usize> = HashMap::new();
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|c| !c.is_empty())
        .for_each(|c| *counts.entry(c.cell_type).or_insert(0) += 1);
    counts
}
```

Three iterator features in one go:

- `grid.iter()` — immutable iteration (we're reading, not mutating).
- `.filter(|c| !c.is_empty())` — drop empty cells before counting.
- `.for_each(|c| *counts.entry(c.cell_type).or_insert(0) += 1)` — the standard "tally" pattern. `entry(...).or_insert(...)` returns a mutable reference to the entry, creating it if absent. The `*` dereferences so `+= 1` increments the value, not the reference.

Then in your render block, draw the counts:

```rust
let counts = count_cells(&grid);
let mut y = screen_height() - 110.0;
for (cell_type, count) in &counts {
    let line = format!("{:>5}  {}", count, cell_type.name());
    draw_text(&line, screen_width() - 130.0, y, 16.0, LIGHTGRAY);
    y += 16.0;
}
```

(`name()` returns a `&'static str` — add it as a method on `CellType` if you haven't already; see Session 6.)

`{:>5}` is *right-pad to width 5* so columns align. The result is a tiny live element census in the bottom-right.

### 4. Add the `Steam` variant — 2 minutes

Add it to the enum:

```rust
enum CellType {
    // ...
    Steam,
}
```

And the colour:

```rust
// In CellType::colour:
CellType::Steam => Color::new(0.85, 0.85, 0.90, 0.70),  // pale, semi-transparent
```

Don't add it to the selector — steam is something you *make*, not paint.

### 5. Water boils, steam condenses — 6 minutes

Modify `update_water` to add the boil check at the top:

```rust
fn update_water(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    // State change: water at boiling point becomes steam.
    if grid[row][col].temperature >= 100.0 {
        grid[row][col] = Cell {
            cell_type:   CellType::Steam,
            temperature: 110.0,
            lifetime:    200,        // ~3.3 seconds at 60fps before condensing
        };
        return;
    }

    // ... rest of water behaviour from Session 5 unchanged ...
}
```

Add `update_steam`:

```rust
fn update_steam(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    // Cool down a touch each frame.
    grid[row][col].temperature = (grid[row][col].temperature - 0.4).max(20.0);

    // Lifetime expires OR cooled below 60°C: condense back to water.
    let lifetime = grid[row][col].lifetime;
    if lifetime == 0 || grid[row][col].temperature < 60.0 {
        grid[row][col] = Cell {
            cell_type:   CellType::Water,
            temperature: 50.0,
            lifetime:    0,
        };
        return;
    }
    grid[row][col].lifetime = lifetime - 1;

    // Rise (same shape as smoke from Session 11).
    if row == 0 { return; }
    if fastrand::f32() > 0.65 { return; }
    if grid[row - 1][col].is_empty() {
        grid[row - 1][col] = grid[row][col];
        grid[row][col]     = Cell::empty();
        return;
    }
    let try_left = fastrand::bool();
    let order: [i32; 2] = if try_left { [-1, 1] } else { [1, -1] };
    for dx in order {
        let nc = col as i32 + dx;
        if nc < 0 || nc >= COLS as i32 { continue; }
        let nc = nc as usize;
        if grid[row - 1][nc].is_empty() {
            grid[row - 1][nc] = grid[row][col];
            grid[row][col]    = Cell::empty();
            return;
        }
    }
}
```

Dispatch it in the rising pass:

```rust
match grid[row][col].cell_type {
    CellType::Fire    => update_fire    (grid, row, col),
    CellType::OilFire => update_oil_fire(grid, row, col),
    CellType::Smoke   => update_smoke   (grid, row, col),
    CellType::Steam   => update_steam   (grid, row, col),
    _ => {}
}
```

**Save. Run.** Build a stone bowl filled with water (key `2`). Heat the bowl floor from below: press `H` (heat-source mode from Session 9) then drag fire underneath. The water heats. At 100°C cells start *turning into steam* and rising. The steam reaches the top, cools, and **falls back as water**. *Convection.*

> **The Wow Moment.** Make a sealed-ish space: stone walls on left, right, and top, water at the bottom. Drop fire under the floor. Watch the water boil, steam rise, hit the stone ceiling, cool, condense, drip back down, get re-boiled. **A closed water cycle in your sandbox.** This is what every climate modeller has dreamed about since the 1950s, running on your laptop in 60 lines of code. Save a clip.

### 6. (Optional) Iterator-based maximum temperature — 3 minutes

A useful debugging readout: what's the hottest cell on the grid right now?

```rust
let hottest = grid.iter()
    .flat_map(|row| row.iter())
    .map(|c| c.temperature)
    .fold(0.0f32, |acc, t| acc.max(t));

draw_text(&format!("max T: {:.0}°C", hottest), 8.0, 60.0, 18.0, RED);
```

`.fold(initial, |acc, x| ...)` is the most general iterator terminal — it folds the whole stream into a single value using your closure. The variants `.sum()`, `.product()`, `.max()`, `.min()` are all special-cased folds.

You can also write:

```rust
let hottest = grid.iter().flat_map(|r| r.iter()).map(|c| c.temperature)
    .fold(f32::NEG_INFINITY, f32::max);
```

`f32::max` is a free function with the right signature, so it can be passed directly.

---

## Linux (Ubuntu) note

Iterators compile down to the same machine code as hand-written loops — the "zero-cost abstraction" promise. To verify on Ubuntu:

```bash
cargo build --release
ls -lh target/release/sand-sim
```

The binary size won't change measurably when you swap a `for` loop for an iterator chain. If you want to *see* the machine code, install `cargo-asm`:

```bash
cargo install cargo-asm
cargo asm sand_sim::cool_pass --rust
```

(Optional, advanced — not required for the course.)

PipeWire on Ubuntu 22.04+ sometimes complains in the terminal about XDG_RUNTIME_DIR when you launch a long-running app over SSH. If you see warnings like `Failed to start XDG_RUNTIME_DIR/pipewire-0`, audio is disabled but graphics still work. Cosmetic. To silence: `export PIPEWIRE_LATENCY=512/48000` before `cargo run`.

---

## Common mistakes

### `error[E0277]: ... is not an iterator`

You called an iterator method on something that isn't one. `Vec` has `.iter()` to get an iterator; calling `.map()` on the `Vec` directly doesn't work. The fix is always `v.iter().map(...)`.

### `for_each` mutates but the changes don't stick

You called `.iter()` (immutable) instead of `.iter_mut()`. Or you collected before mutating — iterating over a collection of *copies* doesn't update the original. Always use `.iter_mut()` for in-place mutation.

### Steam never appears

Water's temperature isn't reaching 100°C. Either the heat-source brush is off, or the cool-pass is decaying heat faster than the fire can add it. Put `println!("{}", grid[r][c].temperature)` somewhere temporary to confirm temperatures are rising.

### Steam appears but immediately condenses

You set the condense threshold too high (e.g. `< 90.0` instead of `< 60.0`). Steam spawned at 110°C cools by 0.4 per frame; if it condenses below 90°C, it only survives 50 frames (less than a second). The `< 60.0` threshold gives steam a few seconds of life.

### HashMap counts are wrong on subsequent frames

You forgot to clear the map each frame. Either re-create the HashMap each call (`HashMap::new()` at the top), or `counts.clear()` before counting. The version above creates a fresh map per call — simplest and fast enough.

### `.collect()` won't compile — "type annotations needed"

`.collect()` is generic over what to collect *into*. Tell it: `.collect::<Vec<i32>>()`. The turbofish (`::<...>`) is one of Rust's most distinctive bits of syntax.

---

## Session challenge

Pick one — no solution provided.

1. **Steam falls if too cold.** Below 30°C, steam should fall instead of rise (it's now water-vapour-in-air which sinks if cold enough). Toggle the iteration order or add a `if temperature < 30 { update_steam_falling(...) }` branch.
2. **`count_cells` returns a `Vec<(CellType, usize)>` sorted by count.** Use `.collect()` into a `Vec`, then `.sort_by(|a, b| b.1.cmp(&a.1))` to sort descending. Display them ranked.
3. **Sliding average frame counter using `.fold()`.** Keep the last 60 frame times in a ring buffer; each frame, average them with `.fold(0.0, |acc, t| acc + t) / 60.0`. Smoother than `get_fps()`.
4. **Ice from very cold water.** Add a `CellType::Ice` variant; in `update_water`, if temperature falls below `-5.0°C`, become ice. (Lands properly in Session 15 — beat them to it.)

---

## Quick reference

| What | Code |
|---|---|
| Immutable iter | `v.iter()` |
| Mutable iter | `v.iter_mut()` |
| Owning iter | `v.into_iter()` |
| Transform each | `.map(\|x\| x * 2)` |
| Keep matching | `.filter(\|x\| x > &0)` |
| Drop n items | `.skip(n)` |
| Take n items | `.take(n)` |
| Sum | `.sum::<i32>()` |
| Count | `.count()` |
| Build a `Vec` | `.collect::<Vec<_>>()` |
| Flatten nesting | `.flat_map(\|row\| row.iter())` |
| Fold into one value | `.fold(init, \|acc, x\| ...)` |
| Pair with index | `.enumerate()` |
| Pair two iters | `.zip(other_iter)` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 13**. Worth recording:

- A clip of the closed water cycle (boil → rise → condense → drip)
- Your sentence on why iterators are "zero-cost" — paraphrase from above in your own words
