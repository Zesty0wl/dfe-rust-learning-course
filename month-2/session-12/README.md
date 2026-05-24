# Session 12 — Oil and Explosive Reactions

> **Stuck on a word?** Things like *flammable*, *neighbourhood*, *Moore neighbourhood*, *cascade* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your sandbox has **oil** — denser than water, highly flammable, ignites violently — and dropping fire onto an oil slick produces a **chain reaction** that races across the surface in a couple of seconds.

---

## What you'll learn

- `Vec` patterns you haven't used yet: building a list of neighbour offsets, iterating over `&[T]`
- **Moore neighbourhood** (8 surrounding cells) vs **Von Neumann neighbourhood** (4 cardinal)
- Tuning multiple probabilities together for "cascading" behaviour
- Why oil-on-water and water-on-oil are *different* (density)
- A first taste of designing **per-element constants** in a single block

---

## The big idea

Three different elements, three different flammabilities:

- **Wood** — 20% spread chance, burns slow, leaves smoke.
- **Oil** — 80% spread chance, burns fast, big heat output.
- **(Coming: gunpowder, Session 21)** — explodes outward in a radius rather than spreading per neighbour.

That spectrum gives the simulation real *character*. A wood fire is something you live with; an oil fire is something you flee from. Both are the same code path, just different numbers.

You'll also meet the Moore neighbourhood — the *eight* cells around a target, not just the four. Oil spreads better diagonally than wood does, because liquid surfaces touch each other along diagonals. Switching neighbourhood is a one-line change with surprisingly different visual results.

---

## Concepts covered

- Defining a const array of `(i32, i32)` neighbour offsets
- Iterating with `for (dr, dc) in NEIGHBOURS_MOORE`
- Two new variants: `Oil`, `OilFire`
- Density-based swap: oil floats on stone, water floats on oil
- Tuned constants: `OIL_SPREAD_CHANCE`, `OIL_BURN_TICKS`, `OIL_IGNITE_TEMP`

---

## Building towards `sand-sim`

Oil is the **first element with non-trivial physics interactions**: it falls like water, floats above stone but below water, and *cascades* under fire. Every subsequent element borrows part of this template — lava (Session 15) falls like oil and ignites like oil-fire; gunpowder (Session 21) uses oil's neighbour-list pattern with a much bigger neighbourhood. Today's per-element constants block is the model you'll keep extending.

---

## Step-by-step walkthrough

> **Where you should be.** Session 11 finished. Fire spreads probabilistically along wood, burns out, leaves smoke. The grid is `Vec<Vec<Cell>>` with `cell_type`, `temperature`, and `lifetime` fields.

### 1. Two new variants — 2 minutes

Add to the enum:

```rust
enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
    Wood,
    Fire,
    Smoke,
    Oil,        // dark green-brown, flammable
    OilFire,    // burning oil — same shape as Fire but hotter and shorter
}
```

The compiler will list every match-without-wildcard that needs the new arms. Fix `colour`:

```rust
CellType::Oil      => Color::new(0.20, 0.18, 0.10, 1.0),  // dark olive
CellType::OilFire  => Color::new(1.00, 0.65, 0.20, 1.0),  // bright yellow-orange
```

Add to your selector array and a key binding:

```rust
let elements = [
    CellType::Sand, CellType::Water, CellType::Stone,
    CellType::Wood, CellType::Fire, CellType::Oil,
];

if is_key_pressed(KeyCode::Key6) { selected = CellType::Oil; }
```

### 2. The neighbour helper — 3 minutes

Above your update functions, define both neighbourhoods so you can pick per element:

```rust
/// Von Neumann (4 cardinal directions).
const NEIGHBOURS_4: [(i32, i32); 4] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
];

/// Moore (all 8 surrounding cells).
const NEIGHBOURS_8: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];
```

These are `const` arrays — exact same idea as Session 2's element constants, scaled up. Once defined, they're reusable across all per-element update functions.

### 3. Per-element constants block — 2 minutes

Make oil's tuning explicit and grouped:

```rust
// Oil tuning.
const OIL_SPREAD_CHANCE:    f32 = 0.80;    // very flammable
const OIL_BURN_TICKS:       u8  = 30;      // fast burn
const OIL_HEAT_RADIATE:     f32 = 18.0;
const OIL_IGNITE_TEMP:      f32 = 150.0;   // auto-ignites above this
```

Compare with fire's constants from Session 11. Side-by-side numbers like these are *the* readable way to document elemental behaviour. By Session 14 you'll have a dozen of these blocks and they'll be the simulation's design doc.

### 4. Oil falling and floating — 5 minutes

`update_oil` follows water's shape but with **density-aware** behaviour: oil is *less dense* than water (so oil rises through water) and *more dense* than air (so it falls through air). The simplest expression:

