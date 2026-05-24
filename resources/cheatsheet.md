# Rust cheat sheet — sand-sim edition

A one-stop syntax reference for the patterns used in the 24-session course. Open this when you forget a syntax detail mid-session.

> Sand-sim-specific snippets use the project's element/cell shapes — `CellType`, `Cell`, `ReactionOutcome`. Generic Rust snippets work for any project.

---

## Cargo

| What | Command |
|---|---|
| New project | `cargo new my-project` |
| New library | `cargo new --lib my-lib` |
| Run debug | `cargo run` |
| Run release | `cargo run --release` |
| Build only | `cargo build --release` |
| Add dependency | `cargo add macroquad --features audio` |
| Remove dependency | `cargo remove fastrand` |
| Update deps | `cargo update` |
| Test | `cargo test` |
| Format | `cargo fmt` |
| Lint | `cargo clippy --all-targets -- -D warnings` |
| Fast check | `cargo check` |
| Clean build artifacts | `cargo clean` |

`Cargo.toml` for sand-sim:

```toml
[package]
name    = "sand-sim"
version = "1.0.0"
edition = "2024"

[dependencies]
macroquad   = { version = "0.4", features = ["audio"] }
fastrand    = "2"
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
```

---

## Variables and constants

```rust
let x = 5;                   // immutable by default
let mut y = 10;              // mut = mutable
let z: i32 = 42;             // explicit type
const COLS: usize = 120;     // compile-time constant
static FOO: i32 = 0;         // global; mutable statics need `unsafe`
```

Shadowing — re-bind a name, possibly changing its type:

```rust
let count = 5;
let count = "five";    // legal: new binding, different type
```

---

## Scalar types

| Type | Range | Notes |
|---|---|---|
| `i8`/`i16`/`i32`/`i64`/`i128`/`isize` | signed integers | `isize` = pointer-size |
| `u8`/`u16`/`u32`/`u64`/`u128`/`usize` | unsigned | `usize` for array indices |
| `f32`/`f64` | IEEE 754 floats | f64 default |
| `bool` | true/false | 1 byte |
| `char` | 4-byte Unicode scalar | not 1 byte |

Default integer is `i32`; default float is `f64`. Use suffixes to override:

```rust
let n = 42u8;
let pi = 3.14159f32;
```

Arithmetic overflow:
- Debug build: panics.
- Release build: wraps silently. Use `.checked_add`, `.saturating_add`, `.wrapping_add` for explicit behaviour.

---

## Strings

```rust
let s1: &str = "hello";              // string slice, immutable, &'static
let s2: String = String::from("hi"); // owned, growable
let s3 = "world".to_string();        // same
let s4 = format!("{} {}", s1, s2);   // formatted

s2.push_str(" there");               // append
let n = s1.len();                    // byte length (not char count)
let chars = s1.chars().count();      // char count
```

`&str` is the read-only slice; `String` is the owned, growable buffer. Function parameters should usually take `&str`, not `String`.

---

## Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b                    // last expression = return value
}

fn no_return() {             // unit type `()` returned
    println!("hi");
}

// Multiple return values via tuple.
fn min_max(v: &Vec<i32>) -> (i32, i32) {
    (*v.iter().min().unwrap(), *v.iter().max().unwrap())
}

// Generic function.
fn first<T>(v: &Vec<T>) -> &T {
    &v[0]
}

// Generic with trait bound.
fn print_all<T: std::fmt::Debug>(v: &Vec<T>) {
    for item in v { println!("{:?}", item); }
}
```

---

## Control flow

```rust
// if-else (expression, not statement)
let n = if condition { 1 } else { 2 };

// loop (infinite)
let result = loop {
    if done { break 42; }    // break with value
};

// while
while x > 0 { x -= 1; }

// for over range
for i in 0..10  { /* 0..9  */ }
for i in 0..=10 { /* 0..10 */ }

// for over iterator
for item in v.iter() { /* &T */ }
for item in v.iter_mut() { /* &mut T */ }
for item in v.into_iter() { /* T, consumes v */ }

// Index + value
for (i, item) in v.iter().enumerate() { /* ... */ }
```

---

## match

Exhaustive — must cover all cases. The compiler enforces it.

```rust
match value {
    0 => println!("zero"),
    1 | 2 => println!("one or two"),
    3..=5 => println!("three to five"),
    n if n < 0 => println!("negative: {n}"),
    _ => println!("anything else"),
}

