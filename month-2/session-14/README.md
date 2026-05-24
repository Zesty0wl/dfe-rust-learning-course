# Session 14 — Acid and the Reactions Architecture

> **The most important session in Month 2.** This is the refactor that turns `sand-sim` from "a program with lots of if-statements" into "a program with a clear architecture." Read it twice.

> **Stuck on a word?** Things like *HashMap*, *key*, *value*, *architecture*, *refactor*, *table-driven* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session every reaction in the simulation lives in a single **`HashMap<(CellType, CellType), ReactionOutcome>`** — and your first new element, **acid**, is added in three lines without touching any existing reaction code. Adding a new reaction stops being an if-statement edit and starts being a table row.

---

## What you'll learn

- `HashMap<K, V>` — Rust's hash table, keyed by anything that's `Hash + Eq`
- Why the `(CellType, CellType)` tuple needs `Hash + Eq` derived on `CellType`
- Building the reactions table once, at startup, and querying it per frame
- A `ReactionOutcome` with optional fields — partial transformations
- The general principle: **table-driven code beats branchy code** as soon as you have more than four cases

---

## The big idea

You currently have a `react` function that's a big `match` on `(source, target)`. That worked for two reactions (fire+wood, fire+oil). Add ice, lava, acid, glass, gunpowder, concrete, rust, mud — and the function explodes to 80 lines of nearly-identical arms.

The refactor is: **lift the logic into data.** Build a `HashMap` once, where the key is a `(CellType, CellType)` pair and the value is the `ReactionOutcome`. The function `react` collapses to one line: `REACTIONS.get(&(source, target)).copied()`.

This is **table-driven programming**. It scales linearly with the number of reactions instead of multiplicatively. Adding a reaction is one HashMap insert. Reading the reaction list is one glance at a function. Testing a reaction is one call to the table with a tuple.

This pattern is *everywhere* in production code — game engines, compilers, parsers, network protocols. Today you do it for chemistry. Once you've done it once, you'll spot opportunities to do it everywhere.

---

## Concepts covered

- `HashMap<K, V>` construction with `.insert(k, v)`
- `HashMap::get(&k)` returning `Option<&V>`
- `.copied()` — turn `Option<&V>` into `Option<V>` when V is `Copy`
- Building a static-ish table inside a function (`once_cell` / `LazyLock` mentioned, not required)
- Optional reaction fields: `replace_source: Option<CellType>` so a reaction can leave one cell alone
- The "probability" field — a reaction that fires probabilistically each frame
- Adding **acid** with one HashMap entry per reactable material

---

## Building towards `sand-sim`

Today's HashMap is the **architectural backbone** of the rest of the project. Session 15 adds lava + ice as three table rows. Session 19's recipe system iterates the table to detect "newly-possible reactions." Session 22 adds time-based variants by extending `ReactionOutcome` with a `delay_ticks` field — no change to the call site.

Future you, three months from now, looking at someone else's sandbox code that has 200 lines of `match` arms, will mutter "should be a table" and immediately know what to do.

---

## Step-by-step walkthrough

> **Where you should be.** Session 13 finished. Eight cell types working (sand, water, stone, wood, fire, oil, oil-fire, smoke, steam). The `react` function is a `match` on `(CellType, CellType)` with three or four reactions in it.

### 1. Upgrade `ReactionOutcome` — 3 minutes

Replace the Session 10 version:

```rust
#[derive(Debug, Clone, Copy)]
struct ReactionOutcome {
    /// What the source cell becomes. `None` = leave it alone.
    new_source: Option<CellType>,
    /// What the target neighbour becomes. `None` = leave it alone.
    new_target: Option<CellType>,
    /// Heat released at the reaction site (degrees).
    heat: f32,
    /// Per-frame probability the reaction fires (1.0 = always, 0.0 = never).
    probability: f32,
}

impl ReactionOutcome {
    fn replace_both(source: CellType, target: CellType, heat: f32) -> Self {
        Self {
            new_source: Some(source),
            new_target: Some(target),
            heat,
            probability: 1.0,
        }
    }

    fn consume_target(replacement_for_target: CellType, heat: f32, probability: f32) -> Self {
        Self {
            new_source: None,                              // source unchanged
            new_target: Some(replacement_for_target),
            heat,
            probability,
        }
    }
}
```

