# Session 5 — Pattern Matching and Multiple Elements

> **Stuck on a word?** Things like *pattern*, *arm*, *exhaustive*, *wildcard* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your sandbox has **three working elements** with three different physics: **sand** falls and piles, **water** falls *and flows sideways*, **stone** sits still and acts as a wall.

---

## What you'll learn

- `match` properly — the more powerful, more readable cousin of `if`/`else if`
- Why **exhaustive matching** is one of Rust's superpowers (the compiler stops you forgetting cases)
- Range patterns (`1..=3`), match guards (`n if n > 5`), and the `_` wildcard
- Designing different per-element physics by reusing the same loop shape
- How to test that a refactor didn't break what worked

---

## The big idea

A sandbox needs at least three element behaviours: **solids that pile** (sand), **liquids that flow** (water), and **static walls** (stone). Three behaviours, all in the same per-cell update step.

`match` is the Rust feature that turns "if it's sand do this; else if it's water do that; else if it's stone do nothing; else ???" into a single statement that the compiler audits for completeness. It's the same idea as a `switch` in C/JavaScript, but Rust forces you to handle every possible value — including the case you forgot existed.

When Session 14 brings in the reactions HashMap, you'll see this taken to its logical conclusion: every interaction in the simulation routed through pattern matching. Today is the practice round.

---

## Concepts covered

- `match` expressions with literal arms
- The `_` wildcard arm (catch-all)
- Range patterns: `1..=10` (inclusive), `1..10` (exclusive)
- Match guards: `n if n > 0`
- Why match is an *expression* and `if` chains aren't
- A first taste of element-specific update functions: `update_sand`, `update_water`, `update_stone`

---

## Building towards `sand-sim`

Today is the moment the sandbox stops being "the sand demo." Once water flows and stone holds it back, you can build a dam. A waterfall. A funnel of sand pouring into a stone reservoir. The simulation goes from "physics toy" to "world you can shape." Session 6 polishes the type system; Session 7 wraps a UI around it; Session 8 ships v0.1.

---

## Step-by-step walkthrough

> **Where you should be.** Session 4 ended with a clean `step` → `update_cell` → `update_sand` shape and a `match cell { SAND => ..., _ => {} }` in `update_cell`. If your `update_cell` is still a chain of `if`s, refactor it to `match` first — today builds straight on top.

### 1. Turn `match` from cosmetic to load-bearing — 4 minutes

Open `update_cell` and add the dispatch for water and stone:

```rust
fn update_cell(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    let cell = grid[row][col];

    match cell {
        SAND  => update_sand (grid, row, col),
        WATER => update_water(grid, row, col),
        STONE => {} // Stone never moves on its own. Match arm intentionally empty.
        EMPTY => {} // Same for empty cells.
        _     => {} // Any future element we haven't taught yet.
    }
}
```

Two things to notice. **First**, `STONE => {}` is more honest than no arm at all — it documents that stone has no update logic. **Second**, the `_` wildcard catches anything we haven't named. The compiler doesn't *force* you to spell out `STONE` and `EMPTY` separately (the wildcard would cover them), but writing them out makes the intent obvious.

You're using `match` here as a **dispatcher** — a small bit of logic at the top of a function that hands work off to a specialised routine. This is one of the three or four shapes you'll see in *every* substantial Rust codebase.

### 2. Write `update_water` — 8 minutes

Water falls like sand, but if it can't fall it tries to *flow sideways* before giving up:

```rust
fn update_water(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    // 1. Try straight down.
    if row + 1 < ROWS && grid[row + 1][col] == EMPTY {
        grid[row + 1][col] = WATER;
        grid[row][col]     = EMPTY;
        return;
    }

    // 2. Try diagonally down.
    let try_left_first = fastrand::bool();
    let order: [i32; 2] = if try_left_first { [-1, 1] } else { [1, -1] };

    for dx in order {
        let nc = col as i32 + dx;
        if nc < 0 || nc >= COLS as i32 { continue; }
        let nc = nc as usize;
        if row + 1 < ROWS && grid[row + 1][nc] == EMPTY {
            grid[row + 1][nc] = WATER;
            grid[row][col]    = EMPTY;
            return;
        }
    }

    // 3. Try purely sideways — this is what makes water different from sand.
    for dx in order {
        let nc = col as i32 + dx;
        if nc < 0 || nc >= COLS as i32 { continue; }
        let nc = nc as usize;
        if grid[row][nc] == EMPTY {
            grid[row][nc] = WATER;
            grid[row][col] = EMPTY;
            return;
        }
    }
}
```

**Save. Run.** Press `2` to select water, drag to draw a pool. Water falls, fills the bottom, and *spreads sideways* until it can't anymore. Place stone (key `3`) to build walls. **You just built a working liquid simulator.**

**First runnable checkpoint.** This is the most satisfying point of Month 1 so far. Take a screenshot for your DofE log.

> **The Wow Moment.** Build a stone bowl (a U-shape of stone). Pour water into it from above. Watch it settle, fill to the brim, then **overflow** down the sides. *That whole behaviour is three rules: down, diagonal-down, sideways.* You didn't program "fill bowls" or "overflow." It's an emergent property of the rules + 60Hz.

### 3. Why sand doesn't flow sideways — 1 minute

Compare `update_sand` and `update_water`. The *only* difference is that water has a step 3 (the pure-sideways block). That's why sand piles up at an angle and water lies flat. **You expressed a physical law of fluids with five lines of code.**