// Destructure tuples
match (x, y) {
    (0, 0) => "origin",
    (x, 0) => "on x-axis",
    _ => "general",
}

// Destructure enums
match cell.cell_type {
    CellType::Empty => /* ... */,
    CellType::Sand | CellType::Gunpowder => /* falls */,
    CellType::Fire | CellType::OilFire => /* burns */,
    _ => /* other */,
}

// if let — match one variant
if let Some(value) = optional { /* use value */ }
if let CellType::Lava = cell.cell_type { /* lava-specific */ }

// let-else — destructure or diverge
let Some(value) = optional else { return; };
```

---

## Iterators

```rust
let v = vec![1, 2, 3, 4, 5];

// Transformations
v.iter().map(|x| x * 2);               // Iterator<Item=i32>
v.iter().filter(|x| **x > 2);          // Iterator<Item=&i32>
v.iter().enumerate();                  // Iterator<Item=(usize, &i32)>
v.iter().zip(other.iter());            // Iterator<Item=(&A, &B)>
v.iter().flat_map(|x| 0..*x);          // flatten nested iterators
v.iter().take(3);                      // first 3
v.iter().skip(2);                      // skip first 2

// Terminals (consume the iterator)
v.iter().sum::<i32>();                 // 15
v.iter().count();                      // 5
v.iter().max();                        // Some(&5)
v.iter().min();                        // Some(&1)
v.iter().any(|x| *x > 4);              // true
v.iter().all(|x| *x > 0);              // true
v.iter().find(|x| **x == 3);           // Some(&3)
v.iter().fold(0, |acc, x| acc + x);    // 15
v.iter().collect::<Vec<_>>();          // back into Vec
v.iter().for_each(|x| println!("{x}"));// side effects

// Mutable iteration
v.iter_mut().for_each(|x| *x *= 2);

// Sand-sim grid pattern
grid.iter_mut()
    .flat_map(|row| row.iter_mut())
    .filter(|c| !c.is_empty() && c.temperature > 20.0)
    .for_each(|c| c.temperature -= 0.5);
```

---

## Closures

```rust
let add = |a, b| a + b;                   // type inferred
let sq:  fn(i32) -> i32 = |x| x * x;      // explicit fn pointer

// Capturing
let multiplier = 10;
let times_ten = |x| x * multiplier;       // borrows multiplier

// Move closure — takes ownership
let s = String::from("hello");
let prints = move || println!("{}", s);   // owns s

// Storing closures
let mut callbacks: Vec<Box<dyn Fn(i32) -> i32>> = Vec::new();
callbacks.push(Box::new(|x| x + 1));
callbacks.push(Box::new(|x| x * 2));

// The three closure traits
//   Fn      - borrows captures immutably; can be called many times
//   FnMut   - borrows captures mutably; can mutate captures
//   FnOnce  - consumes captures; can be called only once
```

Inside an iterator chain that nests closures, `move` on the outer is usually needed:

```rust
let radius = 5i32;
let offsets = (-radius..=radius)
    .flat_map(move |dr| (-radius..=radius).map(move |dc| (dr, dc)));
```

---

## Enums

```rust
// Plain marker enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
    Wood,
    Fire,
    Oil,
    OilFire,
    Smoke,
    Steam,
    Acid,
    Lava,
    Ice,
    Gunpowder,
    Glass,
    Concrete,
    WetConcrete,
    Iron,
    Rust,
    Mud,
    Ash,
    Carbon,
}

// Methods on the enum
impl CellType {
    pub fn name(self) -> &'static str { match self { /* ... */ } }
    pub fn colour(self) -> Color { match self { /* ... */ } }
    pub fn density(self) -> u8 { match self { /* ... */ } }
}

// Enum with data — variants carry payloads
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
```

Two enums you'll use constantly:

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

Common methods:

```rust
opt.is_some();           // bool
opt.unwrap();            // panics if None
opt.unwrap_or(default);  // returns default if None
opt.map(|x| x * 2);      // transform inside
opt.and_then(|x| ...);   // chain Option-returning ops
opt.ok_or(error_value);  // Option → Result
opt.expect("message");   // unwrap with custom panic message

