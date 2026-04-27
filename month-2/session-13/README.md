# Session 13: Error Handling

## What You'll Learn

How Rust handles things going wrong without exceptions. The `Result<T, E>` type, the `?` operator that makes error propagation painless, custom error enums, and the trick of returning `Result` from `main()` itself.

## The Big Idea

Other languages mostly use **exceptions** — code throws, the runtime unwinds the stack, somewhere a `try`/`catch` catches it (or the program crashes). Rust takes a different approach: **errors are values**, returned from functions like any other value.

```rust
enum Result<T, E> {
    Ok(T),    // success, here's the value
    Err(E),   // failure, here's the error
}
```

Functions that can fail return `Result<T, E>`. The caller is forced to handle both cases — exactly like `Option<T>` from Session 10. There's no hidden control flow; you can read a function and see exactly what can go wrong.

The downside: matching every `Result` everywhere would be tedious. So Rust has the **`?` operator**: a one-character shortcut that means "if this is `Ok`, unwrap it; if it's `Err`, return it from the enclosing function". This makes error handling *concise* while still being explicit.

## Concepts Covered

- `Result<T, E>` — the enum
- `.unwrap()` vs `.expect()` vs `match` vs `?`
- The `?` operator and how it propagates errors up
- `eprintln!` for printing to stderr
- `std::process::exit(code)`
- Custom error types (an enum)
- Returning `Result` from `main()` (gets you `?` support in `main`)

## Building Towards `world-generator`

The world generator parses a `--seed` value from the command line. If the user types `--seed banana`, that's a parse error — `Result::Err`. Today you'll build the seed-parser and learn the right way to report bad input. In Session 15 you'll plug it into the project.

---

## Step-by-Step Walkthrough

### 1. The simplest version: `.unwrap()`

`examples/parse_seed/src/main.rs`:

```rust
fn main() {
    let s = "42";
    let n: u64 = s.parse().unwrap();
    println!("Parsed: {}", n);
}
```

`.parse()` returns `Result<u64, ParseIntError>`. `.unwrap()` says "I'm sure it's `Ok` — give me the value or panic". Fine for prototyping. Terrible for production. If `s` were `"banana"`, this crashes the whole program with a stack trace.

### 2. Better: `match`

```rust
let s = "banana";
match s.parse::<u64>() {
    Ok(n)  => println!("Parsed: {}", n),
    Err(e) => eprintln!("Parse error: {}", e),
}
```

`eprintln!` prints to **stderr** instead of stdout. Errors and progress messages go to stderr by convention; actual program output goes to stdout. This matters when users pipe your output into other programs.

### 3. Define your own error type

For most programs you don't want raw `ParseIntError` leaking everywhere. Define a clean enum:

```rust
#[derive(Debug)]
enum SeedError {
    Empty,
    NotANumber(String),
    OutOfRange(u64),
}
```

Then write a parser that uses it:

```rust
fn parse_seed(s: &str) -> Result<u64, SeedError> {
    if s.is_empty() {
        return Err(SeedError::Empty);
    }
    let n: u64 = s
        .parse()
        .map_err(|_| SeedError::NotANumber(s.to_string()))?;
    if n > 1_000_000 {
        return Err(SeedError::OutOfRange(n));
    }
    Ok(n)
}
```

Two new things:

- **`.map_err(|_| ...)`** — if the inner result is `Err`, transform the error. We don't care about the original `ParseIntError` (the `_`), we just produce our own `SeedError::NotANumber`.
- **`?`** — if `Result` is `Ok(value)`, give me `value`; otherwise return the `Err` from `parse_seed` *immediately*. It's the punchline of Rust error handling.

### 4. `Result` from `main`

The classic main signature is `fn main() {}`. But it can also be:

```rust
fn main() -> Result<(), SeedError> {
    let args: Vec<String> = std::env::args().collect();
    let seed = parse_seed(args.get(1).map(|s| s.as_str()).unwrap_or(""))?;
    println!("Seed: {}", seed);
    Ok(())
}
```

`Result<(), SeedError>` means "no real value on success, a `SeedError` on failure". `Ok(())` is "success with no payload". `()` is the "unit" type — Rust's name for "nothing".

When `main` returns `Err(...)`, Rust prints the error in `Debug` format and exits with a non-zero status code. Useful, but for polished CLIs you usually want to print your own friendly message and exit explicitly with `std::process::exit(1)` (see Step 5).

### 5. Friendlier error reporting

```rust
fn run() -> Result<(), SeedError> {
    let args: Vec<String> = std::env::args().collect();
    let raw = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let seed = parse_seed(raw)?;
    println!("Seed parsed: {}", seed);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        match e {
            SeedError::Empty            => eprintln!("Error: please pass a seed, e.g. --seed 42"),
            SeedError::NotANumber(s)    => eprintln!("Error: '{}' is not a number", s),
            SeedError::OutOfRange(n)    => eprintln!("Error: seed {} is too big (max 1,000,000)", n),
        }
        std::process::exit(1);
    }
}
```

This is the production pattern: a `run()` that returns `Result`, a thin `main()` that turns errors into nice user-facing messages and a non-zero exit code. Shells and other programs can detect failures because of the exit code.

---

## Common Mistakes

- **Using `?` in a function that doesn't return `Result`** — the compiler will complain. Either return `Result`, or handle the error inline with `match`.
- **`unwrap()` on user input** — guarantees a future panic. Always parse user input with proper error handling.
- **Forgetting `.map_err`** — if the inner `Err` type is different from your function's `Err` type, `?` won't work directly. Either implement `From` (advanced) or use `.map_err(|e| ...)`.
- **Mixing `eprintln!` and `println!` at random** — pick a convention. Errors → stderr; data → stdout. Tools that pipe your output rely on this.

---

## Session Challenge

Add a `--width N --height N` parser. Both must be positive integers between 5 and 200. Define `WorldArgsError` covering missing values, non-numeric values, and out-of-range values. Wire it into `run()`. Write at least three test invocations and predict what each will print before running them:

```text
$ cargo run -- --seed 42 --width 30 --height 10
$ cargo run -- --seed banana --width 30 --height 10
$ cargo run -- --seed 42 --width 3 --height 10
```

---

## Quick Reference

```rust
fn might_fail(x: i32) -> Result<i32, String> {
    if x < 0 { Err(format!("negative: {}", x)) }
    else     { Ok(x * 2) }
}

// .unwrap() — panic on Err
let n = might_fail(3).unwrap();          // 6

// .expect() — panic with custom message
let n = might_fail(3).expect("expected non-negative");

// match — handle both
match might_fail(3) {
    Ok(n)  => println!("got {}", n),
    Err(e) => println!("err: {}", e),
}

// .unwrap_or — default on Err
let n = might_fail(-1).unwrap_or(0);     // 0

// ? — propagate
fn double_then(x: i32) -> Result<i32, String> {
    let y = might_fail(x)?;              // returns Err if x<0
    Ok(y + 1)
}
```

---

## DofE Log Reminder

Row 13. You're 13/24 — over halfway through.
