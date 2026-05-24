# Session 23 — Polish and Secrets

> **Stuck on a word?** Things like *state machine*, *enum-with-data*, *easter egg*, *hidden recipe* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session your sim has a **title screen**, a **state machine** (`enum GameState { Title, Playing, Codex, Paused }`) that drives the whole loop, three **hidden recipes** that aren't listed anywhere, and at least one **easter egg** keystroke that does something delightful. Everything stays clean — the state machine *reduces* the amount of code in `main.rs`, not bloats it.

---

## What you'll learn

- `enum GameState` as a top-level state machine
- Transitioning states with `match` — the safe, total replacement for `if`-tree dispatch
- "Modes" — when different keys mean different things depending on which state you're in
- Hidden recipes — easter-egg gameplay
- The "polish polish polish" discipline: tightening details no one will *consciously* notice but everyone will *feel*

---

## The big idea

Right now `main.rs`'s loop is "always rendering, always stepping, plus a TAB-toggle for the codex." That works for one mode. It doesn't scale to "title screen, then playing, with optional pause overlay or codex overlay."

A **state machine** is one enum and one `match`. Each state knows what input keys mean, what gets rendered, and what gets updated. Transitioning is one line. It's the simplest fix for "this is becoming a tangle of booleans" — and the moment you reach for it is the moment your code goes from hobbyist to professional.

Hidden recipes are the *gameplay* polish. The codex doesn't show them. The player only finds them by accidental experimentation — that "oh!" moment is why people play discovery games until 4am.

---

## Concepts covered

- `enum GameState` with no fields (pure-marker variants)
- `match state { ... }` exhaustive dispatch
- A pattern: `state = match state { ... }` to atomically transition
- Conditional rendering: each state has its own render block
- Hidden recipes that don't appear in `build_recipes` but in a separate `build_secret_recipes`

---

## Building towards `sand-sim`

This is the last "new feature" session. Session 24 is the milestone — write the README, the retrospective, tag v1.0, push. Everything from today flows into the v1.0 demo: title screen on launch, polished interactions, hidden delights for the keen player.

---

## Step-by-step walkthrough

> **Where you should be.** Session 22 finished. Concrete and iron work. The element count is up around fifteen.

### 1. The state enum — 2 minutes

In `main.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Title,
    Playing,
    Codex,
    Paused,
}
```

No payloads. Pure marker enum.

### 2. The state machine in main — 8 minutes

Replace the existing loop:

```rust
let mut state = GameState::Title;

loop {
    state = match state {
        GameState::Title => {
            render_title();
            if is_key_pressed(KeyCode::Enter) {
                GameState::Playing
            } else if is_key_pressed(KeyCode::L) {
                if let Ok(loaded) = persist::load("save.json") {
                    grid = loaded;
                    GameState::Playing
                } else {
                    eprintln!("No save found");
                    GameState::Title
                }
            } else {
                GameState::Title
            }
        }
        GameState::Playing => {
            ui::handle_input(&mut grid, &mut selected, &mut brush_radius);
            step(&mut grid);
            audio.tick();
            let counts = count_cells(&grid);
            audio.trigger(&counts, /* ... */);
            check_recipes(&grid, &mut discoveries);

            clear_background(BLACK);
            render_grid(&grid, heatmap);
            draw_selector(selected, brush_radius, &discoveries);
            draw_legend();
            draw_hud(&counts);

            if is_key_pressed(KeyCode::Tab)   { GameState::Codex  }
            else if is_key_pressed(KeyCode::Space) { GameState::Paused }
            else { GameState::Playing }
        }
        GameState::Codex => {
            // Sim is frozen; codex on top of the last frame.
            // Don't step; just re-render last grid + codex overlay.
            clear_background(BLACK);
            render_grid(&grid, heatmap);
            codex::draw_codex(&discoveries);

            if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::Escape) {
                GameState::Playing
            } else {
                GameState::Codex
            }
        }
        GameState::Paused => {
            // Sim frozen; brief pause banner.
            clear_background(BLACK);
            render_grid(&grid, heatmap);
            draw_pause_banner();

            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Escape) {
                GameState::Playing
            } else {
                GameState::Paused
            }
        }
    };
    next_frame().await;
}
```

The `state = match state { ... }` shape is the clean transition idiom. Each arm returns the *next* state. Adding a new state (e.g. `Settings`) is one new variant + one new arm.

### 3. The title screen — 5 minutes

