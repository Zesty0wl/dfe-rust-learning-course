# Rust Cheat Sheet

A compact quick reference covering everything used in this course. Keep this open in another tab.

---

## Cargo

```bash
cargo new my_project          # new binary project
cargo new my_lib --lib        # new library
cargo run                     # build + run
cargo run --release           # optimised build
cargo run -- --foo bar        # everything after `--` goes to your program
cargo build                   # build only
cargo check                   # type-check, no codegen (fast)
cargo test                    # run tests
cargo clippy                  # extra lints
cargo fmt                     # format the code
cargo add some_crate          # add a dependency
cargo doc --open              # build + open docs for your project + deps
```

## Variables

```rust
let x = 5;                    // immutable
let mut y = 5;                // mutable
let z: i32 = 5;               // explicit type
let pi = 3.14159_f64;         // type suffix
const MAX: u32 = 1000;        // compile-time constant
let x = "hello";              // shadow: rebind same name with new type
```

## Scalar types

| Type | Description | Example |
|---|---|---|
| `i8` `i16` `i32` `i64` `i128` `isize` | Signed integers | `let n: i64 = -42;` |
| `u8` `u16` `u32` `u64` `u128` `usize` | Unsigned integers | `let n: u64 = 42;` |
| `f32` `f64` | Floats | `let pi: f64 = 3.14;` |
| `bool` | Boolean | `let b = true;` |
| `char` | A single Unicode scalar | `let c = '♪';` |

## Strings

```rust
let s: &str = "hello";              // string slice (borrowed)
let owned: String = String::from("hello");
let owned: String = "hello".to_string();
let combined = format!("{} world", owned);
let parts: Vec<&str> = "a,b,c".split(',').collect();
```

## Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b                            // last expression returned (no semicolon)
}

fn shout(msg: &str) {                // no return = returns ()
    println!("{}!", msg.to_uppercase());
}
```

## Control flow

```rust
if x > 0 { ... } else if x == 0 { ... } else { ... }

for i in 0..10 { ... }               // 0..=10 includes 10
while x < 100 { ... }
loop { if done { break; } }
let result = loop { break 42; };     // loops can return values
```

## `match`

```rust
match note % 12 {
    0 => "C",
    1 => "C#",
    2..=4 => "low",                  // range
    n if n > 9 => "high",            // guard
    _ => "other",                    // catch-all
}
```

## Enums

```rust
enum ScaleType {
    Major,
    Minor,
    Pentatonic,
}

enum Tile {
    Ocean { depth: u8 },
    Mountain(u16),
    Plains,
}
```

## Structs

```rust
struct Block {
    kind: u8,
    x: i32,
    y: i32,
}

impl Block {
    fn new(kind: u8) -> Self { Block { kind, x: 0, y: 0 } }
    fn is_solid(&self) -> bool { self.kind != 0 }
    fn move_to(&mut self, x: i32, y: i32) { self.x = x; self.y = y; }
}
```

## `Option<T>` and `Result<T, E>`

```rust
let n: Option<i32> = Some(5);
let m: Option<i32> = None;
match n { Some(v) => println!("{v}"), None => println!("nothing") }
let v = n.unwrap_or(0);

fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;         // ? early-returns the Err
    Ok(n * 2)
}
```

## Collections

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
v.push(4);
let first = v[0];
for x in &v { println!("{x}"); }

use std::collections::HashMap;
let mut h: HashMap<String, i32> = HashMap::new();
h.insert("a".to_string(), 1);
let one = h.get("a");                // returns Option<&i32>
```

## Iterators

```rust
let evens: Vec<i32> = (1..=10).filter(|n| n % 2 == 0).collect();
let sum: i32 = evens.iter().sum();
let doubled: Vec<i32> = evens.iter().map(|n| n * 2).collect();

for (i, x) in v.iter().enumerate() { ... }
let pairs: Vec<_> = (1..).zip(["a","b","c"]).collect();
```

## Closures

```rust
let add = |a, b| a + b;
let n = 10;
let plus_n = move |x| x + n;        // moves n into closure
```

## Traits

```rust
use std::fmt;

trait Describable { fn describe(&self) -> String; }

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block({})", self.kind)
    }
}
```

## Generics

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { ... }
struct Pair<A, B> { first: A, second: B }
fn print_anything(x: impl fmt::Display) { println!("{x}"); }
fn dyn_iter() -> Box<dyn Iterator<Item = i32>> { Box::new(0..10) }
```

## Modules

```rust
// in src/main.rs
mod notes;            // imports src/notes.rs
mod scales;
use notes::Note;

// in src/notes.rs
pub struct Note { pub name: String }
pub fn from_midi(n: u8) -> Note { ... }
```

## Error handling pattern

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string("data.txt")?;
    let n: i32 = f.trim().parse()?;
    println!("{n}");
    Ok(())
}
```

## Common derives

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pitch { ... }
```

## Print/format

```rust
println!("Hello, {}!", name);
println!("x = {x}, y = {y}");        // since Rust 2021
println!("{:.3}", 3.14159);          // 3.142
println!("{:5}", 42);                // pad to width 5
println!("{:?}", thing);             // Debug format
println!("{:#?}", thing);            // pretty Debug
eprintln!("error: {}", msg);         // to stderr
```
