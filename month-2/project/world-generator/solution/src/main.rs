// Month 2 project: world-generator
// Deterministic procedural terrain generator. No external crates.

use std::collections::HashMap;
use std::fmt;
use std::process::ExitCode;

// ---------- Types ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ocean,
    Plains,
    Forest,
    Mountain,
    Desert,
}

impl Tile {
    fn name(&self) -> &'static str {
        match self {
            Tile::Ocean    => "Ocean",
            Tile::Plains   => "Plains",
            Tile::Forest   => "Forest",
            Tile::Mountain => "Mountains",
            Tile::Desert   => "Desert",
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Tile::Ocean    => '~',
            Tile::Plains   => '▒',
            Tile::Forest   => '▓',
            Tile::Mountain => '▲',
            Tile::Desert   => '.',
        };
        write!(f, "{}", c)
    }
}

struct World {
    seed: u64,
    width: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
}

impl World {
    fn generate(seed: u64, width: usize, height: usize) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::with_capacity(height);
        for y in 0..height {
            let mut row: Vec<Tile> = Vec::with_capacity(width);
            for x in 0..width {
                let n = hash(seed, x, y);
                row.push(tile_for(n));
            }
            grid.push(row);
        }
        Self { seed, width, height, grid }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Seed: {}  |  World: {}x{}\n", self.seed, self.width, self.height));
        for row in &self.grid {
            for tile in row {
                s.push_str(&format!("{}", tile));
            }
            s.push('\n');
        }
        s
    }

    fn stats(&self) -> HashMap<Tile, u32> {
        let mut counts: HashMap<Tile, u32> = HashMap::new();
        for tile in self.grid.iter().flatten() {
            *counts.entry(*tile).or_insert(0) += 1;
        }
        counts
    }
}

// ---------- Hash / noise ----------

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

fn tile_for(n: f64) -> Tile {
    if n < 0.30      { Tile::Ocean }
    else if n < 0.55 { Tile::Plains }
    else if n < 0.78 { Tile::Forest }
    else if n < 0.90 { Tile::Mountain }
    else             { Tile::Desert }
}

// ---------- CLI ----------

#[derive(Debug)]
enum ArgError {
    NotANumber(String, String),    // (name, value)
    OutOfRange(String, u64),       // (name, value)
    MissingValue(String),
}

struct Args {
    seed: u64,
    width: usize,
    height: usize,
}

fn parse_args() -> Result<Args, ArgError> {
    let mut seed: u64 = 0;
    let mut width: usize = 80;
    let mut height: usize = 24;

    let raw: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--seed" => {
                let v = raw.get(i + 1).ok_or_else(|| ArgError::MissingValue("--seed".into()))?;
                seed = v.parse().map_err(|_| ArgError::NotANumber("--seed".into(), v.clone()))?;
                i += 2;
            }
            "--width" => {
                let v = raw.get(i + 1).ok_or_else(|| ArgError::MissingValue("--width".into()))?;
                let n: u64 = v.parse().map_err(|_| ArgError::NotANumber("--width".into(), v.clone()))?;
                if !(5..=400).contains(&n) {
                    return Err(ArgError::OutOfRange("--width".into(), n));
                }
                width = n as usize;
                i += 2;
            }
            "--height" => {
                let v = raw.get(i + 1).ok_or_else(|| ArgError::MissingValue("--height".into()))?;
                let n: u64 = v.parse().map_err(|_| ArgError::NotANumber("--height".into(), v.clone()))?;
                if !(3..=200).contains(&n) {
                    return Err(ArgError::OutOfRange("--height".into(), n));
                }
                height = n as usize;
                i += 2;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => {
                eprintln!("warning: unknown argument: {}", other);
                i += 1;
            }
        }
    }

    Ok(Args { seed, width, height })
}

fn print_help() {
    println!("world-generator — deterministic ASCII terrain\n");
    println!("USAGE:");
    println!("    cargo run -- [OPTIONS]\n");
    println!("OPTIONS:");
    println!("    --seed <N>      Seed for the world (default 0)");
    println!("    --width <N>     World width  in tiles, 5..=400 (default 80)");
    println!("    --height <N>    World height in tiles, 3..=200 (default 24)");
    println!("    -h, --help      Show this help");
}

fn print_arg_error(e: &ArgError) {
    match e {
        ArgError::NotANumber(name, v) => eprintln!("Error: {} expects a number, got '{}'", name, v),
        ArgError::OutOfRange(name, v) => eprintln!("Error: {} = {} is out of range", name, v),
        ArgError::MissingValue(name)  => eprintln!("Error: {} requires a value", name),
    }
}

// ---------- main ----------

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            print_arg_error(&e);
            eprintln!("Run with --help for usage.");
            return ExitCode::from(1);
        }
    };

    let world = World::generate(args.seed, args.width, args.height);
    print!("{}", world.render());
    println!();
    println!("Legend: ~ Ocean  ▒ Plains  ▓ Forest  ▲ Mountains  . Desert");

    let stats = world.stats();
    let order = [Tile::Ocean, Tile::Plains, Tile::Forest, Tile::Mountain, Tile::Desert];
    let mut parts: Vec<String> = Vec::new();
    for t in order {
        let n = stats.get(&t).copied().unwrap_or(0);
        parts.push(format!("{} {}", t.name(), n));
    }
    println!("Stats:  {}", parts.join("  "));

    ExitCode::SUCCESS
}