```rust
fn update_oil(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    // 1. Auto-ignite if hot enough (this is the "secondary ignition" trick).
    if grid[row][col].temperature >= OIL_IGNITE_TEMP {
        grid[row][col] = Cell {
            cell_type: CellType::OilFire,
            temperature: grid[row][col].temperature,
            lifetime: OIL_BURN_TICKS,
        };
        return;
    }

    // 2. Try to fall straight down through empty or water (because oil is denser than air, lighter than water).
    if row + 1 < ROWS {
        let below = grid[row + 1][col].cell_type;
        if matches!(below, CellType::Empty) {
            let me = grid[row][col];
            grid[row + 1][col] = me;
            grid[row][col]     = Cell::empty();
            return;
        }
        if matches!(below, CellType::Water) {
            // Oil should not sink into water; instead try sideways.
            // (Density: oil < water, so oil floats above water.)
        }
    }

    // 3. Diagonal-down (same as water, but only into empty).
    let try_left = fastrand::bool();
    let order: [i32; 2] = if try_left { [-1, 1] } else { [1, -1] };
    for dx in order {
        let nc = col as i32 + dx;
        if nc < 0 || nc >= COLS as i32 { continue; }
        let nc = nc as usize;
        if row + 1 < ROWS && grid[row + 1][nc].is_empty() {
            grid[row + 1][nc] = grid[row][col];
            grid[row][col]    = Cell::empty();
            return;
        }
    }

    // 4. Sideways spread.
    for dx in order {
        let nc = col as i32 + dx;
        if nc < 0 || nc >= COLS as i32 { continue; }
        let nc = nc as usize;
        if grid[row][nc].is_empty() {
            grid[row][nc]  = grid[row][col];
            grid[row][col] = Cell::empty();
            return;
        }
    }
}
```

That's water's body of code with the auto-ignite at the top and a couple of `matches!` checks. **Notice how the four-step structure — auto-ignite, down, diagonal-down, sideways — will be reused for lava and acid later.**

Also let water swap with oil to model the density inversion. In `update_water`, just before the "try sideways" step, add:

```rust
    // Oil floats on water — if there's oil directly below, swap up.
    if row + 1 < ROWS && matches!(grid[row + 1][col].cell_type, CellType::Oil) {
        let me = grid[row][col];
        grid[row][col]     = grid[row + 1][col];
        grid[row + 1][col] = me;
        return;
    }
```

### 5. The oil-fire update — 5 minutes

Almost identical to fire from Session 11, but with oil's hotter, shorter constants and the Moore neighbourhood:

```rust
fn update_oil_fire(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    let lifetime = grid[row][col].lifetime;
    if lifetime == 0 {
        // Oil burns hotter; almost always leaves smoke.
        grid[row][col] = Cell {
            cell_type: CellType::Smoke,
            temperature: grid[row][col].temperature.max(100.0),
            lifetime: 60,
        };
        return;
    }
    grid[row][col].lifetime = lifetime - 1;

    // Spread through the 8 surrounding cells (oil sheets spread along diagonals).
    for (dr, dc) in NEIGHBOURS_8 {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);

        grid[nr][nc].heat(OIL_HEAT_RADIATE);

        if matches!(grid[nr][nc].cell_type, CellType::Oil) && fastrand::f32() < OIL_SPREAD_CHANCE {
            grid[nr][nc] = Cell {
                cell_type: CellType::OilFire,
                temperature: 400.0,
                lifetime: OIL_BURN_TICKS,
            };
        }
        // Bonus: oil-fire can ignite wood too, but at lower probability.
        if matches!(grid[nr][nc].cell_type, CellType::Wood) && fastrand::f32() < 0.10 {
            grid[nr][nc] = Cell::new(CellType::Fire);
        }
    }
}
```

Dispatch it:

```rust
fn update_cell(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    match grid[row][col].cell_type {
        CellType::Sand    => update_sand    (grid, row, col),
        CellType::Water   => update_water   (grid, row, col),
        CellType::Oil     => update_oil     (grid, row, col),
        CellType::Fire    => update_fire    (grid, row, col),
        CellType::OilFire => update_oil_fire(grid, row, col),
        CellType::Smoke   => update_smoke   (grid, row, col),
        _ => {}
    }
}
```

(`update_oil` goes in the bottom-to-top pass; `update_oil_fire` in the top-to-bottom pass, alongside fire.)

**First runnable checkpoint.** Run. Paint a horizontal pool of oil. Drop a single fire cell at one end. **Watch the oil ignite end-to-end in about a second, the whole surface becoming a sheet of yellow-orange flame.**

> **The Wow Moment.** Build a stone bowl. Fill the bottom third with water. Pour oil on top — see it sit *above* the water (density-correct). Drop a fire cell on the oil. **The entire oil surface lights up in a chain reaction, the water boils underneath from radiated heat, smoke billows up, and the fire eventually exhausts itself when the oil's gone — leaving you with a still-warm pool of water.** That whole emergent narrative came from ~40 lines of `update_oil` + `update_oil_fire` plus per-element constants. Save a clip; this is your strongest evidence-of-emergent-behaviour moment so far.

### 6. (Optional) Oil-on-fire is more violent than wood-on-fire — 3 minutes

