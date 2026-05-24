# Session 18 — File I/O: Save and Load

> **Stuck on a word?** Things like *serialise*, *deserialise*, *JSON*, *Result*, *error propagation* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session pressing **`S` saves the entire world to `save.json`** and **`L` loads it back**. Close the app, reopen, press `L` — the simulation resumes where you left off. The save file is human-readable JSON you can inspect in VS Code.

---

## What you'll learn

- **`serde`** — the de facto Rust serialisation framework, used by almost every Rust project that handles structured data
- **`serde_json`** — the JSON binding
- `#[derive(Serialize, Deserialize)]` — three words, infinite leverage
- `Result<T, E>` — Rust's error type, the cousin of `Option`
- The `?` operator — early-returns errors with one character
- `std::fs::write` / `std::fs::read_to_string`

---

## The big idea

Persistence is the moment a *toy* becomes a *tool*. Right now the only way to "save" a satisfying configuration is to leave the window open forever. With save/load, you can capture an interesting state, share it, iterate on it, come back to it tomorrow.

Two ideas land today. **`Result<T, E>`** is `Option<T>`'s cousin: instead of "value or nothing," it's "value or *reason for failure*." Every I/O operation returns a `Result`, because everything that touches a disk can fail (file missing, permission denied, disk full). The **`?` operator** is the syntactic sugar that makes propagating those errors painless — `let s = fs::read_to_string("save.json")?;` reads "if the read fails, return the error from this function."

The other big idea: **let `serde` do the work**. You'll add three derive attributes (`Serialize`, `Deserialize`) and four lines of code, and your entire `Vec<Vec<Cell>>` becomes round-trippable JSON. **You write almost nothing.**

---

## Concepts covered

- `serde = { version = "1", features = ["derive"] }`
- `serde_json = "1"`
- `#[derive(Serialize, Deserialize)]` on `Cell`, `CellType`, and the save wrapper
- `Result<T, E>`, `Ok(...)`, `Err(...)`
- The `?` operator and `fn main() -> Result<(), Box<dyn Error>>`
- `serde_json::to_string_pretty` and `from_str`
- The conventional `.json` file layout for saves

---

## Building towards `sand-sim`

Save/load is **the gate to multi-session play.** Session 19's recipe system stores discoveries — those need to persist too. Session 20's codex remembers what you've unlocked — also persistent. Session 23 adds a "title screen" that asks "new game or load?" By the end of Month 3, the same JSON file holds the world, the recipes you've discovered, and the codex state. Today is the foundation.

---

## Step-by-step walkthrough

> **Where you should be.** Session 17 finished. `main.rs` is short. The project is split across `elements.rs`, `reactions.rs`, `simulation.rs`, `rendering.rs`, `ui.rs`, `audio.rs`. The sim runs.

### 1. Add the dependencies — 2 minutes

```bash
cargo add serde --features derive
cargo add serde_json
```

`Cargo.toml`:

```toml
[dependencies]
macroquad   = { version = "0.4", features = ["audio"] }
fastrand    = "2"
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
```

`serde` is just the framework. `serde_json` is one of many output formats — there are also `serde_yaml`, `bincode`, `toml`, `ciborium`. Once your types implement `Serialize + Deserialize`, you can swap formats without changing your types.

### 2. Derive on `CellType` and `Cell` — 2 minutes

Open `src/elements.rs`. Add `Serialize, Deserialize` to both derives:

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellType {
    Empty,
    Sand,
    Water,
    // ... etc ...
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type:   CellType,
    pub temperature: f32,
    pub lifetime:    u8,
}
```

That's it. Both types are now serialisable.

### 3. A new module — `persist.rs` — 5 minutes

Create `src/persist.rs`:

```rust
use std::fs;
use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::elements::{Cell, CellType, COLS, ROWS};

#[derive(Serialize, Deserialize)]
struct SaveState {
    /// File-format version. Increment when the layout changes.
    version: u32,
    grid: Vec<Vec<Cell>>,
}

