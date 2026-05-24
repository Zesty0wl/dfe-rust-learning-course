# Session 19 — The Recipe System: Unlocking Elements

> **Stuck on a word?** Things like *closure*, *recipe*, *discovery*, *predicate*, *higher-order function* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session the element selector **starts with only four elements unlocked** (sand, water, stone, wood) and **the rest are earned by experimentation** — get water near acid for the first time and "diluted acid" becomes available; touch fire to sand and "glass" appears. Saved across sessions, persistently.

---

## What you'll learn

- **Closures** — anonymous functions with environment capture
- `Fn`, `FnMut`, `FnOnce` — the three traits all closures implement
- The `move` keyword — taking ownership instead of borrowing
- Storing closures in a `Vec` for table-driven *predicates*
- The "recipe" pattern: a closure that returns `true` when an unlock condition is met

---

## The big idea

A **recipe** is "a check that, when true, unlocks something." Today, recipes are checks on the *grid state*. "Was acid ever adjacent to water?" "Did fire ever touch sand?"

The natural Rust shape is a `Vec` of closures: each closure takes the grid and returns `true` if its unlock condition is currently met. Each frame you run them all; the ones that fire enable a new element in the selector.

Closures are the most-loved Rust feature for people coming from JS, Python, or Swift. They're functions you can pass around, store, capture variables in. Used carefully they're a superpower; used carelessly they cause lifetime headaches. Today is a controlled introduction — every closure in the recipe table has the same simple shape, so the lifetime questions don't arise.

---

## Concepts covered

- `|grid| { ... }` syntax for closures
- The three closure traits: `Fn` (borrow), `FnMut` (mutate captures), `FnOnce` (consume captures)
- `Box<dyn Fn(...) -> bool>` — storing closures behind a trait object
- `move` for closures: `move |grid| { ... }`
- `.any(|cell| ...)` — iterator method that returns true if any item matches
- A `Recipe { name, predicate, unlocks }` struct holding a closure inside

---

## Building towards `sand-sim`

Today's recipe table is the **gameplay engine** of v1.0. Session 20 builds the codex UI on top — discovered elements appear in colour; locked ones as grey silhouettes. Session 21 adds gunpowder and glass as new elements gated by recipes. Session 23 adds three *hidden* recipes that aren't hinted anywhere. By v1.0 ship, the recipe table is the spine of the discovery experience.

---

## Step-by-step walkthrough

> **Where you should be.** Session 18 finished. Save/load work. The project is in modules. Eleven elements exist. The selector currently shows all of them.

### 1. The recipe data structure — 4 minutes

Create `src/recipes.rs`:

```rust
use crate::elements::{Cell, CellType, COLS, ROWS};
use serde::{Serialize, Deserialize};

/// A recipe: a check on the grid that, when first true, unlocks an element.
pub struct Recipe {
    pub name: &'static str,
    pub unlocks: CellType,
    pub predicate: Box<dyn Fn(&Vec<Vec<Cell>>) -> bool + Send + Sync>,
}

/// Track which elements the player has unlocked.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discoveries {
    pub unlocked: Vec<CellType>,
}

impl Discoveries {
    pub fn new() -> Self {
        Self {
            unlocked: vec![
                CellType::Sand,
                CellType::Water,
                CellType::Stone,
                CellType::Wood,
            ],
        }
    }

    pub fn is_unlocked(&self, t: CellType) -> bool {
        self.unlocked.contains(&t)
    }

    pub fn unlock(&mut self, t: CellType) -> bool {
        if !self.is_unlocked(t) {
            self.unlocked.push(t);
            true
        } else {
            false
        }
    }
}
```

`Box<dyn Fn(...) -> bool + Send + Sync>` is a *trait object*. It's the only practical way to put multiple different closures in the same `Vec` — each closure has a unique concrete type, but they all implement `Fn(...) -> bool`, so we hide behind the trait.

`+ Send + Sync` are extra trait bounds that say "this is safe to send across threads." Not used today, but they cost nothing and make the type easier to compose later.

### 2. The recipe table — 6 minutes

