# Session 19: Closures and Iterators (Deep Dive)

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

## What You'll Learn

You met closures and iterators in Session 12. Today you go *below the surface*: the three closure traits (`Fn`, `FnMut`, `FnOnce`), the `move` keyword, more iterator adaptors (`zip`, `chain`, `flat_map`, `take`, `skip`), and the holy grail — **writing your own iterator**.

The session output: an infinite oscillator iterator that produces audio samples for a chosen waveform. Drop a `.take(sample_rate)` on it and you have one second of audio. This is *literally* a building block of the final project.

## The Big Idea

In Session 12 you used closures casually: `|x| x + 1`. But closures are richer than they look — every closure secretly implements one (or more) of three traits, depending on what it does with its captured variables:

- **`FnOnce`** — can be called *at most once*. It might consume (move) its captured variables.
- **`FnMut`** — can be called many times, and may mutate captured variables.
- **`Fn`** — can be called many times, only reads captured variables.

`Fn ⊆ FnMut ⊆ FnOnce` — a `Fn` closure is also a `FnMut` and `FnOnce`. The compiler picks the most permissive trait it can.

You usually don't think about this until you write a function that *takes* a closure as a parameter and have to choose `impl Fn(...)`, `impl FnMut(...)`, or `impl FnOnce(...)`.

The `move` keyword forces a closure to **take ownership** of the variables it captures, instead of borrowing them. Essential for closures that outlive their surrounding scope (e.g., closures sent to another thread, or returned from a function).

## Concepts Covered

- The three closure traits and when to use which
- `move ||` closures
- More iterator adaptors: `.zip()`, `.flat_map()`, `.chain()`, `.take()`, `.skip()`, `.cycle()`, `.step_by()`
- Lazy evaluation — adaptors do nothing until consumed
- `Iterator::collect()` into many types: `Vec`, `String`, `HashMap`, `Result<Vec<_>, _>`
- Implementing `Iterator` for your own struct

## Building Towards `midi-synth`

Today's deliverable, `Oscillator`, is the engine you'll bolt into Session 21. Same code, virtually unchanged. You're literally writing the synth one piece at a time.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-3/session-19/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-3/session-19/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. The closure traits, illustrated

```rust
fn main() {
    let x = 10;

    let read       = |y: i32| y + x;          // captures x by reference  → Fn
    let mut total  = 0;
    let mut accum  = |y: i32| total += y;     // captures total by &mut    → FnMut
    let s = String::from("hello");
    let consume    = move || println!("{}", s); // moves s into closure    → FnOnce

    println!("{}", read(5));                  // 15
    accum(1); accum(2); accum(3);
    println!("{}", total);                    // 6
    consume();                                // hello   (s is now owned by closure)
    // println!("{}", s);                     // ERROR — s moved
}
```

Most of the time you write a closure and never think about which trait it implements — the compiler does it for you.

### 2. `move` for "take it with you"

When does `move` matter? When the closure outlives the scope of the captured variable:

```rust
fn make_counter() -> impl FnMut() -> u32 {
    let mut n = 0;
    move || {           // without `move`, n would be borrowed and you'd get an error
        n += 1;
        n
    }
}

let mut c = make_counter();
println!("{} {} {}", c(), c(), c());     // 1 2 3
```

`n` is local to `make_counter`. We return a closure that uses `n`. Without `move`, the closure would borrow `n` from `make_counter`'s stack frame — but `make_counter` is about to return and that frame is gone. With `move`, the closure owns its own `n` going forward. Compiler-enforced safety.

### 3. The big iterator adaptors you didn't see in Session 12

```rust
let xs = vec![1, 2, 3];
let ys = vec!['a', 'b', 'c'];

let zipped: Vec<(i32, char)> = xs.iter().copied().zip(ys.iter().copied()).collect();
// [(1,'a'), (2,'b'), (3,'c')]

let chained: Vec<i32> = (1..=3).chain(10..=12).collect();
// [1, 2, 3, 10, 11, 12]

let flatted: Vec<i32> = vec![vec![1,2], vec![3,4]].into_iter().flatten().collect();
// [1, 2, 3, 4]

let mapped: Vec<i32> = (1..=3).flat_map(|n| vec![n, n*10]).collect();
// [1, 10, 2, 20, 3, 30]      — flat_map = map + flatten

let stepped: Vec<i32> = (0..20).step_by(5).collect();
// [0, 5, 10, 15]

let cycled: Vec<i32> = vec![1, 2, 3].into_iter().cycle().take(7).collect();
// [1, 2, 3, 1, 2, 3, 1]      — repeats forever; take limits it
```

`.cycle()` is interesting — it makes a *finite* iterator infinite. You normally have to combine it with `.take(n)` to actually consume it.

### 4. `collect` is more flexible than you thought

```rust
let s: String = vec!['h', 'i'].into_iter().collect();    // "hi"
use std::collections::HashMap;
let m: HashMap<i32, i32> = (0..3).map(|n| (n, n*n)).collect();  // {0:0, 1:1, 2:4}

// Result-aware collect — gather Ok values, short-circuit on first Err
let parsed: Result<Vec<i32>, _> = vec!["1","2","3"].iter().map(|s| s.parse()).collect();
// Ok(vec![1, 2, 3])
let bad:    Result<Vec<i32>, _> = vec!["1","oops","3"].iter().map(|s| s.parse()).collect();
// Err(...) — stops at "oops"
```

The `Result` form is unbelievably useful — it turns a vec of `Result`s into a `Result` of vec, returning the first error encountered. Saves you a manual loop.

### 5. Writing your own iterator: `Oscillator`

Now the main event. We define a struct that holds the state of a tone generator, and implement the `Iterator` trait so you can use *all* the adaptors above with it.

