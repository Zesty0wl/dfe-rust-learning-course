# Session 16 — Polish and Milestone (v0.2 ships)

> **Stuck on a word?** Things like *milestone*, *heatmap*, *HUD*, *overlay*, *release tag* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **`sand-sim` v0.2 ships**. It has a toggleable heat-map overlay, an on-screen element counter, three new audio events (fire crackle, lava sizzle, oil-ignition thump), a tuned reactions table, and a git tag `v0.2`.

---

## What you'll learn

- The polish discipline — what to do when the program already "works"
- Toggling debug overlays without bloating the main loop
- Audio with multiple sound sources, played at the right times
- Balancing constants by feel (and how to know when to stop)
- The release ceremony: commit, tag, push, reflect

---

## The big idea

You've shipped one milestone already (Session 8). This one is bigger: more elements, more complex chemistry, three sounds, an overlay. The discipline is the same — declare what counts as "done," do *exactly* that, ship.

The temptation in a project session is to keep adding "just one more thing." Resist. Each addition is a future maintenance cost. **Cut the scope, polish what's there, ship.**

---

## Concepts covered

- Toggleable overlay rendering (`if heatmap { ... }`)
- Multiple `Sound` handles in a single `main`
- Per-event cooldowns to avoid audio spam
- The "element count" HUD from Session 13, now permanently on
- Constant tuning by side-by-side comparison
- `git tag -a v0.2`

---

## Building towards `sand-sim`

This session caps Month 2. `month-2/milestone/sand-sim-v0.2/` becomes the snapshot tagged `v0.2`. Month 3 starts by copying it to `month-3/milestone/sand-sim-v1.0/`. The element counter HUD becomes the Session 20 codex layout. The heat-map overlay becomes the Session 17 module split's first new module.

---

## Step-by-step walkthrough

> **Where you should be.** Session 15 finished. Lava, ice, acid, fire, oil, steam, smoke, wood, water, sand, stone — eleven elements (counting empty) all interact via the reactions table. Brush, selector, hold-to-pause, clear, FPS counter all work from Session 8.

### 1. Heat-map overlay — 5 minutes

A debug view that colours every cell by temperature, overriding its normal colour. Toggle with `T`.

```rust
let mut heatmap_enabled = false;

// In input block:
if is_key_pressed(KeyCode::T) { heatmap_enabled = !heatmap_enabled; }
```

Render the grid with a branch:

```rust
        for row in 0..ROWS {
            for col in 0..COLS {
                let cell = grid[row][col];
                if cell.is_empty() { continue; }
                let x = col as f32 * CELL_SIZE;
                let y = row as f32 * CELL_SIZE;
                let colour = if heatmap_enabled {
                    heatmap_colour(cell.temperature)
                } else {
                    cell.render_colour()
                };
                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, colour);
            }
        }
```

Define the heatmap colour function:

```rust
fn heatmap_colour(temperature: f32) -> Color {
    // Map temperature to a colour ramp:
    // -50°C blue -> 0°C cyan -> 100°C white -> 500°C orange -> 1500°C red.
    let t = ((temperature + 50.0) / 1550.0).clamp(0.0, 1.0);
    // Simple piecewise ramp.
    if t < 0.25 {
        let u = t / 0.25;
        Color::new(0.0, u, 1.0, 1.0)
    } else if t < 0.5 {
        let u = (t - 0.25) / 0.25;
        Color::new(u, 1.0, 1.0 - u, 1.0)
    } else if t < 0.75 {
        let u = (t - 0.5) / 0.25;
        Color::new(1.0, 1.0 - u * 0.5, 0.0, 1.0)
    } else {
        let u = (t - 0.75) / 0.25;
        Color::new(1.0, 0.5 - u * 0.5, 0.0, 1.0)
    }
}
```

**Press `T`** while the sim is running with a fire-and-water demo. The whole grid recolours as a thermal map. Press again to return to normal. Adds about 10 seconds of "feels professional" to the visual demo.

