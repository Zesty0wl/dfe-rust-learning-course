# Session 20 — The Codex UI

> **Stuck on a word?** Things like *generic*, *type parameter*, *trait bound*, *trait object*, *silhouette* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session pressing **`TAB` opens a Pokédex-style codex**: a grid of squares, one per element. Discovered ones are in colour with their name underneath; undiscovered ones are grey silhouettes labelled "???". Hovering over an entry shows a brief description.

---

## What you'll learn

- **Generics**: `fn draw_box<T: Drawable>(item: T)` — writing functions that work for many types
- **Trait bounds**: `T: Display + Clone` — constraining what a generic can be
- **Trait objects**: `Box<dyn Trait>` — picking the type at runtime
- The difference between **monomorphisation** (generics) and **dynamic dispatch** (trait objects)
- A small UI layout system: rows × columns, with a hover state

---

## The big idea

You've been using generics implicitly since `Vec<T>` and `HashMap<K, V>`. Today you write your own.

A **generic function** is a function whose type isn't fixed at the call site — it's *filled in* by the caller. `fn first<T>(v: &Vec<T>) -> &T { &v[0] }` works for any `T`. The compiler generates a version per type used (monomorphisation).

A **trait object** (`Box<dyn Trait>`) goes the other way: instead of generating versions per type, the compiler holds a single function that walks a runtime vtable. Slower per call (one indirect jump) but flexible.

Today's codex uses generics for the visible part — a `fn draw_entry<T: ElementInfo>(entry: &T)` — and trait objects in the codex's internal list because each entry has slightly different metadata. Both patterns in one feature.

---

## Concepts covered

- `fn name<T: Trait>(x: T)` — generic function with trait bound
- `struct Pair<A, B> { first: A, second: B }` — generic struct
- `trait ElementInfo` — defining a small trait
- `Box<dyn ElementInfo>` — heterogeneous list of trait objects
- `where` clauses for complex bounds
- `impl Trait` in return position
- `let mouse_over = is_inside(swatch_rect, mouse_position());`

---

## Building towards `sand-sim`

The codex is **the most player-facing UI** of v1.0. It's also the proof of concept for any future "menu" — settings, recipe browser, level select. The generic `draw_entry` is reusable enough that you'd build the entire pause menu the same way.

---

## Step-by-step walkthrough

> **Where you should be.** Session 19 finished. Recipes work. `Discoveries::is_unlocked` tells you what's available. The selector filters to discovered elements.

### 1. The `ElementInfo` trait — 3 minutes

In `src/elements.rs`, below the `Cell` impl:

```rust
pub trait ElementInfo {
    fn cell_type(&self) -> CellType;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn colour(&self) -> Color;
}

/// A concrete record of metadata per element.
pub struct ElementEntry {
    pub cell_type:   CellType,
    pub name:        &'static str,
    pub description: &'static str,
}

impl ElementInfo for ElementEntry {
    fn cell_type(&self)   -> CellType    { self.cell_type }
    fn name(&self)        -> &'static str { self.name }
    fn description(&self) -> &'static str { self.description }
    fn colour(&self)      -> Color       { self.cell_type.colour() }
}
```

A **trait** is a contract: "anything that implements `ElementInfo` provides these four methods." Same idea as interfaces in Java/C# but with much more powerful generics support.

