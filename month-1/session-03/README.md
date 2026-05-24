# Session 3 — Sand Falls

> **Stuck on a word?** Things like *iteration*, *swap*, *seed*, *deterministic* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **sand will fall under gravity and pile up naturally** — drag the mouse, watch it pour, watch it settle into hills with a real angle of repose.

---

## What you'll learn

- Nested `for` loops over a 2D grid
- Why traversal order matters: **bottom-to-top vs top-to-bottom** is the difference between sand falling and sand teleporting
- How to swap two values in a `Vec` cleanly
- Bringing in your first random-number library: `fastrand`
- Why "small per-cell rules + a 60Hz loop" produces big emergent behaviour

---

## The big idea

You're about to build a **cellular automaton**. That's the technical name for: a grid where every cell looks at its neighbours, applies a small fixed rule, and updates. Run that loop 60 times a second and the macroscopic behaviour — sand piling up at the right angle, water seeking its level, fire spreading — *emerges* from the tiny rules.

Today's rule is two sentences:

1. If the cell below a sand grain is empty, move down.
2. If not, try diagonally down-left or down-right (pick at random).

That's it. That's gravity *and* the angle of repose. The pile is not coded anywhere — it's what those two rules look like after a few seconds.

---

## Concepts covered

- Nested `for` loops with a `.rev()` reverse-iteration trick
- `Vec` mutation: `grid[a][b] = grid[c][d]` plus the cleaner `std::mem::swap`
- `fastrand` crate, `cargo add fastrand`
- `fastrand::bool()` to pick a direction
- Why you separate **reading** from **writing** in a simulation step

---

## Building towards `sand-sim`

This is the moment the project stops being "a paint program" and becomes "a simulation." Once gravity works, every later element (water, fire, lava, gunpowder) inherits the same architectural shape: each cell looks at its neighbours and decides whether to move or change. Session 5 reuses today's gravity rule and adds a sideways rule on top to make water. Session 11 inverts it (up instead of down) for fire. The skeleton is today's.

---

## Step-by-step walkthrough

> **Where you should be.** You finished [Session 2](../session-02/README.md). Pressing `1`, `2`, `3` selects sand/water/stone; clicking draws the selected element in its own colour. The grid is `Vec<Vec<u8>>`.

### 1. Add `fastrand` — 1 minute

We need a coin flip to pick "fall left" vs "fall right" when a sand grain is blocked directly below. Rust's standard library doesn't include random numbers (it would mean dragging in cryptography). A tiny crate does the job:

```bash
cargo add fastrand
```

`Cargo.toml` now has:

```toml
[dependencies]
macroquad = "0.4"
fastrand = "2"
```

`fastrand` is **not cryptographically secure** — it's a fast, tiny pseudo-random generator perfect for simulations. (Crypto uses something else; we don't need it here.)

### 2. Write the gravity step — 8 minutes

Add a new function above `main`:

```rust
fn step(grid: &mut Vec<Vec<u8>>) {
    // Iterate from the bottom row up. A sand grain at row R that
    // falls to row R+1 must NOT be moved again on this same frame —
    // and bottom-to-top traversal guarantees we visit R+1 BEFORE R.
    for row in (0..ROWS - 1).rev() {
        for col in 0..COLS {
            if grid[row][col] != SAND {
                continue;
            }

            // Try straight down first.
            if grid[row + 1][col] == EMPTY {
                grid[row + 1][col] = SAND;
                grid[row][col]     = EMPTY;
                continue;
            }

            // Blocked directly below — try a diagonal.
            // Pick left-first or right-first by coin flip so piles are symmetric.
            let try_left_first = fastrand::bool();
            let (a, b) = if try_left_first { (-1, 1) } else { (1, -1) };

            for dx in [a, b] {
                let nc = col as i32 + dx;
                if nc < 0 || nc >= COLS as i32 {
                    continue;
                }
                let nc = nc as usize;
                if grid[row + 1][nc] == EMPTY {
                    grid[row + 1][nc] = SAND;
                    grid[row][col]    = EMPTY;
                    break;
                }
            }
        }
    }
}
```

Then, inside the `loop` in `main`, call it just before the drawing block:

```rust
        step(&mut grid);
```

Save. Run. **Click and hold to pour sand.** Watch it fall, hit the bottom, and start piling.

> **The Wow Moment.** Pour sand onto one spot for a few seconds. It piles up into a perfect cone with the natural slope of real sand. You did not code "the pile is a cone." You wrote two rules — "fall down, or try a diagonal if blocked" — and the cone *emerges*. **This is your first cellular automaton.** Send a screenshot to someone who knows nothing about programming. The reaction is uniformly "wait, how?"

### 3. Why bottom-to-top? — read this, it matters

If you wrote the outer loop as `for row in 0..ROWS - 1` (top-to-bottom), here's what would happen:

1. Visit row 0. A sand grain falls to row 1.
2. Visit row 1. **The same grain is now there.** It falls to row 2.
3. Visit row 2. Same grain. Falls to row 3.
4. ...continues all the way down in **a single frame**.

Sand wouldn't fall. It would teleport. Every grain you place would hit the bottom instantly with no animation.

`.rev()` flips the iterator order so we start at row 78 and work upward to row 0. By the time we look at row 5, rows 6, 7, 8, … have already been updated for this frame — and we won't re-touch them.