Add to your `react` function (from Session 10):

```rust
// Fire-on-oil = oil ignites with bonus heat.
(CellType::Fire, CellType::Oil) | (CellType::Oil, CellType::Fire) => Some(ReactionOutcome {
    new_source: CellType::OilFire,
    new_target: CellType::OilFire,
    heat: 120.0,
}),
```

Now even non-spreading per-neighbour reactions can detonate oil. The "spread" of fire was probabilistic and slow; this reaction is *instant* on direct contact. Together they produce the characteristic feel of oil ignition: a small flame quickly turns into a sheet of fire.

---

## Linux (Ubuntu) note

This session is the busiest yet — three update passes, a reactions pass, and per-cell heat radiation. On a typical Ubuntu laptop you should still hit 60 FPS in `--release`. If you're below:

- **Profile with `perf`** (Ubuntu's profiler — install with `sudo apt install -y linux-tools-common linux-tools-generic`):

  ```bash
  cargo build --release
  perf record --call-graph=dwarf ./target/release/sand-sim
  # play with the sim for ~10 seconds, then close it
  perf report
  ```

  Almost certainly the heat-radiation loop dominates — fine to leave alone, it's only 8 cells × non-empty cells per frame.
- **Wayland transparency.** The `Smoke` colour uses alpha `0.65` (semi-transparent grey). Under some Wayland compositors this can render as opaque grey instead. If you want the transparency to definitely work, alpha-blend by hand: read the cell *behind* the smoke and `mix_colours` in `render_colour`. Cleaner solution that doesn't depend on the compositor.
- The Moore neighbourhood loop touches 8 cells per fire cell. If `cargo run --release` ever stops working at 60 FPS even on a desktop, lower `COLS` and `ROWS` (the simulation is O(rows × cols)).

---

## Common mistakes

### Oil sinks into water

You forgot the "oil floats on water" swap in `update_water`. Without it, oil falls and water falls, but neither knows about the density difference. The swap in step 4 is the fix.

### Oil ignition cascades but then *spreads* indefinitely

You forgot to decrement `lifetime` in `update_oil_fire`. The fire never expires. Confirm `grid[row][col].lifetime = lifetime - 1;` (after the `if lifetime == 0` check).

### `error[E0277]: 'CellType' doesn't implement 'Copy'` after adding variants

Adding a variant doesn't remove `Copy` from the enum, but if you accidentally introduced a variant that holds a non-`Copy` payload (e.g. `Oil(Vec<u8>)`), the whole enum loses `Copy`. Keep variant payloads to `Copy`-types (`u8`, `f32`, tuples of those).

### Oil-fire spreads through stone walls

The Moore neighbourhood includes diagonals. A wall of stone one cell thick won't block diagonal spread — the diagonal cell touches the next column. Either use a 2-thick wall, or gate spread on "stone is between source and target." (Realistic? No. But it produces a tighter visual.) Most simulators accept the diagonal leak.

### Frame rate tanks when oil pool ignites

Each oil-fire cell does an 8-neighbour pass. A 50×50 oil pool = 2500 fire cells × 8 = 20,000 neighbour visits per frame. Still fine on most machines in `--release`, sluggish in debug. **Always `--release` for the wow-moment demos.**

---

## Session challenge

Pick one — no solution provided.

1. **Burning sand → glass.** When sand's temperature exceeds 600°C, turn it into a new `CellType::Glass` cell that's static and transparent. (You'll be tempted to do this with a reaction; the cleaner way is to check temperature inside `update_sand`.) Foreshadows Session 21.
2. **Oil floats on water, water floats on lava (Session 15 preview).** Add the inverse swap to oil's update: if there's water directly above, swap up. Now oil and water reliably stratify by density even after stirring.
3. **Wind affects oil-fire.** Wire wind direction into the spread probabilities: with wind blowing left, oil-fire's `(0, -1)` neighbour gets 2× spread chance, `(0, 1)` gets 0.5×.
4. **Oil-fire jets.** When oil-fire spreads to an oil cell, with 5% probability, also ignite the cell *two* squares away in the same direction. Visually models "the flame jumped."

---

## Quick reference

| What | Code |
|---|---|
| Const array | `const NEIGHBOURS_4: [(i32, i32); 4] = [...]` |
| Iterate offsets | `for (dr, dc) in NEIGHBOURS_4 { ... }` |
| Per-element constants | `const OIL_SPREAD_CHANCE: f32 = 0.80;` |
| Density swap | swap up if below cell is lighter |
| Multiple cells touch a cell | use Moore (`NEIGHBOURS_8`) |
| `matches!` with multiple arms | `matches!(c, CellType::Oil \| CellType::Wood)` |
| Profile a release build (Ubuntu) | `perf record --call-graph=dwarf ./target/release/sand-sim` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 12**. Worth recording:

- A clip of the stone-bowl / water / oil / fire wow moment from step 5
- A short paragraph contrasting "wood fire" and "oil fire" *as physics simulations* — what changed in your code, and why the visual result is so different
