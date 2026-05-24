# Session 10 — Enums with Data and `Option`: Modelling Reactions

> **Stuck on a word?** Things like *variant*, *payload*, *null*, *Option*, *Some*, *None* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your sandbox has **wood**, and **wood next to fire ignites** — your first formal *reaction*. The reaction is expressed as a function `react(a, b) -> Option<ReactionOutcome>`, the same shape that Session 14 lifts into a HashMap.

---

## What you'll learn

- **Enum variants with data**: `CellType::Wood(u8)` — variants that *carry* a value
- **`Option<T>`** — Rust's replacement for `null`, and why it's a billion-dollar improvement
- `Some(...)` and `None` constructors
- Pattern-matching on `Option` and unwrapping safely
- Returning `Option` to mean "maybe this happens, maybe nothing does"

---

## The big idea

In most languages, "no value" is `null` / `nil` / `None` / `undefined` — and the system silently lets you treat it as if it were a real value, until it explodes at runtime. Tony Hoare, who invented null, called it his "billion-dollar mistake."

Rust has no `null`. Instead, anything that *might* be absent has the type `Option<T>`, which is just an enum:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

To use the inner `T`, you have to handle both arms. The compiler refuses to let you forget. **Every null-pointer bug in your future career — gone, by construction.**

Today you also meet **variants that carry data**. `CellType::Wood(u8)` means "a wood cell, plus an extra byte" — for now, that byte is the wood's *charring level* (0 = pristine, 100 = ash). Variants-with-data are how Rust replaces inheritance: instead of a `Wood` *class* that extends `Cell`, you have a `Wood(u8)` *variant* that contains its own data inline.

---

## Concepts covered

- `enum CellType { ..., Wood(u8), Fire }`
- `Option<T>` and its two variants
- `match Some(x) => ... None => ...`
- `if let Some(x) = opt { ... }` shorthand
- `Option::unwrap()`, `Option::unwrap_or(default)` — and when each is appropriate
- A `ReactionOutcome` struct: what changes when a reaction fires

---

## Building towards `sand-sim`

The signature `fn react(a: CellType, b: CellType) -> Option<ReactionOutcome>` is the seam every later element plugs into. Session 11 calls it from the per-cell update to spread fire. Session 14 collapses dozens of `if`/`match` arms into a single HashMap keyed on `(a, b)` that returns the same `Option<ReactionOutcome>`. Today you write the first reaction (wood + fire) the long way; later sessions just add rows to the table.

---

## Step-by-step walkthrough

> **Where you should be.** Session 9 finished. Your grid is `Vec<Vec<Cell>>`, each `Cell` has `cell_type: CellType` and `temperature: f32`, and pressing `H` drops in cells at 200°C.

### 1. Two new variants in `CellType` — 2 minutes

Open the enum and add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
    Wood,        // for today: pristine, no data yet
    Fire,        // pure ignition source
}
```

(We'll add the `(u8)` charring payload later in the session.)

`cargo check`. Every match without a wildcard will complain about `Wood` and `Fire` being uncovered. Fix `cell_colour` / `CellType::colour`:

```rust
CellType::Wood  => Color::new(0.45, 0.30, 0.15, 1.0),  // chocolate brown
CellType::Fire  => Color::new(1.00, 0.45, 0.10, 1.0),  // bright orange
```

And add to your selector array in `draw_selector`:

```rust
let elements = [CellType::Sand, CellType::Water, CellType::Stone, CellType::Wood, CellType::Fire];
```

Bind keys `4` and `5` in the input block:

```rust
        if is_key_pressed(KeyCode::Key4) { selected = CellType::Wood; }
        if is_key_pressed(KeyCode::Key5) { selected = CellType::Fire; }
```

`cargo run`. Wood paints brown, fire paints orange. Neither does anything yet.

### 2. The reaction signature — 3 minutes

Add this above the per-element update functions:

```rust
#[derive(Debug, Clone, Copy)]
struct ReactionOutcome {
    /// What the *source* cell becomes (the one running the rule).
    new_source: CellType,
    /// What the *target* (neighbour) cell becomes.
    new_target: CellType,
    /// How much heat to release at the reaction site.
    heat: f32,
}

