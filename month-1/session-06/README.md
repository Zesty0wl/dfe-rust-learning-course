# Session 6 — Enums: Giving Elements Proper Names

> **Stuck on a word?** Things like *enum*, *variant*, *derive*, *trait* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your grid stores **a proper `enum CellType`** instead of raw `u8`s. The compiler will refuse to let you mistype an element, refuse to let you forget to handle a case, and refuse to let `SAND + 1` mean anything.

---

## What you'll learn

- `enum` declarations and *variants*
- The most-used derives: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`
- Why "make invalid states unrepresentable" is the headline Rust slogan
- How exhaustive `match` becomes life-changingly useful once you're matching on an enum
- A tiny preview of Month 2: `enum` *variants with data*

---

## The big idea

A `u8` can hold values 0–255. Your simulation uses four: `EMPTY = 0`, `SAND = 1`, `WATER = 2`, `STONE = 3`. The other 252 values are nonsense for your program. If you write `grid[row][col] = 17;` by accident, nothing complains — `cell_colour` returns BLACK, `update_cell`'s wildcard arm does nothing, and the cell just *quietly behaves wrong*.

An `enum` is Rust saying: *give me the exact list of legal values, and I'll guarantee no others can sneak in*. `grid[row][col] = 17;` won't even compile. Forgetting a case in a `match` won't compile. Comparing `CellType::Sand` to `CellType::Water` does the right thing without you having to remember that `1 != 2`.

This is **"make invalid states unrepresentable."** It's the single most repeated piece of advice in Rust circles, and it's what makes Rust programs feel like they refuse to break.

---

## Concepts covered

- `enum` definitions
- `#[derive(...)]` attributes
- Why `Copy` matters for small types and what changes if you don't have it
- `PartialEq` and `Eq` for `==` comparisons
- `Hash` for HashMap keys (preview of Session 14)
- Replacing `u8` constants throughout the codebase with a single mass-rename

---

## Building towards `sand-sim`

Today is the **last "type-system" upgrade** you do in Month 1. Once `CellType` is in place, every later element addition (water-already-done, fire, oil, lava, acid, glass, …) is a *one-line addition to the enum* — and the compiler then walks you through every `match` that needs a new arm. This is the closest Rust gets to magic.

Session 14's `HashMap<(CellType, CellType), ReactionOutcome>` *requires* `Hash` and `Eq` on the key — those derives go on today.

---

## Step-by-step walkthrough

> **Where you should be.** Session 5 ended with three elements working (sand, water, stone) and `update_cell` dispatching via `match` on the `u8`. If anything is wobbly, run `cargo run` first and confirm pour-pile-flow still works before you touch the type system.

### 1. Declare the enum — 2 minutes

At the top of `src/main.rs`, just below `use macroquad::prelude::*;`, replace your `const EMPTY: u8 = 0;` block with:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
}
```

**What just happened.** `enum CellType { ... }` declares a new type. Its only legal values are the four listed. Each value is called a **variant**.

The `#[derive(...)]` line tells the compiler to auto-generate a few standard implementations:

- **`Debug`** — lets you print the value with `{:?}`: `println!("{:?}", CellType::Sand)` outputs `Sand`. Essential for debugging.
- **`Clone`** — lets you make a copy explicitly with `.clone()`.
- **`Copy`** — lets the variable be copied *implicitly* on assignment (like an integer). Without `Copy`, `let a = thing; let b = thing;` would *move* `thing` into `a` and `b` would be a compile error. Small "value-like" types should be `Copy`. Big collections shouldn't.
- **`PartialEq`** and **`Eq`** — let you use `==` and `!=`. `PartialEq` is the basic one; `Eq` is a "marker" trait that promises the equality is total (no `NaN`-style weirdness). Both are needed for HashMap keys.
- **`Hash`** — needed for HashMap keys. We don't use it today; we add it now so Session 14 doesn't need a refactor.

### 2. Update the grid's element type — 1 minute

Change the `grid` initialisation in `main`:

```rust
    let mut grid: Vec<Vec<CellType>> = vec![vec![CellType::Empty; COLS]; ROWS];
```

(Was `vec![vec![0u8; COLS]; ROWS]`.) Save. **Don't run yet** — there will be a cascade of errors. That's the point of this exercise.

### 3. Let the compiler walk you through every breakage — 8 minutes

Run `cargo check` (faster than `cargo run` because it skips codegen).

You'll get a wall of errors like:

```text
error[E0308]: mismatched types
  --> src/main.rs:42:39
   |
42 |             draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, cell_colour(cell));
   |                                                        ----------- ^^^^ expected `u8`, found `CellType`
```

This is what people mean when they say "the compiler is your friend." Each error points at exactly one place that assumed `u8` and now sees `CellType`. **Fix them one at a time.**

The fixes are mechanical:

- Anywhere it says `if cell == SAND`, replace with `if cell == CellType::Sand`.
- Anywhere it says `grid[row][col] = SAND;`, replace with `grid[row][col] = CellType::Sand;`.
- `cell_colour(cell: u8)` becomes `cell_colour(cell: CellType)`.
- The `match cell { SAND => ..., _ => {} }` inside `cell_colour` and `update_cell` becomes `match cell { CellType::Sand => ..., _ => {} }`.
- `selected: u8` becomes `selected: CellType`, initialised as `let mut selected = CellType::Sand;`.

**As you fix each, `cargo check` again.** The error count shrinks. By the time it hits zero, everything compiles and behaves identically to Session 5 — but now the type system is guarding the gate.

**First runnable checkpoint.** `cargo run`. Pour sand, pour water, place stone. **Visually identical to Session 5.** The win isn't visible — it's structural.

### 4. Delete the old `const`s — 1 minute

Once `cargo check` is clean, search for `const EMPTY`, `const SAND`, `const WATER`, `const STONE` and delete them. They're dead. (If anything still references them, you missed a spot — the compiler will scream again, which is good.)

### 5. Exhaustive matching, the payoff — 4 minutes

Now, the magic moment. Add a fifth variant — don't worry about implementing it yet:

```rust
enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
    Wood,   // new
}
```

Run `cargo check`. The compiler now lists *every match in the program that doesn't handle `Wood`*:

```text
error[E0004]: non-exhaustive patterns: `Wood` not covered
  --> src/main.rs:55:11
```

It points at `cell_colour`, at `update_cell`, at `element_name` — every match expression that uses `_ =>` is still fine, but if you removed your wildcard arms (which you can, now that the enum is closed), the compiler **listed every place a Wood-shaped hole exists**. This is what's meant by "the compiler refactors with you."

For now, delete the `Wood` variant again — we'll add it for real in Session 11.

> **The Wow Moment.** Add the `Wood` variant back temporarily. Run `cargo run` with a deliberately incomplete `match` (no `_` arm). The compiler refuses to even build — it knows you forgot to handle wood. **Most languages let this kind of bug ship to production. Rust catches it before the file is saved.** That's the safety story in one error message.

### 6. (Optional) `impl CellType` for tidy code — 4 minutes

Move `cell_colour` and `element_name` onto the enum as methods:

```rust
impl CellType {
    fn colour(self) -> Color {
        match self {
            CellType::Sand  => Color::new(0.95, 0.78, 0.40, 1.0),
            CellType::Water => Color::new(0.20, 0.55, 0.95, 1.0),
            CellType::Stone => Color::new(0.55, 0.55, 0.60, 1.0),
            CellType::Empty => BLACK,
        }
    }

    fn name(self) -> &'static str {
        match self {
            CellType::Sand  => "sand",
            CellType::Water => "water",
            CellType::Stone => "stone",
            CellType::Empty => "empty",
        }
    }
}
```

Then call sites become `cell.colour()` and `cell.name()`. Both `match`es are now **exhaustive without a wildcard** — and if you add `Wood` next session, the compiler will list both methods as "non-exhaustive" with a one-line tweak each.

Notice the parameter is `self` (not `&self`) — `CellType` is `Copy`, so passing-by-value costs nothing and lets you call `.colour()` repeatedly without ownership games.

