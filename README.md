# DfE Rust Learning Course

A 24-session, 12-week Rust programming course built around music, Minecraft, and a bit of maths. Designed as a complete evidence pack for the **Duke of Edinburgh Award (DofE) Skill section** at any level — usable by any DofE participant aged ~14+.

> *"A language empowering everyone to build reliable and efficient software."* — [rust-lang.org](https://www.rust-lang.org)

---

## Who this course is for

- Beginner-to-intermediate programmers (some prior exposure to Python, JavaScript, or similar helps but isn't required)
- Pitched at a curious teenage learner; examples lean on **music**, **Minecraft**, and **maths** as familiar territory, but no specialist knowledge is assumed
- Suitable as a structured 12-week skill activity for the **Duke of Edinburgh Award** at Bronze, Silver, or Gold level

You don't need to be a maths whiz or own a MIDI keyboard. The MIDI keyboard projects in Month 3 have file-based fallbacks.

---

## What you'll build

By the end of the course, you'll have written three real, working pieces of software:

| Month | Project | What it does |
|---|---|---|
| 1 | `music-theory-cli` | A command-line tool that prints scales and chord progressions for any root note. |
| 2 | `world-generator` | A seeded procedural terrain generator that prints Minecraft-style ASCII worlds. |
| 3 | `midi-synth` | A MIDI synthesiser that turns `.mid` files (or live keyboard input) into audio. |

Every project compiles and runs unmodified on Windows, macOS, and Linux.

---

## How the course works

- **24 sessions**, ~1 hour each
- **2 sessions per week** for 12 weeks
- Each session has a written walkthrough and a folder of runnable example code
- Every month ends with a 2-session mini-project build
- Every session ends with an optional **challenge** (no solution provided — that one's for you)
- Every session ends with a **DofE log reminder** so the evidence pack stays up to date

---

## Repository layout

```
.
├── README.md            (you are here)
├── SETUP.md             Install Rust, Cargo, VS Code on Windows / macOS / Linux
├── LICENSE              MIT
├── .gitignore
├── dfe/                 Duke of Edinburgh Award evidence pack
├── month-1/             Foundations  →  music-theory-cli
├── month-2/             Intermediate →  world-generator
├── month-3/             Advanced     →  midi-synth
└── resources/           Cheatsheet and glossary
```

Each `session-XX/` folder contains a `README.md` (the lesson) and an `examples/` folder of `cargo run`-able sample code.

---

## Prerequisites

- Rust (stable, **1.75 or newer**) installed via [rustup](https://rustup.rs)
- Cargo (comes with Rust)
- VS Code with the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension
- Python 3 (for the Session 1 speed demo only)
- **Linux only:** `libasound2-dev` for Month 3 audio (`sudo apt install libasound2-dev`)
- **Month 3 stretch goal only:** a USB MIDI keyboard (any class-compliant model)

Step-by-step instructions for all three platforms are in [`SETUP.md`](./SETUP.md).

---

## How to use this repo

1. **Pre-flight:** work through [`month-1/session-00/`](./month-1/session-00/) — install Rust, create a GitHub account, fork this repo, and make your first commit. ~45 minutes, one-off.
2. If you're doing this for DofE, read [`dfe/README.md`](./dfe/README.md), [`dfe/print-checklist.md`](./dfe/print-checklist.md), and [`dfe/github-workflow.md`](./dfe/github-workflow.md) **before** Session 1, and decide whether to log on paper, in git, or both.
3. Open [`month-1/README.md`](./month-1/README.md) and start at Session 1.
4. After every session, fill in that session's page in your printed booklet **or** edit `dfe/session-log.md` and `git commit` (or both — see [`dfe/github-workflow.md`](./dfe/github-workflow.md)).
5. After each mini-project, complete the corresponding milestone reflection in `dfe/`.
6. After Session 24, write your participant statement and have your assessor sign off.

Get stuck? Each session has a `solution/` or `examples/` folder showing the working answer.

---

## How long does each session take?

The written content for each session is sized for ~1 hour of focused work. Project sessions (7, 8, 15, 16, 21–24) tend to run a little longer because you're building real software.

---

## Course philosophy

- **Get to the code fast.** Theory exists to make code make sense, not the other way round.
- **Projects are the point.** Concepts are introduced because you need them for the project.
- **Respect the learner.** No talking down. Real terminology, explained clearly.
- **Make it fun.** Music, Minecraft, and maths thread through almost every example.
- **All sample code works.** No pseudocode. Every snippet is a complete, runnable Cargo project.

---

## Licence

MIT. See [`LICENSE`](./LICENSE). Fork it, remix it, teach it. If you build something cool, open a PR.

---

## Acknowledgements

Built around the Rust language and its incredible community. Special thanks to the maintainers of `cargo`, `rust-analyzer`, `hound`, `midly`, `midir`, `cpal`, `clap`, `colored`, and `fastrand` — all crates used in this course.
