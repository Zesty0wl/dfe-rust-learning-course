# Month 2 — Intermediate

**Theme:** Structuring data and building systems.

**Project:** [`world-generator`](./project/world-generator/) — a seed-based procedural terrain generator that prints an ASCII map to the terminal. Different seeds = different worlds, exactly like Minecraft.

```text
$ cargo run -- --seed 42 --width 60 --height 20

Seed: 42  |  World: 60x20
~~~~~~~~~~~~~~~▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒~~~~~~~~~~~~........
~~~~~~~~~~~~▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒~~~~~~~~~~~~~........
...
Legend: ~ Ocean  ▒ Plains  ▓ Forest  ▲ Mountains  . Desert
Stats:  Ocean 412  Plains 318  Forest 287  Mountains 91  Desert 92
```

By the end of Month 2 you will have built a **deterministic procedural world generator** from scratch, with no external crates required for the core engine. You'll have completed Milestone 2 of your DofE evidence pack.

---

## Sessions

| # | Title | Concepts |
|---|---|---|
| 9  | [Structs and Methods](./session-09/) | `struct`, `impl`, `&self`, `#[derive(Debug)]` |
| 10 | [Enums with Data and `Option<T>`](./session-10/) | Variants with data, `Option`, `Some`/`None` |
| 11 | [Collections — `Vec` and `HashMap`](./session-11/) | `Vec` in depth, `HashMap`, choosing between them |
| 12 | [Iterators and Closures (Introduction)](./session-12/) | `.iter()`, `.map()`, `.filter()`, `.collect()`, closures |
| 13 | [Error Handling](./session-13/) | `Result<T, E>`, `?`, custom error enums |
| 14 | [Traits](./session-14/) | Defining/implementing traits, `Display`, `Debug`, trait bounds |
| 15 | [Mini-Project Build Part 1 — World Core](./session-15/) | Project work — generation engine |
| 16 | [Mini-Project Build Part 2 — Render and Polish](./session-16/) | Rendering, biomes, statistics |

---

## What's New in Month 2

In Month 1 you wrote programs. In Month 2 you start designing **systems**. That means:

- Modelling real things as **types** (a `Block`, a `Tile`, a `World`)
- Storing many of them in **collections** (`Vec`, `HashMap`)
- Processing them with **iterators** rather than manual loops
- Handling **failure** as a first-class concept with `Result`
- Adding **shared behaviour** with **traits** (Rust's answer to interfaces)

By Month 2's end you should be reading other people's Rust code without panic and writing your own without copying every line from a tutorial.

---

## DofE Reminder

Every session this month earns one row in your [`session-log.md`](../dfe/session-log.md). At the end of Session 16, fill in [`milestone-2-reflection.md`](../dfe/milestone-2-reflection.md). That's evidence of **eight more weeks of regular Skill activity** for your assessor.
