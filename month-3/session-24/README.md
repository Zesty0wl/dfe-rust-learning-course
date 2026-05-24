# Session 24 — Showcase and Retrospective (v1.0 ships)

> **Stuck on a word?** Things like *retrospective*, *changelog*, *MVP*, *post-mortem* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **`sand-sim` v1.0 ships**. It has a polished README, a project-root README updated to point at the v1.0 directory, a `cargo clippy` pass with no warnings, a tagged release on your fork, and the milestone-3 reflection complete. **You can hand the binary to a friend, send the GitHub link to a recruiter, and proudly screenshot the codex for your DofE folder.**

You also do something specific to this session: **the retrospective.** What you built, what you learned, what's next.

---

## What you'll learn

- The ship discipline — polishing the last 10% takes the same time as the previous 50%
- `cargo clippy` — the Rust linter
- `cargo fmt` — automatic formatting
- The retrospective form — "what worked, what surprised, what next"
- Pointers to the major next-step crates and topics (rayon, wasm32, Bevy, embedded)

---

## The big idea

A project isn't done when the last feature works. It's done when **someone else can pick it up, understand it, and do something with it.** That's the polish work — README, tests, formatting, commit hygiene, a sensible release note.

The retrospective is the second half. You spent three months on `sand-sim`. **What did you actually learn?** Not "I learned Rust" — be specific. *"I learned that table-driven code reads better than branchy code, and the moment to refactor is when you have more than four cases."* That kind of sentence is worth thirty interview questions answered.

---

## Concepts covered

- `cargo clippy --all-targets -- -D warnings`
- `cargo fmt --check`
- README structure for a portfolio project
- Git tagging conventions (`v1.0.0`)
- The retrospective: three questions, honest answers
- Future-direction pointers: **rayon**, **wasm32**, **Bevy**, **embedded Rust**, fluid-dynamics

---

## Building towards `sand-sim`

This is the milestone. **You ship.** Then you move on.

---

## Step-by-step walkthrough

> **Where you should be.** Session 23 finished. Title screen, state machine, hidden recipes, easter egg all work. Codex is gorgeous. The sim has 15+ elements, table-driven chemistry, multi-second timed reactions, persistent saves.

### 1. `cargo clippy` and `cargo fmt` — 8 minutes

From `month-3/milestone/sand-sim-v1.0/`:

```bash
cargo clippy --all-targets -- -D warnings
```

`-D warnings` turns every clippy warning into an error. You'll get a list. Common ones:

- `redundant_field_names` — `Cell { cell_type: cell_type }` should be `Cell { cell_type }`.
- `needless_return` — drop the final `return`.
- `useless_vec` — `vec![1, 2, 3].iter()` → `[1, 2, 3].iter()`.
- `single_match` — single-arm `match` should be `if let`.

Fix each. They're not strict requirements but they're a respected convention. Code that passes `clippy -D warnings` reads as "this person knows the language."

Then:

```bash
cargo fmt
```

Auto-formats everything. Commit the result as a separate "cargo fmt pass" commit so the diff is small.

Then verify:

```bash
cargo clippy --all-targets -- -D warnings && cargo fmt --check && cargo build --release && cargo run --release --example smoke_test
```

