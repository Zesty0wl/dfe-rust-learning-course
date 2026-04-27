# Month 1 — Foundations

**Theme:** Learning the language through music theory.

**Project:** [`music-theory-cli`](./project/music-theory-cli/) — a command-line tool that prints the notes, intervals, and diatonic chord progression for any scale.

```text
$ cargo run -- --root C --scale major
Scale: C Major
Notes: C  D  E  F  G  A  B
Intervals: W  W  H  W  W  W  H
Chords: Cmaj  Dmin  Emin  Fmaj  Gmaj  Amin  Bdim
```

---

## Sessions

| # | Title | Concepts |
|---|---|---|
| 1 | [Why Rust? History, Setup, and the Speed Demo](./session-01/) | `cargo new`, `cargo run`, `fn main`, `println!` |
| 2 | [Variables, Types, and Mutability](./session-02/) | `let`, `let mut`, scalar types, shadowing |
| 3 | [Functions, Expressions, and Basic I/O](./session-03/) | `fn`, expressions, `std::io`, `.parse()` |
| 4 | [Control Flow](./session-04/) | `if`, `loop`, `while`, `for`, `break`, `continue` |
| 5 | [Pattern Matching and `match`](./session-05/) | `match`, exhaustiveness, guards |
| 6 | [Enums and Strings](./session-06/) | `enum`, `String`/`&str`, `Vec` preview |
| 7 | [Mini-Project Build Part 1 — Scale Generator](./session-07/) | Project work |
| 8 | [Mini-Project Build Part 2 — Chord Progressions and Polish](./session-08/) | `Cargo.toml` deps, `colored` |

By the end of Session 8 you will have a working `music-theory-cli` command-line tool that you can run from your own terminal, and you'll have completed Milestone 1 of your DofE evidence pack.

---

## How to use this month

Work through the sessions in order. Each one's `README.md` is the lesson; the `examples/` folder contains the runnable code from the lesson. To run any example:

```bash
cd month-1/session-02/examples/chromatic_scale
cargo run
```

After each session, fill in your DofE log entry in [`../dfe/session-log.md`](../dfe/session-log.md).
