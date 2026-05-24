# Glossary

Every piece of jargon used in this course, in plain English. Skim this when something doesn't make sense.

---

## Rust language

**Borrow** — Reading or writing data through a reference (`&x` or `&mut x`) rather than taking ownership of it. The compiler enforces strict rules: many readers OR one writer, never both at the same time.

**Borrow checker** — The part of the Rust compiler that enforces the borrow rules. When it rejects your code, it's usually right — and the error message will tell you what's wrong. It's a co-pilot, not an enemy.

**Cargo** — Rust's build tool. `cargo new`, `cargo build`, `cargo run`, `cargo add` — that's the everyday vocabulary.

**Cargo.toml** — The file at the root of every Cargo project that lists its dependencies (other crates) and metadata.

**Closure** — An anonymous function that can capture variables from the surrounding scope. Written like `|x| x + 1`. Used heavily with iterators.

**Compiled** — Translated from source code into a native binary by `cargo build`. The resulting file can run directly on your CPU without any other program present. The opposite of *interpreted*, where every time you run the program, another program reads your source code one line at a time and does what it says.

**Crate** — A Rust library or package. `macroquad` is a crate. `serde` is a crate. Crates live on <https://crates.io> and are added to your project with `cargo add <name>`.

**Derive** — A way to automatically generate boilerplate code for a struct or enum, written as `#[derive(Debug, Clone)]` above the definition. Saves typing.

**Enum** — A type with a finite, named set of values. `CellType::Sand` is one variant of the enum `CellType`. The compiler forces you to handle every variant in a `match`.

**Generics** — Code that works with multiple types, written once. `Vec<T>` is generic — `T` can be anything. `fn biggest<T: Ord>(items: &[T]) -> &T` is a generic function.

**Immutable** — Cannot be changed after creation. `let x = 5` makes `x` immutable; `let mut x = 5` makes it mutable. Rust defaults to immutable, which catches bugs.

**Iterator** — A value you can walk through one element at a time. `vec.iter()`, `0..10`, and `text.chars()` are all iterators. They chain: `vec.iter().filter(…).map(…).collect()`.

**Lifetime** — A compile-time annotation describing how long a reference is valid. Usually inferred; you only write them out explicitly when the compiler asks.

**Macro** — A piece of code that expands into more code at compile time. `println!` is a macro (the `!` is the giveaway). So is `vec![1, 2, 3]`.

**Match** — Rust's pattern-matching expression. Like a `switch` statement on steroids — the compiler forces you to handle every case.

**Module** — A namespace inside a crate. Declared with `mod` and brought into scope with `use`. Used to split a project across multiple files.

**Option** — The enum `Option<T>` with two variants: `Some(value)` and `None`. Rust's replacement for null pointers. You can't ignore `None` — the compiler makes you handle it.

**Ownership** — Every value in Rust has exactly one owner. When the owner goes out of scope, the value is dropped. This is how Rust manages memory without a garbage collector.

**Result** — The enum `Result<T, E>` with two variants: `Ok(value)` and `Err(error)`. Rust's replacement for exceptions. Combine with the `?` operator to bubble errors up.

**rustup** — The installer for Rust. Use it to install Rust itself and to switch between stable/beta/nightly toolchains.

**Slice** — A view into part of a `Vec` or array. Written `&v[2..5]`. Cheap, doesn't copy.

**Stable toolchain** — The released version of Rust. New stable versions ship every six weeks. This course uses stable throughout.

**Struct** — A custom type with named fields. The basic building block of Rust data design. `struct Cell { temperature: f32 }`.

**Trait** — A set of methods that a type can implement. Like an interface in Java or a protocol in Swift. `Display`, `Debug`, `Clone` are traits. You can write your own.

**Tuple** — A fixed-length anonymous collection of values, possibly of different types: `(3, "hello", 4.2)`. Access with `.0`, `.1`, `.2`.

**Type inference** — The compiler figuring out a type from context. `let x = 5` — `x` is `i32` because that's the default integer. You only have to write types when the compiler can't guess.

**Vec** — A growable array. The workhorse of Rust data structures. `Vec<T>` holds any number of `T`s.

---

## Simulation, cellular automata, games

