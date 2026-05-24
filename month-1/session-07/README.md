# Session 7 — Project Build Part 1: Element Selector and Brush

> **Stuck on a word?** Things like *brush*, *scroll wheel*, *UI overlay*, *clamp* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your sandbox has a **proper UI** — a row of element swatches in the corner showing which element is selected, **keyboard shortcuts** (1/2/3), and a **brush** whose radius you scale with the scroll wheel.

---

## What you'll learn

- The first of two **project sessions** for Month 1 — no new Rust concept, just applying what you've already got
- Drawing UI overlays in macroquad without any UI library
- Stamping a circular brush into the grid
- `clamp()` for "keep this number in this range"
- A cleaner separation between game state and rendering

---

## The big idea

Most of programming is **gluing existing pieces together with care**. You already know everything technical you need; today is about shaping the user experience.

The two visible upgrades:

1. **Element selector** — a horizontal strip of squares, one per element, with the selected one outlined.
2. **Brush** — clicking no longer paints one cell. It paints a circle of radius *N*, where *N* is adjustable.

Both upgrades use only knowledge you already have. The interesting part is the *taste* questions: how big should the swatches be? Where do they go? Should the brush be a circle or a square? You'll get answers by making something, looking at it, and iterating. **This is what real product work feels like.**

---

## Concepts covered

- Mouse wheel input: `mouse_wheel()`
- Drawing overlays *after* the grid in the same frame
- Stamping a brush region into a `Vec<Vec<CellType>>`
- `i32::clamp` and `f32::clamp`
- `for dy in -r..=r` nested over `for dx in -r..=r` with a circle check
- Refactoring the input handler into its own function

---

## Building towards `sand-sim`

This session begins the **milestone phase** of Month 1. Work today goes straight into `month-1/milestone/sand-sim-v0.1/` (a copy of your Session 6 project). Session 8 polishes it; the result is v0.1, the first release. The earlier session folders (sessions 1–6) stay where they are as snapshots — don't change them.

---

## Step-by-step walkthrough

> **Where you should be.** Session 6 ended with `CellType` enums, sand/water/stone working. If sand-pouring-onto-water doesn't quite work yet, that's fine — today is about UI, not physics.

### 0. Copy your Session 6 project into the milestone folder — 2 minutes

From the repo root:

```bash
mkdir -p month-1/milestone/sand-sim-v0.1
cp -R month-1/session-06/solution/. month-1/milestone/sand-sim-v0.1/
cd month-1/milestone/sand-sim-v0.1
cargo run     # confirm it still works in the new location
```

If you've been working in `session-06/starter/` instead, copy from that folder. From now on, run from `sand-sim-v0.1`. Session 6's `starter/` and `solution/` stay frozen as the "what Session 6 looked like" snapshot.

(If `month-1/milestone/sand-sim-v0.1/` already exists with a starter scaffold, copy your `src/main.rs` and any tweaks to `Cargo.toml` into it.)

### 1. Brush radius state — 2 minutes

Above the `loop` in `main`, add:

```rust
    let mut brush_radius: i32 = 2;   // in cells
```

And inside the loop, just after the keyboard input block, read the scroll wheel:

```rust
        let (_, scroll_y) = mouse_wheel();
        if scroll_y != 0.0 {
            brush_radius = (brush_radius + scroll_y.signum() as i32).clamp(1, 12);
        }
```

`mouse_wheel()` returns `(x_scroll, y_scroll)`; we only care about `y`. `signum()` flattens whatever scroll-speed your OS reports into `-1.0` / `0.0` / `+1.0` so the radius changes by exactly one step per click of the wheel. `clamp(1, 12)` keeps the radius sane — minimum 1 (single cell), max 12.

### 2. Apply the brush — 5 minutes

Pull the existing single-cell paint code out into a function:

```rust
fn paint(grid: &mut Vec<Vec<CellType>>, centre_row: i32, centre_col: i32, radius: i32, cell: CellType) {
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            // Circular shape — comment this `if` out to get a square brush.
            if dx * dx + dy * dy > radius * radius {
                continue;
            }
            let r = centre_row + dy;
            let c = centre_col + dx;
            if r < 0 || r >= ROWS as i32 || c < 0 || c >= COLS as i32 {
                continue;
            }
            grid[r as usize][c as usize] = cell;
        }
    }
}
```

The `dx * dx + dy * dy > radius * radius` check is Pythagoras without the square root: a cell is inside a circle of radius `r` if `dx² + dy² ≤ r²`. Faster than `sqrt`, exact for integers.