### 2. Audio events — 6 minutes

Three new sounds to source from freesound.org (filter to CC0):

- `fire.wav` — low crackle (or two-second loop you'll re-trigger).
- `lava.wav` — wet sizzle.
- `boom.wav` — short thump for oil ignition.

Drop them all in `assets/`. Update CREDITS.md:

```markdown
# Audio credits — sand-sim v0.2

- `sand.wav` — short sand-pour SFX, CC0, sourced from <freesound.org/...> on YYYY-MM-DD.
- `fire.wav` — fire crackle, CC0, sourced from <freesound.org/...> on YYYY-MM-DD.
- `lava.wav` — lava sizzle, CC0, sourced from <freesound.org/...> on YYYY-MM-DD.
- `boom.wav` — explosion thump, CC0, sourced from <freesound.org/...> on YYYY-MM-DD.
```

Load them all:

```rust
let sand_sound  = load_sound("assets/sand.wav").await.unwrap();
let fire_sound  = load_sound("assets/fire.wav").await.unwrap();
let lava_sound  = load_sound("assets/lava.wav").await.unwrap();
let boom_sound  = load_sound("assets/boom.wav").await.unwrap();
```

Trigger each based on grid state, with per-sound cooldowns:

```rust
let mut cd_sand: u32 = 0;
let mut cd_fire: u32 = 0;
let mut cd_lava: u32 = 0;
let mut cd_boom: u32 = 0;

// inside loop, after `step(&mut grid)`:

// Tick all cooldowns down.
for cd in [&mut cd_sand, &mut cd_fire, &mut cd_lava, &mut cd_boom] {
    if *cd > 0 { *cd -= 1; }
}

let counts = count_cells(&grid);

// Sand pour while button held.
if selected == CellType::Sand && is_mouse_button_down(MouseButton::Left) && cd_sand == 0 {
    play_sound_once(&sand_sound);
    cd_sand = 18;
}

// Fire crackle while *any* fire is present.
if (counts.get(&CellType::Fire).copied().unwrap_or(0) > 0
    || counts.get(&CellType::OilFire).copied().unwrap_or(0) > 0) && cd_fire == 0 {
    play_sound_once(&fire_sound);
    cd_fire = 90;        // ~1.5 seconds between crackle plays
}

// Lava sizzle when lava is touching water (presence-based).
if counts.get(&CellType::Lava).copied().unwrap_or(0) > 0
    && counts.get(&CellType::Steam).copied().unwrap_or(0) > 5
    && cd_lava == 0 {
    play_sound_once(&lava_sound);
    cd_lava = 60;
}

// Boom on oil ignition — detect the count rising.
if let Some(_) = oil_just_ignited(&counts, &mut prev_oilfire_count) {
    if cd_boom == 0 {
        play_sound_once(&boom_sound);
        cd_boom = 30;
    }
}
```

The `oil_just_ignited` helper compares this frame's oil-fire count to the previous frame's:

```rust
fn oil_just_ignited(counts: &HashMap<CellType, usize>, prev: &mut usize) -> Option<()> {
    let now = counts.get(&CellType::OilFire).copied().unwrap_or(0);
    let jumped = now > *prev + 3;     // a "jump" of more than 3 cells per frame
    *prev = now;
    if jumped { Some(()) } else { None }
}
```

You'll need a `let mut prev_oilfire_count: usize = 0;` above the loop.

### 3. Tune the reactions — 5 minutes

Now you have audio, you'll *hear* when reactions are wrong (boom playing twice for one event, lava sizzling forever even after the water's gone). Spend 5 minutes adjusting:

- Cool-down lengths for each sound
- Probability of fire spreading via the table (`probability: 0.5` for fire+wood feels different from `1.0`)
- Lava's neighbour-heat amount (raise from 40 to 60 and lava becomes more "menacing")

The goal isn't perfection. The goal is: *the demo you'd send to a friend feels good for two minutes*.

