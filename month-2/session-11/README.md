# Session 11 — Fire

> **Stuck on a word?** Things like *probability*, *lifetime field*, *seed*, *jitter* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **fire spreads probabilistically, burns out over time, and heats the cells around it** — the same fire you've ever seen in any pixel-art simulator. Plus a second new element: **smoke**, which rises and dissipates.

---

## What you'll learn

- `fastrand::f32()` for probabilities — "fires this rule with 30% chance"
- Adding a **`lifetime` field** to your `Cell` struct, so fire knows how long it's been burning
- Top-to-bottom iteration order for *rising* elements (smoke), the mirror of Session 3's bottom-to-top
- Heat radiating to non-reacting neighbours
- Tuning numbers until the visual effect "feels right" — the *design* layer of simulation

---

## The big idea

Fire isn't a single rule. It's three rules running simultaneously:

1. **Spread** — each frame, with some probability, fire ignites adjacent flammable cells.
2. **Burn out** — fire is consumed by its own existence; after N frames it becomes empty (or smoke).
3. **Radiate heat** — even non-reactive neighbours warm up just from proximity.

The third rule is the one that gives fire its *menace*. A flame near wood doesn't just stay still and wait for the wood to touch it — it heats it, and the heating-vs-cooling balance is what makes flame look alive.

You'll express all three with the data structure from Session 9 (`Cell.temperature`) plus one new field (`lifetime`) and a single probability constant. **Three numbers and you have fire.**

---

## Concepts covered

- `fastrand::f32() < 0.3` — fixed-probability gating
- Adding a struct field: `lifetime: u8`
- `cell.lifetime = cell.lifetime.saturating_sub(1);`
- Iteration order for upward-moving cells (top-to-bottom)
- A heat-radiation pass separate from reactions
- "Designer numbers" — constants like `FIRE_BURN_TICKS = 60` that you'll tune by feel

---

## Building towards `sand-sim`

Today, fire becomes a *first-class* element with all the properties — spread, decay, heat output. Every later session that uses fire (oil ignition Session 12, lava Session 15, gunpowder Session 21) reuses today's three-rule shape. The `lifetime` field also unlocks ice melting (Session 15), concrete setting (Session 22), and steam condensing (Session 13).

---

## Step-by-step walkthrough

> **Where you should be.** Session 10 finished. Wood ignites on contact with fire. There's a `react` function returning `Option<ReactionOutcome>`. The grid is `Vec<Vec<Cell>>` and each cell has `cell_type` and `temperature`.

### 1. Add a `lifetime` field — 3 minutes

In your `Cell` struct:

```rust
#[derive(Debug, Clone, Copy)]
struct Cell {
    cell_type:   CellType,
    temperature: f32,
    lifetime:    u8,        // counts down; 0 = expired
}

impl Cell {
    fn new(cell_type: CellType) -> Self {
        let lifetime = match cell_type {
            CellType::Fire => 60,     // ~ 1 second at 60fps
            _              => 0,
        };
        Cell { cell_type, temperature: 20.0, lifetime }
    }

    fn empty() -> Self {
        Cell::new(CellType::Empty)
    }
}
```

`cargo check`. The compiler lists every spot that built `Cell { ... }` manually (the `paint` function from Session 9) and demands the new field. Replace those with `Cell::new(cell_type)` so the lifetime default kicks in automatically, or pass `0` explicitly:

```rust
grid[r][c] = Cell { cell_type, temperature, lifetime: 0 };
```

### 2. The fire update rule — 8 minutes

Add a dedicated `update_fire` function:

```rust
const FIRE_SPREAD_CHANCE: f32 = 0.20;   // chance per neighbour per frame
const FIRE_HEAT_RADIATE:  f32 = 8.0;    // °C per frame per neighbour
const FIRE_BURN_TICKS:    u8  = 60;     // how long a fresh fire lasts

fn update_fire(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    // 1. Burn-out: decrement lifetime; if zero, become smoke (or empty).
    let lifetime = grid[row][col].lifetime;
    if lifetime == 0 {
        // 30% chance of leaving smoke behind, 70% of just disappearing.
        if fastrand::f32() < 0.3 {
            grid[row][col] = Cell {
                cell_type:   CellType::Smoke,
                temperature: grid[row][col].temperature,
                lifetime:    40,
            };
        } else {
            grid[row][col] = Cell::empty();
        }
        return;
    }
    grid[row][col].lifetime = lifetime - 1;

    // 2. Spread + radiate heat: look at all four neighbours.
    for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);

        // Always radiate heat (cheap; doesn't change cell type).
        grid[nr][nc].heat(FIRE_HEAT_RADIATE);

        // Maybe spread to flammable cells.
        if matches!(grid[nr][nc].cell_type, CellType::Wood) && fastrand::f32() < FIRE_SPREAD_CHANCE {
            grid[nr][nc] = Cell::new(CellType::Fire);
        }
    }
}
```

