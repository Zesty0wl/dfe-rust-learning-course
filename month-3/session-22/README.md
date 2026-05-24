# Session 22 — Concrete, and Rust

> Yes, the pun is intentional. You're writing **Rust** (the language) to simulate **rust** (iron oxide). The compiler can't tell the difference; you can. Lean in.

> **Stuck on a word?** Things like *Instant*, *Duration*, *time-aware*, *state change* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **wet concrete** cures into hardened concrete after 30 seconds, and **iron** rusts (turns reddish-brown) over 2 minutes when exposed to water, oxygen, or both. Both behaviours use `std::time::Instant` to track elapsed game time per cell — the first time per-cell state has had a *time component* in this project.

---

## What you'll learn

- `std::time::Instant` and `std::time::Duration`
- `.elapsed()`, `.checked_add()`
- Adding a `set_at: Option<Instant>` field to `Cell`
- Time-thresholded state changes that read like real physics
- The difference between *frames-since* timing (Session 11 fire) and *seconds-since* timing (today's concrete)

---

## The big idea

Until now, time in your sim has meant *frames*. Fire burns for 60 frames; gunpowder cooldown is 30 frames. Frames are tied to your monitor's refresh rate — at 60 FPS, 60 frames = 1 second, fine. But if FPS drops, *everything slows down with it*. The fire that "should" burn for 1 second now burns for 1.5 seconds on a struggling machine.

`std::time::Instant` lets you ask the operating system for wall-clock time. Concrete cures over `Duration::from_secs(30)` regardless of FPS. Iron rusts over `Duration::from_secs(120)`. The behaviour is independent of frame rate — what physicists and engineers call *frame-rate independent timing*.

This is the discipline that separates toys from tools. Every shipped game uses both kinds of timing — frame-bound for tight animation loops, wall-clock for slow gameplay events. Today you do both.

---

## Concepts covered

- `Instant::now()`
- `Duration::from_secs(30)`, `Duration::from_millis(500)`
- `instant.elapsed()` returns a `Duration`
- `duration > Duration::from_secs(30)` is just `<` on durations
- `Option<Instant>` field on `Cell`
- Why time fields break naive `Copy` and how to keep `Cell: Copy`

---

## Building towards `sand-sim`

Concrete and iron-rust are the **realism-tier elements** — the ones that make a viewer say "oh, it's tracking *real time*?" Session 23 reuses `Instant` for the title-screen idle-timeout. Session 24's release readme cites time-based state changes as a v1.0 capability.

---

## Step-by-step walkthrough

> **Where you should be.** Session 21 finished. Gunpowder and glass work. Eleven elements (twelve counting empty), all with recipes.

### 1. Add the variants — 2 minutes

```rust
pub enum CellType {
    // ...
    Concrete,       // hardened
    WetConcrete,    // curing
    Iron,           // shiny grey
    Rust,           // crumbly red-brown
}

// colour:
CellType::Concrete    => Color::new(0.60, 0.60, 0.60, 1.0),  // mid grey
CellType::WetConcrete => Color::new(0.50, 0.55, 0.55, 1.0),  // wet, slightly darker
CellType::Iron        => Color::new(0.70, 0.70, 0.75, 1.0),  // silvery
CellType::Rust        => Color::new(0.55, 0.30, 0.20, 1.0),  // oxide red-brown
```

### 2. Extend `Cell` with a time field — 5 minutes

This is the structurally interesting change. Add `set_at: Option<Instant>` to `Cell`:

```rust
use std::time::Instant;

#[derive(Debug, Clone, Copy)]                                  // serde derives can stay
pub struct Cell {
    pub cell_type:   CellType,
    pub temperature: f32,
    pub lifetime:    u8,
    pub set_at:      Option<Instant>,                          // NEW
}
```

Wait — `Instant` doesn't implement `Serialize`. Two ways to fix:

**Option A: serde-skip the field.**

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type:   CellType,
    pub temperature: f32,
    pub lifetime:    u8,
    #[serde(skip)]
    pub set_at:      Option<Instant>,
}
```

`#[serde(skip)]` means "don't serialise this field, and on deserialise, default it." Loaded saves get `None` for `set_at`, which means "no curing in progress."

**Option B: store seconds-since-epoch as f64.** A bit cleaner because everything serialises naturally:

```rust
pub set_at_secs: Option<f64>,        // seconds since UNIX epoch
```

Either works. Option A is shorter for the simulation; option B is more portable. Use A.

Update `Cell::new`:

```rust
impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Self {
            cell_type,
            temperature: starting_temp(cell_type),
            lifetime:    starting_lifetime(cell_type),
            set_at:      starting_set_at(cell_type),
        }
    }
}

fn starting_set_at(t: CellType) -> Option<Instant> {
    match t {
        CellType::WetConcrete | CellType::Iron => Some(Instant::now()),
        _ => None,
    }
}
```