`examples/oscillator_iter/src/main.rs`:

```rust
#[derive(Clone, Copy)]
pub enum Waveform { Sine, Square, Sawtooth, Triangle }

pub struct Oscillator {
    waveform: Waveform,
    sample_rate: u32,
    frequency: f32,
    sample_index: u64,
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency: f32, sample_rate: u32) -> Self {
        Self { waveform, sample_rate, frequency, sample_index: 0 }
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let t = self.sample_index as f32 / self.sample_rate as f32;
        let phase = 2.0 * std::f32::consts::PI * self.frequency * t;

        let v = match self.waveform {
            Waveform::Sine     => phase.sin(),
            Waveform::Square   => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },
            Waveform::Sawtooth => 2.0 * (self.frequency * t - (self.frequency * t + 0.5).floor()),
            Waveform::Triangle => 1.0 - 4.0 * (self.frequency * t - (self.frequency * t + 0.5).floor()).abs(),
        };

        self.sample_index += 1;
        Some(v)
    }
}
```

Three things make this work:

1. **`type Item = f32;`** — declares what this iterator produces.
2. **`fn next(&mut self) -> Option<Self::Item>`** — the *only* method you must implement. Returns `Some(value)` for each iteration, `None` to stop. Ours never returns `None` — it's an **infinite** iterator. (That's why `.take(n)` is essential when consuming it.)
3. **`Iterator for Oscillator`** — once implemented, you get `.map`, `.filter`, `.take`, `.zip`, `.collect`, etc. for free.

### 6. Use it

```rust
fn main() {
    let osc = Oscillator::new(Waveform::Sine, 440.0, 44100);

    let one_second: Vec<f32> = osc.take(44100).collect();
    println!("Generated {} samples; first 5 = {:?}", one_second.len(), &one_second[..5]);

    // Different waveforms, half a second each
    for wf in [Waveform::Sine, Waveform::Square, Waveform::Sawtooth, Waveform::Triangle] {
        let osc = Oscillator::new(wf, 220.0, 44100);
        let buf: Vec<f32> = osc.take(22050).collect();
        let max = buf.iter().cloned().fold(f32::MIN, f32::max);
        let min = buf.iter().cloned().fold(f32::MAX, f32::min);
        println!("{:?}: {} samples, range [{:.2}, {:.2}]", wf as u8, buf.len(), min, max);
    }
}
```

Notice the second loop: `Oscillator` becomes an iterator of `f32` you can throw any iterator combinator at. That's the win.

---

## Common Mistakes

- **Forgetting `move` for closures returned from a function or sent to a thread** — the borrow checker will tell you, but the error can be cryptic. The mental rule: "does this closure outlive the scope of any captured variable?" If yes, you need `move`.
- **Calling `.collect()` without a type annotation** — Rust can't guess what you're collecting into. Use `let v: Vec<_> = ...` or turbofish `.collect::<Vec<_>>()`.
- **Iterating an infinite iterator without `.take(n)`** — your program runs forever. Always cap infinite iterators.
- **`Vec` of `Result` vs `Result` of `Vec`** — `.collect::<Vec<Result<T, E>>>()` keeps the Errs in the vec; `.collect::<Result<Vec<T>, E>>()` short-circuits on the first Err. Pick deliberately.

---

## Session Challenge

1. Add `Waveform::Noise` that returns a random value in `[-1.0, 1.0]` per sample (use `fastrand`).
2. Add a `vibrato_freq: f32` field to `Oscillator`. In `next()`, modulate the *frequency* by a small slow sine — `freq * (1.0 + 0.01 * (2.0 * PI * 5.0 * t).sin())` for a 5 Hz vibrato. Listen to the difference (you can write samples to a WAV using your Session 18 code).
3. Make a "chord" iterator that holds three `Oscillator`s and returns the *average* of their three samples each step. (Hint: store `Vec<Oscillator>` and call `.next()` on each in turn.)

---

## Quick Reference

```rust
// Closure traits
fn taking_fn      <F: Fn(i32) -> i32>(f: F)     { f(0); f(1); }      // many calls, no mutation
fn taking_fn_mut  <F: FnMut(i32) -> i32>(mut f: F) { f(0); f(1); }   // many calls, can mutate captures
fn taking_fn_once <F: FnOnce(i32) -> i32>(f: F) { f(0); }            // exactly one call

// move
let s = String::from("x");
std::thread::spawn(move || println!("{}", s));  // s moves into the new thread

// Custom iterator
struct Counter(u32);
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 < 5 { self.0 += 1; Some(self.0) } else { None }
    }
}
let v: Vec<u32> = Counter(0).collect();         // [1, 2, 3, 4, 5]
```

---

## Further Reading

Curated extra material on the topics covered in this session (Closures and Iterators (deep dive)). All free; all current as of writing.

- [**The Rust Book** — *Closures: Anonymous Functions That Capture Their Environment* (13.1)](https://doc.rust-lang.org/book/ch13-01-closures.html) — Read again now that you've seen them at scale.
- [**The Rust Book** — *Processing a Series of Items with Iterators* (13.2)](https://doc.rust-lang.org/book/ch13-02-iterators.html) — The companion chapter, with the lazy-evaluation explanation that makes everything click.
- [**`itertools` crate** — extra adapters](https://docs.rs/itertools/latest/itertools/) — Adds `chunks`, `tuple_windows`, `cartesian_product`, and dozens more useful adapters not in `std`.
- [**Niko Matsakis — *Closures Magic Functions***](https://smallcultfollowing.com/babysteps/blog/2014/05/13/focusing-on-ownership/) — From a Rust language designer; how closures relate to ownership.

---
## DofE Log Reminder

Row 19. Note in your log that you wrote your first custom iterator — that's a real Rust milestone.
