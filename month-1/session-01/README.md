# Session 1: Why Rust? History, Setup, and the Speed Demo

## What You'll Learn

Why Rust exists, why it's worth your time, and how to get a "Hello, World!" running on your own machine. By the end of this session you'll have written and run two Rust programs and watched Rust beat Python by roughly a hundred to one in a fair race.

## The Big Idea

Programming languages have generations. **C** is the grandfather — blazing fast but dangerous; one wrong move and your program crashes or gets hacked. **Python** is the friendly modern option — easy to read, but slow and lets you do all sorts of silly things. **Rust** is the breakthrough: it's *as fast as C*, but it has a system called the **borrow checker** that prevents whole categories of bugs at compile time. Rust literally won't let you write certain classes of broken code.

> "Other languages let you make mistakes. Rust makes the compiler your co-pilot."

It has been voted the **most loved programming language** on Stack Overflow's annual developer survey for **nine years running**. It's used in the Linux kernel, parts of the Windows kernel, Firefox, Cloudflare, AWS, Discord (who famously rewrote performance-critical systems from Go to Rust), and the entire WebAssembly ecosystem. That's not a hobby language. That's serious infrastructure.

## Concepts Covered

- The Rust ecosystem: `rustc`, `cargo`, `rustup`
- `cargo new` to scaffold a project
- The structure of a tiny Rust program (`fn main`, `println!`)
- `cargo run` to build and run
- Why compiled languages are dramatically faster than interpreted ones

## Building Towards `music-theory-cli`

Today is mostly motivation and setup, but everything you do later in Month 1 begins with `cargo new` and a `fn main`. By the end of this session you'll have done both at least three times, so the rhythm becomes second nature.

---

## Step-by-Step Walkthrough

### 1. The speed demo: Pi by random sampling

The mathematician's trick: imagine a square 1 unit on each side, with a quarter-circle of radius 1 fitted inside one corner. The square has area `1`. The quarter-circle has area `π/4`. If you fire random points uniformly into the square and ask "what fraction land inside the quarter-circle?", the answer (eventually) approaches `π/4`. Multiply by 4 and you get π.

> **The maths in one line:** points where `x² + y² ≤ 1` are inside the quarter-circle. So `π ≈ 4 × (points inside) / (total points)`.

We're going to estimate Pi this way using **100 million samples**, in both Python and Rust. Run them both. Time them. Compare.

#### The Python version (`examples/pi_python.py`)

```python
import random
import time

def estimate_pi(n):
    inside = 0
    for _ in range(n):
        x = random.random()
        y = random.random()
        if x * x + y * y <= 1.0:
            inside += 1
    return 4.0 * inside / n

start = time.time()
result = estimate_pi(100_000_000)
elapsed = time.time() - start
print(f"Pi ≈ {result:.6f}")
print(f"Time: {elapsed:.2f} seconds")
```

Run it: `python pi_python.py`. On a modern PC this takes roughly **35–60 seconds**.

#### The Rust version (`examples/pi_rust/`)

```rust
use std::time::Instant;

fn estimate_pi(n: u64) -> f64 {
    let mut rng = fastrand::Rng::new();
    let mut inside: u64 = 0;
    for _ in 0..n {
        let x = rng.f64();
        let y = rng.f64();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    4.0 * inside as f64 / n as f64
}

fn main() {
    let n: u64 = 100_000_000;
    let start = Instant::now();
    let result = estimate_pi(n);
    let elapsed = start.elapsed();
    println!("Pi ≈ {:.6}", result);
    println!("Time: {:.2?}", elapsed);
}
```

Run it: `cd examples/pi_rust && cargo run --release`. On the same machine: **under 1 second**, and very often under half a second.

> ### 🚀 Why is Rust so much faster?
>
> Three reasons, mostly:
>
> 1. **Compiled vs interpreted.** Rust is translated to native machine code ahead of time. Python interprets your source one line at a time, every time you run it.
> 2. **No object overhead.** In Python every number is a wrapped object on the heap. In Rust an `f64` is just 8 bytes of memory the CPU loves.
> 3. **No GIL.** Python's Global Interpreter Lock means even multi-threaded Python often runs on one core. Rust has no such limitation (we're not even using threads here — but it matters when you do).
>
> Same algorithm. Same machine. Different orders of magnitude. Welcome to systems programming.