pub fn save(grid: &Vec<Vec<Cell>>, path: &str) -> Result<(), Box<dyn Error>> {
    let state = SaveState { version: 1, grid: grid.clone() };
    let json = serde_json::to_string_pretty(&state)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load(path: &str) -> Result<Vec<Vec<Cell>>, Box<dyn Error>> {
    let json = fs::read_to_string(path)?;
    let state: SaveState = serde_json::from_str(&json)?;
    if state.version != 1 {
        return Err(format!("unsupported save version {}", state.version).into());
    }
    if state.grid.len() != ROWS || state.grid[0].len() != COLS {
        return Err(format!("save dimensions {}x{} don't match {}x{}",
            state.grid.len(), state.grid[0].len(), ROWS, COLS).into());
    }
    Ok(state.grid)
}
```

Register the module in `main.rs`:

```rust
mod persist;
```

A lot landed in step 3. Let's unpack it:

- **`SaveState`** wraps the grid in a versioned struct. Future saves can change the layout; the version field lets us refuse to load old saves rather than crashing.
- **`Result<T, E>`** is just an enum:

  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```

  We return `Result<(), Box<dyn Error>>` — *"either nothing succeeded, or an error of any type happened."* `Box<dyn Error>` is a *trait object* that holds any type implementing `Error`. Comes up properly in Session 20.
- **`?`** at end of `fs::write(path, json)?;` means "if this returned `Err(e)`, return `Err(e)` from the enclosing function. Otherwise unwrap the `Ok`." It replaces a `match`-pattern that would otherwise look like:

  ```rust
  let json = match serde_json::to_string_pretty(&state) {
      Ok(j) => j,
      Err(e) => return Err(Box::new(e)),
  };
  ```

  Eleven lines compressed to one.
- **`serde_json::to_string_pretty`** does indented JSON. There's also `to_string` (one-liner) and `to_writer_pretty` (writes directly to a file handle, no intermediate `String`).
- **`from_str`** parses JSON back into your type. `serde` derives the parsing code.

### 4. Wire keys in `main.rs` — 3 minutes

```rust
// In your input-handling block (probably in ui.rs::handle_input):
if is_key_pressed(KeyCode::S) {
    match persist::save(&grid, "save.json") {
        Ok(()) => println!("Saved to save.json"),
        Err(e) => eprintln!("Save failed: {}", e),
    }
}
if is_key_pressed(KeyCode::L) {
    match persist::load("save.json") {
        Ok(new_grid) => grid = new_grid,
        Err(e) => eprintln!("Load failed: {}", e),
    }
}
```

If `handle_input` is a separate function, pass `&mut grid` through it so it can replace the grid on load.

**Save. Run.** Build a satisfying scene. Press `S`. Open `save.json` in VS Code — beautiful indented JSON, easy to read. Close the running sim. Run `cargo run --release` again. Press `L`. **Your scene is back.**

> **The Wow Moment.** Open `save.json` in VS Code. You see the entire world as a JSON array of rows of cells, each cell with its type, temperature, and lifetime. **You can hand-edit it.** Change a few `"Sand"` to `"Lava"` in the file, save, press `L` in the running sim — your edits appear in the world. **You added a level editor in zero code.** This is the leverage `serde` gives you: the save format *is* the level format *is* the share format.

### 5. Auto-save on close — 4 minutes

Saving feels good but only manually. Add an auto-save on every Nth frame:

```rust
let mut autosave_counter: u32 = 0;

// inside loop, after step():
autosave_counter += 1;
if autosave_counter >= 60 * 30 {           // every 30 seconds
    if let Err(e) = persist::save(&grid, "autosave.json") {
        eprintln!("Autosave failed: {}", e);
    }
    autosave_counter = 0;
}
```

Now you have two save files — manual `save.json` and rolling `autosave.json`. Good safety net.

### 6. (Optional) Compress — 3 minutes

JSON pretty-printed for 120×80 cells gets large (~700KB). Switch to non-pretty for distribution:

```rust
let json = serde_json::to_string(&state)?;       // not _pretty
```

Or use `bincode` for binary, ~50KB instead of ~700KB:

```toml
[dependencies]
bincode = "1"
```

```rust
use bincode;
let bytes = bincode::serialize(&state)?;
fs::write(path, bytes)?;
```

(Bincode loses the "human-readable, hand-editable" superpower of JSON. Trade-off.)

---

## Linux (Ubuntu) note

File I/O is one of the few areas where path conventions still bite cross-platform.

- **Use relative paths.** `"save.json"` is relative to the *current working directory*, which for `cargo run` is the directory containing `Cargo.toml`. On Linux, this means saves land in `month-3/milestone/sand-sim-v1.0/save.json`. Easy to find.
- **Don't write to `~/` or `/tmp/` with hard-coded paths.** `~/` doesn't expand inside Rust string literals (`"~/save.json"` literally tries to write to a folder called `~`). Use `std::env::var("HOME")` to read the home directory:

  ```rust
  let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
  let path = format!("{}/.sand-sim/save.json", home);
  ```

  Then `std::fs::create_dir_all` before writing.
- **Permissions.** Saves write with mode `644` by default on Linux (owner read+write, group/others read). If you `cd` into a directory you don't own and run the sim, the save will fail. Symptom: `Permission denied (os error 13)`.
- **Concurrent saves.** If two processes write `save.json` simultaneously, you can get a half-written file. Real apps use `tempfile` + `rename` (atomic on Linux). Overkill for `sand-sim` v1.0; mentioned because it's the only real production trap.

To check the file size after saving:

```bash
ls -lh save.json
```

A 120×80 pretty-JSON save is around 600–800 KB. Non-pretty is ~120 KB. Bincode is ~50 KB.

---

## Common mistakes

### `error[E0277]: the trait 'Serialize' is not implemented for 'Cell'`

You forgot to add `Serialize` to the derive on `Cell` *or* on `CellType`. Both need it. If a struct has a non-serialisable field, the whole struct can't be derived.

### `Error: invalid value: integer 13, expected variant index 0 <= i < 12`

You loaded a save file written before adding a new variant to `CellType`. `serde` serialises enum variants by name *or* by index depending on configuration. Bump the `version` field and refuse to load old saves, or migrate the file by hand.

### Save file is huge

You're using `to_string_pretty`. Switch to `to_string` for non-debug usage, or `bincode` for binary. A 120×80 grid in pretty JSON is ~700KB; bincode is ~50KB.

### `error: cannot find macro 'Serialize' in this scope`

You forgot the `derive` feature on `serde`. `cargo add serde --features derive` then check `Cargo.toml` shows `serde = { version = "1", features = ["derive"] }`.

### `Error: failed to fill whole buffer` on load

The save file is empty or corrupted. Open it; if it's zero bytes, your save failed silently and the load is seeing nothing. Add `eprintln!` on the save Err arm to catch the failure.

### Auto-save makes the simulation hitch every 30 seconds

JSON serialisation of 9,600 cells takes ~10ms — visible as a frame-drop. Two fixes: (a) serialise asynchronously on a background thread, (b) switch to bincode (3-5× faster), (c) save less often. For v1.0, the easiest is to accept the brief hitch.

### `error: cannot move out of grid because it is borrowed`

You wrote `let grid = persist::load("save.json").unwrap();` inside the loop, shadowing the outer `grid`. Use `grid = ...` instead (no `let`) to update the existing variable.

---

## Session challenge

Pick one — no solution provided.

1. **Snapshot history.** Save every N frames to `snapshots/snapshot-NNNN.json`. Add keys `[` and `]` to step backward/forward through snapshots. You've built a debugger time-machine in 20 lines.
2. **Diff two saves.** Read two save files and print which cells differ. Useful for "what changed in the last 30 seconds?"
3. **YAML output.** Add `serde_yaml`. Save the same `SaveState` as `save.yaml`. Compare file sizes and readability. (Tradeoffs: YAML allows comments, JSON doesn't; YAML is more error-prone parser; JSON is universally supported.)
4. **CSV output for the grid.** Export only the `cell_type` of each cell as a CSV grid — one row per row, comma-separated. Loads in Excel/Google Sheets and shows the grid as a typed table. Different audience, different format.

---

## Quick reference

| What | Code |
|---|---|
| Add serde | `cargo add serde --features derive` |
| Derive | `#[derive(Serialize, Deserialize)]` |
| Pretty JSON | `serde_json::to_string_pretty(&value)?` |
| Compact JSON | `serde_json::to_string(&value)?` |
| Parse JSON | `serde_json::from_str::<T>(&s)?` |
| Write file | `std::fs::write(path, contents)?` |
| Read file | `std::fs::read_to_string(path)?` |
| `?` operator | `let s = thing()?;` |
| Result type | `Result<T, E>` |
| Boxed error | `Box<dyn std::error::Error>` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 18**. Worth recording:

- A screenshot of `save.json` open in VS Code alongside the sim it represents — visible evidence the world is now writable
- Your sentence on the `?` operator — "one character that turns ten lines of error handling into one"