res.is_ok();
res.unwrap();
res.unwrap_or(default);
res.map(|x| ...);
res.map_err(|e| ...);
res.ok();                // Result → Option
res?;                    // early-return Err
```

---

## Structs

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub cell_type:   CellType,
    pub temperature: f32,
    pub lifetime:    u8,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Self { cell_type, temperature: 20.0, lifetime: 0 }
    }
    pub fn empty() -> Self { Self::new(CellType::Empty) }
    pub fn is_empty(&self) -> bool {
        matches!(self.cell_type, CellType::Empty)
    }
    pub fn heat(&mut self, delta: f32) {
        self.temperature = (self.temperature + delta).min(2000.0);
    }
}

// Construct
let c = Cell::new(CellType::Sand);
let c2 = Cell { cell_type: CellType::Water, ..Cell::empty() };  // struct update syntax
```

Tuple structs and unit structs:

```rust
struct Point(i32, i32);             // tuple struct
let p = Point(3, 4);
let Point(x, y) = p;                // destructure

struct Marker;                      // unit struct (no data)
```

---

## Common derives

| Trait | Why |
|---|---|
| `Debug` | `{:?}` formatting; `dbg!` macro |
| `Clone` | `.clone()` makes deep copies |
| `Copy` | implicit copy on assignment (only for cheap types) |
| `PartialEq, Eq` | `==`, `!=`, HashMap keys (Eq) |
| `Hash` | HashMap/HashSet keys |
| `PartialOrd, Ord` | `<`, `>`, `.sort()` |
| `Default` | `Default::default()` |
| `Serialize, Deserialize` | serde JSON/binary round-trip |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellType { /* ... */ }
```

`Copy` requires that *every* field also be `Copy`. `Option<Instant>` is `Copy`. `String` is not. `Vec<T>` is not.

---

## Vec, HashMap, HashSet

```rust
// Vec
let mut v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];
let v3 = vec![0; 100];       // 100 zeros

v.push(4);
v.pop();
v[0] = 99;
let first = v.first();       // Option<&T>
let len = v.len();
v.sort();
v.sort_by(|a, b| b.cmp(a));  // descending
v.contains(&5);

// HashMap
use std::collections::HashMap;

let mut m: HashMap<String, u32> = HashMap::new();
m.insert("apple".to_string(), 3);
m.get("apple");                 // Option<&u32>
m.contains_key("apple");
m.remove("apple");

// Entry API — atomic insert-or-modify
*m.entry("apple".to_string()).or_insert(0) += 1;

// Iterate
for (key, value) in &m { println!("{key}: {value}"); }

// HashSet
use std::collections::HashSet;
let mut s: HashSet<&str> = HashSet::new();
s.insert("hello");
s.contains("hello");
```

The sand-sim reactions table:

```rust
use std::sync::OnceLock;

static REACTIONS: OnceLock<HashMap<(CellType, CellType), ReactionOutcome>> = OnceLock::new();

fn reactions() -> &'static HashMap<(CellType, CellType), ReactionOutcome> {
    REACTIONS.get_or_init(build_reactions)
}

fn build_reactions() -> HashMap<(CellType, CellType), ReactionOutcome> {
    let mut r = HashMap::new();
    r.insert((CellType::Fire, CellType::Wood),
             ReactionOutcome::replace_both(CellType::Fire, CellType::Fire, 80.0));
    // ...
    r
}
```

---

## Modules and visibility

```rust
// src/main.rs
mod elements;        // loads src/elements.rs (or src/elements/mod.rs)
mod reactions;
mod simulation;

use elements::{Cell, CellType};
use reactions::react;
use simulation::step;
```

In each module file:

```rust
// src/elements.rs
pub const COLS: usize = 120;     // public constant
pub struct Cell { /* fields */ } // public type
pub fn helper() { /* ... */ }    // public function

fn private_helper() { /* ... */ } // module-private
```

Paths:
- `crate::elements::Cell` — absolute (this crate)
- `super::Cell`           — parent module
- `self::Cell`            — this module

Re-exports:
```rust
pub use elements::Cell;          // expose Cell at the parent's path
```

---

## Traits and generics

```rust
// Define a trait
pub trait ElementInfo {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn colour(&self) -> Color;