Iron starts a clock the moment it's placed; the clock is how we know when it's "old enough" to rust.

### 3. Concrete cures — 4 minutes

`update_wet_concrete`:

```rust
use std::time::Duration;

const CURE_TIME: Duration = Duration::from_secs(30);

fn update_wet_concrete(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    let cell = grid[row][col];
    if let Some(set_at) = cell.set_at {
        if set_at.elapsed() >= CURE_TIME {
            grid[row][col] = Cell {
                cell_type:   CellType::Concrete,
                temperature: cell.temperature,
                lifetime:    0,
                set_at:      None,
            };
            return;
        }
    }
    // Wet concrete also flows slowly like very-thick liquid.
    if fastrand::f32() < 0.05 {
        update_liquid(grid, row, col, CellType::WetConcrete);
    }
}
```

Wet concrete is sluggish (5% chance per frame to flow, vs. water's 100%). It's intuitively "thick." Then after `CURE_TIME` it transitions to `CellType::Concrete`, which is static like stone.

In `simulation.rs::step` dispatch:

```rust
match cell_type {
    CellType::WetConcrete => update_wet_concrete(grid, row, col),
    // ... etc ...
}
```

### 4. Iron rusts — 4 minutes

```rust
const RUST_TIME: Duration = Duration::from_secs(120);

fn update_iron(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    let cell = grid[row][col];
    let mut has_water_neighbour = false;
    for (dr, dc) in NEIGHBOURS_4 {
        let nr = row as i32 + dr; let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        let n = grid[nr as usize][nc as usize];
        if matches!(n.cell_type, CellType::Water | CellType::Acid | CellType::Steam) {
            has_water_neighbour = true;
            break;
        }
    }

    if !has_water_neighbour { return; }  // dry iron doesn't rust

    if let Some(set_at) = cell.set_at {
        if set_at.elapsed() >= RUST_TIME {
            grid[row][col] = Cell {
                cell_type:   CellType::Rust,
                temperature: cell.temperature,
                lifetime:    0,
                set_at:      None,
            };
        }
    }
}
```

Iron only rusts if a water (or steam or acid) cell is adjacent. Dry iron sits forever. Wet iron transitions after 120 seconds of contact.

### 5. Recipe gates — 2 minutes

In `recipes.rs`:

```rust
    // WetConcrete: from sand + water + stone (cement-like mix).
    recipes.push(Recipe {
        name: "Wet Concrete",
        unlocks: CellType::WetConcrete,
        predicate: Box::new(|grid| {
            // Hand-wave: three adjacent kinds in the same neighbourhood.
            count_adjacent_triple(grid, CellType::Sand, CellType::Water, CellType::Stone)
        }),
    });

    // Iron: from very high heat applied to stone (smelting).
    recipes.push(Recipe {
        name: "Iron",
        unlocks: CellType::Iron,
        predicate: Box::new(|grid| {
            cells_match(grid, |c| c.cell_type == CellType::Stone && c.temperature > 1500.0)
        }),
    });
```

The `count_adjacent_triple` helper is the same shape as `adjacent_pair` from Session 19 — left as a tiny exercise.

### 6. The reactions for placement — 2 minutes

When a player places `WetConcrete` while still wet, it spawns with `set_at: Some(Instant::now())` automatically (from `Cell::new`). Same for `Iron`. No extra wiring.

For symmetry, add a couple of reactions:

```rust
    // Iron + Lava = Iron (heat, but no transformation; iron is high-melting).
    // (Or: Iron + Lava = Lava — iron melts at the upper end of lava temp.
    //   Pick one. The realistic choice is "iron stays solid in lava in the short term.")

    // Rust + Acid = Empty (acid eats rust faster than iron).
    r.insert((Rust, Acid), ReactionOutcome::replace_both(Empty, Empty, 0.0));
    r.insert((Acid, Rust), ReactionOutcome::replace_both(Empty, Empty, 0.0));
```

### 7. Run and demo — 2 minutes

**Save. Run.**

Discover the recipes (cluster sand-water-stone for wet concrete; smelt stone with lava for iron). Place a row of wet concrete on a flat floor. **Look at the clock — note the time.** Carry on building. After 30 seconds, look back: **the wet concrete is now dry concrete.** Visibly *the same cells*, different colour, different behaviour.

Now place a few iron cells. Splash water on top. **Wait 2 minutes.** The iron turns red-brown. **Splash acid on the rust** — it dissolves immediately.

> **The Wow Moment.** Build a small structure with wet concrete walls. Notice they *flow* very slowly (the 5%/frame liquid pass). Watch for 30 seconds. **The walls stop flowing — they've cured.** Your sim now has materials that *change state over real, wall-clock time*. **You're running Rust to simulate rust** — the kind of joke that, when said in an interview, gets you remembered.
>
> (Yes, you're running a programming language called Rust to simulate the chemical process of rust forming on iron. Yes, the author of Rust chose the name partly because *rust* in fungi-and-chemistry contexts means "something that just *happens* over time without anyone tending it." The connection is real. Tell people.)

---

## Linux (Ubuntu) note

`Instant::now()` on Linux calls `clock_gettime(CLOCK_MONOTONIC, ...)` — a fast, monotonic, sub-microsecond-precision clock. Reading the clock 9,600 times per frame (once per cell) is well under 100 microseconds total. Safe to call freely.

- **Monotonic, not wall-clock.** `Instant` *cannot* go backwards. NTP adjustments don't affect it. So your concrete timers can't be glitched by the user changing the system time — important detail if you ever ship to malicious environments.

- **Don't serialise `Instant`.** It's a process-local handle. Even within the same process across save/load, an `Instant` from one boot may not be meaningful on another. The `#[serde(skip)]` choice in step 2 is the right one. Saved-and-reloaded concrete restarts its cure timer; saved-and-reloaded iron restarts its rust timer. That's defensible behaviour for a sim ("the cells are paused while saved").

- **For frame-rate-independent animation across save/load**, you'd want to serialise *elapsed* time as a `Duration` (which IS serialisable in serde via the `humantime-serde` crate). Skip for now; mention as a Session 24 polish item.

- Performance check (`perf` on Ubuntu):

  ```bash
  cargo build --release
  perf stat -e task-clock ./target/release/sand-sim
  ```

  Adding the time-based passes should add < 1% to CPU. If you see more, something is wrong (probably allocating per-cell).

---

## Common mistakes

### `error[E0277]: 'Instant' doesn't implement 'Serialize'`

You added `Instant` to a serde-derived struct without `#[serde(skip)]`. Add the attribute or use a serialisable replacement (`f64` seconds).

### Concrete cures instantly

Your `CURE_TIME` is `Duration::from_millis(30)` not `from_secs(30)`. Read your constants twice.

### Iron rusts even when dry

You forgot the `has_water_neighbour` guard, or you're checking `is_empty()` for the wrong thing. Print the neighbours for a single cell once.

### Concrete cures while paused

Your `step` function still runs `update_wet_concrete` even when the sim is paused — the time-based update doesn't care about pause. Either gate the `update_wet_concrete` call on `!paused`, or store *elapsed time outside pause* by recording when each pause started.

### `error: cannot copy 'Cell'` after adding the Instant field

`Instant` IS `Copy` (it's `Copy + Clone` since 1.0). If you're seeing this, you accidentally removed `Copy` from `Cell`'s derive elsewhere. Re-add.

### `Duration` math is awkward

Subtracting durations can panic on underflow (`Duration::from_secs(2) - Duration::from_secs(3)`). Use `.checked_sub()` returning `Option<Duration>`. Or stay in `.elapsed()` comparisons — those don't underflow.

### Save file size jumped

If you went with **option B** (storing `Option<f64>`), every cell now carries 9 bytes more. For a 120×80 grid, that's ~90KB extra in pretty JSON. Use option A or switch to bincode.

---

## Session challenge

Pick one — no solution provided.

1. **Carbonation.** Wet concrete near CO₂ (no such cell yet — invent it; smoke approximates) cures faster.
2. **Rust spreads.** Rust adjacent to iron makes the iron rust faster. Add a per-cell "rust progress" field; sharing-via-neighbour is a classic chain-reaction pattern.
3. **Tarnish for copper.** Add `CellType::Copper` and `CellType::Patina`. Copper exposed to *any* gas (smoke, steam, water vapour) develops patina over 90 seconds. Easy variation on iron.
4. **Earthquake.** Press `E` while paused: every wet-concrete cell with an interior cavity (an empty cell directly below) "collapses" downward, regardless of curing state. Demo: build a hollow wet-concrete dome; press `E`; watch it implode.

---

## Quick reference

| What | Code |
|---|---|
| Now (monotonic) | `Instant::now()` |
| Elapsed | `instant.elapsed()` (returns `Duration`) |
| Duration constants | `Duration::from_secs(30)`, `from_millis(500)` |
| Compare | `if start.elapsed() >= Duration::from_secs(30) { ... }` |
| `Option<Instant>` | `if let Some(t) = cell.set_at { ... }` |
| Serde-skip a field | `#[serde(skip)] pub field: Type,` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 22**. Worth recording:

- A timelapse of wet concrete curing — record the start (place cells, note timestamp), let it run, capture the end (cured cells, new timestamp). Real-time elapsed.
- Make the *Rust-to-simulate-rust* joke at least once aloud to a real person. Note their reaction in your log. (Engineering culture is built on these.)