Dispatch it from `update_cell`:

```rust
fn update_cell(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    match grid[row][col].cell_type {
        CellType::Sand   => update_sand (grid, row, col),
        CellType::Water  => update_water(grid, row, col),
        CellType::Fire   => update_fire (grid, row, col),
        CellType::Smoke  => update_smoke(grid, row, col),
        _ => {}
    }
}
```

(Smoke comes next.)

**First runnable checkpoint.** `cargo run`. Drop a single fire cell on top of a wood beam. It glows, spreads probabilistically along the beam, and the neighbours visibly warm to red. Where the fire passed, smoke now occasionally lingers. **The wood doesn't *all* ignite at once any more** — there's a real flicker, because the spread is probabilistic.

### 3. The smoke update rule — 4 minutes

Smoke rises and decays. The iteration direction is *top-to-bottom* (so a rising cell isn't re-updated after moving up this frame).

```rust
const SMOKE_RISE_CHANCE: f32 = 0.6;

fn update_smoke(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    // Decay first.
    let lifetime = grid[row][col].lifetime;
    if lifetime == 0 {
        grid[row][col] = Cell::empty();
        return;
    }
    grid[row][col].lifetime = lifetime - 1;

    // Try to rise (the inverse of sand falling).
    if row == 0 || fastrand::f32() > SMOKE_RISE_CHANCE { return; }

    if grid[row - 1][col].is_empty() {
        grid[row - 1][col] = grid[row][col];
        grid[row][col]     = Cell::empty();
        return;
    }

    // Blocked — try diagonally up.
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

And add the variant + colour:

```rust
// In CellType enum:
Smoke,

// In CellType::colour:
CellType::Smoke => Color::new(0.30, 0.30, 0.30, 0.65),  // semi-transparent grey
```

Now `step` needs to handle both directions. Split it:

```rust
fn step(grid: &mut Vec<Vec<Cell>>) {
    // Reactions (Session 10).
    for row in 0..ROWS {
        for col in 0..COLS {
            try_react(grid, row, col);
        }
    }
    // Fire-rule and smoke (both work better top-to-bottom).
    for row in 0..ROWS {
        for col in 0..COLS {
            match grid[row][col].cell_type {
                CellType::Fire  => update_fire (grid, row, col),
                CellType::Smoke => update_smoke(grid, row, col),
                _ => {}
            }
        }
    }
    // Falling elements (bottom-to-top).
    for row in (0..ROWS - 1).rev() {
        for col in 0..COLS {
            match grid[row][col].cell_type {
                CellType::Sand  => update_sand (grid, row, col),
                CellType::Water => update_water(grid, row, col),
                _ => {}
            }
        }
    }
}
```

**Save. Run.** Drop fire on a wood beam. Watch it spread, glow, burn out, and *smoke rises away from the embers*.

> **The Wow Moment.** Build a tall stack of wood (full vertical column). Drop a single fire cell at the bottom. **Watch the burn travel up the column, smoke trailing above, embers fading at the bottom, the whole thing breathing.** Pause (Space). Look at the screen. *That is fire.* You wrote three rules — spread, burn-out, radiate — and the visual is indistinguishable from what real fire looks like in a 2D game. (Compare your output to any of the Noita / Powder Toy fire gifs on YouTube. You're in the same neighbourhood.)

### 4. Tune until it feels right — 4 minutes

Open `FIRE_SPREAD_CHANCE` and try different values:

- `0.05` — fire crawls; a beam takes 30 seconds to consume.
- `0.20` — current default; brisk but watchable.
- `0.5` — explosive; a beam goes up in under a second.

Same with `FIRE_BURN_TICKS`:

- `30` — fire dies before it can spread much.
- `60` — current default; balanced.
- `120` — fire lingers; long burns; lots of smoke.

**There's no correct value.** This is the *design* layer of simulation. Pick numbers that make the thing feel the way you want it to feel. Different elements in Month 2 will use very different probabilities — oil in Session 12 uses 0.8 (highly volatile), acid in Session 14 uses 0.05 (corrodes slowly).

### 5. (Optional) Wind — 3 minutes

Add a global wind direction:

```rust
let mut wind_dx: i32 = 0;   // -1 = leftward, 0 = none, 1 = rightward

if is_key_pressed(KeyCode::Left)  { wind_dx = -1; }
if is_key_pressed(KeyCode::Right) { wind_dx =  1; }
if is_key_pressed(KeyCode::Up)    { wind_dx =  0; }
```

In `update_smoke`, bias the diagonal pick:

```rust
let order: [i32; 2] = if wind_dx < 0 { [-1, 1] }
                       else if wind_dx > 0 { [1, -1] }
                       else if fastrand::bool() { [-1, 1] } else { [1, -1] };
```

Now arrow keys steer the smoke plume. Easy to extend: bias the fire spread too, so wind blows flames sideways.

---

## Linux (Ubuntu) note

Today's logic doubles the per-frame work: a reactions pass, a fire-and-smoke pass, plus the falling pass. On Ubuntu you might see FPS slip below 60 in debug mode with a busy grid.

- **Always `cargo run --release` on lower-end Ubuntu hardware** (Raspberry Pi 4, older Chromebooks running Linux). The release build is roughly 5–10× faster for this workload. Debug builds insert bounds-check assertions on every grid access.
- If you've installed `rust-analyzer` from Ubuntu's apt repo, it's usually a few releases behind. Prefer `rustup component add rust-analyzer` (after which the VS Code extension picks it up automatically) — it tracks the active Rust toolchain.
- The `fastrand` crate is deterministic given a seed. To reproduce a bug, call `fastrand::seed(42)` at the top of `main`. Useful on Linux where you can pipe `dmesg | grep ` to compare runs.

---

## Common mistakes

### Fire spreads instantly across the whole grid in one frame

You iterated `update_fire` in the bottom-to-top loop (alongside sand). Top-to-bottom iteration order would re-visit a just-ignited cell on the same frame. Make sure fire and smoke live in the *top-to-bottom* pass; sand and water in the bottom-to-top pass.

### Fire never burns out

You forgot to decrement `lifetime`. Or you initialised it to 0 in `Cell::new`. Confirm `CellType::Fire => 60` in the new-match. Test: paint a single fire cell, count seconds until it disappears. Should be ~1.

### Smoke piles up at the top

That's actually realistic — smoke that hits the ceiling stays there until it decays. If you want it to dissipate faster, halve the `lifetime` in `update_fire` when spawning smoke.

### `error[E0277]: the trait bound 'CellType: PartialEq' is not satisfied`

You added a new variant to `CellType` after wiring `Hash` on it. The derive should already include `Hash, PartialEq, Eq` from Session 6. Check the derive list still has all five (Debug, Clone, Copy, PartialEq, Eq, Hash).

### Fire heats walls and floors needlessly

That's fine — the heat just sits in the stone cell. Future sessions (lava ↔ stone reactions) will use that heat. If you'd rather only heat flammables, gate the radiation call with `matches!(grid[nr][nc].cell_type, CellType::Wood | CellType::Empty)`.

### Saturating subtraction surprise

If you wrote `grid[row][col].lifetime -= 1;` on a `u8` that's already 0, you'll panic in debug (overflow) or silently wrap to 255 in release. Either check `if lifetime > 0` first (as above), or use `lifetime = lifetime.saturating_sub(1);` which clamps at 0.

---

## Session challenge

Pick one — no solution provided.

1. **Embers.** When fire burns out, sometimes (10% chance) leave behind a darker, slower-burning "ember" cell that lives 90 frames and doesn't spread. Mostly cosmetic; lovely visual.
2. **Wood ignites by temperature alone.** Right now wood only burns from direct fire contact. Add: if wood's temperature > 250°C, it spontaneously ignites. Now standing next to a fire long enough catches you. (Real physics, in three lines.)
3. **Smoke condenses into water.** If smoke reaches the top row and stays there for 100 frames, turn it into water. *Closes the loop:* fire → smoke → water → drips back down → puts fire out. Self-extinguishing fires.
4. **Heat-map overlay.** Press `T` to toggle a debug view: render every cell at temperature > 100°C with a thin yellow border. Lets you see heat invisibly conducting through walls.

---

## Quick reference

| What | Code |
|---|---|
| Probability gate | `if fastrand::f32() < 0.3 { ... }` |
| Saturating subtraction | `n.saturating_sub(1)` |
| Saturating addition | `n.saturating_add(1)` |
| Lifetime per type | `match cell_type { CellType::Fire => 60, _ => 0 }` |
| Cardinal neighbours | `for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)]` |
| Two-phase step | reactions → top-to-bottom (rising) → bottom-to-top (falling) |
| Seed the RNG | `fastrand::seed(42)` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 11**. Worth recording:

- A short video clip of a wood beam burning end-to-end with smoke rising
- The values you ended up settling on for `FIRE_SPREAD_CHANCE` and `FIRE_BURN_TICKS`, and a sentence on *why* (this is "design taste" evidence — assessors love it)
