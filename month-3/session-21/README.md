# Session 21 — Gunpowder and Glass

> **Stuck on a word?** Things like *move closure*, *flat_map*, *chain*, *explosion radius* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **gunpowder** detonates when touched by fire (radial explosion in an N-cell radius), and **glass** is forged where lava meets sand. Both elements are gated behind their recipes from Session 19. The detonation algorithm is built with iterator chains and `move` closures — Rust's most expressive iterator pattern, used in earnest for the first time.

---

## What you'll learn

- `move` closures — capturing variables by ownership, not reference
- `.flat_map(...)` for "expand each item into many" — used to enumerate explosion radius
- `.chain(...)` — concatenating iterators
- Computing a circle of cell offsets with iterators alone, no nested loops
- Bringing it all together: a single expression that lists every affected cell

---

## The big idea

The simple explosion algorithm has two nested `for` loops:

```rust
for dr in -radius..=radius {
    for dc in -radius..=radius {
        if dr*dr + dc*dc <= radius*radius {
            // affect (row+dr, col+dc)
        }
    }
}
```

That works. But the iterator-chain version reads as a *description* of the affected cells:

```rust
let affected = (-radius..=radius)
    .flat_map(move |dr| (-radius..=radius).map(move |dc| (dr, dc)))
    .filter(move |(dr, dc)| dr*dr + dc*dc <= radius*radius);
```

That iterator is then consumable: `affected.for_each(|(dr, dc)| { ... })`. Composable, testable, named. Same machine code.

**Today's win is twofold**: gameplay (gunpowder and glass make the simulation *fun*), and language (move closures land properly as the syntax that makes complex iterator chains work).

---

## Concepts covered

- `move` keyword on closures
- `..=` inclusive range vs `..` half-open range
- `.flat_map(closure -> iterator)`
- `.chain(other)` — sequential concatenation of iterators
- Storing iterator results in a `Vec` with `.collect()`
- Borrow-checker subtleties solved by `move`

---

## Building towards `sand-sim`

Gunpowder + glass are the **showy** elements that demos love. They're also natural recipe candidates (gunpowder is unlocked by fire + smoke; glass by sand + lava). Today's iterator-chain pattern is reused in Session 22 for the time-aware concrete-and-rust simulation and in Session 23 for the title-screen particle background.

---

## Step-by-step walkthrough

> **Where you should be.** Session 20 finished. Codex works. Discoveries persist. Ready to add two new elements.

### 1. Add the variants — 2 minutes

In `src/elements.rs`:

```rust
pub enum CellType {
    // ...
    Gunpowder,
    Glass,
}

// colour:
CellType::Gunpowder => Color::new(0.40, 0.30, 0.25, 1.0),  // dark brown, like sand+coal
CellType::Glass     => Color::new(0.80, 0.90, 1.00, 0.50), // pale, transparent

// density:
CellType::Gunpowder => 130,
CellType::Glass     => 220,
```

Glass is heavier than stone (denser, like real soda-lime glass). Gunpowder is light, settles into piles like sand.

### 2. Gunpowder behaves like sand — 1 minute

In `simulation.rs`, dispatch gunpowder to the existing `update_sand` (it falls and stacks the same way):

```rust
match cell_type {
    CellType::Sand | CellType::Gunpowder => update_sand(grid, row, col),
    // ...
}
```

### 3. Glass is static — 1 minute

Glass is solid like stone. No update function needed.

### 4. The explosion — 8 minutes

The big new code. In `simulation.rs`:

```rust
const GUNPOWDER_RADIUS: i32 = 6;
const GUNPOWDER_HEAT:   f32 = 800.0;

/// Detonate gunpowder at (row, col): empty the radius, add heat to the perimeter.
pub fn detonate(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    let radius = GUNPOWDER_RADIUS;

    // Build the list of affected offsets using iterators (today's exercise).
    let affected: Vec<(i32, i32)> = (-radius..=radius)
        .flat_map(move |dr| (-radius..=radius).map(move |dc| (dr, dc)))
        .filter(move |(dr, dc)| dr*dr + dc*dc <= radius*radius)
        .collect();

    for (dr, dc) in &affected {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);
        let dist2 = (dr*dr + dc*dc) as f32;

        if dist2 < (radius as f32 * 0.5).powi(2) {
            // Inner core: vaporise everything to fire.
            grid[nr][nc] = Cell {
                cell_type:   CellType::Fire,
                temperature: GUNPOWDER_HEAT,
                lifetime:    60,
            };
        } else {
            // Outer ring: scatter (set to empty) and add heat.
            if !grid[nr][nc].is_empty() && fastrand::f32() < 0.7 {
                grid[nr][nc] = Cell::empty();
            }
            grid[nr][nc].heat(GUNPOWDER_HEAT * (1.0 - dist2.sqrt() / radius as f32));
        }
    }
}

/// Per-frame: any gunpowder with a hot neighbour detonates.
fn check_gunpowder_ignition(grid: &mut Vec<Vec<Cell>>) {
    let mut to_detonate: Vec<(usize, usize)> = Vec::new();
    for r in 0..ROWS {
        for c in 0..COLS {
            if grid[r][c].cell_type != CellType::Gunpowder { continue; }
            for (dr, dc) in NEIGHBOURS_4 {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
                let n = grid[nr as usize][nc as usize];
                if matches!(n.cell_type, CellType::Fire | CellType::OilFire | CellType::Lava)
                    || n.temperature > 250.0 {
                    to_detonate.push((r, c));
                    break;
                }
            }
        }
    }
    for (r, c) in to_detonate {
        detonate(grid, r, c);
    }
}
```

Call it from `step`, after `try_react` and before the rising/falling passes:

```rust
pub fn step(grid: &mut Vec<Vec<Cell>>) {
    reaction_pass(grid);
    check_gunpowder_ignition(grid);
    rising_pass(grid);
    falling_pass(grid);
    cool_pass(grid);
}
```

The `move` keyword in `flat_map(move |dr| ...)` is necessary because the inner closure (`map(move |dc| (dr, dc))`) captures `dr` by value — without `move`, `dr` would be borrowed from the outer closure's environment and the borrow checker would complain that the borrow doesn't live long enough.

This is the most common single-line idiom that confuses Rust learners. The rule is simple: **when nesting closures that produce iterators, `move` the outer closure(s) so the inner ones can capture by value.**

### 5. Glass formation — 4 minutes

Glass is made by sand + lava in the reactions table. In `reactions.rs`:

```rust
    // -- Glass formation --
    // Sand + Lava: glass + lava (lava persists, sand fuses).
    r.insert((Sand, Lava), ReactionOutcome::replace_both(Glass, Lava, 80.0));
    r.insert((Lava, Sand), ReactionOutcome::replace_both(Lava, Glass, 80.0));
```

That's it for glass — three lines.

### 6. Recipe gates — 2 minutes

In `recipes.rs`:

```rust
    // Gunpowder: discovered by holding fire near smoke (charcoal proxy).
    recipes.push(Recipe {
        name: "Gunpowder",
        unlocks: CellType::Gunpowder,
        predicate: Box::new(|grid| {
            adjacent_pair(grid, CellType::Fire, CellType::Smoke)
        }),
    });

    // Glass: discovered the first time sand-and-lava meet.
    recipes.push(Recipe {
        name: "Glass",
        unlocks: CellType::Glass,
        predicate: Box::new(|grid| {
            adjacent_pair(grid, CellType::Sand, CellType::Lava)
        }),
    });
```

### 7. Optional: visual shockwave — 3 minutes

A two-frame yellow ring at the explosion radius is a polish touch:

```rust
// In detonate, after the cell-affecting loop:
flash_ring(row, col, radius);

fn flash_ring(_r: usize, _c: usize, _radius: i32) {
    // Stash (r, c, radius, time) in a global Vec<EffectMarker> and let
    // rendering.rs draw a yellow circle outline that fades over 12 frames.
}
```

Adds ~30 lines for a noticeable visual upgrade.

**Save. Run.** Discover both elements (place sand next to lava → glass unlocks; fire next to smoke → gunpowder unlocks). Build a wood structure. Place gunpowder under it. Drop one fire cell. **The structure detonates** — wood goes flying, fire scatters in a circle, heat propagates outward, smoke billows. Now build a glass dome over a small chamber — light it up, watch the fire stay contained by transparent walls.

> **The Wow Moment.** Build a long horizontal line of gunpowder cells, like a fuse. Place fire at one end. **Watch the chain detonation race down the line at one cell per frame, leaving a crater behind it.** That's gunpowder being lit by gunpowder being lit by gunpowder — a self-perpetuating reaction. **Your physics simulator just learned about chain reactions in the gameplay sense, not just the chemistry sense.** Save the clip; it's the best demo gif you'll ever produce.

---

## Linux (Ubuntu) note

The explosion involves a burst of audio (the `boom.wav` from Session 16). On Ubuntu specifically:

- **Audio latency.** PipeWire's default buffer can be 30-60ms. A chain-detonation fuse may play `boom` ten times in 200ms — close to the buffer length. Some triggers will be dropped (mixer can't keep up). Cosmetic. To tighten:

  ```bash
  export PIPEWIRE_LATENCY=128/48000
  cargo run --release
  ```

  Halves the latency at the cost of higher CPU. If you're running an Ubuntu laptop on battery, the higher CPU may matter.

- **`fastrand::f32` thread safety.** `fastrand` uses a thread-local RNG by default, which is the right thing for a single-threaded sim. If you ever rayon-parallelise the detonation pass, each rayon thread gets its own RNG automatically. No code change needed.

- **Performance.** The `(-radius..=radius).flat_map(...).filter(...).collect()` pattern allocates a Vec per detonation. With `GUNPOWDER_RADIUS = 6`, that's ~113 cells per allocation. A long fuse with 50 detonations per second is ~6,000 cell positions allocated and freed per second. Negligible. If you ever scale up, reuse a single `Vec` (clear it each detonation).

---

## Common mistakes

### `error: closure may outlive borrowed value`

The classic. You have a closure that captures `dr` by reference, but `dr` is local to the outer closure. Add `move` to the outer closure. If that doesn't fix it, also add `move` to the inner. The borrow checker is telling you: "the inner closure needs to own its captures because it lives longer than the outer scope."

### Detonation leaves a square crater, not a circle

You forgot the `.filter(|(dr, dc)| dr*dr + dc*dc <= radius*radius)`. Without it, the iterator yields a square grid of offsets. Add the filter back.

### Chain detonation skips cells

You're iterating `for r in 0..ROWS` and *mutating* the grid mid-iteration. The list of cells to detonate is built first (`to_detonate`), and *then* applied. If you collapsed the two phases into one, gunpowder cells get cleared before being seen. Always two-phase: collect targets, then apply effects.

### `error[E0507]: cannot move out of grid`

Inside the iterator chain you tried to take ownership of a grid cell. Iterators over `&Vec` yield `&T`. Use `.copied()` (when T: Copy) or `.cloned()` (when T: Clone) to materialise a value, or pattern-match by reference: `|(dr, dc)| dr*dr + dc*dc <= ...`.

### Glass doesn't form

The reaction is in the table, but the recipe pass hasn't unlocked it — so you can't *place* sand. Wait. To test: temporarily unlock everything by hand. `discoveries.unlocked = catalogue().into_iter().map(|e| e.cell_type).collect();`.

### Gunpowder detonates from any heat

`temperature > 250.0` is the threshold. If your heatmap shows lava sitting at 1200°, then any adjacent gunpowder ignites correctly. If wood ignition heat radiates above 250°, gunpowder placed near burning wood will also detonate — intentional or not? Tune the threshold.

---

## Session challenge

Pick one — no solution provided.

1. **Variable-radius detonations.** Gunpowder cells store a `power: u8` field. Larger power → larger radius. Build a "demolition" workflow where you stack powders for compound effect.
2. **Glass with thickness.** Glass that's been hit by heat above 1000°C cracks (visual: render with a darker outline) and breaks (becomes sand) on next contact with a non-glass cell.
3. **Shrapnel.** When gunpowder detonates, scatter "shrapnel" — a few new cells of the previously-occupying type, hurled outward into nearby empty space. (Very satisfying; non-trivial.)
4. **The fuse pattern as a function.** Generalise the chain-detonation into `fn chain_react(grid: &mut, start: (usize, usize), trigger: impl Fn(Cell)->bool, effect: impl FnMut(usize, usize))`. Use it for future cascading effects.

---

## Quick reference

| What | Code |
|---|---|
| Inclusive range | `0..=10` (includes 10) |
| Half-open range | `0..10` (excludes 10) |
| `flat_map` | `iter.flat_map(\|x\| inner_iter(x))` |
| `chain` | `iter1.chain(iter2)` |
| Move closure | `move \|x\| x + captured` |
| Collect to Vec | `.collect::<Vec<_>>()` |
| Tuple destructure | `\|(a, b)\| ...` |
| Squared distance | `dr*dr + dc*dc` (avoid sqrt) |
| `&Vec<T>` iter | `v.iter()` yields `&T` |
| `&mut Vec<T>` iter | `v.iter_mut()` yields `&mut T` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 21**. Worth recording:

- Your chain-detonation fuse clip
- Your sentence on `move`: "what does adding `move` to a closure actually change?" — this is interview question candy
