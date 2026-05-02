# Session 20: Generics and Advanced Traits

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

## What You'll Learn

How to write code once that works for *any* type that meets your requirements. Generics, trait bounds, `impl Trait` shorthand, dynamic dispatch with `Box<dyn Trait>`, and just enough about lifetimes to read compiler errors. By the end you can explain *why* `Vec<T>` is one type that works for any `T`.

## The Big Idea

You've used generics already without naming them: `Vec<i32>`, `Vec<String>`, `Option<u8>`, `HashMap<&str, u32>`, `Result<T, E>`. The `<T>`, `<K, V>` is the generic-type machinery. Now you write your *own* generic types.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

largest(&[1, 2, 3]);                  // works: i32 is PartialOrd
largest(&["a", "b", "c"]);            // works: &str is PartialOrd
largest(&[Vec::<i32>::new()]);        // doesn't compile: Vec<i32> isn't PartialOrd
```

The angle brackets `<T: PartialOrd>` say "this function works for any type T, as long as T can be compared with `>`". The `T: PartialOrd` part is a **trait bound** — it constrains what T can be.

Two flavours of polymorphism:

- **Static dispatch** (generics + monomorphisation) — the compiler generates a separate copy of the function for each concrete type used. Zero runtime cost. Bigger binaries.
- **Dynamic dispatch** (`Box<dyn Trait>`) — one copy of the code, type figured out at runtime via a hidden vtable. Tiny runtime cost, smaller binaries, more flexibility.

You'll meet both today.

## Concepts Covered

- Generic functions: `fn foo<T>(x: T)`
- Generic structs: `struct Pair<T, U> { a: T, b: U }`
- Trait bounds: `<T: Display + Clone>`
- `where` clauses for readable bounds
- `impl Trait` shorthand in arguments and return types
- `Box<dyn Trait>` — heap allocation + dynamic dispatch
- Trait objects vs generics — when to choose which
- Lifetimes — `'a` syntax, when the compiler insists on them

## Building Towards `midi-synth`

The synth needs to handle different *kinds* of waveform-generators (sine, square, sawtooth, triangle, perhaps user-supplied) behind a single interface. Two ways to do that:

- Generics — `struct Voice<O: Iterator<Item = f32>>` — fast, but each `Voice<Sine>` and `Voice<Square>` are different types.
- Trait objects — `struct Voice { osc: Box<dyn Iterator<Item = f32>> }` — one type, runtime-flexible.

Today you build both and see the trade-off in real code.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-3/session-20/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-3/session-20/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. Generic functions

```rust
fn pair<T>(a: T, b: T) -> (T, T) { (a, b) }

let p1 = pair(1, 2);              // (i32, i32)
let p2 = pair("hi", "there");     // (&str, &str)
```

The compiler generates two specialised versions of `pair`: one for `i32`, one for `&str`. This is **monomorphisation** — single source, multiple compiled versions.

### 2. Trait bounds

`<T>` alone says "any type" — so you can do almost nothing inside the function. To do useful work you constrain T:

```rust
use std::fmt::Display;

fn announce<T: Display>(x: T) {
    println!(">> {}", x);
}
```

`T: Display` means "T must implement Display" — only types that implement Display can be passed in. Inside the function, the compiler trusts `T` has `Display`'s methods.

Multiple bounds:

```rust
fn describe<T: Display + Clone>(x: T) -> T {
    println!("{}", x);
    x.clone()
}
```

Or with a `where` clause for legibility:

```rust
fn describe<T>(x: T) -> T
where
    T: Display + Clone,
{
    println!("{}", x);
    x.clone()
}
```

Same thing. `where` is preferred when bounds get long.

### 3. Generic structs

```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: std::fmt::Display> Pair<T> {
    fn announce(&self) {
        println!("first: {}, second: {}", self.first, self.second);
    }
}
```

Note `impl<T: Display> Pair<T>` — bounds go on the impl block. You can have multiple impl blocks for different bounds:

```rust
impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self { Self { first: a, second: b } }
}

impl<T: PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.first > self.second { &self.first } else { &self.second }
    }
}
```

`Pair::new` works for *any* `T`. `larger` only for ones that implement `PartialOrd`.

### 4. `impl Trait` shorthand

For simple cases, you can drop the explicit generic and write `impl Trait`:

```rust
fn announce(x: impl Display) {                 // same as fn announce<T: Display>(x: T)
    println!("{}", x);
}

fn make_iter() -> impl Iterator<Item = i32> {  // returns "some iterator of i32"
    (1..=10).filter(|n| n % 2 == 0)
}
```

`impl Trait` in argument position is just sugar for a generic. In return position it's more useful — it lets you return a complex type without naming it. But the *exact* type is fixed at the function level; the caller doesn't get to choose.

### 5. Dynamic dispatch with `Box<dyn Trait>`

Sometimes you need a *runtime* choice between several concrete types behind one trait. Generics don't help — they're picked at compile time. Use a **trait object**:

```rust
trait Greeter { fn greet(&self) -> String; }

struct Hello;
impl Greeter for Hello { fn greet(&self) -> String { String::from("Hello!") } }

struct Hej;
impl Greeter for Hej { fn greet(&self) -> String { String::from("Hej!") } }

let greeters: Vec<Box<dyn Greeter>> = vec![Box::new(Hello), Box::new(Hej)];
for g in &greeters {
    println!("{}", g.greet());
}
```

`dyn Greeter` is a **trait object** — a fat pointer that stores both a pointer to the data and a vtable for the trait's methods. `Box<...>` puts it on the heap (required because the size of `dyn Greeter` isn't known at compile time).

