# Session 9: Structs and Methods

## What You'll Learn

How to bundle related data into a single named type — a `struct` — and attach behaviour to it with `impl` blocks. This is the moment where Rust starts to feel like it's modelling *real things*, not just shuffling numbers.

## The Big Idea

Up until now you've used Rust's built-in types — numbers, strings, booleans. That's fine for arithmetic, but try modelling a Minecraft block with just integers:

```rust
let block_type = 1;
let x = 10;
let y = 64;
let z = 200;
let hardness = 1.5;
```

Five separate variables, no relationship between them, and `block_type = 1` could mean anything. A **struct** lets you say *"these things belong together, and here's what they're called"*:

```rust
struct Block {
    kind: u8,
    x: i32,
    y: i32,
    z: i32,
    hardness: f32,
}
```

Now `Block` is a **type**, like `i32` or `String` — except *you* defined it. Once you've got the type, you can attach **methods** to it with `impl`:

```rust
impl Block {
    fn is_solid(&self) -> bool {
        self.kind != 0
    }
}
```

`impl Block { ... }` literally means "implementations for `Block`". Inside it, `fn` definitions are methods, callable as `block.is_solid()`.

## Concepts Covered

- `struct` definition with named fields
- Creating instances with `BlockType { field: value }` syntax
- Field access with `.`
- `impl` blocks — methods and associated functions
- `&self`, `&mut self` (and `self`)
- Associated functions — constructors like `Block::new(...)`
- `#[derive(Debug)]` — get a free `{:?}` printer
- Field shorthand: if a variable's name matches the field name, just write the name once

## Building Towards `world-generator`

In Sessions 15 and 16 you'll define a `World` struct with a 2D grid of `Tile` values. The `World` will have methods like `World::generate(seed, width, height)` and `world.render()`. Today you build the foundations on a smaller example: a `Block`.

---

## Step-by-Step Walkthrough

### 1. Define a Block

`examples/block_struct/src/main.rs`:

```rust
#[derive(Debug)]
struct Block {
    kind: u8,
    x: i32,
    y: i32,
    z: i32,
    hardness: f32,
}
```

`#[derive(Debug)]` is an **attribute** — a tiny instruction to the compiler. This one says "auto-implement the `Debug` trait for me", which lets us print the struct with `{:?}`.

### 2. Create one and inspect it

```rust
fn main() {
    let stone = Block {
        kind: 1,
        x: 10,
        y: 64,
        z: 200,
        hardness: 1.5,
    };

    println!("{:?}", stone);
    println!("Stone is at y={}", stone.y);
}
```

Notice you access fields with `.` exactly like properties in any other language. The `{:?}` formatter prints the whole struct in debug form — `{:#?}` does the same but pretty-printed across lines.

### 3. Add an `impl` block

This is where Rust differs from a lot of object-oriented languages. The struct definition only contains data. Behaviour lives in a separate `impl Block { ... }` block:

```rust
impl Block {
    fn new(kind: u8, x: i32, y: i32, z: i32) -> Self {
        let hardness = match kind {
            0 => 0.0,
            1 => 1.5,
            2 => 2.0,
            _ => 0.5,
        };
        Self { kind, x, y, z, hardness }
    }

    fn is_solid(&self) -> bool {
        self.kind != 0
    }

    fn display_char(&self) -> char {
        match self.kind {
            0 => ' ',
            1 => '#',
            2 => '%',
            _ => '?',
        }
    }
}
```

Three things to notice:

- **`Self` (capital S)** — inside an `impl` block, `Self` is shorthand for the type being implemented. `Self { kind, x, y, z, hardness }` is the same as writing `Block { ... }`. It's a bit of typing saved and it makes refactoring easier.
- **`fn new` has no `self` parameter** — that makes it an **associated function**, not a method. You call it with `Block::new(...)` (double colon), not `block.new(...)`. This is Rust's convention for constructors.
- **`fn is_solid(&self)` *does* have `&self`** — that's an immutable borrow of the instance. Now you call it with method syntax: `stone.is_solid()`.

### 4. Use them

```rust
fn main() {
    let stone = Block::new(1, 10, 64, 200);
    let air   = Block::new(0,  0,  0,   0);

    println!("Stone is solid? {}", stone.is_solid()); // true
    println!("Air is solid?   {}", air.is_solid());   // false
    println!("Stone renders as: {}", stone.display_char());
}
```

### 5. Field shorthand

Inside `Block::new` we wrote `Self { kind, x, y, z, hardness }`. If a variable's name matches the field name, you don't need `kind: kind`. Cleaner code.

### 6. `&self` vs `&mut self` vs `self`

Three flavours of `self` parameter — pick based on what you're doing:

- `&self` — read the data, don't change it. Like a `const` reference in C++. Most methods.
- `&mut self` — change the data. Required if you assign to fields or call other `&mut self` methods.
- `self` — *consume* the instance. Caller can't use it afterwards. Rare; used for builder patterns and conversions.

The compiler will tell you exactly which one you need — if you write `&self` and try to mutate a field, the error is precise.

```rust
impl Block {
    fn break_it(&mut self) {
        self.kind = 0;       // now it's air
        self.hardness = 0.0;
    }
}
```

To call `break_it`, the variable itself must be `mut`:

```rust
let mut stone = Block::new(1, 10, 64, 200);
stone.break_it();
```

---

## Common Mistakes

- **Forgetting `Self` capital** — `self` (lowercase) is the instance, `Self` (uppercase) is the type.
- **Calling `Block::new` with dot syntax** — it has no `self`, so `block.new(...)` doesn't work; use `Block::new(...)`.
- **Calling a `&mut self` method on a non-mut variable** — `let block = ...; block.break_it();` fails. Make it `let mut block = ...;`.
- **Forgetting `#[derive(Debug)]`** — then `println!("{:?}", block)` errors. The fix is one line above the struct.

---

## Session Challenge

Add a `Player` struct with `name: String`, `health: u32`, `position: (i32, i32, i32)`, and `inventory: Vec<u8>` (a list of block kinds the player is carrying).

Implement:

- `Player::new(name: &str)` — starts with 20 health, position `(0, 64, 0)`, empty inventory
- `take_damage(&mut self, amount: u32)` — subtracts from health, doesn't go below 0
- `is_alive(&self) -> bool`
- `pick_up(&mut self, kind: u8)` — pushes onto inventory
- `summary(&self) -> String` — returns something like `"Steve: 18hp at (0, 64, 0), carrying 3 blocks"`

(Quick taste of `String` and `Vec` — you'll see both in detail in Session 11.)

---

## Quick Reference

```rust
struct Foo {
    a: i32,
    b: String,
}

impl Foo {
    fn new(a: i32, b: String) -> Self {       // associated function
        Self { a, b }
    }

    fn doubled_a(&self) -> i32 {              // immutable method
        self.a * 2
    }

    fn add_to_b(&mut self, s: &str) {         // mutable method
        self.b.push_str(s);
    }
}

let f = Foo::new(3, String::from("hi"));
println!("{}", f.doubled_a());                // 6
```

---

## DofE Log Reminder

Open [`dfe/session-log.md`](../../dfe/session-log.md), find row **9**, fill in date, time, what you built, and what you learned. Don't put it off — it takes 60 seconds and your assessor needs it.
