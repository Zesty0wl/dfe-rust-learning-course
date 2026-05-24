# Session 15 — Lava, Ice, and Chain Reactions

> **Stuck on a word?** Things like *chain reaction*, *temperature threshold*, *phase change*, *table-driven* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your sandbox has **lava** (falling, hot, burns wood, solidifies on water contact) and **ice** (static, cold, melts to water near heat) — **each added in roughly six lines of table changes plus one tiny update function.** Drop lava onto a frozen lake and watch a multi-step chain reaction play out without you scripting any of it.

---

## What you'll learn

- Adding non-trivial elements *without changing existing element code* — the test of a good architecture
- Designing chain reactions: A becomes B, B reacts with C, C produces D — emergent narratives
- Re-using `update_liquid` (or a generic liquid update) for lava without copy-paste
- A `liquid_density` field on cells, or per-type density constants — letting lava sink through water
- Why "no new code paths needed" is the real measure of yesterday's refactor

---

## The big idea

Sessions 11–14 built the **machinery**. Today you reap the rewards: add two elements with three or four lines of code each, and the rest of the world *just works*. Lava reacts with water through the table. Lava cools through the existing temperature pass. Lava ignites wood through the same table entries fire uses, with different probabilities. Ice melts because of heat radiation — the *same* heat radiation that boils water.

The most satisfying single moment of Month 2: when you realise you've made fire, water, oil, wood, smoke, steam, acid, lava, and ice — nine elements — interact pairwise in dozens of distinct ways, and almost all of it lives in one HashMap. **That is what good architecture buys you.**

---

## Concepts covered

- Reusing `update_liquid` for lava (generic over type)
- A `density()` method on `CellType` returning `u8` — heavier liquids sink
- Per-type cool/heat constants instead of a single global
- The reactions added today: lava+water, lava+wood, lava+ice, ice+fire, ice+lava, ice+water
- The fact that **no existing reaction in `build_reactions` changes** — only additions

---

## Building towards `sand-sim`

This is the last "new element" session of Month 2. Session 16 adds polish, audio, and the milestone reflection that ships v0.2. Month 3 reuses today's chain-reaction lessons for the recipe system (Session 19) — recipes are reactions tagged with "this should unlock a new selector entry."

---

## Step-by-step walkthrough

> **Where you should be.** Session 14 finished. The `REACTIONS` HashMap and `react()` work. Acid corrodes; fire reacts via the table; eight elements exist (sand, water, stone, wood, fire, oil, oilfire, smoke, steam, acid).

### 1. Add the two new variants — 2 minutes

```rust
enum CellType {
    // ...
    Lava,
    Ice,
}
```

Add their colours:

```rust
// In CellType::colour:
CellType::Lava => Color::new(1.00, 0.30, 0.05, 1.0),  // bright molten orange
CellType::Ice  => Color::new(0.75, 0.90, 1.00, 0.85), // pale icy blue, slight transparency
```

Wire them into the selector and hotkeys:

```rust
// In your selector + key handler:
let elements = [Sand, Water, Stone, Wood, Fire, Oil, Acid, Lava, Ice];
if is_key_pressed(KeyCode::Key8) { selected = CellType::Lava; }
if is_key_pressed(KeyCode::Key9) { selected = CellType::Ice;  }
```

### 2. Lava is a hot liquid — 3 minutes

Lava falls like water but is much hotter. The neat way: extend `update_liquid` from Session 14 to take a starting temperature:

```rust
fn update_liquid(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize, my_type: CellType) {
    // ... falling logic (water from Session 5 generalised) ...
}
```

Lava also auto-radiates heat (like fire but without spreading):

```rust
fn update_lava(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    // Heat the four neighbours.
    for (dr, dc) in NEIGHBOURS_4 {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        grid[nr as usize][nc as usize].heat(40.0);
    }
    // Then fall like a liquid.
    update_liquid(grid, row, col, CellType::Lava);
}
```

Dispatch:

```rust
match grid[row][col].cell_type {
    CellType::Lava => update_lava(grid, row, col),
    CellType::Water | CellType::Oil | CellType::Acid => update_liquid(grid, row, col, _ct),
    // ...
}
```

Set lava's initial temperature to a sensible value in `Cell::new`:

```rust
fn new(cell_type: CellType) -> Self {
    let temperature = match cell_type {
        CellType::Lava => 1200.0,
        CellType::Ice  => -10.0,
        _ => 20.0,
    };
    let lifetime = match cell_type {
        CellType::Fire => 60,
        _ => 0,
    };
    Cell { cell_type, temperature, lifetime }
}
```

### 3. Lava reactions — three table rows — 3 minutes

In `build_reactions`:

```rust
    // -- Lava reactions --
    // Lava + Water = stone (lava cools and solidifies). Heat dumped to the water.
    r.insert((Lava, Water), ReactionOutcome::replace_both(Stone, Steam,  60.0));
    r.insert((Water, Lava), ReactionOutcome::replace_both(Steam, Stone,  60.0));

    // Lava + Wood = lava + fire (lava ignites wood instantly).
    r.insert((Lava, Wood), ReactionOutcome::replace_both(Lava, Fire, 100.0));
    r.insert((Wood, Lava), ReactionOutcome::replace_both(Fire, Lava, 100.0));

    // Lava + Ice = water + water (huge temperature differential cancels).
    r.insert((Lava, Ice), ReactionOutcome::replace_both(Stone, Water, 200.0));
    r.insert((Ice, Lava), ReactionOutcome::replace_both(Water, Stone, 200.0));
```

**Six lines, three reactions.** That's the whole of "lava's chemistry."

### 4. Ice — one update function — 3 minutes