**This is the kind of subtle algorithmic detail that makes you sound experienced.** It also generalises: any cellular automaton that involves *movement in a known direction* needs to iterate in the opposite direction. Fire rises — Session 11 will iterate top-to-bottom for fire.

### 4. (Optional) Use `std::mem::swap` for clarity — 2 minutes

The two lines `grid[row+1][col] = SAND; grid[row][col] = EMPTY;` are a swap. Rust's standard library has a function for that:

```rust
// Two simultaneous mutable borrows of `grid` need split_at_mut.
let (top, bottom) = grid.split_at_mut(row + 1);
std::mem::swap(&mut top[row][col], &mut bottom[0][col]);
```

For this session the assignment form is clearer; you'll see `swap` used more in Month 2. Mentioned here so it doesn't look mysterious later.

### 5. (Optional) Vary the spread probability — 2 minutes

In real sand, grains don't always slide off a slope; sometimes they catch. Add a probability gate to the diagonal:

```rust
            // Only try the diagonal 70% of the time. The rest of the time, sand stays.
            if fastrand::f32() > 0.7 {
                continue;
            }
```

Higher probability → flatter piles. Lower → steeper, more stubborn piles. Tweak it until the pile looks right to you.

---

## Linux (Ubuntu) note

`cargo add fastrand` only talks to crates.io over HTTPS — no system packages involved. It works identically on Ubuntu, macOS, and Windows.

If you're behind a corporate proxy (e.g. a school or college network blocks crates.io), set `CARGO_HTTP_PROXY` in `~/.cargo/config.toml`:

```toml
[http]
proxy = "http://proxy.example.org:8080"
```

If the simulation feels sluggish, run `cargo run --release` — Linux release builds are typically the fastest of the three OSes, and the difference for a cellular automaton is dramatic.

---

## Common mistakes

### Sand teleports straight to the bottom in one frame

You wrote `for row in 0..ROWS - 1` without the `.rev()`. Top-to-bottom iteration means each falling grain gets updated repeatedly in the same frame. Add `.rev()` and it'll animate properly.

### `error[E0502]: cannot borrow grid as mutable more than once at a time`

Rust won't let you write `grid[row][col] = grid[row+1][col]` while *also* mutating both cells in the same expression. Split it into two assignments via a temporary:

```rust
let temp = grid[row + 1][col];
grid[row + 1][col] = grid[row][col];
grid[row][col] = temp;
```

Or use `std::mem::swap` with `split_at_mut` as shown above.

### Sand falls right through stone walls

You forgot to check `grid[row+1][col] == EMPTY` — you're moving sand into any non-sand cell, including stone. The whole condition is: move down **only if the cell below is empty**.

### Sand always piles to the left (or always to the right)

You're not coin-flipping the direction. Without `fastrand::bool()`, sand always tries left before right (or vice versa). That biases every pile to one side. Fix: pick the order randomly per cell.

### Pile looks "blocky" — flat plateaus instead of a cone

You're iterating `for col in 0..COLS` left-to-right every row. That introduces a left-bias even with the coin flip, because the grain that "wins" a diagonal slot is the leftmost one to ask. The fix that lands in Session 7 is to alternate column-order every frame. For now, the bias is fine.

### Window freezes for a moment then catches up

You set `CELL_SIZE` to something tiny (like `1.0` or `2.0`) and now you have a million cells. The simulation step is O(rows × cols) per frame. Up the cell size or shrink the grid. (You can also `cargo run --release` for a 5–10× speed boost — at the cost of a slower compile.)

---

## Session challenge

Pick one — no solution provided.

1. **Hold-to-pour acceleration.** While the mouse is held, increase the brush radius from 1 to 4 over a couple of seconds, so a steady press starts as a trickle and turns into a stream.
2. **Stone vs sand collision.** Place a horizontal line of stone partway up the window. Pour sand on it. Right now sand piles cleanly on stone — verify it does. Now place a single-cell stone gap and watch sand find its way through.
3. **Avalanche key.** Press `A` to set all sand cells to "loose" — for one frame, raise the spread probability to 1.0 so existing piles collapse and re-settle. (You'll need a transient state — easiest is a `let mut avalanche_frame = false;` and a check inside `step`.)
4. **Frame counter and FPS print.** Every 60 frames, `println!("fps = {:.1}", get_fps());`. macroquad's `get_fps()` returns the current frame rate.

---

## Quick reference

| What | Code |
|---|---|
| Add a crate | `cargo add fastrand` |
| Random bool | `fastrand::bool()` |
| Random f32 in 0..1 | `fastrand::f32()` |
| Random int in range | `fastrand::i32(0..ROWS as i32)` |
| Reverse a range | `for row in (0..ROWS).rev() { ... }` |
| Swap two cells | `let t = a; a = b; b = t;` (or `std::mem::swap`) |
| Skip remainder of loop iter | `continue;` |
| Stop a loop early | `break;` |
| FPS this frame | `get_fps()` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 3**. The most useful things to note today:

- Your own one-sentence explanation of why **bottom-to-top** iteration matters (this is the kind of subtlety that comes up in interviews and good answers earn nods)
- The specific moment the sand-pile "looked right" — and what you tweaked to get it there
