# Session 14: Traits

## What You'll Learn

Traits are Rust's answer to interfaces / abstract base classes — a way to say "any type that can do X". Once you grasp traits, the standard library starts to make sense (everything is built on them) and your own code becomes vastly more reusable.

## The Big Idea

A `trait` is a list of method signatures. Any type that **implements** that trait promises to provide those methods. Then any function can ask for "a thing that implements Trait T", no matter what concrete type the caller chose.

```rust
trait Describable {
    fn describe(&self) -> String;
}
```

Anything that implements `Describable` can be `.describe()`d. A `Tile`. A `Player`. A `Block`. The trait *defines an ability*; the `impl` *grants that ability* to a specific type.

The standard library is built on traits:
- `Display` — controls how `{}` formats your type for users
- `Debug` — controls how `{:?}` formats it for developers
- `Clone` — `.clone()` on your type
- `Copy` — your type is so simple it can be implicitly copied (like `i32`)
- `PartialEq` — `==` works on your type
- `Iterator` — your type is an iterator
- `From` / `Into` — convert one type to another

You've already used most of these via `#[derive(...)]`. Today you'll write your own.

## Concepts Covered

- `trait` definition with method signatures
- `impl Trait for Type { ... }` blocks
- Default method bodies
- `Display` and the `std::fmt::Formatter` machinery (the one tricky bit)
- Trait bounds in function signatures: `fn show<T: Display>(x: T) { ... }`
- `#[derive]` for `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`

## Building Towards `world-generator`

Session 15 will need `Tile` to be `Clone` (so the grid can store independent copies) and to implement `Display` (so `print!("{}", tile)` works). Today you write exactly that pattern on a small example.

---

## Step-by-Step Walkthrough

### 1. Define a trait

`examples/describable/src/main.rs`:

```rust
trait Describable {
    fn describe(&self) -> String;

    // A default method body — implementors can override or use this
    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}
```

`describe` has no body — implementors *must* provide one. `shout` has a default body — implementors *may* override.

### 2. A type to implement it on

```rust
#[derive(Debug, Clone)]
enum Tile {
    Ocean { depth: u8 },
    Plains,
    Mountain { height: u16 },
}
```

`#[derive(Debug, Clone)]` tells the compiler "implement `Debug` and `Clone` for me automatically using sensible defaults". Both are real trait implementations behind the scenes.

### 3. Implement Describable

```rust
impl Describable for Tile {
    fn describe(&self) -> String {
        match self {
            Tile::Ocean { depth }     => format!("ocean ({}m deep)", depth),
            Tile::Plains              => String::from("plains"),
            Tile::Mountain { height } => format!("mountain ({}m tall)", height),
        }
    }
    // We don't override shout(); we get the default for free.
}
```

Now any `Tile` can `.describe()` and `.shout()`.

### 4. The `Display` trait — for `{}` formatting

`Display` is in `std::fmt`. It looks like this:

```rust
use std::fmt;

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Tile::Ocean { .. }    => '~',
            Tile::Plains          => '▒',
            Tile::Mountain { .. } => '▲',
        };
        write!(f, "{}", symbol)
    }
}
```

Three things to notice:

- The trait name is **`fmt::Display`** (`Display` in the `fmt` module).
- The required method is `fn fmt`, taking a **`&mut Formatter`**. Formatter is what `println!` and friends pass in behind the scenes.
- Instead of returning a `String`, you `write!` into the formatter — same syntax as `println!` but it writes to `f` instead of stdout. Returns `fmt::Result`, conventionally just `write!(...)`.

Now you can do:

```rust
let m = Tile::Mountain { height: 1500 };
println!("{}", m);   // ▲
```

### 5. Trait bounds on functions

Once a trait exists, you can write generic functions that work for *any* type that implements it:

```rust
fn announce<T: Describable>(thing: &T) {
    println!(">> {}", thing.describe());
}
```

`<T: Describable>` reads "a generic type T that implements Describable". Any `T` works as long as the trait is implemented for it.

You can stack bounds: `<T: Describable + Clone>`. You can use `where` clauses for readability:

```rust
fn announce<T>(thing: &T) where T: Describable + Clone { ... }
```

### 6. Why derive matters

`#[derive(Clone)]` saved you from writing this:

```rust
impl Clone for Tile {
    fn clone(&self) -> Self {
        match self {
            Tile::Ocean { depth }     => Tile::Ocean { depth: *depth },
            Tile::Plains              => Tile::Plains,
            Tile::Mountain { height } => Tile::Mountain { height: *height },
        }
    }
}
```

Derive does that automatically when every field is itself `Clone`. The same goes for `Debug`, `PartialEq`, `Hash`, etc. Embrace derive — it's how production Rust code works.

`Copy` is special: it means "this type is so cheap to duplicate that the compiler can do it implicitly". Numeric types, `bool`, `char` are `Copy`. Things that own heap data (`String`, `Vec`) cannot be `Copy`. Whether to derive `Copy` for your own enum/struct is a design call — if all fields are `Copy` and the type is "small", consider it.

---

## Common Mistakes

- **Forgetting to bring the trait into scope** — `use std::fmt::Display;` (or use `fmt::Display`) before you can refer to it.
- **`println!("{}", x)` errors with "trait `Display` not implemented for X`** — derive `Debug` and use `{:?}` while developing; implement `Display` for end-user output.
- **Confusing `Display` and `Debug`** — `Display` is for users (the `Tile` shows as `~`), `Debug` is for developers (the `Tile` shows as `Ocean { depth: 8 }`). Both are valid; both serve different audiences.
- **Trying to derive `Eq` on a struct with floats** — `f32`/`f64` are only `PartialEq`, not `Eq` (because of NaN). Either use `PartialEq` only, or change the field type.

---

## Session Challenge

Add a second trait, `Symbol`, with one method `fn symbol(&self) -> char`. Implement it on `Tile`. Then write a generic function `print_grid<T: Symbol>(grid: &Vec<Vec<T>>)` that prints any 2D grid as long as the cells implement `Symbol`. Test it on `Vec<Vec<Tile>>`.

Bonus: implement `Symbol` for `char` itself (so any `Vec<Vec<char>>` works too — think Minecraft text input).

---

## Quick Reference

```rust
trait Greet {
    fn greet(&self) -> String;
    fn shout(&self) -> String { self.greet().to_uppercase() }     // default
}

struct Dog;
impl Greet for Dog {
    fn greet(&self) -> String { String::from("woof") }
}

let d = Dog;
println!("{}", d.greet());   // woof
println!("{}", d.shout());   // WOOF (default)

// Display
use std::fmt;
struct Pt(i32, i32);
impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Trait bounds
fn show<T: fmt::Display>(x: T) { println!("{}", x); }

// Most-used derives
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord { x: i32, y: i32 }
```

---

## DofE Log Reminder

Row 14. After Sessions 15 and 16 you'll write the second milestone reflection.