If you wanted *honey* (very viscous), you'd only run the sideways step 30% of the time. *Mercury* (very flowing) — extra repeated sideways passes. The whole consistency spectrum is just probabilities on the sideways rule.

### 4. Range patterns and guards — `match` in earnest — 5 minutes

Add this helper somewhere in your file. It returns a friendly name for any element:

```rust
fn element_name(cell: u8) -> &'static str {
    match cell {
        EMPTY => "empty",
        SAND  => "sand",
        WATER => "water",
        STONE => "stone",
        n if n >= 100 => "advanced",   // match guard — covers fire, oil, etc later
        _ => "unknown",
    }
}
```

Then in your main loop, after computing the label, print it instead of the `if` chain from Session 2:

```rust
        let label = element_name(selected);
        draw_text(label, 8.0, 20.0, 24.0, WHITE);
```

What you just used:

- `EMPTY => "empty",` — **literal arm**. Matches exactly that value.
- `n if n >= 100 => "advanced",` — **named arm with a guard**. The `n` binds the matched value to a name; the `if` runs only on the matched arm. Guards open up arbitrarily complex conditions.
- `_ => "unknown",` — **wildcard arm**. Must come last (Rust matches in order).

You could also use a range:

```rust
match midi_note % 12 {
    0      => "C",
    1 | 3  => "C# or D#",         // or-pattern
    4..=6  => "E or F or F#",     // inclusive range
    _      => "other",
}
```

Or-patterns (`|`) and ranges (`..=`) are surprisingly handy once you spot the use cases.

### 5. (Optional) `&'static str` — what's that quote? — 2 minutes

In the `element_name` signature you have `-> &'static str`. The `'static` is a **lifetime**. It says "the returned string lives for the entire program" — which is true, because string literals like `"sand"` are baked into the compiled binary. We'll cover lifetimes properly when they get interesting (Month 3). For now: any function that returns a string literal needs `&'static str`.

---

## Common mistakes

### `error[E0004]: non-exhaustive patterns: '_' not covered`

`match` on a `u8` must handle every value 0–255. You spelled out `SAND`, `WATER`, `STONE`, `EMPTY` and forgot `_`. Add `_ => {}` at the bottom.

### `error[E0308]: 'match' arms have incompatible types`

You wrote `match` to return a value, but one arm returns `&str` and another returns `String`. Every arm must produce the same type. Fix: convert with `.to_string()`, or change the function signature to `String` and call `.to_string()` on the literals.

### Water still acts like sand

You forgot to add the `update_water` branch to the `match` in `update_cell`. The compiler doesn't warn you here — there's already a default `_ => {}` catching water — so the symptom is silent: water just sits there. Add `WATER => update_water(grid, row, col)`.

### Water "teleports" sideways across the whole row in one frame

Same issue as Session 3's sand-teleporting bug, but lateral: water moves right, the inner `for col in 0..COLS` visits the moved-right water again *this frame*, and it moves again. Cleanest fix: **alternate column iteration direction each frame.** Add a frame counter:

```rust
let mut frame: u64 = 0;
loop {
    frame += 1;
    // ...
    step(&mut grid, frame);
}

fn step(grid: &mut Vec<Vec<u8>>, frame: u64) {
    for row in (0..ROWS - 1).rev() {
        if frame % 2 == 0 {
            for col in 0..COLS { update_cell(grid, row, col); }
        } else {
            for col in (0..COLS).rev() { update_cell(grid, row, col); }
        }
    }
}
```

The bias cancels itself out frame by frame. (Session 7 polishes this further.)

### Stone vanishes when you click it

*(Linux note continues after Common mistakes.)*

You probably forgot to update the keypress block for `Key3 => selected = STONE`. Or — fun bug — your `cell_colour` function still has the `STONE` arm pointing at `BLACK`. Stone is *drawn* as background colour, so you think it disappeared. Check `cell_colour`.

---

## Session challenge

Pick one — no solution provided.

1. **Sand on water sinks.** Currently sand placed in water just sits on top because `update_sand` only moves into `EMPTY`. Add a rule: if the cell below is `WATER`, swap the two (sand goes down, water comes up). Hint: introduces a `swap_cells` helper.
2. **Floating debris.** Add a fourth element, "wood" (`const WOOD: u8 = 4;`, brown), which floats *up* through water and sits on top. `update_wood`: if cell below is WATER, swap up.
3. **Water evaporates at the top row.** Every few frames, if row 0 contains water, set it to EMPTY. Watch the water "level" slowly drop in a sealed container. (Real evaporation, in two lines.)
4. **A toggle for water turbulence.** Press `T` to switch a global `mut turbulent: bool`. When true, increase the probability of sideways spread to 1.0 *and* let water also flow against gravity diagonally upward 5% of the time. (Looks awful. Also weirdly mesmerising.)

---

## Quick reference

| What | Code |
|---|---|
| Match a value | `match cell { SAND => ..., _ => ... }` |
| Wildcard arm | `_ => { /* handle anything else */ }` |
| Or-pattern | `1 \| 2 \| 3 => ...` |
| Inclusive range | `0..=9 => "single digit"` |
| Exclusive range | `0..10 => "single digit"` |
| Match guard | `n if n > 0 => "positive"` |
| Bind matched value | `n @ 0..=9 => println!("got {n}")` |
| Static string return | `fn name() -> &'static str { "hi" }` |
| Empty arm (do nothing) | `STONE => {}` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 5**. Worth recording:

- What "exhaustive matching" means and why it caught a bug for you (or stopped a bug from being possible)
- The moment water first overflowed your stone bowl — that's a real emergent-behaviour story to put in your participant statement later