### 4. Permanent element counter HUD — 2 minutes

Move the Session 13 `count_cells` HUD out of "optional" and into the always-on render block. Bottom-right corner:

```rust
let counts = count_cells(&grid);
let mut y = screen_height() - 8.0 - 16.0 * counts.len() as f32;
let mut entries: Vec<(CellType, usize)> = counts.into_iter().collect();
entries.sort_by(|a, b| b.1.cmp(&a.1));        // descending by count
for (cell_type, count) in entries {
    let line = format!("{:>5}  {}", count, cell_type.name());
    draw_text(&line, screen_width() - 130.0, y, 16.0, LIGHTGRAY);
    y += 16.0;
}
```

Re-runs `count_cells` each frame (cheap; the simulation is already O(rows × cols)).

### 5. README and milestone — 5 minutes

Replace `month-2/milestone/sand-sim-v0.2/README.md` with the real thing:

```markdown
# sand-sim v0.2

A real-time falling-sand chemistry sandbox in Rust. Eleven elements (sand, water, stone, wood, fire, smoke, oil, oilfire, steam, acid, lava, ice) with table-driven reactions, temperature simulation, and audio.

## Run

```bash
cargo run --release
```

## Controls

- **1-9** — select sand, water, stone, wood, fire, oil, acid, lava, ice
- **H** — toggle heat-source brush (next click drops at 200°C)
- **T** — toggle heat-map overlay
- **L-click drag** — paint
- **R-click drag** — erase
- **Scroll** — brush size
- **Space** — pause / unpause
- **C** — clear

## Architecture

- `Cell { cell_type, temperature, lifetime }` — the per-cell unit.
- `REACTIONS: HashMap<(CellType, CellType), ReactionOutcome>` — every pairwise interaction lives here.
- Three update passes per frame: reactions → top-to-bottom (rising) → bottom-to-top (falling).

## Credits

Audio: see `assets/CREDITS.md` (alongside this README inside the milestone folder).

## What's next

Month 3 (see `month-3/README.md` at the repo root) adds modules, save/load, a recipe-based discovery system, the codex UI, and ships v1.0.
```

Complete [`dfe/milestone-2-reflection.md`](../../dfe/milestone-2-reflection.md). Same as Session 8 — be specific.

### 6. Commit and tag — 2 minutes

From the repo root:

```bash
git add -A
git commit -m "Ship sand-sim v0.2 — chemistry, heatmap, audio"
git tag -a v0.2 -m "Month 2 milestone: table-driven reactions, 9 new elements, audio"
git push origin main
git push origin v0.2
```

> **The Wow Moment.** Run `cargo run --release`. Press `H` once. Build a stone tower with a tiny pool of oil inside. Drop a single fire cell on the oil. **Hear the *thump*.** Watch the oil race into flame, lava-coloured flames, smoke billowing. Press `T`. **See the heat propagating outward as a thermal map.** Press `T` again. **Listen to the fire crackle continue until the oil is gone.** You built a real-time physics simulator that the average person would assume was made by a small studio. *In 16 sessions.*

---

## Linux (Ubuntu) note

The audio polish doubles down on the Session 8 audio guidance. Quick checklist before shipping v0.2 on Ubuntu:

```bash
# 1. ALSA headers installed?
dpkg -l libasound2-dev | grep ii

# 2. PipeWire running?
systemctl --user status pipewire pipewire-pulse | grep Active

# 3. Audio device picked correctly?
pactl list short sinks
```

If all three are happy, your four sounds should play. Specific to v0.2:

- **Multiple simultaneous sounds.** macroquad on Linux mixes via the chosen backend. On PipeWire you may notice all four sounds layer correctly. On older PulseAudio-only systems, you may get ducking — the latest-triggered sound takes priority and earlier ones are cut. Cosmetic.
- **Performance with 11 elements.** Even a busy grid with hundreds of reacting cells should stay above 55 FPS in `--release` on a modern Ubuntu laptop. If you're seeing drops, try:

  ```bash
  cargo build --release
  ./target/release/sand-sim
  ```

  Bypassing `cargo run` shaves ~5ms of process-startup overhead.
