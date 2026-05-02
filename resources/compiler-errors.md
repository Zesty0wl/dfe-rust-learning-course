# How to Read a Rust Compiler Error

> *"The compiler is the friendliest grumpy teacher you'll ever meet. It refuses to let bad code compile, but it almost always tells you exactly what's wrong and how to fix it."*

Every Rust learner — including professional Rust developers — spends time staring at compiler errors. The errors are unusually long and unusually helpful, but the **first time** you see each one it can feel intimidating. This page translates the most common errors you'll meet across this course into plain English.

If your error isn't listed, the [official "Error Index"](https://doc.rust-lang.org/error_codes/error-index.html) covers every single one Rust can produce — search for the `Exxxx` code shown in your error.

---

## How to read any Rust error

Every Rust error has the same anatomy. Here's a real one:

```text
error[E0382]: borrow of moved value: `s`
  --> src/main.rs:5:20
   |
3  |     let s = String::from("hello");
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
4  |     let t = s;
   |             - value moved here
5  |     println!("{}", s);
   |                    ^ value borrowed here after move
   |
help: consider cloning the value if the performance cost is acceptable
   |
4  |     let t = s.clone();
   |              ++++++++

For more information about this error, try `rustc --explain E0382`.
```

Reading order:

1. **The headline** (`error[E0382]: borrow of moved value: 's'`) — *what* went wrong, in one line.
2. **The location** (`--> src/main.rs:5:20`) — *where* in your code.
3. **The annotated source** — three columns of code with arrows pointing at exactly the problematic spans. **Read this slowly.** The arrows are the most useful diagnostic information you'll ever get.
4. **The `help:` block** — the compiler's *suggested* fix. Often you can copy it almost verbatim. Sometimes it's wrong; treat it as a strong hint, not gospel.
5. **The `rustc --explain` line** — run this in your terminal for a longer essay-style explanation of the error category. Worth doing once per new error code so you've seen it.

---

## The errors you'll meet most often

### `error[E0425]: cannot find value 'x' in this scope`

**What it means:** You used a name that doesn't exist (yet) in this scope. Usually a typo or a forgotten `let`.

**Likely fixes:**

- Typo? `lenght` → `length`.
- Forgot to declare it? Add `let x = ...;` before you use it.
- Defined in another module? Add a `use` statement.

---

### `error[E0596]: cannot borrow 'x' as mutable, as it is not declared as mutable`

**What it means:** You tried to change a variable, but you didn't say it could change.

**Fix:** add `mut`:

```rust
let x = 5;       // ❌ immutable
x = 6;
let mut x = 5;   // ✅ mutable
x = 6;
```

This is one of Rust's signature design choices — immutability is the default. The fix is always the same: add `mut`.

---

### `error[E0382]: borrow of moved value` / `use of moved value`

**What it means:** Rust's ownership rules. A value of a non-`Copy` type (like `String`, `Vec`, custom structs) was *moved* to a new owner, and then you tried to use the old name afterwards.

```rust
let s = String::from("hello");
let t = s;                       // s moved into t
println!("{}", s);               // ❌ s no longer owns anything
```

**Three possible fixes — pick the one that matches your intent:**

1. **You only need to read `s`** — borrow it instead of moving:
   ```rust
   let t = &s;                    // t borrows s
   println!("{}", s);             // ✅ s still valid
   ```
2. **You really need a second copy** — clone:
   ```rust
   let t = s.clone();             // independent copy
   println!("{}", s);             // ✅
   ```
3. **You're done with `s` after using it** — just reorder so the use happens first:
   ```rust
   println!("{}", s);             // use first
   let t = s;                     // then move
   ```

This error is *the* Rust experience. Once you understand it, you understand ownership.

---

### `error[E0308]: mismatched types`

**What it means:** A function expected one type and got another. The error always shows what was expected and what was found.

```text
error[E0308]: mismatched types
   |
5  |     greet(s);
   |     ----- ^ expected `&str`, found `String`
```

**Common fixes:**

- **`String` → `&str`:** add `&` to borrow.
  ```rust
  greet(&s);                     // String → &str via deref coercion
  ```
- **`&str` → `String`:** call `.to_string()` or `String::from(...)`.
- **`i32` vs `i64` etc.:** add an explicit cast: `n as i64`.
- **`f32` vs `f64`:** use a literal suffix: `3.14_f32`.

---

### `error[E0277]: the trait bound 'X: Trait' is not satisfied`

**What it means:** Some piece of code requires `X` to implement a particular trait, but it doesn't. Usually `Display`, `Debug`, `PartialEq`, `Clone`, or `Copy`.

```text
error[E0277]: `MyType` doesn't implement `Debug`
   |
5  |     println!("{:?}", x);
   |                      ^ `MyType` cannot be formatted using `{:?}`
```

**Likely fix:** add the missing derive:

```rust
#[derive(Debug)]                  // for {:?}
#[derive(Display)]                // doesn't exist as a derive — implement manually
#[derive(Clone, Copy)]            // for .clone() and let-by-value
#[derive(PartialEq, Eq)]          // for ==
```

If you can't derive (`Display` is the common case), implement it manually:

```rust
impl std::fmt::Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "your text here")
    }
}
```

---

