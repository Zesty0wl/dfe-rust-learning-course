// Month 2 project: world-generator (STARTER)
//
// Build the world generator step-by-step in Sessions 15 & 16.
// If you get truly stuck, peek at ../solution/src/main.rs — but try first!
//
// SESSION 15 GOALS:
//   1. Define the Tile enum.
//   2. Define the World struct with seed, width, height, grid: Vec<Vec<Tile>>.
//   3. Implement the hash function (provided below).
//   4. Implement World::generate to fill the grid using hash + thresholds.
//
// SESSION 16 GOALS:
//   5. Implement Display for Tile (one char per biome).
//   6. Implement World::render → String.
//   7. Implement World::stats → HashMap<Tile, u32>.
//   8. Add the CLI: --seed, --width, --height with proper Result-based errors.

use std::collections::HashMap;
use std::fmt;

// ---------- Types ----------

// TODO (Session 15): define Tile with variants Ocean, Plains, Forest, Mountain, Desert.
// Derive Debug, Clone, Copy, PartialEq, Eq, Hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    // TODO: variants
    Placeholder,
}

// TODO (Session 16): impl fmt::Display for Tile so println!("{}", tile) prints the right char.

// TODO (Session 15): define `struct World { seed, width, height, grid }`.
struct World {
    // TODO
}

impl World {
    // TODO (Session 15): pub fn generate(seed: u64, width: usize, height: usize) -> Self
    //   - allocate Vec<Vec<Tile>> of `height` rows, each `width` long
    //   - fill via hash() + tile_for()
    // TODO (Session 16): pub fn render(&self) -> String  (header + grid)
    // TODO (Session 16): pub fn stats(&self) -> HashMap<Tile, u32>
}

// ---------- Hash / noise (provided — you don't need to invent this) ----------

fn hash(seed: u64, x: usize, y: usize) -> f64 {
    let mut h = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(x as u64)
        .wrapping_mul(2891336453)
        .wrapping_add(y as u64);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    (h as f64) / (u64::MAX as f64)
}

// TODO (Session 15): map a [0, 1) value to a Tile using thresholds.
fn tile_for(_n: f64) -> Tile {
    // Suggested thresholds:
    //   < 0.30  Ocean
    //   < 0.55  Plains
    //   < 0.78  Forest
    //   < 0.90  Mountain
    //   else    Desert
    Tile::Placeholder
}

// ---------- CLI (Session 16) ----------

// TODO (Session 16): define enum ArgError, struct Args, fn parse_args() -> Result<Args, ArgError>.

fn main() {
    println!("world-generator (starter) — fill in TODOs in Sessions 15 & 16!");
    let n = hash(42, 0, 0);
    println!("Smoke test: hash(42, 0, 0) = {:.4}  (will be the same on every run)", n);
}