```rust
pub fn build_recipes() -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = Vec::new();

    // Recipe: Fire — discovered by placing wood next to a heat source.
    // (For Month 3, we assume fire is always available; this is just for illustration.)
    recipes.push(Recipe {
        name: "Fire",
        unlocks: CellType::Fire,
        predicate: Box::new(|grid| {
            cells_match(grid, |c| c.cell_type == CellType::Wood && c.temperature > 150.0)
        }),
    });

    // Recipe: Oil — discovered when wood is fully charred.
    recipes.push(Recipe {
        name: "Oil",
        unlocks: CellType::Oil,
        predicate: Box::new(|grid| {
            // (Pretend "ash" comes from wood with very high charring. Simplified.)
            cells_match(grid, |c| c.cell_type == CellType::Wood && c.temperature > 500.0)
        }),
    });

    // Recipe: Acid — discovered the first time fire melts stone (i.e., lava + water = stone happens).
    recipes.push(Recipe {
        name: "Acid",
        unlocks: CellType::Acid,
        predicate: Box::new(|grid| {
            adjacent_pair(grid, CellType::Lava, CellType::Water)
        }),
    });

    // Recipe: Lava — discovered the first time fire meets stone.
    recipes.push(Recipe {
        name: "Lava",
        unlocks: CellType::Lava,
        predicate: Box::new(|grid| {
            adjacent_pair(grid, CellType::Fire, CellType::Stone)
        }),
    });

    // Recipe: Ice — discovered when steam condenses at the top.
    recipes.push(Recipe {
        name: "Ice",
        unlocks: CellType::Ice,
        predicate: Box::new(|grid| {
            grid[0].iter().any(|c| matches!(c.cell_type, CellType::Steam))
        }),
    });

    recipes
}

/// Helper: does any cell match the predicate?
fn cells_match(grid: &Vec<Vec<Cell>>, pred: impl Fn(&Cell) -> bool) -> bool {
    grid.iter().flat_map(|row| row.iter()).any(pred)
}

/// Helper: is there at least one (a, b) pair of adjacent cells?
fn adjacent_pair(grid: &Vec<Vec<Cell>>, a: CellType, b: CellType) -> bool {
    for r in 0..ROWS {
        for c in 0..COLS {
            if grid[r][c].cell_type != a { continue; }
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
                if grid[nr as usize][nc as usize].cell_type == b { return true; }
            }
        }
    }
    false
}
```

Each `Box::new(|grid| { ... })` is a closure literal boxed into a `dyn Fn`. The `|grid|` is the parameter list; the body is whatever condition matters.

Notice **`impl Fn(&Cell) -> bool`** in `cells_match`'s signature — that's a *generic* form. It accepts any closure (or function pointer) with that signature, *without* boxing. We don't need `Box` here because the closure isn't stored; it's used and dropped.

### 3. The recipe-check pass — 3 minutes

In `main.rs` or `simulation.rs`:

```rust
use crate::recipes::{Recipe, Discoveries, build_recipes};

let recipes = build_recipes();
let mut discoveries = Discoveries::new();

// Inside the loop, just after step():
for recipe in &recipes {
    if !discoveries.is_unlocked(recipe.unlocks) && (recipe.predicate)(&grid) {
        if discoveries.unlock(recipe.unlocks) {
            println!("Discovered: {}!", recipe.name);
            // TODO Session 20: trigger a brief on-screen banner.
        }
    }
}
```

`(recipe.predicate)(&grid)` calls the boxed closure — the parens around `recipe.predicate` are required because of how Rust parses method-like syntax on boxed values.

### 4. Filter the selector to discovered elements — 2 minutes

In `ui::draw_selector`:

```rust
pub fn draw_selector(selected: CellType, brush_radius: i32, discoveries: &Discoveries) {
    let all = [
        CellType::Sand, CellType::Water, CellType::Stone, CellType::Wood,
        CellType::Fire, CellType::Oil, CellType::Acid, CellType::Lava, CellType::Ice,
    ];

    let visible: Vec<CellType> = all.iter().copied()
        .filter(|t| discoveries.is_unlocked(*t))
        .collect();

    // ... rest of the swatch-drawing loop using `visible` ...
}
```

The keys that pick elements should also gate on discovery — pressing `5` for fire before fire is unlocked should do nothing.

### 5. Persist discoveries — 2 minutes

`Discoveries` already has `Serialize, Deserialize`. Save it alongside the grid in `persist.rs`:

```rust
#[derive(Serialize, Deserialize)]
struct SaveState {
    version: u32,
    grid: Vec<Vec<Cell>>,
    discoveries: Discoveries,        // new
}
```

