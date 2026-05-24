# Session 2 — Variables, Types, and Giving Sand a Colour

> **Stuck on a word?** Things like *immutable*, *mutable*, *constant*, *cast* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your sandbox will store **typed elements** — not just raw `1`s — and clicking will draw whichever element you've selected, in its own colour.

---

## What you'll learn

- The difference between `let` and `let mut`, and why Rust insists on the distinction
- Rust's scalar number types — `u8`, `usize`, `i32`, `f32` — and how to choose between them
- `const` for values that never change
- `if`/`else if`/`else` for branching
- How a tiny helper function can turn a one-line magic number into self-documenting code

---

## The big idea

Right now your grid stores `u8` values, but a `1` is just a `1` — the code has no idea whether `1` means *sand*, *water*, or *the third element you'll add next session*. **Types and names are how you make a program understand its own data.**

Two tools today. `const` gives a number a permanent, all-caps name (`SAND = 1`). And a tiny function — `cell_colour(c: u8) -> Color` — maps that name to whatever colour you want on screen. Now `grid[y][x] = SAND` reads like English, and changing the sand colour is a one-line edit.

This is **the** programming superpower: replacing magic numbers with names and small functions. Half of professional Rust is just this, repeated.

---

## Concepts covered

- `let`, `let mut`, *immutable by default*
- Scalar types: `u8`, `usize`, `i32`, `f32`, `bool`
- Type inference vs explicit annotations (`let x: u8 = 0;`)
- `const` for compile-time constants
- `if`/`else if`/`else` returning a value
- macroquad's `Color::new(r, g, b, a)` constructor
- `is_key_pressed(KeyCode::...)` for one-shot key events

---

## Building towards `sand-sim`

Last session your grid held raw `u8`s and every drawn cell was yellow. Today it'll still hold `u8`s — the upgrade to a proper `enum` waits until Session 6 — but the *meaning* of those bytes will be pinned down with named constants. Tomorrow (Session 3) sand starts falling, and the gravity rule needs to know what's sand and what isn't. Named constants make that rule readable: `if grid[y+1][x] == EMPTY { ... }`.

You'll also lay the keyboard groundwork for the element selector that lands in Session 7.

---

## Step-by-step walkthrough

> **Where you should be.** You've finished [Session 1](../session-01/README.md). Your `sand-sim/` folder opens a window and clicking draws yellow squares. If not, fix that first — every session builds on the last.

### 1. Open the project and name the elements — 3 minutes

Open `src/main.rs` in VS Code. Just below `use macroquad::prelude::*;` add these constants:

```rust
const COLS: usize = 120;
const ROWS: usize = 80;
const CELL_SIZE: f32 = 6.0;

// Element identifiers — stored in the grid as plain u8.
const EMPTY: u8 = 0;
const SAND:  u8 = 1;
const WATER: u8 = 2;
const STONE: u8 = 3;
```

(You probably already have the first three from Session 1. Move them up if not, and add the four element constants.)

**What's a `const`?** A constant. A name attached to a value that *never* changes during the program. You write them in `SCREAMING_SNAKE_CASE` by convention so a reader instantly knows "this is fixed at compile time." Constants are zero-cost — the compiler inlines them.

**Why `u8`?** A `u8` is an *unsigned 8-bit integer*: values 0–255. We only need four element types, so the smallest integer that fits is plenty. (`u8` also takes one byte of memory per cell. The whole 120 × 80 grid is 9,600 bytes. Pleasingly small.)

### 2. A colour for each element — 4 minutes

Just below the constants, add a helper function:

```rust
fn cell_colour(cell: u8) -> Color {
    if cell == SAND {
        Color::new(0.95, 0.78, 0.40, 1.0)   // warm sand yellow
    } else if cell == WATER {
        Color::new(0.20, 0.55, 0.95, 1.0)   // ocean blue
    } else if cell == STONE {
        Color::new(0.55, 0.55, 0.60, 1.0)   // grey
    } else {
        BLACK                               // EMPTY or anything unknown
    }
}
```

**What this does.** Takes a `u8`, returns a macroquad `Color`. `Color::new(r, g, b, a)` builds a colour from four floats in the range `0.0 – 1.0`:

- `r`, `g`, `b` — the red, green, and blue channels
- `a` — the alpha (transparency); `1.0` is fully opaque

`BLACK` is a constant that macroquad's prelude exposes (it's just `Color::new(0.0, 0.0, 0.0, 1.0)`).

**Why a function?** Imagine if every drawing site in your code had its own copy of the colour values. Change sand from yellow to pink and you'd have to find every spot. With a function there's one place to change. (This is **DRY**: Don't Repeat Yourself. You'll see this advice constantly.)

### 3. Use the helper in the drawing loop — 2 minutes

Find your drawing loop from Session 1 and update it:

```rust
        for row in 0..ROWS {
            for col in 0..COLS {
                let cell = grid[row][col];
                if cell != EMPTY {
                    let x = col as f32 * CELL_SIZE;
                    let y = row as f32 * CELL_SIZE;
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, cell_colour(cell));
                }
            }
        }
```

The only real change: `YELLOW` → `cell_colour(cell)`. The `if cell != EMPTY` is the same as before, just using the named constant.

**First runnable checkpoint.** Save. `cargo run`. Click around — sand still appears yellow, because you've only been drawing sand. The wow comes in step 4.

### 4. Pick which element to draw — 6 minutes

Above the `loop`, add a mutable variable to track the currently selected element:

```rust
    let mut selected: u8 = SAND;
```

Note the `mut`. Without it, you couldn't reassign `selected` later. **Rust variables are immutable by default**, which is the opposite of most languages — and one of the reasons Rust code is so reliable. You opt in to mutation, and the compiler audits every change.