This is how you store a heterogeneous list of "things that all do X". Generics can't do this — `Vec<T>` requires every element to be the same `T`.

### 6. Lifetimes — the bare minimum

You'll see `'a` (read "tick a") everywhere in error messages. Here's the punchline: when you have a function returning a reference, the compiler needs to know which input the reference came from. For 99% of cases, it figures it out automatically. Sometimes it needs a hint:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

`<'a>` declares a lifetime parameter. `&'a str` reads "a `&str` borrowed for at least lifetime `'a`". The function signature says: "all three references share a lifetime `'a` — the returned one lives at least as long as both inputs". The compiler enforces this.

If you see `error: missing lifetime specifier`, you've returned a reference and the compiler can't figure out the source. Annotating with `'a` like above usually fixes it.

You will rarely write lifetime annotations by hand in this course — it's mostly a *reading* skill. We're flagging it now so the syntax doesn't surprise you.

### 7. The example: a generic and a dynamic oscillator

`examples/generic_oscillator/src/main.rs` shows both styles side by side. A `Voice<O>` parameterised by oscillator type (static), and a `DynVoice` storing `Box<dyn Iterator<Item = f32>>` (dynamic). Identical behaviour, very different machine code.

---

## Generics vs Trait Objects: Picking

| Question | Generics | Trait objects (`dyn`) |
|---|---|---|
| Speed | Faster (no vtable) | Tiny vtable cost |
| Binary size | Larger (one copy per type) | Smaller |
| Flexibility | Type fixed at compile | Type chosen at runtime |
| Heterogeneous collections | No | Yes |
| Method-call ergonomics | Same | Same |

**Default to generics. Reach for `Box<dyn Trait>` when you need a heterogeneous collection or when binary size matters.**

---

## Common Mistakes

- **Trying to compare two `T`s without `T: PartialOrd`** — `if a > b` won't compile until you add the bound.
- **`Vec<dyn Trait>` directly** — doesn't compile, because `dyn Trait` has unknown size. Always wrap: `Vec<Box<dyn Trait>>`.
- **Returning two different concrete types from one branch** — `if cond { Hello } else { Hej }` fails type-check. Either return `Box<dyn Greeter>` for both branches, or refactor.
- **Lifetime annotations look scary** — they almost always come out where you'd expect once you read the error. Don't panic.

---

## Session Challenge

Take the `Oscillator` from Session 19. Build:

1. A `Voice<O: Iterator<Item = f32>>` struct that holds an oscillator and a `gain: f32`. `next()` returns the next sample multiplied by `gain`. (Generic — fast.)
2. A `DynVoice` struct that holds `Box<dyn Iterator<Item = f32>>` and `gain: f32`. (Dynamic — flexible.)
3. A `Vec<DynVoice>` containing one of each waveform type. Iterate and print the first five samples of each.

Time both versions on a 1-million-sample buffer with `std::time::Instant`. Note the difference — generics will probably be slightly faster, but you'll find both are *fast enough* for audio.

---

## Quick Reference

```rust
// Generic function with bounds
fn show<T: std::fmt::Display + Clone>(x: T) -> T {
    println!("{}", x); x.clone()
}

// where clause
fn show2<T>(x: T) -> T
where T: std::fmt::Display + Clone {
    println!("{}", x); x.clone()
}

// Generic struct
struct Pair<A, B> { a: A, b: B }
impl<A, B> Pair<A, B> {
    fn new(a: A, b: B) -> Self { Self { a, b } }
}

// impl Trait
fn doubled() -> impl Iterator<Item = i32> { (1..=5).map(|n| n * 2) }

// Trait object
trait Speak { fn say(&self); }
let speakers: Vec<Box<dyn Speak>> = vec![/* ... */];

// Lifetimes
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split(' ').next().unwrap_or(s)
}
```

---

## Further Reading

Curated extra material on the topics covered in this session (Generics and Advanced Traits). All free; all current as of writing.

- [**The Rust Book** — *Generic Types, Traits, and Lifetimes* (chapter 10)](https://doc.rust-lang.org/book/ch10-00-generics.html) — The full story, including monomorphisation.
- [**The Rust Book** — *Advanced Traits* (19.2)](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html) — Associated types, default type parameters, supertraits, the newtype pattern.
- [**The Rustonomicon** — *for the unsafe and curious*](https://doc.rust-lang.org/nomicon/) — When you outgrow the Book, this is the next book. Difficult; brilliant.
- [**Jon Gjengset — *Crust of Rust* video series**](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) — Long-form deep-dives by a Rust expert. The *Iterators* and *Lifetimes* episodes pair perfectly with this session.

---

## Stuck?

You're not the first. Three places that work when you're properly stuck:

- [**Rust Discord** — `#beginners`](https://discord.gg/rust-lang-community) (fastest; people are friendly)
- [**`/r/learnrust`**](https://www.reddit.com/r/learnrust/) (paste your code + the error; usually answered within hours)
- [**`users.rust-lang.org`**](https://users.rust-lang.org/) (slower; thorough; answers stay searchable for years)

When the compiler error is the thing confusing you, [`resources/compiler-errors.md`](../../resources/compiler-errors.md) translates the most common ones into plain English.

Asking for help isn't cheating — real Rust developers do it daily. Search first; if no luck, post a [minimal reproducible example](https://stackoverflow.com/help/minimal-reproducible-example).

---
## DofE Log Reminder

Row 20. Final sprint: four sessions left. They're all the project — make sure your dev setup is happy.