/// Given two adjacent cell types, return what they should become.
/// `None` means "no reaction; leave them alone."
fn react(source: CellType, target: CellType) -> Option<ReactionOutcome> {
    match (source, target) {
        // Fire eats wood. Both ends become fire; lots of heat.
        (CellType::Fire, CellType::Wood) | (CellType::Wood, CellType::Fire) => Some(ReactionOutcome {
            new_source: CellType::Fire,
            new_target: CellType::Fire,
            heat: 80.0,
        }),
        _ => None,
    }
}
```

A few things to notice:

- **The return type is `Option<ReactionOutcome>`**, not `ReactionOutcome`. Most cell pairs don't react. Returning `None` for the common case is *the* idiomatic Rust pattern.
- **The match has a tuple pattern.** `match (source, target)` destructures both at once. The `|` is the "or" pattern — `(Fire, Wood) | (Wood, Fire)` means either order works.
- **The wildcard arm returns `None`.** Without it, the compiler would demand you spell out every possible pair (5 × 5 = 25, growing).

### 3. Call `react` from the per-cell update — 5 minutes

In `update_cell` (or wherever you dispatch per-cell logic), add a reaction step that runs *before* movement:

```rust
fn try_react(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    let source = grid[row][col].cell_type;
    if matches!(source, CellType::Empty) { return; }

    // Try all four cardinal neighbours.
    let neighbours = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in neighbours {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);

        let target = grid[nr][nc].cell_type;
        if let Some(outcome) = react(source, target) {
            grid[row][col].cell_type    = outcome.new_source;
            grid[nr][nc].cell_type      = outcome.new_target;
            grid[row][col].heat(outcome.heat);
            grid[nr][nc].heat(outcome.heat);
            return;          // one reaction per cell per frame
        }
    }
}
```

Watch the new syntax: **`if let Some(outcome) = react(source, target)`**. This is the most common way to use `Option`. It's exactly the `match` pattern in one line:

```rust
match react(source, target) {
    Some(outcome) => { /* use outcome */ }
    None => {}      // do nothing
}
```

When you only care about the `Some` case, `if let` is much cleaner.

Wire it into the per-frame step:

```rust
fn step(grid: &mut Vec<Vec<Cell>>) {
    // 1. Reactions pass.
    for row in 0..ROWS {
        for col in 0..COLS {
            try_react(grid, row, col);
        }
    }
    // 2. Movement pass (Session 3–7 logic).
    for row in (0..ROWS - 1).rev() {
        for col in 0..COLS {
            update_cell(grid, row, col);
        }
    }
}
```

The order matters: reactions first, then movement. Otherwise a falling sand grain might land next to wood mid-frame and not get a chance to react until the next frame.

**Save. Run.** Build a stack of wood (key `4`). Drop a single fire cell (key `5`) at the top. **It spreads.** Slowly, because fire only spreads to direct neighbours per frame — but it spreads.

> **The Wow Moment.** Build a long horizontal beam of wood and put a single fire dot on the leftmost cell. Watch the fire ripple along the beam, one cell per frame, until the whole beam is burning. **You wrote a chemical reaction.** The two-line `react` function is the entire rule. Real chemistry simulations use the same shape, scaled up: a function that takes the participants and returns what they become.

### 4. Variants with data: pristine vs charring wood — 5 minutes

Right now wood ignites instantly on contact. Let's make it take a few ticks. Change the variant:

```rust
enum CellType {
    // ...
    Wood(u8),        // u8 = char level, 0 = pristine, 100 = ash
    // ...
}
```

`cargo check` rains errors — match arms must cover the new shape. Update them.

First, the `colour` match arm:

```rust
// In CellType::colour:
CellType::Wood(char_level) => {
    let darken = char_level as f32 / 100.0;
    Color::new(
        0.45 * (1.0 - darken),
        0.30 * (1.0 - darken),
        0.15 * (1.0 - darken),
        1.0,
    )
}
```

Anywhere you previously wrote `CellType::Wood` as a value (paint sites, selector array, etc.), change it to:

```rust
CellType::Wood(0)        // pristine wood
```

And add a new `react` arm so fire chars wood over time:

```rust
// In react:
(CellType::Fire, CellType::Wood(c)) | (CellType::Wood(c), CellType::Fire) => {
    if c >= 80 {
        // Fully charred — turn to fire.
        Some(ReactionOutcome { new_source: CellType::Fire, new_target: CellType::Fire, heat: 80.0 })
    } else {
        // Still charring — increment the char level.
        Some(ReactionOutcome { new_source: CellType::Fire, new_target: CellType::Wood(c + 20), heat: 20.0 })
    }
}
```

Note `CellType::Wood(c)` in the pattern — `c` binds the inner `u8` so you can use it. Same as destructuring a struct.

Run it. Now fire takes a few ticks to fully consume each wood cell. The wood visibly darkens. **You modelled charring with one extra byte per variant.**

### 5. `unwrap` vs `unwrap_or` vs `?` — 3 minutes

You'll see all three of these everywhere in Rust code. Here's the etiquette:

- **`opt.unwrap()`** — "I know this is `Some`. If it's `None`, panic." Use only when you can *prove* the value is present.
- **`opt.unwrap_or(default)`** — "Give me the inner value, or this default if it's `None`." Always safe.
- **`opt?`** — Early-returns `None` from the enclosing function if the value is `None`. Lands properly in Session 18 with `Result`.

Avoid `unwrap()` unless you have a comment justifying why the value is guaranteed. In real codebases, every `unwrap` is a small signed promise: *"I'm sure this never fails."*

---

## Linux (Ubuntu) note

Nothing OS-specific in this session — `Option`, enum payloads, and reactions are pure Rust. Two practical Ubuntu notes:

- **Compile times are now noticeably longer** (3–8 seconds for `cargo check`, 10–20 seconds for first `cargo run` after changing the enum). The cause is the cascade of error-fix-error-fix that the enum payload change triggers. This is normal. The fix isn't to make the compiler faster; it's to lean on `cargo check` (skips codegen and linking).
- If `rust-analyzer` in VS Code shows phantom errors after the `Wood(u8)` rename — squiggles on lines that compile fine — `Ctrl+Shift+P` → *"Rust Analyzer: Restart Server"*. Common Ubuntu hiccup, harmless.

---

## Common mistakes

### `error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 0 fields`

You wrote `CellType::Wood(c)` in a match arm but didn't update the variant to `Wood(u8)`. Or the opposite: variant has `(u8)` but a match arm has bare `CellType::Wood`. Spell out the data even if you ignore it: `CellType::Wood(_) => ...`.

### `Option<ReactionOutcome>` won't return from `react` — `expected ReactionOutcome, found Option<...>`

You forgot to wrap the return value in `Some(...)`. The function returns `Option<ReactionOutcome>`, so the only valid returns are `Some(outcome)` or `None`.

### Fire spreads in only one direction

Your neighbour loop `[(-1, 0), (1, 0), (0, -1), (0, 1)]` is missing one of the four pairs. Each pair is `(dr, dc)` — row delta, column delta. Verify all four cardinals are present.

### Fire consumes all wood in one frame

You forgot the `return` at the end of `try_react`'s `if let Some(...)` block. Without it, after igniting one neighbour, the loop keeps going and ignites the others too. One reaction per cell per frame is the rule.

### `cargo run` panics with "called `Option::unwrap()` on a `None`"

You used `unwrap()` somewhere it wasn't safe — common with `react(...).unwrap()`. Replace with `if let Some(outcome) = react(...)` to handle the `None` case cleanly.

### Wood doesn't visibly darken

The `colour` method recomputes per frame, but you forgot to pass the `char_level` field through. Check `CellType::Wood(char_level) => ...` actually uses `char_level` in the colour formula.

---

## Session challenge

Pick one, no solution provided.

1. **Water extinguishes fire.** Add `(CellType::Fire, CellType::Water) | (CellType::Water, CellType::Fire) => Some(ReactionOutcome { new_source: CellType::Empty, new_target: CellType::Water, heat: -50.0 })`. Now fire dies on contact with water. (Bonus: turn the water into steam — Session 13 territory.)
2. **A `match` on `Option<ReactionOutcome>` with both arms.** Rewrite the `if let` block in `try_react` as an explicit `match`. Notice both forms compile to the same thing; pick which reads better to you.
3. **A reaction that requires a third cell.** "Fire next to wood next to wood" — only ignite if there are at least *two* wood neighbours, modelling that lone wood embers go out. (Compute a count first, then call `react` only above threshold.)
4. **`react` returning a `Vec<ReactionOutcome>`.** Refactor so a single call can return multiple state changes (e.g. heat-up the four other neighbours in addition to the reacted cells). Forward-compatible with explosions in Session 21.

---

## Quick reference

| What | Code |
|---|---|
| Variant with data | `enum E { Wood(u8) }` |
| Make one | `CellType::Wood(0)` |
| Pattern with binding | `CellType::Wood(c) => println!("{c}")` |
| Pattern ignoring data | `CellType::Wood(_) => ...` |
| `Option<T>` | `Option<ReactionOutcome>` |
| Build | `Some(value)` / `None` |
| Match an option | `match opt { Some(x) => ..., None => ... }` |
| Shorthand | `if let Some(x) = opt { ... }` |
| Default fallback | `opt.unwrap_or(default)` |
| Tuple match | `match (a, b) { (X, Y) => ... }` |
| Or-pattern | `(X, Y) \| (Y, X) => ...` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 10**. Worth recording:

- Your one-sentence explanation of why `Option<T>` is "better than null" — assessors love this one
- A short clip / screenshot of fire eating a wood beam end-to-end (the wow moment)