```rust
fn render_title() {
    clear_background(BLACK);
    let title = "sand-sim";
    let subtitle = "a Rust-powered physics sandbox";
    let prompt = "press ENTER to begin   |   L to load   |   ESC to quit";

    let tdim = measure_text(title, None, 96, 1.0);
    let sdim = measure_text(subtitle, None, 24, 1.0);
    let pdim = measure_text(prompt, None, 18, 1.0);

    draw_text(title, (screen_width() - tdim.width) / 2.0,
              screen_height() / 2.0 - 60.0, 96.0, Color::new(0.95, 0.80, 0.30, 1.0));
    draw_text(subtitle, (screen_width() - sdim.width) / 2.0,
              screen_height() / 2.0, 24.0, LIGHTGRAY);

    // Pulse the prompt so it's noticed.
    let alpha = ((get_time() * 2.0).sin() * 0.3 + 0.7) as f32;
    draw_text(prompt, (screen_width() - pdim.width) / 2.0,
              screen_height() / 2.0 + 80.0, 18.0,
              Color::new(1.0, 1.0, 1.0, alpha));

    // Bonus: drift falling sand in the background as eye candy.
    title_screen_idle_particles();
}

fn title_screen_idle_particles() {
    // Procedurally drift some sand-coloured pixels down the screen using
    // get_time() as a phase. Doesn't need to be physically correct — purely visual.
    for i in 0..40 {
        let phase = (get_time() as f32 * 0.5 + i as f32 * 0.37) % 1.0;
        let x = (i as f32 * 197.0) % screen_width();
        let y = phase * screen_height();
        draw_rectangle(x, y, 3.0, 3.0, Color::new(0.93, 0.80, 0.50, 0.6));
    }
}
```

The drifting sand-coloured pixels are a five-line eye-candy effect that turns a static splash into a living screen. Demos love it.

### 4. The pause banner — 1 minute

```rust
fn draw_pause_banner() {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.35));
    let text = "PAUSED — press SPACE to resume";
    let dim = measure_text(text, None, 32, 1.0);
    draw_text(text, (screen_width() - dim.width) / 2.0,
              screen_height() / 2.0, 32.0, WHITE);
}
```

### 5. Three hidden recipes — 6 minutes

In `recipes.rs`, add a second builder:

```rust
pub fn build_secret_recipes() -> Vec<Recipe> {
    let mut s: Vec<Recipe> = Vec::new();

    // Secret 1: Mud — wet sand. Place water on sand.
    s.push(Recipe {
        name: "Mud",
        unlocks: CellType::Mud,
        predicate: Box::new(|grid| {
            adjacent_pair(grid, CellType::Sand, CellType::Water)
        }),
    });

    // Secret 2: Ash — what's left after a *lot* of wood burns.
    // (Tracked by recipe condition: smoke cell at the top of the grid + no live fire.)
    s.push(Recipe {
        name: "Ash",
        unlocks: CellType::Ash,
        predicate: Box::new(|grid| {
            grid[0].iter().any(|c| matches!(c.cell_type, CellType::Smoke))
                && grid.iter().flat_map(|r| r.iter())
                    .filter(|c| matches!(c.cell_type, CellType::Fire | CellType::OilFire))
                    .count() == 0
        }),
    });

    // Secret 3: Pure carbon — diamond-like, decorative. Triggered by oil + extreme heat.
    s.push(Recipe {
        name: "Carbon",
        unlocks: CellType::Carbon,
        predicate: Box::new(|grid| {
            cells_match(grid, |c| matches!(c.cell_type, CellType::Oil) && c.temperature > 1800.0)
        }),
    });

    s
}
```

(Add `CellType::Mud`, `CellType::Ash`, `CellType::Carbon` variants with their own colours. Behaviour can be minimal — mud falls like wet sand, ash is static and fades, carbon is shiny and static.)

The trick: **`build_secret_recipes` is called and its results merged into the active recipe list, but `catalogue()` (the codex) does not list these elements.** The codex shows them only once they're discovered.

```rust
let mut recipes = build_recipes();
recipes.extend(build_secret_recipes());
```

In `codex.rs`:

```rust
let visible_entries: Vec<&ElementEntry> = catalogue().iter()
    .filter(|e| discoveries.is_unlocked(e.cell_type()) || known_via_catalogue(e.cell_type()))
    .collect();
```