(If you don't have a `smoke_test` example, skip it. The chain just confirms the project is in clean shape.)

### 2. Project-root README — 5 minutes

Edit `/README.md` (the top-level repo README) to point at v1.0:

```markdown
# Rust Learning Course — sand-sim project

A three-month Rust course delivered through the Duke of Edinburgh Award scheme. Build a falling-sand physics simulator over 24 sessions, shipping three releases.

## Releases

- **v0.1** — `month-1/milestone/sand-sim-v0.1/` — sand, water, stone, brush, audio.
- **v0.2** — `month-2/milestone/sand-sim-v0.2/` — fire chemistry, lava, ice, table-driven reactions.
- **v1.0** — `month-3/milestone/sand-sim-v1.0/` — recipe discovery, codex UI, save/load, polish.

## Quick run

```bash
cd month-3/milestone/sand-sim-v1.0
cargo run --release
```

## Course materials

- Month 1: Foundations (see `month-1/README.md`)
- Month 2: Chemistry (see `month-2/README.md`)
- Month 3: The Alchemy Game (see `month-3/README.md`)
- Glossary at `GLOSSARY.md`
- Cheat sheet at `resources/cheatsheet.md`

## DofE evidence

The `dfe/` folder holds the participant log, milestone reflections, and assessor briefing.
```

### 3. The v1.0 README — 8 minutes

Edit `month-3/milestone/sand-sim-v1.0/README.md`:

```markdown
# sand-sim v1.0

A real-time, table-driven physics-and-chemistry sandbox in Rust. 15+ elements, recipe-based discovery, save/load, codex UI, hidden secrets.

(Drop a `screenshot.png` in `assets/` and reference it here with a standard markdown image tag.)

## Run

```bash
cargo run --release
```

Recommended: 1080p+ display, audio enabled.

## Controls

| Key | Action |
|---|---|
| Enter | (title) Start new game |
| L | (title) Load save |
| 1-9 | (play) Select element |
| L-click drag | Paint |
| R-click drag | Erase |
| Scroll | Brush size |
| Space | Pause |
| Tab | Open codex |
| H | Heat-source brush |
| T | Toggle heatmap |
| S | Save |
| L | (play) Load |
| C | Clear world |
| Esc | Back / quit overlay |

## Elements

Sand, water, stone, wood, fire, smoke, oil, oil-fire, steam, acid, lava, ice, gunpowder, glass, concrete, wet concrete, iron, rust — plus three secret elements. Discover them by experimenting.

## Architecture

- `Cell { cell_type, temperature, lifetime, set_at }` — per-cell unit.
- `REACTIONS: HashMap<(CellType, CellType), ReactionOutcome>` — every pairwise interaction.
- `Recipe { name, unlocks, predicate: Box<dyn Fn(&grid) -> bool> }` — gameplay discovery.
- State machine: `GameState { Title, Playing, Codex, Paused }`.
- Modules: elements, reactions, simulation, rendering, ui, audio, recipes, codex, persist.

## Save format

JSON, v1, human-readable. Edit by hand at your own risk.

## Credits

Audio: credits live alongside this README in `assets/CREDITS.md`.

Course: see the repository root `README.md`.

## License

MIT.

## What's next

This was a course project. Real-world extensions:

- **Performance**: parallelise the cell update with [rayon](https://crates.io/crates/rayon) — split the grid into chunks per CPU core. Likely 4-8× speedup on a modern Ubuntu laptop.
- **Web**: target `wasm32-unknown-unknown` and ship a playable demo on a static page. macroquad supports this directly. ~3 lines of `Cargo.toml` change.
- **Bigger engine**: port to [Bevy](https://bevyengine.org/) — the major Rust game engine. ECS, async asset loading, scene graph. Your sim becomes a *system* in Bevy's terms.
- **Real fluid dynamics**: replace the cellular-automaton water with a true Navier-Stokes grid solver. Slower per frame but visually astonishing. Many tutorials exist.
- **Embedded**: a smaller version with ~50×30 cells could run on a Raspberry Pi Pico in `no_std` Rust, painting cells to an OLED. The same `CellType` enum, the same reactions table, the same architecture.
```

### 4. Tag the release — 1 minute

From the repo root:

```bash
git add -A
git commit -m "Ship sand-sim v1.0 — recipes, codex, save/load, polish"
git tag -a v1.0.0 -m "Month 3 milestone: 15+ elements, recipe discovery, codex UI, time-aware reactions, hidden secrets"
git push origin main
git push origin v1.0.0
```

### 5. The retrospective — 10 minutes

Open [`dfe/milestone-3-reflection.md`](../../dfe/milestone-3-reflection.md). Three sections, three honest answers.

**What you built.** Bullet form. The specifics. *"15 element types, table-driven chemistry, codex UI with generics, save/load with serde, frame-rate-independent state changes."* The hiring manager reading your CV in 6 months wants exact numbers, not "a Rust project."

**What you learned that surprised you.** Specifics again. *"The `?` operator turned what would have been my biggest pain point (error handling) into one character per call site."* *"Closures behind `dyn Fn` are how you make table-driven gameplay work."* These are interview-day quotes.

**What's next for you.** Be honest. Maybe it's "I want to learn embedded Rust." Maybe it's "I want to ship a real game on Steam." Maybe it's "I'm done with games; I want to learn Rust for backend services." All three are great answers. Pick the one that's true.

### 6. The "What You've Built" mental check — 5 minutes

Don't skip this. List, in your own words, the *concepts* you now genuinely understand:

- Variables, types, constants
- Functions, parameters, return types
- Control flow: if/else, loops, match
- Pattern matching with guards and ranges
- Enums (plain and data-bearing) and `Option`/`Result`
- Structs with methods, derives
- Modules, visibility, project structure
- HashMap, Vec, iterators, closures
- Generics and trait objects
- `serde` for JSON serialisation
- `std::time` for wall-clock timing
- `OnceLock` for static initialisation
- Borrow checker, references, ownership
- Cargo, dependencies, profiles, releases

That's a senior-intern-grade Rust skillset. **You can apply for entry-level Rust roles with this list in your CV.** The course handed you the road map; you walked it.

### 7. The send — 2 minutes

Send the GitHub link to:

- The course assessor (DofE).
- One person who knows Rust (ask for honest feedback).
- One person who doesn't know Rust at all (ask if they can run the binary).
- Yourself in six months — schedule an email.

> **The Wow Moment.** Cargo built. Clippy clean. README done. Tag pushed. Retrospective written. **Close your laptop.** Open it tomorrow. Read your own retrospective. **Realise the person who wrote that text wasn't the same person who started Session 1.**
>
> That distance is the entire point of the course. You proved to yourself, in writing, that you can land on an unfamiliar language and *ship*. Everything from here is a variation on that one capability.

---

## Linux (Ubuntu) note

The v1.0 release on Ubuntu is the one you'll actually demo. Last-mile polish:

- **Binary distribution.** `target/release/sand-sim` is a dynamically-linked ELF. To check what it needs:

  ```bash
  ldd target/release/sand-sim
  ```

  Expect: libc, libpthread, libGL, libX11, libasound. Most modern Ubuntu installs have all of these. If you want a "drop the file and it just runs" binary, look at the `cargo-deb` crate to wrap it as a `.deb`:

  ```bash
  cargo install cargo-deb
  cargo deb
  ```

  Produces `target/debian/sand-sim_1.0.0_amd64.deb`. Install with `sudo dpkg -i ...`. Removes with `sudo apt remove sand-sim`. Production-grade.

- **For a fully-portable Linux binary**: build with [musl](https://www.musl-libc.org/) to get a fully-static ELF:

  ```bash
  rustup target add x86_64-unknown-linux-musl
  cargo build --release --target x86_64-unknown-linux-musl
  ```

  Note: this requires statically linking libasound, which is not trivial. For a course project, the standard glibc binary plus `cargo-deb` is the right level.

- **Web demo.** macroquad supports `wasm32-unknown-unknown`:

  ```bash
  rustup target add wasm32-unknown-unknown
  cargo build --release --target wasm32-unknown-unknown
  ```

  Drop the resulting `.wasm` and a small index.html on GitHub Pages — instant playable demo, no install. Worth half an afternoon if you want to share with non-developers.

- **CI.** Add a `.github/workflows/ci.yml` that runs `cargo clippy && cargo fmt --check && cargo test && cargo build --release` on every push. GitHub Actions has Ubuntu runners that match your dev environment perfectly.

---

## Common mistakes

### `cargo clippy` lists 200 warnings

Normal. Work through them. Most are stylistic. The point is to internalise the style, not to fight clippy. Some warnings are genuine smells (e.g. `needless_pass_by_value` — you cloned a `String` when you could have borrowed).

### `cargo fmt` reformatted *everything* including your hand-tuned table

`rustfmt`'s default is "always". Mostly fine. If you have a table you really want preserved, surround it with `#[rustfmt::skip]`:

```rust
#[rustfmt::skip]
const NEIGHBOURS_8: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];
```

### Tag push fails

`git push origin v1.0.0` may fail with "non-fast-forward" if you already pushed a different `v1.0.0`. Delete and remake:

```bash
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0
git tag -a v1.0.0 -m "..."
git push origin v1.0.0
```

### Retrospective feels self-indulgent to write

It isn't. **Future-you doing a coding interview reads this for ammunition.** The interviewer asks "tell me about a project." You don't say "uh, well, I made a thing in Rust." You say *the sentences from your own retrospective*. Worth ten times the time it took to write.

### README links broken when viewed on GitHub

GitHub renders relative links from the file's directory. A link written `Glossary -> GLOSSARY.md` works from the root README, because `GLOSSARY.md` is in the same folder. The same link from `month-3/README.md` needs `../GLOSSARY.md` instead. Test by clicking each link in the GitHub web UI.

### A clippy warning you don't understand

`#[allow(clippy::warning_name)]` above the offending function silences it for that scope. Use sparingly; ideally fix the underlying issue. The warning name is in clippy's output.

---

## Session challenge

These are *strictly stretch*. The ship is the priority. But if you want extras:

1. **Web build.** Add a `web/` folder with the index.html + wasm setup. Host on GitHub Pages. Add the URL to the README.
2. **A unit test suite.** Write tests for `reactions::react()`, `recipes::adjacent_pair`, `simulation::step` (verify empty-grid no-op). Goal: green `cargo test`.
3. **A blog post.** Write a 1000-word "what I learned building sand-sim in 24 sessions." Post it on dev.to or medium. Link from your README and CV.
4. **A second simulation.** Use the same architecture (CellType enum, reactions table, modules) to build something *different* — a Conway's Game of Life variant, a forest-fire model, an elevation/erosion sim. Two days' work; doubles the portfolio impact.

---

## Quick reference

| What | Code |
|---|---|
| Lint | `cargo clippy --all-targets -- -D warnings` |
| Format | `cargo fmt` |
| Format check | `cargo fmt --check` |
| Release build | `cargo build --release` |
| Run release | `cargo run --release` |
| Static check | `cargo check` (faster than build) |
| Tag release | `git tag -a v1.0.0 -m "..."` |
| Push tag | `git push origin v1.0.0` |
| Strip binary | `strip target/release/sand-sim` |
| Web target | `rustup target add wasm32-unknown-unknown` |
| Deb package | `cargo install cargo-deb && cargo deb` |

---

## DofE log reminder

Open all three of:
- [`dfe/session-log.md`](../../dfe/session-log.md) — final session entry
- [`dfe/milestone-3-reflection.md`](../../dfe/milestone-3-reflection.md) — full retrospective
- [`dfe/participant-statement-template.md`](../../dfe/participant-statement-template.md) — your participant-statement draft

This is the session that produces the most assessor-facing artefacts. Be specific. Be honest. Be brief — assessors read a lot of statements; concrete impressions beat lengthy claims.

Then close the laptop. **You built something.**
