# Progress Summary

A static map of the entire course. Use this to give the assessor (or yourself) a quick overview of what each session covered and what evidence it produced.

| Session | Title | Concept(s) introduced | Evidence in this repo |
|---|---|---|---|
| 1 | A Window, a Grid, and Your First Pixel | `cargo new`, `cargo add`, `async fn main`, `loop`, `Vec<Vec<u8>>`, macroquad basics, mouse input | `month-1/session-01/solution/` (windowed dot painter) |
| 2 | Variables, Types, and Giving Sand a Colour | `let`/`let mut`, scalar types, type inference, `const`, named colours, `if`/`else` | `month-1/session-02/solution/` (typed grid + element colours) |
| 3 | Sand Falls | Nested `for` loops, swap-in-Vec, bottom-to-top iteration, `fastrand` | `month-1/session-03/solution/` (gravity + piling) |
| 4 | Control Flow and a Better Update Loop | `if`/`else if`/`else`, `continue`/`break`, bounds checking, refactoring into a function | `month-1/session-04/solution/` (clean update loop) |
| 5 | Pattern Matching and Multiple Elements | `match` expressions, exhaustive matching, `_` wildcard | `month-1/session-05/solution/` (sand + water + stone physics) |
| 6 | Enums — Giving Elements Proper Names | `enum`, `#[derive(Debug, Clone, Copy, PartialEq)]`, matching on enums | `month-1/session-06/solution/` (CellType enum) |
| 7 | Project Build Part 1 — Element Selector and Brush | Project work — applying Sessions 1–6 | `month-1/milestone/sand-sim-v0.1/` (UI + brush in progress) |
| 8 | Project Build Part 2 — Polish and Milestone | FPS counter, pause, clear, erase, audio | `month-1/milestone/sand-sim-v0.1/` (complete) + Milestone 1 reflection |
| 9 | Structs — Giving Cells a Temperature | `struct`, `impl`, `&self`/`&mut self`, associated functions | `month-2/session-09/solution/` (Cell struct with temperature) |
| 10 | Enums with Data and `Option` — Modelling Reactions | Enum variants with data, `Option<T>`, `Some`/`None`, pattern-matching `Option` | `month-2/session-10/solution/` (first formal reaction: wood ignites) |
| 11 | Fire | `fastrand::f32()` probabilities, cell lifetime field | `month-2/session-11/solution/` (fire spreads + burns out) |
| 12 | Oil and Explosive Reactions | `Vec` in depth, neighbour iteration, tuned probabilities | `month-2/session-12/solution/` (oil ignition cascade) |
| 13 | Steam — A State Change | Iterators: `.iter()`, `.enumerate()`, `.map()`, `.filter()` | `month-2/session-13/solution/` (water → steam → water) |
| 14 | Acid and the Reactions Architecture | `HashMap<(CellType, CellType), ReactionOutcome>`, architectural refactor | `month-2/session-14/solution/` (reactions table + acid) |
| 15 | Project Build Part 1 — Lava, Ice, and Chain Reactions | Project work — adding elements via the reactions table | `month-2/milestone/sand-sim-v0.2/` (lava + ice in progress) |
| 16 | Project Build Part 2 — Polish and Milestone | Heat-map overlay, element counts, audio effects | `month-2/milestone/sand-sim-v0.2/` (complete) + Milestone 2 reflection |
| 17 | Modules — Taming the Codebase | `mod`, `pub`, `use`, multi-file projects, `super::` | `month-3/session-17/solution/` (project split across files) |
| 18 | File I/O — Save and Load | `std::fs`, `serde`, `serde_json`, `Result<T, E>`, the `?` operator | `month-3/session-18/solution/` (save / load to JSON) |
| 19 | The Recipe System — Unlocking Elements | Closures, `Fn` trait, iterators: `.any()`, `.collect()` | `month-3/session-19/solution/` (discovery system) |
| 20 | The Codex UI | Generics, `Box<dyn Trait>`, simple UI layout | `month-3/session-20/solution/` (codex with silhouettes) |
| 21 | New Elements — Gunpowder and Glass | `move` closures, iterator chaining, `flat_map` | `month-3/session-21/solution/` (explosions + glass) |
| 22 | New Elements — Concrete and Rust | `std::time`, time-based state changes | `month-3/session-22/solution/` (concrete sets, metal rusts) |
| 23 | Polish and Secrets | `enum GameState`, state-machine pattern | `month-3/session-23/solution/` (title screen + hidden recipes) |
| 24 | Showcase, Retrospective, and What Next | Final polish, `cargo clippy`, project README | `month-3/milestone/sand-sim-v1.0/` (complete) + Milestone 3 reflection + participant statement |

---

**Total:** 24 sessions • 3 milestone releases of one project (`sand-sim` v0.1 → v0.2 → v1.0) • a working `starter/` + `solution/` Cargo project per session • 1 complete DofE evidence pack.