(Add a helper that hides secret elements from the codex until discovered. Or always show all entries, and let the unfilled silhouette reveal that a secret exists. Designer's choice.)

### 6. An easter egg — 3 minutes

The hidden "press K-O-N-A-M-I-up-up..." sequence is the classic. Simpler: a single weird key combo.

```rust
let mut konami_index = 0;
let konami_sequence = [KeyCode::R, KeyCode::U, KeyCode::S, KeyCode::T];

// In handle_input:
for k in &konami_sequence {
    if is_key_pressed(*k) {
        if *k == konami_sequence[konami_index] {
            konami_index += 1;
            if konami_index == konami_sequence.len() {
                trigger_easter_egg();
                konami_index = 0;
            }
            break;
        } else {
            konami_index = 0;
            break;
        }
    }
}

fn trigger_easter_egg() {
    println!("\n  ((  Crab mode activated.  ))\n");
    // Plant a single special cell type, or unlock all secrets, or
    // flip the colour palette to sepia for 10 seconds. Pick one.
}
```

The egg can be anything. The point is: **someone keen will find it and feel smart**. Reward curiosity.

**Save. Run.** The title screen greets you. Press Enter. The sim runs. Press Space — paused. Space again — resumed. Tab — codex. Tab — back. Now type r-u-s-t at the keyboard — easter egg fires.

> **The Wow Moment.** Open the codex when fresh. It shows mostly silhouettes. Now play normally — discover the listed recipes. Open the codex again: most are colour. Now wander: place sand, drop water on it. **Mud unlocks.** It wasn't in the codex hint list at all. **Your sim now has secrets.** Some players will play for an hour and find them all. Some will play for ten hours and miss them. *Both feel proud.* That is what a *finished* discovery game does.

---

## Linux (Ubuntu) note

The state machine itself is pure-language; no OS impact. The easter egg might be:

- **Keyboard layout sensitivity.** R-U-S-T as keys is always on US/UK QWERTY. If the player has a non-Latin layout (AZERTY, Dvorak, Russian), the *physical keys* still register the same `KeyCode::R` because macroquad uses physical key codes, not interpreted characters. Your secret works regardless of layout.

- **Title screen idle-particle CPU.** 40 small rectangles drawn per frame is negligible. But if you ever bump that to 4000, expect CPU usage to climb on Ubuntu laptops on battery. Use `get_fps()` to verify; cap with a `match`-on-FPS.

- **Pause memory.** While paused, your sim isn't calling `step` but is still rendering the static last frame at 60 FPS. CPU sits at near-zero except for the screen redraw. To go further (true low-power pause), wrap `next_frame()` with `tokio::time::sleep(Duration::from_millis(33))` while paused — drops to 30 FPS, saves ~50% CPU. Overkill for a 30-minute play session; mentioned for completeness.

- **Wayland focus behaviour.** When your sim loses focus on Ubuntu Wayland (you alt-tab to another window), macroquad continues to render but key events stop firing. So you can't "pause" by alt-tabbing. Either gate `step` on `window_focused()` (macroquad exposes this) or accept it. The latter is fine; mention in the README.

---

## Common mistakes

### `error: non-exhaustive patterns` after adding a state

Rust forces you to handle every variant in `match`. Adding `GameState::Settings` and forgetting one arm fails to compile — that's a feature, not a bug. The compiler is helping you find every place affected.

### Sim doesn't pause in the Paused state

You forgot to skip the `step` call when not in `Playing`. The match arm structure (step inside the Playing branch only) is the correct pattern. Verify.

### Title screen never advances

`is_key_pressed(KeyCode::Enter)` returns true on the *frame* the key is first pressed. If you accidentally use `is_key_down`, the key being held during the splash will instantly transition. `is_key_pressed` is right.

### Easter egg never fires

The `konami_index = 0` *else* branch resets too aggressively — any unrelated key press during the sequence wipes progress. Loosen: only reset if the *expected next* key in the sequence is pressed and it doesn't match. Other keys (movement, mouse) shouldn't disturb the sequence.

### Codex blocks input completely

In the `GameState::Codex` arm, the player can still move the mouse and hover, but mouse-clicks for the codex layout aren't wired. Either add hover/click handling in `draw_codex`, or accept it as read-only.

### Pause-while-sim-is-loading bug

Loading via `L` on the title screen replaces `grid`. If you transition to Playing immediately, the next-frame iteration sees the new grid — correct. No bug. Mentioned because it's the kind of thing that *feels* fragile and isn't.

---

## Session challenge

Pick one — no solution provided.

1. **Settings state.** Add `GameState::Settings`. Lists adjustable constants (gravity strength, FPS cap, audio volume). Drives the same `match` pattern.
2. **More secrets.** Add three more hidden recipes. Hint at their existence with a single `???` codex entry that doesn't reveal the conditions.
3. **State entry/exit hooks.** Wrap states in a richer enum: `GameState::Playing { last_paused: Option<Instant> }`. Use the timestamp to fade in the unpause overlay. The data-bearing variant pattern is the next level of state-machine sophistication.
4. **Replay mode.** Record key/mouse events to a `Vec<(f64, Event)>` (with timestamps). Add `GameState::Replay`. Press a key to replay your own last session.

---

## Quick reference

| What | Code |
|---|---|
| Marker enum | `enum State { A, B, C }` |
| Exhaustive match | `match state { State::A => ..., }` |
| Transition pattern | `state = match state { ... };` |
| `is_key_pressed` | edge trigger (fires once per press) |
| `is_key_down` | held trigger (fires every frame) |
| Sin-pulse | `((get_time() * 2.0).sin() * 0.3 + 0.7) as f32` |
| Pulse text alpha | `Color::new(1.0, 1.0, 1.0, alpha)` |
| Title-centre text | `(screen_width() - dim.width) / 2.0` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 23**. Worth recording:

- A clip of the title-screen idle-particles plus the pause overlay — both are visible polish
- Which easter egg you chose and why
- Your sentence on state machines: "why is `match`-on-state better than `if`-tree dispatch?" (Answer to memorise: exhaustiveness, locality, transition clarity.)