### `error[E0599]: no method named 'x' found for type 'Y'`

**What it means:** You called a method that doesn't exist on this type, *or* it does exist but in a trait you haven't imported.

**Likely fixes:**

- Typo? `lenght()` → `len()`.
- Method exists on a trait? Add the `use` line for it. Common offenders:
  ```rust
  use std::io::Read;            // for .read_to_string(), etc.
  use std::io::Write;           // for .write_all(), etc.
  use std::io::BufRead;         // for .lines()
  use std::str::FromStr;        // for "...".parse::<T>()
  ```
- Method requires a trait bound your generic doesn't have? Add the bound: `<T: SomeTrait>`.

---

### `error: missing lifetime specifier`

**What it means:** You wrote a function returning a reference, and the compiler can't tell which input the reference came from.

```rust
fn longest(a: &str, b: &str) -> &str {  // ❌ which one are we returning?
    if a.len() > b.len() { a } else { b }
}
```

**Fix:** add a lifetime parameter saying "all three references share a lifetime":

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {  // ✅
    if a.len() > b.len() { a } else { b }
}
```

You'll meet this around session 20. For most of the course, the compiler handles lifetimes automatically.

---

### `error[E0502]: cannot borrow 'x' as mutable because it is also borrowed as immutable`

**What it means:** Rust's "one mutable OR many immutable" borrowing rule. You tried to take a `&mut x` while a `&x` already exists.

```rust
let v = vec![1, 2, 3];
let first = &v[0];                // immutable borrow of v
v.push(4);                        // ❌ mutable borrow of v
println!("{}", first);
```

**Fix:** finish using the first borrow before starting the second.

```rust
let v = vec![1, 2, 3];
let first = &v[0];
println!("{}", first);            // first's lifetime ends here
let mut v = v;
v.push(4);                        // ✅
```

Or restructure so you don't hold both borrows at once. This is the *other* signature Rust experience — and once you see why the rule exists (data races, dangling pointers), you'll appreciate it.

---

### `error[E0061]: this function takes N arguments but M were supplied`

**What it means:** Wrong number of arguments. The compiler tells you both numbers.

**Fix:** count and supply the right number. Watch out for:
- Methods take an implicit `self` you don't pass explicitly.
- Closures' arity must match what's expected (`|x|` not `|x, y|`).

---

### `error[E0507]: cannot move out of borrowed content`

**What it means:** You have a `&T` (read-only borrow) and tried to take ownership of the value behind it. Borrows don't grant ownership.

```rust
fn process(v: &Vec<String>) {
    let s = v[0];                   // ❌ moves String out of borrowed Vec
}
```

**Likely fixes:**

- Take a reference instead: `let s = &v[0];`
- Clone if you need ownership: `let s = v[0].clone();`
- Take the parameter by value if your function really should consume it: `fn process(v: Vec<String>)`.

---

### `error[E0716]: temporary value dropped while borrowed`

**What it means:** You borrowed a reference into a temporary value (something with no name) that got dropped at the end of the expression.

```rust
let s = "hello".to_string().as_str();   // ❌ String temp dropped, &str dangles
```

**Fix:** bind the temporary to a name first:

```rust
let owned = "hello".to_string();
let s = owned.as_str();                 // ✅
```

---

### `error[E0277]: the size for values of type 'dyn Trait' cannot be known at compilation time`

**What it means:** You tried to use a trait object (`dyn Trait`) without putting it behind a pointer.

```rust
let x: dyn Iterator<Item = i32>;       // ❌ unknown size
let x: Box<dyn Iterator<Item = i32>>;  // ✅ Box knows its size (a pointer)
let x: &dyn Iterator<Item = i32>;      // ✅ &-ref knows its size
```

You'll meet this in session 20.

---

## Warnings vs errors

`rustc` also prints **warnings** (yellow) for things that compile but probably aren't right:

```text
warning: unused variable: `x`
warning: variable does not need to be mutable
warning: unused import: `std::collections::HashMap`
```

Warnings don't stop your program from running, but treat them as errors-in-disguise. The course-wide rule of thumb: **fix every warning before you commit**. The fastest way is `cargo clippy`, which catches everything `rustc` warns about *plus* style issues.

---

## When the error genuinely doesn't make sense

Three things to try, in order:

1. **Run `cargo clean && cargo build`.** Sometimes stale build artefacts produce confusing errors. A clean build is honest.
2. **Run `rustc --explain Exxxx`** with the error code from the headline. The explainer pages are detailed and example-rich.
3. **Search the exact error message** (in quotes). 99% of the time someone has hit it on Stack Overflow, /r/learnrust, or the users.rust-lang.org forum.
4. **Ask for help.** See the "Stuck?" section at the end of any session for the channels that work.

---

## Further reading

- [**Rust by Example** — *Errors*](https://doc.rust-lang.org/rust-by-example/error.html) — introduces error handling alongside compiler errors.
- [**The Rust Book** — *Recoverable Errors with `Result`* (chapter 9)](https://doc.rust-lang.org/book/ch09-00-error-handling.html) — once you're past beginner errors, this teaches you to *write* code that produces good errors.
- [**Official Rust Error Index**](https://doc.rust-lang.org/error_codes/error-index.html) — every single `Exxxx` code, explained.
