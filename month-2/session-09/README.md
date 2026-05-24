# Session 9 — Structs: Giving Cells a Temperature

> **Stuck on a word?** Things like *struct*, *field*, *method*, *associated function*, *self* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session every cell in your grid is a **`Cell` struct** with two fields — its type *and* its temperature — and hot cells glow with a warmer colour. You'll have a heat-source brush that drops cells in at 200°C.

---

## What you'll learn

- `struct` declarations and field access
- `impl` blocks for **methods** (`&self`, `&mut self`, `self`)
- **Associated functions** like `Cell::new(...)` (Rust's version of constructors)
- The shift from "cells store one value" to "cells store *structured* data"
- Temperature as a per-cell `f32` — and rendering a heat-map colour ramp

---

## The big idea

Month 1 finished with `Vec<Vec<CellType>>` — every cell knows what it is, nothing more. Month 2's chemistry needs more. Fire spreads because hot cells heat their neighbours; water boils when its temperature crosses 100; lava cools and solidifies. **Cells need to remember things.**

A `struct` is Rust's tool for "this thing has several related pieces of data." Today's struct is small — type + temperature — but the *shape* of the change is profound. Once your grid is `Vec<Vec<Cell>>`, every cell can hold extra state without another refactor. Session 11 adds a `lifetime` field for fire-burnout. Session 18 adds a `material_density` field for save/load. The struct is the seam where chemistry lives.

---

## Concepts covered

- `struct Cell { cell_type: CellType, temperature: f32 }`
- `impl Cell { fn new(...) -> Self { ... } }`
- `&self` (read-only borrow), `&mut self` (mutable borrow), `self` (own)
- Method call syntax: `cell.heat(10.0)` vs `Cell::new(CellType::Sand)`
- `#[derive(Debug, Clone, Copy)]` on small structs
- Interpolating two colours with a `mix` helper to render heat

---

## Building towards `sand-sim`

Today's struct is the **single most important data-structure decision** in the entire course. Every later element needs at least one of:

- **temperature** (fire heats, water cools)
- **lifetime** (fire burns out, steam condenses)
- **state** (concrete sets, metal rusts)

All three are fields on `Cell`. Add them as needed, and the rest of the codebase keeps working because `Cell::new(...)` defaults them to safe values.

---

## Step-by-step walkthrough

> **Where you should be.** Month 1 finished. `month-1/milestone/sand-sim-v0.1/` builds, runs, and is tagged `v0.1`. Today you start in a fresh folder: `month-2/milestone/sand-sim-v0.2/`.

### 0. Branch the project — 2 minutes

From the repo root:

```bash
mkdir -p month-2/milestone/sand-sim-v0.2
cp -R month-1/milestone/sand-sim-v0.1/. month-2/milestone/sand-sim-v0.2/
cd month-2/milestone/sand-sim-v0.2
cargo run     # should run identically to v0.1
```

Bump the package version in `Cargo.toml`:

```toml
[package]
name    = "sand-sim"
version = "0.2.0"
```

`month-2/session-09/starter/` and `month-2/session-09/solution/` (and same for sessions 10–14) work the same way as Month 1: stay-frozen snapshots. Day-to-day work happens in `month-2/milestone/sand-sim-v0.2/`.

### 1. Declare the `Cell` struct — 3 minutes

Just below `enum CellType` add:

```rust
#[derive(Debug, Clone, Copy)]
struct Cell {
    cell_type:   CellType,
    temperature: f32,
}

impl Cell {
    /// Make a cell of the given type at room temperature.
    fn new(cell_type: CellType) -> Self {
        Cell { cell_type, temperature: 20.0 }
    }

    /// A handy shortcut for an empty cell.
    fn empty() -> Self {
        Cell::new(CellType::Empty)
    }

    /// Read-only: is this an air cell?
    fn is_empty(&self) -> bool {
        matches!(self.cell_type, CellType::Empty)
    }

    /// Mutate: add some heat (in degrees).
    fn heat(&mut self, delta: f32) {
        self.temperature = (self.temperature + delta).min(2000.0);
    }
}
```

A few things to walk through:

- **`#[derive(Debug, Clone, Copy)]`** — same derives as for the enum. Because `CellType` is `Copy` and `f32` is `Copy`, the whole struct can be `Copy`.
- **`fn new(cell_type: CellType) -> Self`** is an **associated function** (no `self` parameter). Call as `Cell::new(...)`. Rust calls these "associated functions" not "constructors" because there's no special syntax \u2014 they're just functions namespaced inside `impl`.
- **`fn is_empty(&self)`** \u2014 the `&self` is shorthand for `&self: &Cell`. Read-only borrow. Call as `cell.is_empty()`.
- **`fn heat(&mut self, delta: f32)`** \u2014 mutable borrow. Call as `cell.heat(10.0)`. Won't compile if the cell variable wasn't `let mut`.
- **`matches!(...)`** \u2014 a tiny macro that returns `true` if the value matches the pattern. Saves writing a one-arm `match`.

### 2. Migrate the grid \u2014 4 minutes

Change every `Vec<Vec<CellType>>` to `Vec<Vec<Cell>>`. Top of `main`:

```rust
let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::empty(); COLS]; ROWS];
```

The compiler now lists every place that assumed plain `CellType`. The fixes are mechanical:

- `grid[r][c] = CellType::Sand;` \u2192 `grid[r][c] = Cell::new(CellType::Sand);`
- `if grid[r][c] == CellType::Empty` \u2192 `if grid[r][c].cell_type == CellType::Empty` (or `.is_empty()`)
- `grid[r][c].colour()` (from Session 6) \u2192 `grid[r][c].cell_type.colour()`
- The `paint(...)` helper in Session 7 took `cell: CellType`; either pass `Cell::new(selected)` from the call site, or change `paint`'s signature.

Run `cargo check` after each fix. By the time it's clean, sand still pours, water still flows \u2014 but every cell now has a temperature field, defaulting to 20.

**First runnable checkpoint.** `cargo run`. Visually identical to v0.1 \u2014 you haven't drawn anything new yet. The shape of the change is internal.

### 3. A heat-source brush \u2014 4 minutes

Add a "heat" key. In your input block:

```rust
        if is_key_pressed(KeyCode::H) {
            heat_source = !heat_source;
        }
```

And a brand-new mutable boolean above the loop: `let mut heat_source: bool = false;`.

When the heat-source mode is on, painted cells are dropped at 200\u00b0C:

```rust
fn paint(grid: &mut Vec<Vec<Cell>>, centre_row: i32, centre_col: i32, radius: i32,
         cell_type: CellType, temperature: f32) {
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx*dx + dy*dy > radius*radius { continue; }
            let r = centre_row + dy;
            let c = centre_col + dx;
            if r < 0 || r >= ROWS as i32 || c < 0 || c >= COLS as i32 { continue; }
            grid[r as usize][c as usize] = Cell {
                cell_type,
                temperature,
            };
        }
    }
}
```

Call sites become:

```rust
let temp = if heat_source { 200.0 } else { 20.0 };
paint(&mut grid, row, col, brush_radius, selected, temp);
```

### 4. Render the heat \u2014 6 minutes

The big visible win. Update the grid-drawing loop to interpolate towards red as temperature rises:

```rust
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

fn mix_colours(base: Color, hot: Color, t: f32) -> Color {
    Color::new(
        lerp(base.r, hot.r, t),
        lerp(base.g, hot.g, t),
        lerp(base.b, hot.b, t),
        1.0,
    )
}

impl Cell {
    fn render_colour(&self) -> Color {
        let base = self.cell_type.colour();
        let hot  = Color::new(1.0, 0.3, 0.1, 1.0);
        let t    = ((self.temperature - 20.0) / 200.0).clamp(0.0, 1.0);
        mix_colours(base, hot, t)
    }
}
```

Then update the drawing loop:

```rust
        for row in 0..ROWS {
            for col in 0..COLS {
                let cell = grid[row][col];
                if cell.is_empty() { continue; }
                let x = col as f32 * CELL_SIZE;
                let y = row as f32 * CELL_SIZE;
                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, cell.render_colour());
            }
        }
```

Save. Run. Press `H` to enable heat-source mode. Drop a brush of sand. **Glowing red sand.**

> **The Wow Moment.** Pour a stream of normal (cold) sand. It piles up beige. Press `H`. Pour another stream on top. The new sand drops in *glowing*. The pile is now a beige base with a hot orange ridge on top. **You've simulated temperature as a per-cell property** \u2014 the same data structure that Session 11 will use to make fire spread.

### 5. (Optional) Heat decays over time \u2014 3 minutes

In `step` (after the per-cell update), add a sweep that cools every non-empty cell by a tiny amount per frame:

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

Call it once at the end of `step`. Now hot sand cools to neutral over a couple of seconds. **The chemistry has begun.**

---

## Linux (Ubuntu) note

No new system dependencies this session \u2014 it's all type-system work. Two things worth knowing on Ubuntu:

- The grid is now ~`120 * 80 * sizeof(Cell)` = roughly 153,600 bytes (each `Cell` is `CellType` discriminant + `f32` + padding \u2248 16 bytes). Still trivially small. If `cargo run` ever feels sluggish on a low-RAM Ubuntu laptop, watch `htop` while the sim is running \u2014 memory should be flat at well under 50 MB.
- The heat-render colour mix uses three `f32` lerps per non-empty cell per frame. On a Pi 4 (32-bit Ubuntu) you may see FPS drop under 60 with a busy grid \u2014 `cargo run --release` recovers it fully. From now on, milestone testing should always be `--release` on lower-end hardware.

---

## Common mistakes

### `error[E0382]: borrow of moved value: 'cell'`

You forgot `Copy` in the derive. Without it, `let cell = grid[r][c]; ...; grid[r][c] = cell;` moves the value out, and the second access fails. Add `Copy` to the `#[derive(...)]` (and `Clone` if not there \u2014 `Copy` requires `Clone`).

### `error[E0594]: cannot assign to 'self.temperature' which is behind a '&' reference`

You wrote `fn heat(&self, delta: f32)` instead of `fn heat(&mut self, ...)`. Read-only methods take `&self`; mutating methods take `&mut self`. The compiler enforces the distinction.

### Heat colour clamps to red instantly

You forgot the `.clamp(0.0, 1.0)` in the `t` calculation, or the temperature range is too small. The formula `(self.temperature - 20.0) / 200.0` says \"linearly map 20\u00b0\u20132200\u00b0 to 0\u20131\". For 20\u00b0\u20131000\u00b0 use `/ 980.0`. Pick a value that visually matches what you want.

### Painting clears temperature back to 20\u00b0

You're calling `Cell::new(cell_type)` (which defaults to 20.0) instead of constructing the struct directly with the chosen temperature. Use `Cell { cell_type, temperature }` in `paint`.

### `matches!(self.cell_type, CellType::Empty)` won't compile

You wrote `matches!(self.cell_type, CellType::Empty())` with parens \u2014 only variants that hold data take parens (like `CellType::Wood(2)`). Plain unit variants don't.

---

## Session challenge

Pick one, no solution provided.

1. **A cool-source brush.** Mirror the heat brush \u2014 press `K` for \"cool\" mode \u2014 and drop cells at -50\u00b0C. Render cold cells with a blue tint by extending `render_colour` with a third anchor.
2. **Equilibrate heat with neighbours.** In the cool-pass, instead of decaying towards 20, average each non-empty cell's temperature with its four neighbours (skip empty). The result: heat *diffuses* through solid masses. Real conduction in eight lines.
3. **Click-to-inspect.** Hold `I` and hover the mouse \u2014 print the cell type and temperature of the cell under the cursor to the top-left. Excellent debugging tool you'll use in every later session.
4. **`impl Display for Cell`.** Manually implement `Display` so `println!(\"{}\", cell)` prints something like `Sand@200\u00b0C`. (Recall from Session 6 that `Display` isn't derivable \u2014 you write it by hand.)

---

## Quick reference

| What | Code |
|---|---|
| Struct | `struct Cell { cell_type: CellType, temperature: f32 }` |
| Method (read-only) | `fn is_empty(&self) -> bool { ... }` |
| Method (mutating) | `fn heat(&mut self, delta: f32) { ... }` |
| Associated function | `fn new(t: CellType) -> Self { ... }` |
| Call associated fn | `Cell::new(CellType::Sand)` |
| Call method | `cell.heat(50.0)` |
| Field access | `cell.temperature` |
| `matches!` macro | `matches!(c.cell_type, CellType::Sand)` |
| Linear interpolation | `a + (b - a) * t` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 9**. Worth recording:

- Your sentence describing the difference between `&self`, `&mut self`, and `self` as a method receiver
- A screenshot of the heat-source brush dropping glowing sand on top of cold sand \u2014 visible evidence of \"chemistry has begun\"
