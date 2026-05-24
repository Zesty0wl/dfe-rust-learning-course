# Session 4 — Control Flow and a Better Update Loop

> **Stuck on a word?** Things like *boundary condition*, *refactor*, *short-circuit*, *guard clause* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your simulation will have a **clean, well-structured update function**, sand will **stop falling cleanly at the bottom edge instead of panicking**, and the code will be ready to take on a second element type next session.

---

## What you'll learn

- `if`/`else if`/`else` in proper depth, including chained guard clauses
- `continue` and `break` as escape hatches inside loops
- Why bounds-checking every grid access is non-negotiable, and how to do it elegantly
- The discipline of **extracting a function** — and the compiler's role as your refactoring buddy
- A preview of `match` (it lands properly in Session 5)

---

## The big idea

You've got working sand. You're about to add water, stone-with-different-physics, fire, oil, lava — eight or more behaviours in the same loop. **If you don't tidy the architecture now, that loop becomes unreadable by Session 6.**

Today's work is a small refactor: lift the per-cell logic into a `step()` function with clear boundary checks. No new visible feature — but the next ten sessions become noticeably easier to write. *Refactoring before you need to* is one of the cheapest skills to develop and one of the highest-paid.

The "boundary condition" framing matters too. Every physics simulation in the world — from `sand-sim` to climate models — has to decide what happens at the edge of the grid. Your edge says "sand stops, doesn't wrap." That's a choice. Game of Life often wraps the edges into a torus. Real wave tanks absorb. The simulation behaviour at the edge is part of the simulation.

---

## Concepts covered

- `if`/`else if`/`else` chains
- Boolean operators: `&&` (AND), `||` (OR), `!` (NOT), short-circuit evaluation
- `continue` and `break` for early loop exits
- Safe bounds-checking patterns
- Function extraction: pulling cell logic out of the main loop
- Reading compiler errors as a refactoring tool
- Brief preview of `match` (next session covers it properly)

---

## Building towards `sand-sim`

After today, adding a new element type means **one new branch in the update function**, not a hand-edit through hundreds of lines of nested loops. The Session 5 `match` rewrite, the Session 6 enum upgrade, the Session 11 fire rule, the Session 14 reactions HashMap — every one of those starts from this clean shape.

---

## Step-by-step walkthrough

> **Where you should be.** You finished [Session 3](../session-03/README.md). Sand falls, piles, spreads diagonally. The whole simulation step is inside `main`. If sand-pouring crashes or stops at the wrong place, fix that before refactoring.

### 1. Extract `step()` properly — 3 minutes

If you followed Session 3's structure you may already have a `step(&mut grid)` function. If not, lift the bottom-to-top loop out of `main`:

```rust
fn step(grid: &mut Vec<Vec<u8>>) {
    for row in (0..ROWS - 1).rev() {
        for col in 0..COLS {
            update_cell(grid, row, col);
        }
    }
}
```

And add a new function that handles a *single cell*:

```rust
fn update_cell(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    let cell = grid[row][col];

    if cell == SAND {
        update_sand(grid, row, col);
    }
    // Future: else if cell == WATER { update_water(grid, row, col); }
    //         else if cell == FIRE  { update_fire(grid, row, col); }
}
```

And the sand-specific logic in its own function:

```rust
fn update_sand(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    // Try straight down.
    if grid[row + 1][col] == EMPTY {
        grid[row + 1][col] = SAND;
        grid[row][col]     = EMPTY;
        return;
    }

    // Try a diagonal — pick the order randomly.
    let try_left_first = fastrand::bool();
    let order: [i32; 2] = if try_left_first { [-1, 1] } else { [1, -1] };

    for dx in order {
        let nc = col as i32 + dx;
        if nc < 0 || nc >= COLS as i32 { continue; }
        let nc = nc as usize;
        if grid[row + 1][nc] == EMPTY {
            grid[row + 1][nc] = SAND;
            grid[row][col]    = EMPTY;
            return;
        }
    }
}
```

**Why three functions instead of one?** Each does one job at one level of abstraction:

- `step` knows about the grid as a whole and the iteration order.
- `update_cell` knows about which element rules to apply.
- `update_sand` knows about the physics of sand.

When you add water in Session 5 it'll be `update_water` next to `update_sand`. Same shape, isolated, easy to reason about.

### 2. Bulletproof bounds checking — 4 minutes

`update_sand` currently checks `nc < 0 || nc >= COLS as i32` for the diagonal — good. But `grid[row + 1][col]` assumes `row + 1 < ROWS`. We're saved here because the outer loop only goes up to `ROWS - 1` (so `row + 1 < ROWS` always holds), but it's worth seeing the bullet-proof shape:

```rust
fn in_bounds(row: i32, col: i32) -> bool {
    row >= 0 && row < ROWS as i32 && col >= 0 && col < COLS as i32
}
```

Then any access becomes:

```rust
let nr = row as i32 + 1;
let nc = col as i32 + dx;
if !in_bounds(nr, nc) { continue; }
// safe to use grid[nr as usize][nc as usize]
```

The `!` is logical NOT. `&&` is logical AND. `||` is logical OR. They **short-circuit**: in `a && b`, if `a` is false, `b` is never evaluated. In `a || b`, if `a` is true, `b` is never evaluated. This matters when one of the operands might panic — e.g. `i < v.len() && v[i] > 0` is safe even when `v` is empty, because `v[i]` isn't evaluated.

Add `in_bounds` to your file and use it in `update_sand`. This single helper will be reused dozens of times before the project ends.

### 3. Hook it into `main` — 1 minute

Inside the `loop` in `main`, the per-frame block should now read cleanly:

```rust
        // 1. Input
        if is_key_pressed(KeyCode::Key1) { selected = SAND;  }
        if is_key_pressed(KeyCode::Key2) { selected = WATER; }
        if is_key_pressed(KeyCode::Key3) { selected = STONE; }

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let col = (mx / CELL_SIZE) as usize;
            let row = (my / CELL_SIZE) as usize;
            if row < ROWS && col < COLS {
                grid[row][col] = selected;
            }
        }

        // 2. Simulation
        step(&mut grid);

        // 3. Render
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

That **input → simulate → render** shape is the spine of every real-time program in the world.

**First runnable checkpoint.** `cargo run`. Pour sand. Pour it off the bottom edge. **No panics, sand stops at the bottom.** The animation is identical to Session 3 — but every line of the loop is doing a clearly-named job.

### 4. The `match` preview — 3 minutes

Replace the `if cell == SAND` chain in `update_cell` with a `match`:

```rust
fn update_cell(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    let cell = grid[row][col];

    match cell {
        SAND  => update_sand(grid, row, col),
        // WATER => update_water(grid, row, col),
        // FIRE  => update_fire(grid, row, col),
        _ => {}
    }
}
```

Note the `_` — that's the **wildcard pattern**. It matches anything not handled by the earlier arms. Without it, `match` would fail to compile because it has to handle *every possible `u8` value* (0–255), and we've only spelled out one.

Save. Run. Same behaviour, two fewer keystrokes. Session 5 unpacks `match` properly.

> **The Wow Moment.** Look at your `main` function now. Three short blocks: input, simulate, render. Each *says* what it does. The compiler will tell you exactly where to add water rules tomorrow. *This is what "clean code" feels like* — and you got here without anyone breathing down your neck about style.

### 5. (Optional) Wrap-around boundary as an experiment — 3 minutes

Comment out your bottom edge check and replace it with a wrap:

```rust
let next_row = (row + 1) % ROWS;   // 0 if we'd go off the bottom
if grid[next_row][col] == EMPTY {
    grid[next_row][col] = SAND;
    grid[row][col] = EMPTY;
    return;
}
```

Run it. Sand that falls off the bottom now appears at the top. **You implemented a torus.** Set it back before Session 5 — sand-sim doesn't want a torus — but you've now physically experienced "the boundary is a design choice."

---

## Linux (Ubuntu) note

Pure-Rust session, identical commands on Ubuntu. Two VS Code conveniences worth knowing if you haven't found them yet:

- **`Ctrl+.`** (the *Quick Fix* shortcut) is the same on Ubuntu as on Windows. When rust-analyzer underlines an error, hit `Ctrl+.` to see suggested fixes — it often offers "extract into function," which does today's refactor *for* you. Try it on a selected block.
- **`Ctrl+Shift+P` → "Rust Analyzer: Run"** runs the current binary without leaving the editor. Saves an Alt-Tab to the terminal.

If rust-analyzer is sluggish on Ubuntu, increase its memory cap: open the extension settings and set `rust-analyzer.cargo.allFeatures` to `false` and `rust-analyzer.checkOnSave.command` to `"check"` instead of `"clippy"`.

---

## Common mistakes

### Sand vanishes when it reaches the bottom row

You wrote `for row in (0..ROWS).rev()` instead of `for row in (0..ROWS - 1).rev()`. When `row == ROWS - 1`, the `grid[row + 1]` access is out of bounds and crashes (or silently corrupts in `--release`). Either iterate to `ROWS - 1` exclusive, or add a `row + 1 < ROWS` guard inside the body.

### `error[E0382]: borrow of moved value: grid`

You wrote `step(grid)` without the `&mut`. The function signature is `fn step(grid: &mut Vec<Vec<u8>>)`, so call sites need to match: `step(&mut grid)`. Without the `&mut`, you'd be *moving* the grid into the function (which would then be unusable in `main` after the call).

### `match` complains "non-exhaustive patterns"

You forgot the `_` wildcard arm. `match` on a `u8` covers 256 possible values; the compiler insists every one is handled. Add `_ => {}` at the bottom.

### Refactor broke the simulation

Run `cargo run` after each function extraction, not all at once. If the second extraction broke things, `git diff` will tell you exactly what changed. (If you're not committing per session yet, this is a good week to start — `git add -A && git commit -m "session 3 complete"` after Session 3 done.)

### `error[E0502]: cannot borrow as mutable... already borrowed as immutable`

Trying to read and write the grid in the same expression. E.g. `grid[row + 1][col] = grid[row][col]` looks innocent but isn't — the compiler refuses because the second `grid[row][col]` is an immutable borrow while the assignment is a mutable one. Fix: pull the value into a local first: `let value = grid[row][col]; grid[row + 1][col] = value;`.

---

## Session challenge

Pick one, no solution provided.

1. **Pour rate cap.** Add a counter that limits sand spawns to `N` cells per frame, so dragging slowly produces a trickle and dragging fast doesn't fill the screen instantly.
2. **Edge mirror.** Instead of sand stopping at the bottom, have it bounce one cell upward — the floor "pushes back." Hint: when straight-down is blocked AND both diagonals are blocked AND we're at `row == ROWS - 2`, move the sand *up*.
3. **Visualise the iteration order.** During `step`, draw a faint scanline rectangle for the current row. You'll see the bottom-to-top sweep happening live.
4. **Move `cell_colour` into its own file.** Create `src/colours.rs`, declare `mod colours;` in `main.rs`, mark the function `pub`, call it as `colours::cell_colour(c)`. (Session 17 does this properly; doing it once now demystifies the module system.)

---

## Quick reference

| What | Code |
|---|---|
| Logical AND / OR / NOT | `a && b`, `a \|\| b`, `!a` |
| Early-exit a loop iteration | `continue;` |
| Stop a loop entirely | `break;` |
| Bounds-check helper | `fn in_bounds(r: i32, c: i32) -> bool { ... }` |
| `match` with wildcard | `match v { SAND => ..., _ => {} }` |
| Function with mutable ref | `fn step(grid: &mut Vec<Vec<u8>>) { ... }` |
| Call with mutable ref | `step(&mut grid);` |
| Early return | `return;` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 4**. Two things to record:

- Why you split `step` from `update_cell` from `update_sand` — in your own words. (Naming the *why* is more useful evidence than naming the *what*.)
- One compiler error you hit during the refactor, and how the message told you exactly what to fix. (This is the assessor's favourite kind of growth signal.)