---

### 2. Install Rust

If you haven't already, follow [`SETUP.md`](../../../SETUP.md) for your platform. The minimum you need is `rustc --version` working in a terminal — anything 1.75 or newer is fine.

### 3. Your first Rust program

```bash
cargo new hello_world
cd hello_world
cargo run
```

What just happened?

- `cargo new hello_world` created a folder with a `Cargo.toml` (project metadata) and a `src/main.rs` (your code).
- `cargo run` invoked the compiler and then ran the resulting binary.

Open `src/main.rs`. It looks like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

Three things to notice:

- **`fn main`** is the entry point. Every binary Rust program starts here. The empty `()` after `main` is the (empty) parameter list.
- **`println!`** has an exclamation mark because it's a **macro**, not a function. Macros expand into other code at compile time. You'll meet `format!`, `vec!`, `println!`, and a few others throughout the course.
- **Strings live in double quotes.** Single quotes (`'a'`) are for `char`, a single Unicode character.

### 4. Make it personal

Edit `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
    println!("My name is Leo.");
    println!("I am learning Rust.");
}
```

Run it: `cargo run`. You should see all three lines. The compiler did a full re-build under the hood — this happens silently and very quickly.

### 5. Your first calculation

Replace the file with this:

```rust
fn main() {
    let semitones_per_octave = 12;
    let octaves = 4;
    let total_notes = semitones_per_octave * octaves;
    println!("A 4-octave keyboard has {} notes.", total_notes);
}
```

Three new things:

- `let semitones_per_octave = 12;` — declare a variable.
- Variables in Rust are **immutable by default**. We'll cover `mut` next session.
- The `{}` in the format string is a placeholder filled in by the values that follow.

Run it. You should see `A 4-octave keyboard has 48 notes.` (Sound right? A piano has 88 keys across 7+ octaves; we'll build up to a full piano in Session 4.)

---

## Common Mistakes

### ❌ Forgetting the `!` on `println`

```rust
fn main() {
    println("Hello!");   // ❌ no exclamation mark
}
```

```
error[E0423]: expected function, found macro `println`
```

**Fix:** `println!("Hello!");`. The exclamation mark identifies macros.

### ❌ Forgetting the semicolon

```rust
fn main() {
    let x = 5
    println!("{}", x);   // ❌ no `;` after `let x = 5`
}
```

```
error: expected `;`, found `println`
```

**Fix:** put a `;` at the end of `let x = 5;`. (We'll see expressions vs statements properly in Session 3 — the rule isn't as arbitrary as it looks.)

### ❌ Single quotes around a string

```rust
println!('Hello');   // ❌ single quotes are for char only
```

**Fix:** double quotes. `'a'` is the character `a`; `"a"` is the string `"a"`. Two different types.

### ❌ Running `cargo run` outside a project folder

```
error: could not find `Cargo.toml` in `/Users/leo` or any parent directory
```

**Fix:** `cd` into the project folder first. Cargo needs a `Cargo.toml` to know what to build.

---

## Session Challenge

Open the `pi_rust` example. The current run uses 100 million samples. Try changing it to 1 million, 10 million, 1 billion. How does the runtime scale? How does the accuracy of Pi change? What happens if you also re-run the Python version with 1 billion samples? (Maybe set a kettle going while you wait for that one.)

> Hint: there's nothing in the code that needs explaining for this challenge. You're just changing one number and observing.

---

## Quick Reference

| Thing | Syntax |
|---|---|
| New project | `cargo new my_thing` |
| Run | `cargo run` |
| Run optimised | `cargo run --release` |
| Entry point | `fn main() { ... }` |
| Print a line | `println!("text");` |
| Print with values | `println!("x = {}", x);` |
| Declare variable | `let name = value;` |
| Comment | `// line comment` or `/* block */` |

---

## DofE Log Reminder

> 📝 You've finished Session 1. Before you close the laptop, spend 5 minutes filling in **Session 1** in [`dfe/session-log.md`](../../dfe/session-log.md). Capture the speed difference you saw — that number is fun to look back on. It's your DofE evidence and it only takes a few minutes while it's fresh.