**Boundary condition** — What happens at the edge of a simulation grid. Common choices: stop, wrap around, reflect. In `sand-sim` the edge is solid — particles stop at it.

**Brush** — In a paint or sandbox tool, the area you affect with a single click. The course's brush is a filled circle; the radius scales with the scroll wheel.

**Cellular automaton** — A simulation made up of a grid of cells, where each cell updates based on simple local rules about its neighbours. Conway's Game of Life is the classic example. `sand-sim` is one too.

**Chunk** — A small fixed-size square of the simulation grid. Bigger sims store the world as a grid of chunks for speed and to support effectively-infinite worlds.

**Codex** — In the alchemy game (Month 3), the in-game catalogue of discovered elements. Inspired by the Pokédex.

**Convection** — Heat-driven movement of fluids. Hot water rises, cool water sinks, and you get a cycle. Visible in `sand-sim` Month 2 when steam rises off boiling water.

**Emergent behaviour** — Complex patterns that come out of simple rules. Sand piles forming a natural angle of repose from "swap with the cell below if empty, otherwise try diagonal" is emergent behaviour.

**Frame** — A single update + redraw of the simulation. 60fps means 60 frames per second — the simulation updates 60 times every second.

**fps (frames per second)** — How many times per second the simulation updates. 60fps is the standard target. Below ~30fps things start to feel laggy.

**Grid coordinate** — Position on the simulation grid, measured in cells (not pixels). The cell at `(x, y) = (10, 3)` is 10 cells from the left and 3 cells from the top.

**Heat map** — A visualisation where colour represents temperature. In Sessions 9 onward, optionally pressing `T` overlays a red-orange-yellow gradient on the grid.

**Particle** — Loosely, any cell in the simulation grid that isn't empty. A sand grain is a particle; a wall stone is a particle; a fire spark is a particle.

**Recipe** — In the alchemy game, a hidden combination that produces a new element. Discovered by experimentation.

**Spawn** — Create a new cell of a given type at a given location. The mouse spawns particles when you click; reactions spawn particles when they fire.

**Update loop** — The function that runs once per frame and applies all the physics/chemistry rules to the grid. The heart of the simulation.

---

## Chemistry (light touch)

See [`CHEMISTRY-PRIMER.md`](./CHEMISTRY-PRIMER.md) for the longer treatment. Short definitions:

**Combustion** — A fast oxidation reaction that releases heat and light. Burning wood, burning oil. In the sim: fuel + heat above ignition temperature → fire.

**Endothermic / exothermic** — Endothermic reactions absorb heat (ice melting, water boiling). Exothermic reactions release heat (combustion, lava cooling). The sim models both.

**Ignition temperature** — The temperature at which a material catches fire. Different fuels have different ignition temperatures; in the sim each cell type stores this as a constant.

**Oxidation** — A chemical reaction where a material loses electrons to oxygen. Burning is fast oxidation; rusting is slow oxidation. The sim models the rusting of iron in Session 22.

**Phase change** — A material switching between solid, liquid, and gas. Water → steam is a phase change driven by heat. The sim models it as a temperature threshold.

**Reactant / product** — In a chemical equation, the inputs and the outputs. `water + sodium → sodium hydroxide + hydrogen` has two reactants and two products. The sim's reaction system mirrors this shape exactly.

---

## DofE, repo workflow, miscellaneous

**Assessor** — The person who signs off your DofE Skill section. Cannot be a parent or guardian. Doesn't need to be a Rust expert. See [`dfe/assessor-briefing.md`](./dfe/assessor-briefing.md).

**Commit** — A snapshot of your code saved in git. `git commit -m "message"` records what you've done; `git push` sends it to GitHub.

**Milestone** — One of the three project releases: `sand-sim` v0.1, v0.2, v1.0. Sits at the end of each month.

**Pre-flight** — Session 00. The one-off setup that happens before Session 1: install Rust, fork the repo, make your first commit. ~45 minutes.

**Pull request (PR)** — A proposed change to a git repo, ready to be reviewed and merged. You won't need to open one for the course itself, but it's the standard way to contribute to open-source projects.

**Wow Moment** — A named, deliberate moment in every session designed to surprise or delight. Listed explicitly in each session README. The dopamine beat.
