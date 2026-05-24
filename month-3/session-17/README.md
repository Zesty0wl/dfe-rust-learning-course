# Session 17 — Modules: Taming the Codebase

> **Stuck on a word?** Things like *module*, *crate*, *visibility*, *re-export*, *namespace* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your `main.rs` is **about 25 lines long** — and everything that used to be in it has been split into clean, focused modules in their own files: `simulation.rs`, `elements.rs`, `reactions.rs`, `rendering.rs`, `audio.rs`, `ui.rs`. No new visible feature. Today's win is *structure*.

---

## What you'll learn

- `mod`, `pub`, `use`, and `crate::` — Rust's visibility/namespacing system
- Splitting a single file into a directory of files
- `pub use` re-exports — controlling your module's public API
- Why **encapsulation** matters: the compiler enforces "this is private" in a way most languages can't
- Reading a refactor as a *taxonomy* of what the program does

---

## The big idea

`main.rs` from Session 16 is hovering around 500 lines. That's the upper end of "still readable in one file." Beyond that, **finding the right function takes longer than writing the function**, and that's the moment a single-file project starts to slow you down.

Rust's module system splits a file along three axes:

1. **Files become modules.** A new file in your `src/` folder is a new module, automatically.
2. **Each `pub` item is visible outside its module; the rest is private.**
3. **`use` brings names into scope.** Without it, everything would need its full path.

You'll split `sand-sim` into six modules today. The split is opinionated: every Rust simulation in the wild looks roughly like this. **Reusable scaffolding for every project you build for the rest of your life.**

---

## Concepts covered

- `mod foo;` in `main.rs` → loads `src/foo.rs`
- `pub fn`, `pub struct`, `pub use`
- `crate::` (this crate), `super::` (parent module), `self::` (this module)
- `use crate::elements::CellType;`
- Re-exports: `pub use crate::reactions::REACTIONS;`
- The convention: tests, helpers, and constants belong in the same module as the thing they support

---

## Building towards `sand-sim`

The module split is **the most generally-applicable engineering lesson in the course.** Every later session writes code into the right module rather than piling onto `main.rs`. Session 18's save/load goes in a new `persist.rs`. Session 19's recipes go in `recipes.rs`. Session 20's codex UI extends `ui.rs`. By Session 24's v1.0 ship, the project has maybe ten modules — but `main.rs` is still ~25 lines.

---

## Step-by-step walkthrough

> **Where you should be.** Session 16 finished. v0.2 ships. Now you copy `sand-sim-v0.2/` to `sand-sim-v1.0/` and start refactoring from there.

### 0. Branch the project — 2 minutes

```bash
mkdir -p month-3/milestone/sand-sim-v1.0
cp -R month-2/milestone/sand-sim-v0.2/. month-3/milestone/sand-sim-v1.0/
cd month-3/milestone/sand-sim-v1.0
cargo run --release    # confirm it works in the new location
```

Bump the version in `Cargo.toml`:

```toml
[package]
name    = "sand-sim"
version = "1.0.0-alpha"
```

### 1. Plan the split — 2 minutes

Before touching code, decide on the modules. A good split groups things by **what they're about**, not by what they technically *are*:

| Module | What lives there |
|---|---|
| `elements` | `CellType` enum, `Cell` struct, `density()`, `colour()` |
| `reactions` | `ReactionOutcome`, `build_reactions`, the `REACTIONS` static, `react()` |
| `simulation` | `step()`, all `update_*` functions, neighbour helpers, paint helper |
| `rendering` | `render_grid()`, `heatmap_colour()`, `cell_colour()` |
| `ui` | `draw_selector()`, `draw_legend()`, `draw_hud()`, `count_cells()` |
| `audio` | sound-loading, per-event cooldowns, the trigger logic |

`main.rs` keeps: window config, the high-level loop, glue.

### 2. Create the files — 1 minute

```bash
cd src/
touch elements.rs reactions.rs simulation.rs rendering.rs ui.rs audio.rs
```

### 3. Wire them up in `main.rs` — 1 minute

Top of `main.rs`:

```rust
use macroquad::prelude::*;

mod elements;
mod reactions;
mod simulation;
mod rendering;
mod ui;
mod audio;

use elements::{Cell, CellType, COLS, ROWS, CELL_SIZE};
use simulation::{step, paint};
use rendering::render_grid;
use ui::{draw_selector, draw_legend, draw_hud};
use audio::AudioState;
```

`mod foo;` tells rustc: "look for `src/foo.rs` (or `src/foo/mod.rs`) and load it as a module named `foo`." `use ...` brings those names into the current scope.

### 4. Move `elements.rs` — 5 minutes

Cut these from `main.rs`, paste into `src/elements.rs`, mark public:

```rust
// src/elements.rs

use macroquad::prelude::*;

pub const COLS: usize = 120;
pub const ROWS: usize = 80;
pub const CELL_SIZE: f32 = 6.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
    Wood,
    Fire,
    Smoke,
    Oil,
    OilFire,
    Steam,
    Acid,
    Lava,
    Ice,
}

impl CellType {
    pub fn colour(self) -> Color { /* ... */ }
    pub fn name(self) -> &'static str { /* ... */ }
    pub fn density(self) -> u8 { /* ... */ }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type:   CellType,
    pub temperature: f32,
    pub lifetime:    u8,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self { /* ... */ }
    pub fn empty() -> Self { Self::new(CellType::Empty) }
    pub fn is_empty(&self) -> bool { matches!(self.cell_type, CellType::Empty) }
    pub fn heat(&mut self, delta: f32) {
        self.temperature = (self.temperature + delta).min(2000.0);
    }
}
```

Run `cargo check`. Errors will point to every `main.rs` site that uses `CellType` or `Cell` without the new `use` line. Either add `use crate::elements::CellType;` at the top, or path-qualify (`crate::elements::CellType::Sand`).

### 5. Move `reactions.rs` — 5 minutes

```rust
// src/reactions.rs

use std::collections::HashMap;
use std::sync::OnceLock;
use crate::elements::CellType;

#[derive(Debug, Clone, Copy)]
pub struct ReactionOutcome {
    pub new_source: Option<CellType>,
    pub new_target: Option<CellType>,
    pub heat: f32,
    pub probability: f32,
}

impl ReactionOutcome {
    pub fn replace_both(source: CellType, target: CellType, heat: f32) -> Self { /* ... */ }
}

static REACTIONS: OnceLock<HashMap<(CellType, CellType), ReactionOutcome>> = OnceLock::new();

pub fn reactions() -> &'static HashMap<(CellType, CellType), ReactionOutcome> {
    REACTIONS.get_or_init(build)
}

fn build() -> HashMap<(CellType, CellType), ReactionOutcome> {
    // ... your existing build_reactions body ...
}

pub fn react(source: CellType, target: CellType) -> Option<ReactionOutcome> {
    reactions().get(&(source, target)).copied()
}
```

Note `pub` on the things that `main.rs` and `simulation.rs` need (`ReactionOutcome`, `react`); the `build` function stays private — it's an implementation detail.

### 6. Move `simulation.rs` — 8 minutes

The largest move. Cut every `update_*` function, `step`, `paint`, `try_react`, the `NEIGHBOURS_*` consts. Paste into `src/simulation.rs`. The top of the file should be:

```rust
// src/simulation.rs

use crate::elements::{Cell, CellType, ROWS, COLS};
use crate::reactions::{react, ReactionOutcome};

pub const NEIGHBOURS_4: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
pub const NEIGHBOURS_8: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

pub fn step(grid: &mut Vec<Vec<Cell>>) { /* ... */ }

pub fn paint(grid: &mut Vec<Vec<Cell>>, row: i32, col: i32, radius: i32,
             cell_type: CellType, temperature: f32) { /* ... */ }

fn try_react(/* ... */) { /* ... */ }
fn update_sand(/* ... */) { /* ... */ }
fn update_water(/* ... */) { /* ... */ }
// ... etc ...
```

`step` and `paint` are `pub` because `main.rs` calls them. The per-element `update_*` functions are private — only `step` calls them, and `step` lives in the same module.

### 7. Rendering, UI, audio — 5 minutes