    // Default method body (can be overridden)
    fn label(&self) -> String {
        format!("{}", self.name())
    }
}

// Implement it
impl ElementInfo for Cell {
    fn name(&self)        -> &'static str { self.cell_type.name() }
    fn description(&self) -> &'static str { "" }
    fn colour(&self)      -> Color { self.cell_type.colour() }
}

// Generic function with trait bound
fn render<T: ElementInfo>(item: &T) {
    println!("{}", item.label());
}

// Multiple bounds
fn print_clone<T: std::fmt::Debug + Clone>(x: &T) {
    let copy = x.clone();
    println!("{:?}", copy);
}

// where clause for clarity with many bounds
fn complex<T, U>(t: T, u: U)
where
    T: ElementInfo + Clone,
    U: std::fmt::Debug,
{ /* ... */ }

// impl Trait (return position)
fn make_iter() -> impl Iterator<Item = i32> {
    (0..10).filter(|x| x % 2 == 0)
}

// Trait object — runtime polymorphism
let items: Vec<Box<dyn ElementInfo>> = vec![
    Box::new(thing_a),
    Box::new(thing_b),
];
```

---

## Error handling

```rust
use std::error::Error;
use std::fs;

fn read_config() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string("config.toml")?;
    Ok(contents)
}

// The `?` operator
//   On Result: returns Err early, unwraps Ok.
//   On Option: returns None early, unwraps Some.

// Custom error type (simple)
#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
    VersionMismatch(u32),
}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self { LoadError::Io(e) }
}

impl From<serde_json::Error> for LoadError {
    fn from(e: serde_json::Error) -> Self { LoadError::Json(e) }
}

// Now LoadError can be returned from any function that uses `?`
// on a Result<_, std::io::Error> or Result<_, serde_json::Error>.
```

For most projects, `Box<dyn Error>` is fine. Define a custom error type only when callers need to *distinguish* failures.

---

## Format strings

```rust
println!("{}", 42);                    // 42
println!("{:?}", v);                   // [1, 2, 3]   (Debug)
println!("{:#?}", v);                  // pretty-printed
println!("{:>10}", "hi");              // right-align, width 10
println!("{:<10}", "hi");              // left-align
println!("{:^10}", "hi");              // centred
println!("{:08.3}", 3.14);             // 0-padded, 3 decimals: "0003.140"
println!("{:.0}", 3.7);                // 4
println!("{n}", n = 5);                // named arg (or in-scope variable)

let name = "world";
println!("hello, {name}!");            // 2021+ implicit capture
```

---

## macroquad (the only graphics/audio crate)

```rust
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "sand-sim".to_owned(),
        window_width: 720,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_rectangle(10.0, 10.0, 30.0, 30.0, RED);
        draw_text("hello", 100.0, 100.0, 24.0, WHITE);
        draw_circle(200.0, 200.0, 20.0, BLUE);
        draw_line(0.0, 0.0, 100.0, 100.0, 2.0, GREEN);

        // Input
        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left)  { /* paint */ }
        if is_mouse_button_pressed(MouseButton::Right) { /* erase */ }
        if is_key_pressed(KeyCode::Space) { /* pause toggle */ }
        if is_key_down(KeyCode::Up)       { /* hold-trigger */ }
        let wheel = mouse_wheel().1.signum() as i32;   // -1, 0, or 1

        // Timing
        let dt = get_frame_time();      // seconds since last frame
        let fps = get_fps();
        let t = get_time();             // seconds since start

        next_frame().await;             // surrender the frame
    }
}
```

Common KeyCodes: `Key0..=Key9`, `A..=Z`, `Space`, `Enter`, `Tab`, `Escape`, `Up/Down/Left/Right`, `LeftShift`, `LeftControl`.

Audio (requires `features = ["audio"]` on macroquad):

```rust
use macroquad::audio::{Sound, load_sound, play_sound_once, play_sound, PlaySoundParams};

let sound: Sound = load_sound("assets/fire.wav").await.unwrap();
play_sound_once(&sound);
play_sound(&sound, PlaySoundParams { looped: true, volume: 0.5 });
```

Common colours: `BLACK`, `WHITE`, `RED`, `GREEN`, `BLUE`, `YELLOW`, `MAGENTA`, `GRAY`, `LIGHTGRAY`, `DARKGRAY`, `ORANGE`, `BROWN`, `PINK`. Custom: `Color::new(r, g, b, a)` — all `f32` in 0.0..1.0.

---

## fastrand

```rust
use fastrand;