Then update the mouse handler in `main`:

```rust
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let col = (mx / CELL_SIZE) as i32;
            let row = (my / CELL_SIZE) as i32;
            paint(&mut grid, row, col, brush_radius, selected);
        }

        if is_mouse_button_down(MouseButton::Right) {
            // Right-click erases.
            let (mx, my) = mouse_position();
            let col = (mx / CELL_SIZE) as i32;
            let row = (my / CELL_SIZE) as i32;
            paint(&mut grid, row, col, brush_radius, CellType::Empty);
        }
```

**Save. Run.** Click and drag — you're painting a circle. Scroll the wheel — the circle scales. Right-click — eraser. **First runnable checkpoint.**

### 3. The selector UI — 8 minutes

After the grid-drawing block but before `next_frame().await`, add:

```rust
        draw_selector(selected, brush_radius);
```

And define the function:

```rust
fn draw_selector(selected: CellType, brush_radius: i32) {
    let swatch_size: f32 = 32.0;
    let padding: f32 = 6.0;
    let top: f32 = 8.0;

    let elements = [CellType::Sand, CellType::Water, CellType::Stone];

    for (i, e) in elements.iter().enumerate() {
        let x = 8.0 + (swatch_size + padding) * i as f32;
        draw_rectangle(x, top, swatch_size, swatch_size, e.colour());

        // Outline the selected one.
        if *e == selected {
            draw_rectangle_lines(x, top, swatch_size, swatch_size, 3.0, WHITE);
        }

        // Hotkey hint below.
        let key_label = match e {
            CellType::Sand  => "1",
            CellType::Water => "2",
            CellType::Stone => "3",
            _ => "",
        };
        draw_text(key_label, x + swatch_size / 2.0 - 4.0, top + swatch_size + 14.0, 18.0, GRAY);
    }

    // Brush radius readout.
    let radius_label = format!("brush: {}", brush_radius);
    draw_text(&radius_label, 8.0, top + swatch_size + 36.0, 18.0, WHITE);
}
```

Three things to notice:

- **`draw_rectangle_lines(x, y, w, h, thickness, colour)`** — macroquad's outline call. The selected swatch gets a thick white border.
- **`.iter().enumerate()`** gives you `(index, &item)` pairs as you loop. Idiomatic Rust.
- **`format!`** builds a `String` the same way `println!` builds a printed line. `&radius_label` lets `draw_text` borrow it as a `&str`.

Save. Run. **Press 1, 2, 3 — the selected swatch outline jumps.** Scroll — the readout updates.

> **The Wow Moment.** Open the file. Realise that the swatches' position, size, hotkey labels, and selection outline are all driven from *one array* — `[CellType::Sand, CellType::Water, CellType::Stone]`. Add `CellType::Wood` to the enum (with a brown `colour()` arm), add it to that array, give it key `4`, and **the UI grows itself**. *Data drives the UI.* That's the lesson hidden in the prettiness.

### 4. Pause and clear — 3 minutes

In the keyboard input block add:

```rust
        if is_key_pressed(KeyCode::C) {
            for row in grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = CellType::Empty;
                }
            }
        }

        let paused = is_key_down(KeyCode::Space);
```

And wrap the `step(&mut grid)` call:

```rust
        if !paused {
            step(&mut grid);
        }
```

