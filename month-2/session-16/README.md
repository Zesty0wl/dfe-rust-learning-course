# Session 16: Mini-Project Build Part 2 — Render and Polish

## What You'll Build

Finish `world-generator`:

- `Display` for `Tile` so each biome prints as one character
- `World::render()` that returns a `String` of the whole map
- `World::stats()` that returns a `HashMap<Tile, u32>` of biome counts
- A proper CLI with `--seed`, `--width`, `--height`, with `Result`-based errors and friendly messages
- A legend line and a stats line

End result: identical to the demo at the top of [the project README](../project/world-generator/README.md).

---

## Step-by-Step Walkthrough

Continuing in `starter/src/main.rs`.

### 1. Implement `Display` for `Tile`

```rust
use std::fmt;

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
```

Now `print!("{}", tile)` works. The Unicode block characters render in any modern terminal.

### 2. `render()` builds the whole map as a single `String`

```rust
impl World {
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
}
```

Why return a `String` instead of just `print!`-ing? **Testability**. A function that returns a string can be unit-tested; a function that prints can't (easily). Also, your `main` can choose to send the output anywhere — stdout, a file, the network. Separation of concerns.

### 3. `stats()` with `HashMap` and `flatten`

```rust
use std::collections::HashMap;

impl World {
    fn stats(&self) -> HashMap<Tile, u32> {
        let mut counts: HashMap<Tile, u32> = HashMap::new();
        for tile in self.grid.iter().flatten() {
            *counts.entry(*tile).or_insert(0) += 1;
        }
        counts
    }
}
```

The `*counts.entry(*tile).or_insert(0) += 1;` is the canonical counting idiom from Session 11. The `*tile` (deref) is needed because `iter().flatten()` gives `&Tile` and we want owned `Tile` (cheap because `Copy`).

### 4. The CLI

```rust
#[derive(Debug)]
enum ArgError {
    NotANumber(String, String),    // (name, value)
    OutOfRange(String, u64),       // (name, value)
    MissingValue(String),
}

struct Args { seed: u64, width: usize, height: usize }

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
            "--width" => { /* like above with range check 5..=400 */ }
            "--height" => { /* like above with range check 3..=200 */ }
            _ => i += 1,
        }
    }
    Ok(Args { seed, width, height })
}
```

Look at the full version in `solution/src/main.rs` — it has all three branches plus a `--help` flag.

### 5. `main` ties it together

```rust
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {:?}", e);
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
        parts.push(format!("{} {}", tile_name(&t), n));
    }
    println!("Stats:  {}", parts.join("  "));

    ExitCode::SUCCESS
}
```

`ExitCode` is a return type for `main` that lets you set the exit status cleanly. Returning `ExitCode::SUCCESS` is `0`; `ExitCode::from(1)` is `1`. Shells and CI pick this up.

> Add a small helper `fn tile_name(t: &Tile) -> &'static str` (the solution puts it on the `Tile` impl as `t.name()`). Using `Tile::Mountain` as a key requires `Tile` to derive `Eq, Hash` — which we did in Session 15.

### 6. Try it

```bash
cargo run -- --seed 42 --width 60 --height 20
cargo run -- --seed 42 --width 60 --height 20    # same output
cargo run -- --seed 7  --width 60 --height 20    # totally different
cargo run -- --seed banana                       # nice error message
cargo run -- --width 999                         # out of range error
cargo run -- --help
```

---

## Optional: Colour with `colored`

Edit `Cargo.toml`:

```toml
[dependencies]
colored = "2.1"
```

Then in `render()`:

```rust
use colored::Colorize;

let coloured: String = match tile {
    Tile::Ocean    => "~".blue().to_string(),
    Tile::Plains   => "▒".green().to_string(),
    Tile::Forest   => "▓".bright_green().to_string(),
    Tile::Mountain => "▲".white().to_string(),
    Tile::Desert   => ".".yellow().to_string(),
};
```

Run `cargo build` once to download `colored` (you'll need internet). The whole map will now be coloured in any modern terminal.

---

## Common Mistakes

- **`HashMap<Tile, u32>` errors with "trait `Hash` not implemented"** — derive `Hash, Eq` on `Tile`.
- **Integer overflow on `width * height`** — for sane sizes this is fine, but if you allow huge values, the multiplication can overflow `usize` on 32-bit systems. The range checks (5..=400, 3..=200) prevent this.
- **Forgetting `print!` vs `println!`** — `render()` already includes newlines; using `println!("{}", world.render())` adds a blank line.
- **CLI arg parsing crash on `--seed` at the end with no value** — that's exactly what `MissingValue` handles. Triggers when `raw.get(i + 1)` returns `None`.

---

## Milestone 2 Reflection

You've finished the second project. Open [`../../dfe/milestone-2-reflection.md`](../../dfe/milestone-2-reflection.md) and fill it in *now* while it's fresh:

- What did Month 2 teach you that Month 1 didn't?
- What's something that confused you for a while and then suddenly clicked?
- How does your code feel different from the Month 1 project? More structured? More confident?

Without this reflection, the milestone doesn't really exist on paper. **Write it today.**

---

## DofE Log Reminder

Row 16 — and your **second milestone** is now complete. Two thirds of your DofE evidence is in the bag.

Next month: the synthesizer.