fastrand::seed(42);                  // deterministic
let n = fastrand::i32(0..100);       // 0..99
let f = fastrand::f32();             // 0.0..1.0
let b = fastrand::bool();
let pick = fastrand::choice(&v);     // Option<&T>
```

The sand-sim probabilistic-spread pattern:

```rust
if fastrand::f32() < 0.05 {
    // 5% chance per frame
}
```

---

## serde + serde_json

```rust
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct SaveState {
    pub version: u32,
    pub grid: Vec<Vec<Cell>>,
    #[serde(skip)]
    pub runtime_only: Option<std::time::Instant>,
}

// Save
let s = serde_json::to_string_pretty(&state)?;
fs::write("save.json", s)?;

// Load
let s = fs::read_to_string("save.json")?;
let state: SaveState = serde_json::from_str(&s)?;
```

Attributes:
- `#[serde(skip)]` — never serialise this field.
- `#[serde(default)]` — on deserialise, missing field uses `Default::default()`.
- `#[serde(rename = "name")]` — different name in JSON.
- `#[serde(tag = "type")]` on enums — tagged JSON.

---

## std::time

```rust
use std::time::{Instant, Duration};

let start = Instant::now();
// ... do work ...
let elapsed: Duration = start.elapsed();

if elapsed >= Duration::from_secs(30) { /* timeout */ }
if elapsed > Duration::from_millis(500) { /* half a second */ }

// Compare to durations directly
let cure_time = Duration::from_secs(30);
if start.elapsed() >= cure_time { /* cured */ }
```

`Instant` is monotonic — never goes backwards. Don't serialise it (it's process-local).

---

## OnceLock and lazy statics

```rust
use std::sync::OnceLock;

static CONFIG: OnceLock<HashMap<String, String>> = OnceLock::new();

fn config() -> &'static HashMap<String, String> {
    CONFIG.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("k".into(), "v".into());
        m
    })
}
```

Available since Rust 1.70. Replaces the older `lazy_static!` and `once_cell` patterns.

---

## Ownership and borrowing rules

The three rules:

1. **Each value has exactly one owner.**
2. **You can have any number of `&T` (shared, immutable) references *or* exactly one `&mut T` (exclusive, mutable). Not both.**
3. **References must never outlive what they point to.**

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;          // OK: many shared borrows
// let m = &mut s;    // ERROR: cannot borrow mut while shared borrows exist

println!("{r1} {r2}");
let m = &mut String::from("new");   // OK now — r1, r2 unused

// Move semantics
let a = String::from("hi");
let b = a;            // a moved into b
// println!("{a}");   // ERROR: a is no longer valid
let c = b.clone();    // explicit deep copy

// Copy types (small, cheap, stack-only) copy implicitly
let x = 5;
let y = x;            // i32 is Copy
println!("{x} {y}");  // both still valid

// Function parameters
fn takes_ownership(s: String) { /* drops s at end */ }
fn borrows(s: &str) { /* s lives where it was */ }
fn borrows_mut(s: &mut String) { s.push('!'); }
```

---

## Common patterns

### Sand-sim grid update

```rust
const ROWS: usize = 80;
const COLS: usize = 120;

let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::empty(); COLS]; ROWS];

// Three passes per frame
fn step(grid: &mut Vec<Vec<Cell>>) {
    reaction_pass(grid);
    rising_pass(grid);
    falling_pass(grid);
}

// Top-to-bottom for rising things
for row in 0..ROWS {
    for col in 0..COLS {
        if matches!(grid[row][col].cell_type, CellType::Smoke | CellType::Steam) {
            update_rising(grid, row, col);
        }
    }
}

// Bottom-to-top for falling things
for row in (0..ROWS).rev() {
    for col in 0..COLS {
        if matches!(grid[row][col].cell_type, CellType::Sand | CellType::Water) {
            update_falling(grid, row, col);
        }
    }
}
```

### Neighbour iteration

```rust
const NEIGHBOURS_4: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const NEIGHBOURS_8: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