(You'll need to update `save()` and `load()` to pass `&discoveries` and return both grid and discoveries.)

**Save. Run.** You start with sand, water, stone, wood. Build something with each. Heat wood (place it next to fire — wait, fire isn't unlocked. So first, get fire's recipe condition: heat wood above 150°. Pour water under a heat-source brush. Heat the wood with the warm water. Eventually fire unlocks. Then make lava. Then water-meets-lava unlocks acid. Each unlock is a tiny *aha*.

> **The Wow Moment.** Start a fresh save. Look at the selector: four squares. Play for ten minutes. Look back: eight squares. You've **built a discovery-based progression system in two files and 200 lines.** That's the whole gameplay loop of every alchemy-style game (Doodle God, Little Alchemy, Noita's spell discovery) you've ever played. **Session 20** makes it gorgeous; today's wow is "*the system actually works.*"

### 6. (Optional) Recipe trigger banner — 4 minutes

A two-second on-screen flash when an unlock happens:

```rust
let mut banner: Option<(String, u32)> = None;        // (text, frames left)

// On unlock:
banner = Some((format!("Discovered: {}!", recipe.name), 120));

// Each frame:
if let Some((text, frames_left)) = &mut banner {
    draw_text(text, screen_width()/2.0 - 100.0, 60.0, 32.0, YELLOW);
    *frames_left = frames_left.saturating_sub(1);
    if *frames_left == 0 { banner = None; }
}
```

The `Option<(String, u32)>` cleanly encodes "either no banner, or a banner with N frames remaining."

---

## Linux (Ubuntu) note

Closures and the `Fn` traits are pure-language features — no OS impact. One quality-of-life note for Ubuntu:

- The `println!("Discovered: {}!", recipe.name)` lines are useful when developing, but won't show if you launch via a desktop `.desktop` file (no terminal attached). For Ubuntu, also `eprintln!` to stderr — and journalctl will capture stderr from any process you launch via systemd:

  ```bash
  systemctl --user status sand-sim     # if launched as a user service
  journalctl --user -u sand-sim -f
  ```

  Overkill for a one-person learning project. Mentioned because once you're shipping Linux desktop apps for real, this is how you debug "where did the print go?"

- **Boxed closures and Rust binary size.** Each `Box::new(|grid| {...})` is a small heap allocation. For 5 recipes, that's ~200 bytes of heap. Trivially small. Don't optimise.

- The `+ Send + Sync` bounds on the boxed closure are required *if* you ever pass the recipe table to another thread. For a single-threaded sim, the bounds are unused but cost nothing. Leave them in — future-proofing.

---

## Common mistakes

### `error[E0277]: 'closure' may not be 'Send'`

Your closure captures a non-`Send` variable (e.g. an `Rc<...>`). Either drop the `Send` bound on `Recipe.predicate`, or use `Arc<...>` instead. For the simple recipes in this session, you shouldn't capture anything — the only argument is `grid`, passed in.

### `error[E0596]: cannot borrow data in a '&' reference as mutable`

You wrote `(recipe.predicate)(grid)` and forgot the `&`. Fix: `(recipe.predicate)(&grid)`.

### Recipe never fires

Add a `println!("checking recipe: {} = {}", recipe.name, fired)` inside the loop to confirm the predicate is being called and what it returns. Usually the bug is in the predicate (e.g. testing for adjacency in only one direction).

### Recipe fires on every frame

You forgot the `discoveries.is_unlocked(recipe.unlocks)` guard. Without it, the predicate fires every frame the condition holds; the banner stays up forever; you spam the terminal.

### `error[E0277]: 'CellType' is not 'Copy'`

You're somewhere assuming `CellType` is `Copy` (e.g. `discoveries.unlock(recipe.unlocks)` where `recipe.unlocks` is `CellType`). It IS `Copy` — but if you accidentally removed it from the derive in an earlier refactor, this catches you. Restore `Copy` in `#[derive(...)]`.

### Closure captures cause borrow checker errors

If you write a closure that captures a `&mut` of something inside the loop, Rust may not let you call it. Workaround: keep recipe predicates *pure* — they should only depend on their `&grid` argument. Discovery tracking lives outside the closure.

---

## Session challenge

Pick one — no solution provided.

1. **A reverse-discovery key.** Press `Shift+0` to *forget* everything and reset `Discoveries` to the starting four. Useful for testing recipes during development.
2. **A recipe that requires *all* of (A, B, C) on the grid simultaneously.** E.g. "discover gunpowder when you have wood AND fire AND smoke all in the same world." Single closure, three `cells_match` calls combined with `&&`.
3. **A recipe with a count threshold.** "Have at least 100 lava cells at once." Use `cells_match`'s sibling that returns the count instead of bool.
4. **Recipe difficulty levels.** Add a `difficulty: u8` field. The codex (Session 20) can colour-code recipes: easy (green), medium (yellow), hard (red). Adds dimensionality without adding new mechanics.

---

## Quick reference

| What | Code |
|---|---|
| Closure literal | `\|x\| x * 2` |
| Closure with body | `\|x, y\| { x + y }` |
| Generic-Fn parameter | `fn run(f: impl Fn(i32) -> i32)` |
| Boxed closure | `Box<dyn Fn(i32) -> i32>` |
| Move closure | `move \|x\| x + n` |
| Call a boxed closure | `(boxed)(arg)` |
| `.any(predicate)` | returns `bool` |
| `.all(predicate)` | returns `bool` |
| `.find(predicate)` | returns `Option<&T>` |
| `.count()` after filter | returns `usize` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 19**. Worth recording:

- The first recipe you discovered "by accident" while playing rather than by setting up the condition deliberately — that emergent feel is the whole point
- Your sentence on closures: what's the difference between a closure and a regular function?