The two constructors handle the most common shapes ("both cells transform" and "source eats target slowly"). You'll add more constructors as the table grows.

### 2. The reactions table — 6 minutes

```rust
use std::collections::HashMap;

/// Build the global reactions table. Called once at startup.
fn build_reactions() -> HashMap<(CellType, CellType), ReactionOutcome> {
    use CellType::*;
    let mut r: HashMap<(CellType, CellType), ReactionOutcome> = HashMap::new();

    // -- Fire reactions --
    // Fire + Wood = both become fire (instant; lots of heat).
    r.insert((Fire, Wood), ReactionOutcome::replace_both(Fire, Fire, 80.0));
    r.insert((Wood, Fire), ReactionOutcome::replace_both(Fire, Fire, 80.0));

    // Fire + Oil = both become oil-fire (violent, more heat).
    r.insert((Fire, Oil), ReactionOutcome::replace_both(OilFire, OilFire, 120.0));
    r.insert((Oil, Fire), ReactionOutcome::replace_both(OilFire, OilFire, 120.0));

    // Fire + Water = fire dies, water becomes steam (heat is consumed, hence negative).
    r.insert((Fire, Water), ReactionOutcome::replace_both(Empty, Steam, -50.0));
    r.insert((Water, Fire), ReactionOutcome::replace_both(Empty, Steam, -50.0));

    r
}
```

Use `use CellType::*;` inside the function to drop the `CellType::` prefix on every line — makes the table readable. The `*` glob is normally avoided in Rust, but inside a focused builder function it's fine.

Above `main`, hold the table in a one-shot global:

```rust
use std::sync::OnceLock;

static REACTIONS: OnceLock<HashMap<(CellType, CellType), ReactionOutcome>> = OnceLock::new();

fn reactions() -> &'static HashMap<(CellType, CellType), ReactionOutcome> {
    REACTIONS.get_or_init(build_reactions)
}
```

`OnceLock` (in the standard library since Rust 1.70) lazily initialises the table on first access and gives a static lifetime. We don't have to drag in `once_cell` or `lazy_static`. The closure `build_reactions` runs exactly once.

### 3. Collapse `react` to one line — 2 minutes

```rust
fn react(source: CellType, target: CellType) -> Option<ReactionOutcome> {
    reactions().get(&(source, target)).copied()
}
```

That's it. The whole reaction engine. `.copied()` turns the `Option<&ReactionOutcome>` from `get` into `Option<ReactionOutcome>` (cheap because the struct is `Copy`).

### 4. Use the probability and the optional fields — 4 minutes

In `try_react` from Session 10, update for the new fields:

```rust
fn try_react(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    let source = grid[row][col].cell_type;
    if matches!(source, CellType::Empty) { return; }

    for (dr, dc) in NEIGHBOURS_4 {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);

        let target = grid[nr][nc].cell_type;
        let Some(outcome) = react(source, target) else { continue; };
        if fastrand::f32() > outcome.probability { continue; }

        if let Some(ns) = outcome.new_source {
            grid[row][col] = Cell::new(ns);
        }
        if let Some(nt) = outcome.new_target {
            grid[nr][nc] = Cell::new(nt);
        }
        grid[row][col].heat(outcome.heat);
        grid[nr][nc].heat(outcome.heat);
        return;          // one reaction per cell per frame
    }
}
```

Three new bits of syntax to absorb:

- `let Some(outcome) = react(...) else { continue; };` — the **let-else** form (Rust 1.65+). Cleaner than `match` when you only have one happy path. If the right-hand side is `None`, the `else` block runs and must diverge (`continue`, `break`, `return`, `panic!`, etc.).
- `outcome.new_source.is_some()` could replace `if let Some(ns) = ...` but the `if let` form is idiomatic for "do something with the value if it's there."
- The probability gate is one extra line and changes the whole feel: low-probability reactions (acid eating stone, 5%) feel like slow corrosion; high-probability reactions (fire+oil, 100%) feel instant.