for (dr, dc) in NEIGHBOURS_4 {
    let nr = row as i32 + dr;
    let nc = col as i32 + dc;
    if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32 { continue; }
    let (nr, nc) = (nr as usize, nc as usize);
    // ... use grid[nr][nc] ...
}
```

### `matches!` macro

```rust
if matches!(cell.cell_type, CellType::Water | CellType::Acid | CellType::Oil) {
    // is a liquid
}
```

Equivalent to `cell.cell_type == Water || cell.cell_type == Acid || ...` but shorter and works on enums-with-data without writing field names.

---

## Linux (Ubuntu) notes

A consolidated cheat sheet of the Linux-specific tips scattered through the course.

### Tooling

```bash
# Install Rust the recommended way (not via apt)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Update
rustup update stable

# Toolchain components
rustup component add rustfmt clippy

# Editor support
sudo snap install code --classic        # VS Code
# Then install the rust-analyzer extension from inside VS Code.
```

### Build dependencies

```bash
# For macroquad audio
sudo apt update
sudo apt install -y libasound2-dev pkg-config

# For some opengl-touching projects (macroquad uses ES; usually already present)
sudo apt install -y mesa-utils libgl1-mesa-dev
```

### Audio backend

```bash
# Check PipeWire is running (Ubuntu 22.10+)
systemctl --user status pipewire pipewire-pulse

# Lower-latency for tight game audio
export PIPEWIRE_LATENCY=128/48000

# Check audio outputs
pactl list short sinks

# Test a wav plays at all
aplay assets/fire.wav
```

### Profiling

```bash
# CPU profile
cargo build --release
perf record --call-graph=dwarf ./target/release/sand-sim
perf report

# Memory profile
valgrind --tool=massif ./target/release/sand-sim
ms_print massif.out.NNN
```

### Distribution

```bash
# Check shared library dependencies
ldd target/release/sand-sim

# Strip debug symbols (smaller binary)
strip target/release/sand-sim

# Build a .deb package
cargo install cargo-deb
cargo deb

# Cross-compile for static binary (advanced)
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### CI on Ubuntu (GitHub Actions)

```yaml
# .github/workflows/ci.yml
on: [push]
jobs:
  ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: sudo apt update && sudo apt install -y libasound2-dev pkg-config
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test
      - run: cargo build --release
```

### File-manager file-type association

To make `.rs` files open in VS Code on Ubuntu:

```bash
xdg-mime default code.desktop text/rust
```

(or right-click any `.rs` file in Files → Properties → Open with → VS Code → Set as default.)

### Wayland vs X11

macroquad (via miniquad) uses X11 directly, so on Wayland it runs through **XWayland**, which is installed by default on every mainstream Ubuntu desktop. Confirm with:

```bash
echo $XDG_SESSION_TYPE      # 'wayland' or 'x11'
pgrep Xwayland              # should print a PID on Wayland sessions
```

If your sim has cursor/scroll/HiDPI issues that you suspect are XWayland-specific, log out and log back in on an X11 session (gear icon on the GDM login screen → "Ubuntu on Xorg"). There's no `SDL_VIDEODRIVER`-style runtime switch — miniquad doesn't use SDL.

---

## Compiler error decoder

Quick map of common error codes to "what you probably forgot."

| Code | Likely cause |
|---|---|
| `E0277` | Trait not implemented (e.g. `Serialize` missing, `Hash` missing) |
| `E0382` | Used a value after move |
| `E0432` | Unresolved import — typo or missing `pub` |
| `E0499` | Two mutable borrows at once |
| `E0502` | Mixed mutable + immutable borrow |
| `E0507` | Cannot move out of borrowed content (use `.clone()` or `.copied()`) |
| `E0596` | Tried to mutate behind a shared reference |
| `E0603` | Item is private (add `pub`) |

Run `rustc --explain E0277` for the official long-form explanation of any error code.

---

## Quick links inside this repo

- [GLOSSARY](../GLOSSARY.md) — plain-English definitions
- [Compiler errors](compiler-errors.md) — extended decoder table
- [SETUP](../SETUP.md) — Rust toolchain install per OS
- Session list: [Month 1](../month-1/README.md), [Month 2](../month-2/README.md), [Month 3](../month-3/README.md)
