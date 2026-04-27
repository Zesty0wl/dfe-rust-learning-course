# `world-generator` — Month 2 Project

A seed-based, deterministic procedural terrain generator. Inspired by Minecraft's world generation: type a seed number, get a unique world. Type the same seed again — same world.

## Usage

```text
$ cargo run -- --seed 42 --width 60 --height 20

Seed: 42  |  World: 60x20
[a 60x20 ASCII map of biomes]

Legend: ~ Ocean  ▒ Plains  ▓ Forest  ▲ Mountains  . Desert
Stats:  Ocean 412  Plains 318  Forest 287  Mountains 91  Desert 92
```

Defaults if you omit args: `--seed 0 --width 80 --height 24`.

## How It Works

1. **Parse CLI args** — `--seed`, `--width`, `--height`. All optional.
2. **For every (x, y) in the grid**, compute a deterministic pseudo-random value in `[0, 1)` from the seed and the coordinates using a hash function (no external crates needed).
3. **Map that value to a biome** using fixed thresholds (e.g. `< 0.3` → Ocean, `< 0.55` → Plains, etc.).
4. **Render** the grid char by char, then print a legend and statistics.

The hash function is a small Linear Congruential Generator-style mixer plus a couple of xor-shifts. It's not cryptographic, but it's perfectly fine for visual procedural generation — different seeds give visually independent worlds.

## Folder Layout

- `starter/` — A skeleton with `TODO`s. Use this as your starting point in Sessions 15 and 16.
- `solution/` — A complete, working reference implementation. Look here if you get stuck — but please *try first* and *peek second*.

## Running the Solution

```bash
cd solution
cargo run                            # default world
cargo run -- --seed 7                # different world
cargo run -- --seed 7 --width 100    # wider world, same seed → same biomes left side
cargo run -- --seed 1234 --width 120 --height 40
```

## Concepts Used

- Structs and methods (Session 9)
- Enums with data (Session 10) — `Tile`
- `Vec<Vec<Tile>>` (Session 11)
- Iterators for rendering and statistics (Session 12)
- `Result` and custom errors for argument parsing (Session 13)
- `Display` trait on `Tile` (Session 14)

No external crates required for the core. This entire project runs on the standard library.