- `C` clears the grid.
- Holding `Space` pauses physics. (Use `is_key_pressed` + a toggle if you'd prefer a tap-to-pause instead of hold-to-pause; Session 8 does that properly.)

### 5. (Optional) Mouse-cursor brush preview — 4 minutes

Just before `next_frame().await`, draw a circle at the mouse position showing where paint will land:

```rust
        let (mx, my) = mouse_position();
        let r_pixels = brush_radius as f32 * CELL_SIZE;
        draw_circle_lines(mx, my, r_pixels, 2.0, WHITE);
```

Tiny addition; *massive* improvement in feel. The brush stops being invisible and the size adjustment becomes intuitive.

---

## Linux (Ubuntu) note

More Linux-specific gotchas in this session than usual, because **input** is the thing that varies most between OSes.

**Scroll wheel under Wayland.** Ubuntu 22.04+ defaults to Wayland, and a few laptop touchpads report scroll events as horizontal-only or with very high magnitudes (e.g. `120.0` per click). The `signum()` normalisation in step 1 above handles this — *do not* be tempted to use the raw scroll value. If the wheel does nothing at all, try:

```bash
echo $XDG_SESSION_TYPE
```

If it prints `wayland`, log out and log back in as "Ubuntu on Xorg" (gear icon on the login screen). That confirms whether your issue is Wayland-specific. macroquad runs on both, but a few input edge cases are smoother on Xorg.

**Right-click on a trackpad.** GNOME on Ubuntu disables "secondary-click area" by default on some laptops. Open *Settings → Mouse & Touchpad → Secondary click* and set it to *"Two-finger push"* or *"Bottom-right corner."* If you'd rather avoid the OS dance, bind `E` as a one-shot eraser (covered in session challenge).

**HiDPI scaling.** If you're on a 4K screen with 200% scaling, macroquad's `mouse_position()` may report logical pixels (the small ones) while `screen_width()` reports physical pixels (the big ones). Symptom: your click lands ~2× too far up-and-left of the cursor. Fix: divide mouse position by `miniquad::window::dpi_scale()` if you hit it. Most laptops don't have this issue at 100% scaling.

---

## Common mistakes

### Painting a square instead of a circle

You forgot the `if dx * dx + dy * dy > radius * radius { continue; }` line. Square brush is the default behaviour of the nested loop alone. Add the Pythagoras check.

### Scroll wheel changes the radius by 50 per click

Different OSes return different scroll magnitudes. Some return `1.0` per click, some return `120.0`. Using `scroll_y` directly gives wildly different feel on different machines. `signum()` normalises to `-1.0` / `0.0` / `+1.0` — always one step per click.

### `error[E0277]: the trait bound 'CellType: PartialEq' is not satisfied` on `*e == selected`

You're missing `PartialEq` in the `#[derive(...)]` on `CellType` from Session 6. Add it.

### Painting overwrites stone walls

Working as intended — left-click is "paint with the selected element." If you'd rather paint only into empty cells, change `grid[r as usize][c as usize] = cell;` to:

```rust
if grid[r as usize][c as usize] == CellType::Empty || cell == CellType::Empty {
    grid[r as usize][c as usize] = cell;
}
```

(That preserves "right-click erases any cell, left-click only paints into empties.")

### UI swatches are drawn but covered by sand pouring on top

You're drawing the selector *before* the grid. Reverse: render grid first, UI second. Anything drawn later appears on top in macroquad.

### Right-click does nothing on macOS

If you're on a trackpad, right-click might be a two-finger tap, or it might require holding Control. Test with a real mouse if in doubt, or add a key (e.g. `E`) that toggles "next click erases."

---

## Session challenge

Pick one — no solution provided.

1. **A "blend brush."** Holding `B` while clicking paints a 50/50 mix of `selected` and `Empty` — so a single brush stroke leaves a noisy texture instead of a solid block. Lovely for natural-looking terrain.
2. **Brush-radius wheel zone.** Only respond to the scroll wheel when the mouse is over a small "brush" widget at the bottom of the screen. Forces a UI region the user has to aim at; teaches mouse-position hit-testing.
3. **Numeric on-screen FPS.** macroquad's `get_fps()` gives an integer. Draw it in the top-right corner. Bonus: colour it red if below 30, green if above 50.
4. **Snapshot key.** Press `P` to write a single image of the current grid state to `snapshot.png`. macroquad has `get_screen_data().export_png("snapshot.png")`. Useful for putting a screenshot of progress in your DofE log.

---

## Quick reference

| What | Code |
|---|---|
| Scroll wheel | `let (_, dy) = mouse_wheel();` |
| Clamp a value | `n.clamp(1, 12)` |
| Sign of a number | `x.signum()` *(-1.0, 0.0, or 1.0)* |
| Outline rectangle | `draw_rectangle_lines(x, y, w, h, thickness, colour)` |
| Outline circle | `draw_circle_lines(x, y, r, thickness, colour)` |
| Build a String | `format!("brush: {}", n)` |
| Iterate with index | `for (i, e) in arr.iter().enumerate() { ... }` |
| Mutable iteration | `for cell in row.iter_mut() { *cell = ...; }` |
| Hold-to-pause | `let paused = is_key_down(KeyCode::Space);` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 7**. Worth recording:

- One *design* decision you made today (brush shape? swatch order? hotkey choice?) and what made you pick that option
- Whether you ran into a platform quirk (right-click behaviour, scroll-wheel scaling) — those frustrations are real evidence of cross-platform development