We define `ElementEntry` as the canonical implementor. We *could* implement `ElementInfo` for `CellType` directly, but having a separate struct lets each entry carry richer metadata (a description that doesn't belong on the enum).

### 2. The element catalogue — 4 minutes

```rust
pub fn catalogue() -> Vec<ElementEntry> {
    use CellType::*;
    vec![
        ElementEntry { cell_type: Sand,    name: "Sand",      description: "Loose granular solid; piles at the angle of repose." },
        ElementEntry { cell_type: Water,   name: "Water",     description: "Flowing liquid; boils to steam at 100°C." },
        ElementEntry { cell_type: Stone,   name: "Stone",     description: "Static solid. Dissolved by acid." },
        ElementEntry { cell_type: Wood,    name: "Wood",      description: "Flammable solid; chars before igniting." },
        ElementEntry { cell_type: Fire,    name: "Fire",      description: "Ignition source. Spreads probabilistically." },
        ElementEntry { cell_type: Smoke,   name: "Smoke",     description: "Rises and dissipates." },
        ElementEntry { cell_type: Oil,     name: "Oil",       description: "Dense flammable liquid. Floats on water." },
        ElementEntry { cell_type: OilFire, name: "Oil Fire",  description: "Hot, fast-burning oil." },
        ElementEntry { cell_type: Steam,   name: "Steam",     description: "Rises; condenses below 60°C." },
        ElementEntry { cell_type: Acid,    name: "Acid",      description: "Corrosive liquid. Dissolves stone and wood." },
        ElementEntry { cell_type: Lava,    name: "Lava",      description: "Hot molten rock. Solidifies on water contact." },
        ElementEntry { cell_type: Ice,     name: "Ice",       description: "Cold solid. Melts above 0°C." },
    ]
}
```

### 3. The generic `draw_entry` function — 5 minutes

Create `src/codex.rs`:

```rust
use macroquad::prelude::*;
use crate::elements::{ElementInfo, ElementEntry, catalogue};
use crate::recipes::Discoveries;

const CODEX_COLS: usize = 4;
const CODEX_TILE: f32   = 96.0;
const CODEX_PAD:  f32   = 16.0;

pub fn draw_codex(discoveries: &Discoveries) {
    // Dim the background.
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.75));

    let entries = catalogue();
    let origin_x = (screen_width()  - (CODEX_COLS as f32 * (CODEX_TILE + CODEX_PAD))) / 2.0;
    let origin_y = 60.0;
    let (mx, my) = mouse_position();
    let mut hover: Option<&ElementEntry> = None;

    for (i, entry) in entries.iter().enumerate() {
        let row = i / CODEX_COLS;
        let col = i % CODEX_COLS;
        let x = origin_x + col as f32 * (CODEX_TILE + CODEX_PAD);
        let y = origin_y + row as f32 * (CODEX_TILE + CODEX_PAD);

        let unlocked = discoveries.is_unlocked(entry.cell_type());
        draw_entry(entry, x, y, CODEX_TILE, unlocked);

        // Track hover.
        if mx >= x && mx < x + CODEX_TILE && my >= y && my < y + CODEX_TILE {
            hover = Some(entry);
        }
    }

    if let Some(entry) = hover {
        draw_tooltip(entry, discoveries.is_unlocked(entry.cell_type()), mx, my);
    }
}

fn draw_entry<T: ElementInfo>(entry: &T, x: f32, y: f32, size: f32, unlocked: bool) {
    if unlocked {
        draw_rectangle(x, y, size, size, entry.colour());
        draw_text(entry.name(), x, y + size + 16.0, 18.0, WHITE);
    } else {
        // Greyed-out silhouette.
        draw_rectangle(x, y, size, size, Color::new(0.20, 0.20, 0.20, 1.0));
        draw_text("???", x + size / 2.0 - 14.0, y + size / 2.0 + 8.0, 24.0, DARKGRAY);
    }
    draw_rectangle_lines(x, y, size, size, 2.0, GRAY);
}

fn draw_tooltip<T: ElementInfo>(entry: &T, unlocked: bool, mx: f32, my: f32) {
    let text = if unlocked { entry.description() } else { "Undiscovered." };
    let tw   = measure_text(text, None, 16, 1.0).width;
    draw_rectangle(mx + 12.0, my + 12.0, tw + 16.0, 30.0, Color::new(0.10, 0.10, 0.15, 0.95));
    draw_text(text, mx + 20.0, my + 32.0, 16.0, WHITE);
}
```

Three things to notice:

- **`fn draw_entry<T: ElementInfo>(entry: &T, ...)`** — the function is generic over any `T` that implements `ElementInfo`. The compiler generates a specialised version for every `T` it sees. Today only `ElementEntry` flows through, so only one version. *Future:* if you add `WeatherInfo` or `EnemyInfo` traits, the same UI code works.
- **`measure_text(text, None, 16, 1.0)`** — macroquad's text-measurer. Returns a `TextDimensions { width, height, offset_y }`. Used to size the tooltip background.
- **`Option<&ElementEntry>`** as the hover state is the idiomatic "one of these or nothing" Rust shape. No need for a sentinel "no hover" value.

### 4. The TAB key — 1 minute

In `main.rs` or `ui::handle_input`:

```rust
let mut codex_open = false;

if is_key_pressed(KeyCode::Tab) { codex_open = !codex_open; }
```

In the render block:

```rust
        clear_background(BLACK);
        render_grid(&grid, heatmap);
        draw_selector(selected, brush_radius, &discoveries);
        draw_legend();
        draw_hud(&counts);
        if codex_open {
            codex::draw_codex(&discoveries);
        }
```

**Save. Run.** Press `TAB`. **The codex appears, dimming the world behind it.** Discovered elements glow with their colour and name; undiscovered ones are grey ??? squares. Hover one: tooltip appears. Press `TAB` again to close.

> **The Wow Moment.** Build a long session. Don't open the codex. Play for ten minutes and trigger as many recipes as you can. *Then* press `TAB`. **You see, at a glance, the entire scope of your chemistry — every element you've found in colour, every one you haven't as a mystery.** That feeling — "what's behind the grey squares?" — is the whole reason people play discovery games for 100 hours. **You shipped that feeling in 200 lines.**

### 5. (Optional) Animations — 4 minutes

A satisfying touch: just-unlocked entries pulse for two seconds.

```rust
// In Discoveries:
pub struct Discoveries {
    pub unlocked: Vec<CellType>,
    pub recently_unlocked: Vec<(CellType, f32)>,  // (which, time_unlocked)
}

// On unlock:
self.recently_unlocked.push((t, get_time() as f32));

// In draw_entry:
if let Some((_, time)) = recently_unlocked.iter().find(|(t, _)| *t == entry.cell_type()) {
    let age = get_time() as f32 - time;
    if age < 2.0 {
        let pulse = (age * 6.0).sin() * 0.3 + 1.0;
        let scaled_size = size * pulse;
        // Draw at scaled_size, centred on the tile centre
    }
}
```

(Sketch — fill in for real.)

### 6. (Optional) Heterogeneous list via trait objects — 4 minutes

Right now the catalogue holds only `ElementEntry`s. What if some entries needed *different* shapes — e.g. an entry that points at a recipe rather than an element? Trait objects let you mix:

```rust
fn rich_catalogue() -> Vec<Box<dyn ElementInfo>> {
    let mut out: Vec<Box<dyn ElementInfo>> = Vec::new();
    for e in catalogue() {
        out.push(Box::new(e));
    }
    // Could also push Box::new(some_other_thing_implementing_ElementInfo)
    out
}
```

This costs you compile-time monomorphisation in exchange for runtime flexibility. For `sand-sim` v1.0 the generic version is fine — included here for the conceptual contrast.

---

## Linux (Ubuntu) note

Generics compile separately per type. Adding a new generic implementor (new struct implementing `ElementInfo`) makes incremental builds slightly slower. To see the cost on Ubuntu:

```bash
time cargo build --release      # before adding a new type
# add a struct + impl
time cargo build --release      # after; usually +1-3 seconds
```

For your project's size this is invisible. For a 100,000-line codebase, generics-bloat is a known issue and `dyn Trait` is sometimes preferred for build-time reasons.

**HiDPI tooltip positioning.** If you're on a 4K Ubuntu laptop at 200% scaling, the tooltip might appear off-screen-right when the mouse is near the right edge. Clamp the tooltip x to `screen_width() - tooltip_width - 8.0`. Simple fix; mention this in code.

**Wayland transparency.** The codex background uses alpha `0.75` to dim the world. Most Ubuntu Wayland compositors handle this correctly. If your codex appears opaque black instead of semi-transparent, your compositor is dropping alpha — verify by rendering a fully-transparent rectangle (`Color::new(1.0, 0.0, 0.0, 0.5)` should look pink, not red). If broken, draw a colour-mixed background instead of relying on alpha blending.

---

## Common mistakes

### `error: trait 'ElementInfo' is not object-safe`

If you accidentally added a method to `ElementInfo` that returns `Self` (e.g. `fn clone(&self) -> Self`), the trait can no longer be made into a trait object. Object-safe traits can only use `&self`/`&mut self`/`self` receivers and can't have generic methods or return `Self`. Either drop the offending method or split the trait into two.

### Generic version compiles, trait-object version doesn't

Generic functions are monomorphised — each type gets its own machine code, which can take advantage of inlining. Trait objects share one machine-code body, indirected through a vtable. If a method returns `Self` or takes a generic, you can't put it on a trait object. Generally: start with generics; switch to `dyn` only when you need runtime flexibility.

### Codex tiles overlap or wrap weirdly

The tile sizes and spacing are based on `screen_width()`. If you resize the window, the layout still uses the *initial* width. Fix: read `screen_width()` inside `draw_codex` so the layout re-measures each frame.

### Tooltip flickers when the mouse hovers near tile borders

You're computing hover as `>= x && < x + size` which is correct, but the tooltip itself can overlap a neighbouring tile, briefly stealing the hover. Position the tooltip offset by `(12.0, 12.0)` from the mouse (as above) and clamp to screen bounds.

### `error[E0277]: 'T' doesn't implement 'Sized'`

Your generic constraint missed `Sized`. By default, every generic parameter has an implicit `Sized` bound — only loosen it with `T: ?Sized` if you specifically want unsized types like `[T]` or `dyn Trait`. For `ElementInfo` impls (all owned structs), the default `Sized` bound is what you want.

### Discoveries persistence breaks after adding fields

If you save with v1, add `recently_unlocked`, and try to load — `serde_json` complains about missing fields. Either bump the version (refuse old saves), or use `#[serde(default)]` on the new field:

```rust
#[serde(default)]
pub recently_unlocked: Vec<(CellType, f32)>,
```

---

## Session challenge

Pick one — no solution provided.

1. **Codex sort order.** Show discovered elements first (in discovery order), then undiscovered. Or: alphabetical. Or: by category (solid/liquid/gas/reactive). Add a key (`O`) to cycle sort modes.
2. **Codex search.** Type a letter while the codex is open to filter to entries whose name starts with that letter. Show "type a letter…" as a hint.
3. **A second trait, `IsLiquid`.** Implement it for `ElementEntry` based on cell_type. Filter the codex to only liquids with a toggle key.
4. **Display recipe hints.** For each undiscovered entry, show a one-line cryptic hint ("found near boiling water") instead of `???`. Store hints alongside the recipe.

---

## Quick reference

| What | Code |
|---|---|
| Generic function | `fn f<T: Trait>(x: T)` |
| Multiple bounds | `fn f<T: A + B>(x: T)` |
| `where` clause | `fn f<T>() where T: A + B { ... }` |
| Generic struct | `struct Pair<A, B> { first: A, second: B }` |
| Trait | `trait Foo { fn bar(&self) -> u32; }` |
| Trait object | `Box<dyn Foo>` |
| `impl Trait` arg | `fn f(x: impl Trait)` |
| `impl Trait` return | `fn make() -> impl Trait { ... }` |
| Measure text | `measure_text(s, font, size, scale)` |

---

## DofE log reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 20**. Worth recording:

- A screenshot of the codex showing some discovered and some undiscovered entries — the visible *progression*
- Your sentence on "generics vs trait objects" — when would you reach for each? (Assessors will probe this in interviews if you ever apply for a junior Rust role.)