Same pattern. `rendering.rs` gets the per-frame draw loop and heatmap colour. `ui.rs` gets the selector, legend, HUD, `count_cells`. `audio.rs` gets sound loading and the cooldown triggers — wrap them in a struct for cleanliness:

```rust
// src/audio.rs

use macroquad::audio::{Sound, load_sound, play_sound_once};
use std::collections::HashMap;
use crate::elements::CellType;

pub struct AudioState {
    pub sand: Sound,
    pub fire: Sound,
    pub lava: Sound,
    pub boom: Sound,
    pub cd:   [u32; 4],   // [sand, fire, lava, boom]
    pub prev_oilfire: usize,
}

impl AudioState {
    pub async fn load() -> Self {
        Self {
            sand: load_sound("assets/sand.wav").await.unwrap(),
            fire: load_sound("assets/fire.wav").await.unwrap(),
            lava: load_sound("assets/lava.wav").await.unwrap(),
            boom: load_sound("assets/boom.wav").await.unwrap(),
            cd: [0; 4],
            prev_oilfire: 0,
        }
    }

    pub fn tick(&mut self) {
        for c in &mut self.cd { if *c > 0 { *c -= 1; } }
    }

    pub fn trigger(&mut self, counts: &HashMap<CellType, usize>, mouse_held_sand: bool) {
        // ... the logic from Session 16 ...
    }
}
```

### 8. The new `main.rs` — 5 minutes

After all moves it's a clean ~25 lines (this depends on the six sibling modules built in steps 2–7 above — it isn't a standalone program):

```rust,ignore
use macroquad::prelude::*;

mod elements;
mod reactions;
mod simulation;
mod rendering;
mod ui;
mod audio;

use elements::{Cell, CellType, COLS, ROWS, CELL_SIZE};
use simulation::{step, paint};
use rendering::render_grid;
use ui::{draw_selector, draw_legend, draw_hud, count_cells};
use audio::AudioState;

fn window_conf() -> Conf {
    Conf {
        window_title: "sand-sim v1.0".to_owned(),
        window_width:  (COLS as f32 * CELL_SIZE) as i32,
        window_height: (ROWS as f32 * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::empty(); COLS]; ROWS];
    let mut audio = AudioState::load().await;
    let mut selected = CellType::Sand;
    let mut paused = false;
    let mut heatmap = false;
    let mut brush_radius = 2i32;

    loop {
        ui::handle_input(&mut grid, &mut selected, &mut paused, &mut heatmap, &mut brush_radius);
        if !paused { step(&mut grid); }
        audio.tick();
        let counts = count_cells(&grid);
        audio.trigger(&counts, selected == CellType::Sand && is_mouse_button_down(MouseButton::Left));
        clear_background(BLACK);
        render_grid(&grid, heatmap);
        draw_selector(selected, brush_radius);
        draw_legend();
        draw_hud(&counts);
        next_frame().await;
    }
}
```

That's the high-level shape of the whole program, on one screen. **Save. Run.** Identical behaviour to v0.2. The win is invisible.

> **The Wow Moment.** Open the `src/` folder in your file explorer. You see seven files. Each file is between 40 and 200 lines. Each does *one thing*. You can ask a friend "where does fire spread?" and they answer with five seconds of context: "open `simulation.rs`, look for `update_fire`." That speed of navigation is what every senior engineer is buying with their refactoring effort. **You bought it in one session.**

### 9. (Optional) Re-exports — 2 minutes

If you'd rather write `use crate::Cell` than `use crate::elements::Cell` from `main.rs`, add re-exports:

```rust
// src/lib.rs (or just keep in main.rs)
pub use crate::elements::{Cell, CellType, COLS, ROWS, CELL_SIZE};
pub use crate::reactions::{ReactionOutcome, react};
```

Some teams love this (shorter call sites). Others hate it (hides where things come from). Pick a side; be consistent.

---

## Linux (Ubuntu) note

The split has a real upside on Ubuntu: **incremental builds get much faster** because `cargo` recompiles only the changed module and anything that depends on it. After today, editing `audio.rs` and running `cargo run` recompiles maybe 5% of the codebase instead of 100%.