Ice is static (doesn't move). It just melts when warm:

```rust
fn update_ice(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    if grid[row][col].temperature >= 0.0 {
        grid[row][col] = Cell {
            cell_type:   CellType::Water,
            temperature: 5.0,    // melted-just-now water is cold
            lifetime:    0,
        };
        return;
    }
    // Ice also cools its neighbours slightly (anti-heat).
    for (dr, dc) in NEIGHBOURS_4 {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
        let cell = &mut grid[nr as usize][nc as usize];
        cell.temperature = (cell.temperature - 0.4).max(-50.0);
    }
}
```

Dispatch in the static pass:

```rust
match grid[row][col].cell_type {
    CellType::Ice => update_ice(grid, row, col),
    // ...
}
```

Add ice reactions (for instant phase changes — heat radiation also melts ice slowly via the temperature check above):

```rust
    // -- Ice reactions --
    // Ice + Fire = both die into water + smoke.
    r.insert((Ice, Fire), ReactionOutcome::replace_both(Water, Smoke, -30.0));
    r.insert((Fire, Ice), ReactionOutcome::replace_both(Smoke, Water, -30.0));

    // Ice + Water = stays the same — nothing inserted (cold water just sits beside ice).
```

**Save. Run.**

Build a stone basin. Fill it with water. Drop a single ice cube in: it floats, cools the water near it, just sits there. Now drop a lava cell on top of the ice: **steam bursts, stone forms, water condenses, surrounding ice melts from the heat differential.** Real cascade.

> **The Wow Moment.** Make a "frozen lake" — a row of ice cells at the bottom of a stone bowl with a thin layer of water above them. Drop a single lava cell at the top. Watch:
>
> 1. Lava falls through air, glowing.
> 2. Hits the water — turns to **stone** with **steam** rising.
> 3. The steam reaches ice — heat melts the ice into **water**.
> 4. The new water mixes with the existing water — the temperature drops.
> 5. Eventually the lake re-freezes (if you have a cool-pass enabled) or settles into stratified hot/cold layers.
>
> **You didn't script any of that.** Each step came from a single table entry plus the temperature field. *That* is what people mean when they say a simulator "comes alive."

### 5. Optional: ice on water floats — 3 minutes

Add a `density` method on `CellType`:

```rust
impl CellType {
    fn density(self) -> u8 {
        match self {
            CellType::Stone => 200,
            CellType::Sand  => 160,
            CellType::Lava  => 150,
            CellType::Water => 100,
            CellType::Acid  =>  95,
            CellType::Oil   =>  60,
            CellType::Ice   =>  40,        // ice floats on water — that's why!
            _               =>   0,
        }
    }
}
```

In `update_liquid`, when blocked by a less-dense liquid, swap up:

```rust
    // Density-aware sink: if the cell below has a lower density and is a liquid, swap.
    if row + 1 < ROWS {
        let below_type = grid[row + 1][col].cell_type;
        if matches!(below_type, CellType::Oil | CellType::Water | CellType::Acid)
            && below_type.density() < my_type.density()
        {
            let me = grid[row][col];
            grid[row][col]     = grid[row + 1][col];
            grid[row + 1][col] = me;
            return;
        }
    }
```

Now lava sinks through oil. Ice floats up through water (well, *would* — but ice is static). The principle is in place.

---

## Linux (Ubuntu) note

Nothing OS-specific this session. The same `cargo run --release` story; the same `perf` profiling guidance from Session 12. One Ubuntu-relevant tip:

- The hot-pink lava colour might look slightly washed out on certain Wayland compositors with `Night Light` enabled (Ubuntu's blue-light filter). If your lava looks orange instead of red-orange, check *Settings → Displays → Night Light* and toggle it off temporarily — your colours are correct, the compositor is filtering.

Also worth noting on Ubuntu specifically: `cargo build --release` for this session takes 30–60 seconds on a typical laptop the first time after a clean. Subsequent incremental builds are 1–3 seconds. The first build is slow because LTO has to re-link all of `macroquad` plus your now-larger codebase. Normal.

---

## Common mistakes

### Lava falls straight through water without reacting

You put `update_lava` in the wrong iteration pass. Falling liquids need bottom-to-top iteration. Reactions need their own pass (the reaction pass runs before movement). If lava is updated *after* the reaction pass and falls right through, reactions don't get triggered. Make sure the order is: reactions → top-to-bottom rising → bottom-to-top falling.

### Lava produces stone *and* steam, but the stone is wrong

You inserted `(Lava, Water)` and `(Water, Lava)` with *different* outcomes by mistake. Reactions should be symmetric — the source/target order in the key is the only difference, but the chemistry is the same. Double-check both rows produce stone-and-steam.

### Ice instantly melts on placement

Your default `Cell::new(Ice)` sets temperature to 20.0 (default). Add the special case in `Cell::new` (step 2 above). Ice should spawn at -10°C or colder.

### Ice never melts even next to fire

The fire's heat radiation isn't reaching ice. Check that `update_fire`'s `FIRE_HEAT_RADIATE` (Session 11) actually adds heat to ice cells, and that `update_ice`'s threshold (`temperature >= 0.0`) is correct.

### `error[E0277]: 'CellType' doesn't implement 'Copy'` after adding density()

Adding methods doesn't remove `Copy`. Check that no variant's payload is non-`Copy`. (Most likely: you accidentally typed `CellType::Wood(String)` for some reason. Stick with `u8`/`f32` payloads.)

### `update_liquid` panics with index out of bounds on lava

The density-swap step doesn't check `row + 1 < ROWS` before swapping. Add the guard.

---

## Session challenge

Pick one — no solution provided.

1. **Snow.** Add a `CellType::Snow` variant — a slow-falling, cold solid that melts to water. Snow + heat → water. Snow + Snow stacks (like sand, but loose).
2. **Geothermal vent.** A persistent lava-producer cell (`CellType::LavaSource`) that, every 30 frames, spawns a lava cell in the cell directly above it if empty. Drop a few of these on a map and you get an unending lava flow you can dam.
3. **Glass from sand + lava.** Add `(Sand, Lava) → (Glass, Lava)` (Glass is Session 21 — preview now). Lava walking over sand fuses it.
4. **A reactions-list pretty printer.** Press `R` to dump the current reaction table to the terminal in tabular form (`SRC + TGT -> NEW_SRC + NEW_TGT | heat | prob`). Excellent for debugging additions.

---

## Quick reference

| What | Code |
|---|---|
| Density per type | `fn density(self) -> u8 { match self { ... } }` |
| Density swap | `if below.density() < my.density() && below.is_liquid() { swap }` |
| Per-type spawn temp | `match type { Lava => 1200.0, Ice => -10.0, _ => 20.0 }` |
| Static melt threshold | `if cell.temperature >= 0.0 { become water }` |
| Add a reaction | `r.insert((A, B), ReactionOutcome::replace_both(C, D, heat));` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 15**. Worth recording:

- The frozen-lake chain reaction from step 4 — a short clip is gold
- A sentence in your own words on "the test of a good architecture is *adding* code, not *changing* it" — this is the moment that lesson lands