---

## Common mistakes

### `error[E0599]: no method named 'colour' found for type 'CellType'`

You wrote `cell.colour()` but never opened an `impl CellType { ... }` block, or you put `fn colour` outside the `impl` block. Methods only exist inside `impl`.

### `error: cannot find type 'CellType' in this scope`

You used `CellType` inside a function that's in a different file or module without `use crate::CellType;`. We don't have multi-file modules until Session 17, so this only bites if you anticipated and split files early.

### `error[E0277]: the trait bound 'CellType: Copy' is not satisfied`

You assigned a `CellType` to two variables, but forgot `Copy` in the derive. Add `Copy` to the `#[derive(...)]` list. (You need `Clone` for `Copy` to work — derive both.)

### `match` complains "non-exhaustive patterns" after you removed the wildcard

That's a feature, not a bug. The compiler is showing you exactly where to add the new variant. Add the missing arm.

### Grid renders nothing after the refactor

You probably forgot to change the `vec![vec![0u8; COLS]; ROWS]` initialiser to `vec![vec![CellType::Empty; COLS]; ROWS]`. Without it, the `vec!` macro defaults to `0_u8`-typed inner vecs (depending on inference), and then you get a different cascade of errors. Use `cargo check`, follow the trail.

### `cargo run` still works but a key behaviour changed

Most likely cause: somewhere you wrote `CellType::Sand` instead of `CellType::Water` (or vice versa) during the mass rename. **Read every changed line.** `git diff` is your friend.

---

## Linux (Ubuntu) note

Nothing OS-specific in this session. Identical commands on Ubuntu.

`cargo check` (which you'll lean on heavily today) is genuinely fast on Linux thanks to fast filesystem syscalls and a quick linker. Run it after every two-or-three line change to catch type-system mistakes early — the inner loop on Ubuntu is typically sub-second.

If rust-analyzer in VS Code under-highlights or misses errors during the refactor, hit `Ctrl+Shift+P` → *"Rust Analyzer: Restart Server"*. It re-runs the type-check from scratch and usually clears phantom errors caused by half-saved files.

---

## Session challenge

Pick one, no solution provided.

1. **Move `update_sand` and `update_water` onto `CellType`.** Make them methods (`fn update(self, grid: &mut Vec<Vec<CellType>>, row: usize, col: usize)`). Dispatch becomes `cell.update(grid, row, col)`. Notice the natural shape.
2. **`Display` instead of `Debug`.** Manually `impl std::fmt::Display for CellType` and pretty-print as `"💧 water"`-style strings. (No derive available — `Display` is intentionally not derivable, because human-readable output is a *design* choice.)
3. **Toggle key for "godmode brush".** Press `G` to set the next click to fill an entire connected region of the same `CellType` (a flood fill). Hint: `Vec<(usize, usize)>` as a stack, `match` on `CellType` to decide whether to recurse.
4. **A reverse lookup** — given a key `1`/`2`/`3`/`4`, return the `CellType`. Write it as a function `CellType::from_key(KeyCode) -> Option<CellType>`. This is the shape you'll use in Session 7 for the element selector. (`Option` arrives next month for real.)

---

## Quick reference

| What | Code |
|---|---|
| Declare enum | `enum CellType { Empty, Sand, Water, Stone }` |
| Use a variant | `let c = CellType::Sand;` |
| Common derives | `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]` |
| Match exhaustively | `match c { CellType::Sand => ..., CellType::Water => ..., ... }` |
| Method on enum | `impl CellType { fn name(self) -> &'static str { ... } }` |
| Print with Debug | `println!("{:?}", c)` |
| Compare | `if c == CellType::Sand { ... }` |
| Empty grid | `vec![vec![CellType::Empty; COLS]; ROWS]` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 6**. Highest-signal things to record:

- Your own definition of "make invalid states unrepresentable" — say it back in plain English
- The first compile error the type swap surfaced that you thought was annoying *until* you realised it had caught a real bug (or would have caught one in a bigger codebase)