Now, inside the `loop`, just after `clear_background(BLACK);`, handle the number keys:

```rust
        if is_key_pressed(KeyCode::Key1) { selected = SAND;  }
        if is_key_pressed(KeyCode::Key2) { selected = WATER; }
        if is_key_pressed(KeyCode::Key3) { selected = STONE; }
```

And update the mouse handler to use `selected` instead of the hard-coded `1`:

```rust
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let col = (mx / CELL_SIZE) as usize;
            let row = (my / CELL_SIZE) as usize;
            if row < ROWS && col < COLS {
                grid[row][col] = selected;
            }
        }
```

Save. Run.

**Press `1`, click and drag — sand. Press `2`, click and drag — water. Press `3` — stone.**

> **The Wow Moment.** Open `cell_colour`. Change the sand line to something vivid:
>
> ```rust
> Color::new(1.0, 0.2, 0.8, 1.0)   // electric pink sand
> ```
>
> Save. Run. **Every sand cell, instantly pink.** You changed *one number in one place* and the whole simulation updated. That's what naming gets you. Set it back to a sensible colour before Session 3, when sand starts to fall.

### 5. (Optional) Show the selected element — 3 minutes

Just before `next_frame().await;`, add:

```rust
        let label = if selected == SAND { "SAND" }
                    else if selected == WATER { "WATER" }
                    else if selected == STONE { "STONE" }
                    else { "?" };
        draw_text(label, 8.0, 20.0, 24.0, WHITE);
```

Press `1`, `2`, `3` — the label updates. **Notice the whole `if` chain is one expression**: in Rust, `if`/`else` is an expression that returns a value, like the `?:` ternary in other languages but cleaner. Every branch must produce the same type (here, `&str`).

This is also a preview of `match`, which lands in Session 5 and replaces this kind of chain with something much tidier.

---

## Linux (Ubuntu) note

Everything in this session is pure Rust — no OS-specific code. The commands (`cargo run`, etc.) are identical to macOS and Windows. Two practical Ubuntu tips:

- If `cargo run` opens but flashes white briefly before drawing, you're probably on **Wayland** (default since Ubuntu 22.04). That's expected — macroquad runs fine under Wayland via XWayland. Confirm with `echo $XDG_SESSION_TYPE`; if it prints `wayland`, you're good.
- VS Code's **rust-analyzer** extension is the same as on other OSes. If it says "linker `cc` not found," install build essentials: `sudo apt install -y build-essential`.

---

## Common mistakes

### `error[E0384]: cannot assign twice to immutable variable 'selected'`

You forgot `mut`. `let selected = SAND;` makes it immutable; `let mut selected = SAND;` lets you reassign it.

### `error[E0308]: mismatched types: expected u8, found integer`

You wrote `const SAND = 1;` without the type annotation. `const` requires you to spell out the type explicitly — Rust won't infer it for constants. Fix: `const SAND: u8 = 1;`.

### Colours look washed out or wrong

You forgot that `Color::new` expects values in `0.0 – 1.0`, not `0 – 255`. If you write `Color::new(255.0, 200.0, 100.0, 1.0)`, every channel saturates at maximum (white). Divide by 255 if you're converting from a hex code: `Color::new(255.0/255.0, 200.0/255.0, 100.0/255.0, 1.0)`.

### Number keys don't switch elements

`is_key_pressed` returns `true` only on the single frame the key transitions from up to down. If you used `is_key_down`, every frame you held `1` would re-set `selected = SAND` (fine in this case, but bad once we add menus). For one-shot actions, use `_pressed`. For sustained actions (mouse drag, holding a key), use `_down`.

### Program crashes with `index out of bounds`

Your bounds check from Session 1 is still there, right? `if row < ROWS && col < COLS` before `grid[row][col] = selected`. Without it, a single off-window click panics. **Bounds-check every indexed access.** It's a discipline that pays off forever.

---

## Session challenge

Pick one, no solution provided.

1. **Add a fourth element.** Add `const WOOD: u8 = 4;`, give it a brown colour, bind it to key `4`. (You'll use this in Session 11 when fire arrives.)
2. **Shift+click to fill the whole row.** If the shift key is down, write `selected` to every cell in the same row as the click. (`is_key_down(KeyCode::LeftShift)`)
3. **Cycle colours.** Make sand's red channel pulse over time: `r = 0.5 + 0.5 * (get_time() as f32).sin()`. Pass that into the `SAND` arm of `cell_colour`. (You'll need to thread the current time through, or just call `get_time()` inside the function — both work.)
4. **Print the selected element to the terminal** every time it changes. Use `println!("Selected: {}", label);` — `{}` is the Display formatter (we'll cover this properly in Month 2).

---

## Quick reference

| What | Code |
|---|---|
| Constant | `const SAND: u8 = 1;` |
| Immutable variable | `let n = 5;` |
| Mutable variable | `let mut n = 5; n = 6;` |
| Explicit type | `let x: f32 = 3.14;` |
| Cast | `(mx / CELL_SIZE) as usize` |
| `if` as expression | `let s = if n > 0 { "pos" } else { "neg" };` |
| One-shot key | `is_key_pressed(KeyCode::Key1)` |
| Sustained key | `is_key_down(KeyCode::LeftShift)` |
| Custom colour | `Color::new(r, g, b, a)` *(each 0.0–1.0)* |
| On-screen text | `draw_text("hello", x, y, size, WHITE);` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) (or your printed booklet) and fill in **Session 2**. Two things to capture today:

- What "immutable by default" means in your own words (testing recall is half the value)
- What surprised you about being able to change one constant and see the whole world change colour