- **Distributing the binary.** `target/release/sand-sim` is a dynamically-linked ELF that depends on libc, libasound, libGL, and libX11. To check: `ldd target/release/sand-sim`. Anyone running a similar-or-newer Ubuntu (or any glibc-based distro) should be able to run the binary if you also ship the `assets/` folder next to it.

---

## Common mistakes

### Audio plays for a millisecond then stops

You probably called `play_sound_once` once per frame at 60Hz — each new call cuts the previous. Add cooldowns (step 2 above) so a sound triggers once and is allowed to play to completion.

### Boom sound triggers continuously while oil burns

The `oil_just_ignited` detector needs to compare *frame-to-frame*. Make sure `prev_oilfire_count` is updated *inside* the detector, not outside. The version above does it correctly.

### Heatmap colours look wrong (all blue)

The `t` calculation is off. Verify temperatures are reaching the high end during a fire demo (`println!` the max temperature). Adjust the `+ 50.0 / 1550.0` divisors to match your actual temperature range.

### `git tag v0.2 already exists`

You retried this session. Delete the old tag and remake: `git tag -d v0.2 && git push origin :refs/tags/v0.2 && git tag -a v0.2 ...`.

### Linker error on Ubuntu when adding new sounds

Means `libasound2-dev` isn't installed (or wasn't installed before the first `cargo build`). Install it (`sudo apt install -y libasound2-dev`), then `cargo clean && cargo build`.

### Performance tanks even in --release

Your `count_cells` is being called more than once per frame somehow. Search for `count_cells(` — should appear twice (once for HUD, once for audio triggers). Combine into a single call and pass the `HashMap` to both.

---

## Session challenge

Pick one — no solution provided. (Milestone first; these are stretch.)

1. **Snapshot key.** Press `P` to write `screenshot.png` of the current grid (without HUD). `get_screen_data().export_png("screenshot.png")` writes the entire window; for grid-only, build the image manually from the grid.
2. **Recordable demo.** Save the current state every 60 frames to `recording-N.json`. Add a playback flag (`--play` on command line) that reads them back in sequence. Excellent for capturing demo loops.
3. **A "reaction trace" mode.** Print to the terminal every time the reaction table is hit. After 5 seconds of running, you have a log of every chemistry event the sim performed. Useful evidence for the DofE log.
4. **A "no audio" build.** Run with `--no-default-features` and gate all the `play_sound_once` calls behind `#[cfg(feature = "audio")]`. Useful for headless testing and CI.

---

## Quick reference

| What | Code |
|---|---|
| Multiple sounds | `let s1 = load_sound("a.wav").await?; let s2 = ...` |
| Per-sound cooldown | `if cd == 0 { play_sound_once(&s); cd = N; }` |
| Toggle overlay | `if is_key_pressed(T) { heatmap = !heatmap; }` |
| Sort HashMap entries | `entries.sort_by(\|a, b\| b.1.cmp(&a.1))` |
| HUD bottom-right | `draw_text(&line, screen_width() - 130.0, y, 16.0, LIGHTGRAY)` |
| Tag a release | `git tag -a v0.2 -m "..." && git push origin v0.2` |

---

## DofE log reminder

Open both [`dfe/session-log.md`](../../dfe/session-log.md) and [`dfe/milestone-2-reflection.md`](../../dfe/milestone-2-reflection.md). For Session 16 specifically:

- A short clip of the heatmap overlay during a busy chemistry demo (toggle visible)
- The link to the v0.2 release on your GitHub
- One paragraph contrasting v0.1 and v0.2 — what does v0.2 do that would have been *unthinkable* at the end of Session 8?

→ Onwards to [Month 3: The Alchemy Game](../../month-3/README.md).