To verify on your Ubuntu machine:

```bash
cargo clean
time cargo build --release          # first build, slow (~60s typical)
# Now edit audio.rs (add a comment somewhere)
time cargo build --release          # ~3-8s typical — only audio.rs and main.rs recompile
```

You can also enable `sccache` to share build artifacts across projects:

```bash
cargo install sccache
export RUSTC_WRAPPER=$(which sccache)
```

Add the export to `~/.bashrc` / `~/.zshrc` to make it permanent. Saves serious time across multiple Rust projects.

VS Code's *file explorer* on Ubuntu becomes the primary way you navigate the project after today. The Outline view (`Ctrl+Shift+O` to jump to a symbol in the current file, `Ctrl+T` for workspace-wide) becomes muscle memory.

---

## Common mistakes

### `error[E0432]: unresolved import 'crate::elements::Cell'`

You forgot to mark the item `pub` in the module file. Check `pub struct Cell { ... }`, not `struct Cell { ... }`. Same for `pub fn`, `pub const`, `pub enum`.

### `error[E0603]: function 'update_sand' is private`

Calling a private function from outside its module. Either make it `pub`, or call it via a public function in the same module. The general guideline: keep the surface area as small as possible. Only items used from outside need `pub`.

### Circular imports

`elements.rs` uses `reactions::ReactionOutcome` and `reactions.rs` uses `elements::CellType`. Compiler complains. Solution: one direction wins. Usually the lower-level module (elements) should not depend on the higher-level one (reactions). Re-architect so `ReactionOutcome` lives in `reactions.rs` and `elements.rs` doesn't reference it.

### Tests in the wrong module

If you've been writing tests, leave them at the bottom of the same module they test:

```rust
// in src/elements.rs
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn new_empty_is_empty() { assert!(Cell::empty().is_empty()); }
}
```

`use super::*;` brings the parent module's items into scope.

### `error: file not found for module 'simulation'`

You declared `mod simulation;` in `main.rs` but never created `src/simulation.rs`. Make the file. The compiler is literal: `mod foo` requires `src/foo.rs` or `src/foo/mod.rs`.

### Re-exports break code

If `main.rs` did `use crate::elements::CellType;` and you also add `pub use crate::elements::CellType;` at the top of `main.rs`, you have two names for the same thing. Cosmetic but confusing. Pick one.

---

## Session challenge

Pick one — no solution provided.

1. **A `tests/` directory.** Move integration tests out of `#[cfg(test)] mod tests` and into `tests/grid_invariants.rs` — Rust's convention for *integration* tests. Test that `step` is idempotent on an empty grid, that `react(Lava, Water)` returns the stone+steam outcome, etc.
2. **A `benches/` directory.** Create `benches/step_bench.rs` and use the `criterion` crate to benchmark how long `step` takes on a 1000-fire-cell grid. Compare across `cargo run` and `cargo run --release`.
3. **Cargo workspace.** Split `sand-sim` into a workspace with `engine/` (everything except `main.rs`) and `sand-sim/` (the binary). Useful for the Session 19+ recipe system if you want to share code with a separate codex tool.
4. **`lib.rs` plus `main.rs`.** Convert to a hybrid: `lib.rs` exposes the engine, `main.rs` is the binary that uses it. Same shape as a workspace but in one crate.

---

## Quick reference

| What | Code |
|---|---|
| Make a file a module | `mod foo;` in `main.rs` (loads `src/foo.rs`) |
| Make item visible | `pub fn`, `pub struct`, etc. |
| Cross-module path | `use crate::elements::Cell;` |
| Parent module | `use super::Cell;` |
| This module | `use self::helper;` |
| Re-export | `pub use crate::elements::Cell;` |
| Test in same file | `#[cfg(test)] mod tests { use super::*; ... }` |
| Integration tests | `tests/foo.rs` (own crate, sees only `pub`) |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 17**. Worth recording:

- A screenshot of your `src/` folder open in VS Code's file explorer — six neat files where before there was one giant one
- Your sentence on "what `pub` *means*" — the discipline of opening only what callers need