### 5. Add acid — 4 minutes

This is the payoff. Add the variant:

```rust
enum CellType {
    // ...
    Acid,
}

// colour:
CellType::Acid => Color::new(0.55, 1.00, 0.30, 1.0),  // bright bilious green

// selector + key:
let elements = [Sand, Water, Stone, Wood, Fire, Oil, Acid];
if is_key_pressed(KeyCode::Key7) { selected = CellType::Acid; }
```

Acid falls like water (it's a liquid). Reuse `update_water` *almost* directly — copy it to `update_acid` and swap the type checks. Or, more elegantly:

```rust
fn update_liquid(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize, my_type: CellType) {
    // Body of update_water, but using `my_type` instead of hardcoded WATER.
}

fn update_cell(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    let ct = grid[row][col].cell_type;
    match ct {
        CellType::Water | CellType::Oil | CellType::Acid => update_liquid(grid, row, col, ct),
        // ... etc
    }
}
```

(One generic liquid update used for water, oil, acid. Tighter still.)

**Now the magic part.** Add acid's reactions to the table:

```rust
fn build_reactions() -> HashMap<(CellType, CellType), ReactionOutcome> {
    use CellType::*;
    let mut r = HashMap::new();

    // ... fire reactions above ...

    // -- Acid reactions --
    // Acid + Stone = stone is corroded. 5% per frame; acid is consumed.
    r.insert((Acid, Stone), ReactionOutcome {
        new_source: Some(Empty), new_target: Some(Empty), heat: 5.0, probability: 0.05,
    });
    r.insert((Stone, Acid), ReactionOutcome {
        new_source: Some(Empty), new_target: Some(Empty), heat: 5.0, probability: 0.05,
    });

    // Acid + Wood = wood dissolves faster.
    r.insert((Acid, Wood), ReactionOutcome {
        new_source: Some(Empty), new_target: Some(Empty), heat: 8.0, probability: 0.15,
    });
    r.insert((Wood, Acid), ReactionOutcome {
        new_source: Some(Empty), new_target: Some(Empty), heat: 8.0, probability: 0.15,
    });

    // Acid + Sand = nothing (sand is silica; acid leaves it alone). No table entry needed.

    r
}
```

**Save. Run.** Build a stone wall. Pour acid on it. **Watch the wall dissolve slowly.** Build a wood wall. Pour acid. **Dissolves faster.** Drop acid onto sand. **Nothing happens — sand is acid-proof.** *All three behaviours come from the absence or presence of a table row.* No code change to handle the "sand is inert" case.

> **The Wow Moment.** Open `build_reactions`. Read it top-to-bottom. **It is the chemistry of your world, on one page.** You can hand this function to a friend who doesn't know Rust and they can read it: "fire + wood becomes fire + fire." "Acid + stone, 5%, becomes empty + empty." *Code as documentation.* Adding a new reaction now is a single insert into the table — no `match` arms to extend, no `if`-chains to thread through. **This is what people mean when they say "Rust has great architectural ergonomics."**

### 6. (Optional) Print the reaction table at startup — 2 minutes

For evidence that the table is real:

```rust
fn main_setup() {
    let table = reactions();
    println!("Loaded {} reactions:", table.len());
    for ((src, tgt), out) in table.iter() {
        println!("  {:?} + {:?} -> {:?}", src, tgt, out);
    }
}
```

Call it once at the top of `main`. The terminal now shows your simulation's entire chemistry as a list. Drop a screenshot into your DofE log — it's beautiful evidence.

---

## Linux (Ubuntu) note

`HashMap` uses a SipHash by default — fast and DoS-resistant. The simulation's per-frame hash lookups (~thousand per frame for a busy grid) are well below the noise floor on any Ubuntu machine from the last decade.

If you want to verify under `perf`:

```bash
cargo build --release
perf stat -e task-clock,cycles ./target/release/sand-sim
```

The HashMap lookup will not appear in the hot path. The fire/oil-fire neighbour scan still dominates.

`OnceLock` (used for the static reactions table) is in the standard library since Rust 1.70. If you've been doing the course on a vintage Ubuntu with an old `rustc` from apt, run `rustup update stable` to refresh — apt's `rustc` package on 22.04 may be too old. The recommended path is always `rustup` (covered in `SETUP.md`), not apt.

---

## Common mistakes

### `error: trait 'Hash' is not implemented for 'CellType'`

You added a variant with a non-`Hash` payload (a `Vec`, an `f32`, etc.). Either drop the payload, swap to a `Hash`-able type (`u8` instead of `f32`), or manually `impl Hash for CellType`. Easiest fix: enum payloads must be `Hash + Eq` for the enum to be `Hash + Eq` for use as a HashMap key.

### Acid eats everything including sand

You either added a `(Acid, Sand)` entry to the table, or your `try_react` is mutating cells before checking the probability. Read the table top-to-bottom — only the listed reactions fire. If there's no `(Acid, Sand)` entry, acid leaves sand alone.

### Reactions seem to skip frames randomly

That's the probability gate working. Acid at `probability: 0.05` means a 95% chance per frame that *no reaction fires for that pair*. Over many frames, the acid dissolves the stone — just slowly. Tune up or down to taste.

### `static REACTIONS: OnceLock<...>` — "use of unstable type"

You're on an old `rustc`. `OnceLock` stabilised in 1.70. Run `rustc --version`; should be 1.70 or higher. `rustup update` if not.

### Reactions table is wrong — entries clobber each other

`HashMap::insert` replaces the existing value if the key already exists. If you `insert((Fire, Wood), A)` then `insert((Fire, Wood), B)`, the table holds `B`. This is sometimes the bug (typo in the second key) and sometimes the feature (intentional override). When in doubt, dump the table.

### `let Some(x) = ... else { continue; };` won't compile

You're on Rust pre-1.65, or you forgot the trailing semicolon. Both forms are required.

---

## Session challenge

Pick one — no solution provided.

1. **Acid + acid = nothing** (don't add the entry). But: acid + water = green water (diluted acid, slower). Add a `CellType::DiluteAcid` variant and the corresponding table entry. Less corrosive, longer lifespan.
2. **`build_reactions` from a CSV file.** Read `assets/reactions.csv` at startup; parse rows like `Fire,Wood,Fire,Fire,80.0,1.0`. Now your chemistry is fully data-driven and *adding a reaction doesn't require recompiling Rust*. (Beware: parsing CSV is a Session 18 skill, but you can do it the manual way today.)
3. **Visualise the reaction graph.** Print the reactions as a Graphviz `digraph`:

   ```
   Fire -> Wood [label="ignites"];
   Acid -> Stone [label="dissolves"];
   ```

   Paste the output into [graphviz online](https://dreampuf.github.io/GraphvizOnline/) — you get a *diagram* of your chemistry.

4. **A reaction priority.** Add a `priority: i32` field to `ReactionOutcome`. When two reactions are possible for the same source (e.g. fire next to both wood and oil), pick the highest-priority one rather than the first found. Forward-compat with Session 21's gunpowder.

---

## Quick reference

| What | Code |
|---|---|
| Construct HashMap | `let m: HashMap<K, V> = HashMap::new();` |
| Insert | `m.insert(key, value)` |
| Lookup | `m.get(&key)` returns `Option<&V>` |
| Lookup as value | `m.get(&key).copied()` *(if V: Copy)* |
| Contains? | `m.contains_key(&key)` |
| Iterate entries | `for (k, v) in m.iter() { ... }` |
| Entry API | `*m.entry(k).or_insert(0) += 1;` |
| Static one-shot | `static X: OnceLock<T> = OnceLock::new();` |
| Lazy-init | `X.get_or_init(\|\| compute())` |
| let-else | `let Some(x) = opt else { return; };` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 14**. This is the most-cited session in the participant statement; **be specific**:

- A screenshot of `build_reactions` open in the editor — that's evidence of architectural thinking
- A paragraph in your own words on why "table-driven beats branchy" — this comes back in tech interviews for *years*
- Capture the acid + stone vs acid + wood vs acid + sand demonstration (one short clip works)
