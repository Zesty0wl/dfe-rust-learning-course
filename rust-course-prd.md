# PRD: Learn Rust with Leo
### A 24-Session Rust Programming Course for a 15-Year-Old with Interests in Music, Maths, and Minecraft

---

## Document Purpose

This PRD is a brief for an AI course-generation agent. It defines the complete framework, learning philosophy, session structure, and project specifications for a 24-session Rust programming course. The agent's job is to fill in this framework with full session content: explanations, worked examples, sample code, exercises, and project guidance.

The course will live as an open-source GitHub repository so that others can use and adapt it.

---

## Learner Profile

- **Name:** Leo
- **Age:** 15
- **Experience:** Beginner programmer (some exposure to Python and/or similar)
- **Interests:** Music (has a MIDI keyboard), Minecraft, Mathematics
- **Hardware:** Powerful Windows PC, MIDI keyboard connected via USB
- **Goal:** Complete a structured, skill-building Rust course over 12 weeks as part of a Duke of Edinburgh skill requirement
- **Tone:** Treat Leo as an intelligent teenager, not a child. Avoid being patronising. Be enthusiastic about the language and the projects. Humour is welcome.

---

## Course Overview

| Property | Value |
|---|---|
| Total sessions | 24 |
| Session length | ~1 hour |
| Cadence | 2 sessions per week |
| Total duration | ~12 weeks (3 months) |
| Delivery format | Self-directed with written materials |
| Primary language | Rust (stable toolchain) |
| Build tool | Cargo (standard) |
| Primary platform | Windows 10/11 (Leo's PC) |
| Also supported | macOS 12+, Linux (Ubuntu 22.04+ / Debian 12+ / Arch) |
| Cross-platform | Yes -- all projects compile and run unmodified on all three platforms |

---

## Learning Philosophy and Tone Guidelines

**For the agent generating session content, please follow these principles throughout:**

1. **Get to the code fast.** Theory and context are valuable but should be concise. Every session should have working, runnable code within the first few minutes of reading.

2. **Connect every concept to a real outcome.** Students disengage when they can't see why something matters. Each concept should be framed in terms of what it lets them build.

3. **Projects are the point.** The mini-projects at the end of each month are the emotional anchors of the course. Session content should visibly build towards them. When teaching a new concept, say explicitly: "We'll use this in the project."

4. **Respect the learner's intelligence.** Leo is 15 and interested in maths. Don't oversimplify. Use correct terminology and explain it clearly, but don't talk down to him.

5. **Make it fun.** Music, Minecraft, and maths are the lenses through which concepts should be framed wherever possible. A lesson on loops is more engaging if the example is iterating over musical notes rather than a list of numbers 1 to 10.

6. **Sample code must work.** Every code sample in every session must be complete, correct, and runnable with `cargo run`. Do not use pseudocode or omit imports. Include `Cargo.toml` snippets wherever external crates are introduced.

7. **Each session ends with a challenge.** A short, optional extension task that pushes slightly beyond the session content. No solutions provided for this one -- it's for exploration.

---

## Duke of Edinburgh Award -- Skill Section Alignment

This course is designed to serve as a complete evidence base for the Skill section of the Duke of Edinburgh Award (any level). This section explains the DofE requirements and how the course meets them. The agent generating session content must produce all DofE materials described here.

### DofE Skill Section Requirements

The DofE Skill section requires the participant to:

1. Undertake a skill regularly over a sustained period (Bronze: 3 months; Silver/Gold: longer)
2. Show **evidence of progressive improvement** from beginning to end -- not just participation
3. Have an **assessor** (who must not be a parent or guardian) confirm the activity took place and improvement was made
4. Write a **personal statement** reflecting on what they learned and how they improved
5. Keep a **log** of activity that can be reviewed

This course satisfies all five requirements:

| Requirement | How it is met |
|---|---|
| Regular activity | 2 sessions per week for 12 weeks = 24 documented sessions |
| Evidence of progression | Session logs + 3 milestone projects of increasing complexity, all with git commit history as timestamps |
| Assessor sign-off | Assessor briefing document provided; the assessor reviews the session logs and the three completed projects |
| Personal statement | Template provided in `dfe/participant-statement-template.md` |
| Activity log | Session log template provided; Leo completes one entry per session |

### Who Can Be the Assessor?

The assessor cannot be a parent or guardian. Suitable options include:
- A teacher or sixth-form tutor who can review the project outputs
- A family friend with a technical background who can verify the work is genuine
- A DofE leader at school

The assessor does **not** need to be a Rust expert. The assessor briefing document explains what to look for in plain English.

### DofE Materials the Agent Must Generate

All DofE materials live in a `dfe/` folder at the repository root. The agent must generate all of the following:

#### `dfe/README.md`
A plain-English explanation of this folder for Leo, his parents, and his assessor. Should explain:
- What DofE Skill requires
- How this course provides the evidence
- What Leo needs to fill in (session logs, milestone reflections)
- What the assessor needs to do and when

#### `dfe/session-log-template.md`
A template for a single session entry. Leo copies this and fills it in after every session. Keep it short -- it should take no more than 5-10 minutes to complete. Fields:

```markdown
## Session [N] Log

**Date:**
**Session title:**
**Time spent (approx):**

### What I did today
(2-4 sentences describing what you worked on and what you built or ran)

### What I learned
(What new concept or technique clicked for you today? Try to explain it in your own words.)

### What was hard
(What did you get stuck on, or what didn't make sense at first?)

### What I'd like to explore further
(Optional -- anything that sparked curiosity beyond the session material)
```

#### `dfe/session-log-printable.md`
A print-friendly version of the session log designed to be rendered as a PDF and printed as an A4 booklet. Leo can keep a physical folder alongside the digital repo -- DofE coordinators often appreciate something tangible, and some schools require it.

Layout requirements:
- One session per page
- Large, clearly labelled fields with lines or boxes to write in (use markdown tables with empty cells as a reasonable approximation -- the agent should note that this renders best when printed via a browser's "Print to PDF" from the GitHub markdown view)
- Fields: Session number, Date, Time spent, What I did today (5 lines), What I learned (5 lines), What was hard (3 lines), What I'd explore further (2 lines)
- A footer on each page with space for: Participant name, Assessor initials (confirming the entry was reviewed)
- The document should open with a cover page containing: participant name (blank), award level (blank), skill, start date (blank), end date (blank), and a brief one-paragraph explanation of what the document is for
- All 24 session pages should be pre-generated with the session number and title already filled in -- Leo only writes in the content fields

The agent should include a note at the top of the file explaining how to print it: open the file on GitHub, use the browser's Print function, select "Save as PDF" or a physical printer, and set paper size to A4.

#### `dfe/session-log.md`
A single running document where Leo pastes all 24 session log entries in order. Starts pre-populated with the 24 session titles and dates left blank. This is the primary evidence document.

#### `dfe/milestone-1-reflection.md`, `milestone-2-reflection.md`, `milestone-3-reflection.md`
One reflection document per mini-project, completed after Sessions 8, 16, and 24 respectively. Slightly more substantial than a session log -- 3-5 paragraphs. Template fields:

```markdown
## Milestone [N] Reflection: [Project Name]

**Date completed:**
**Link to project folder:** (GitHub link to your project in the repo)

### What I built
(Describe the project in your own words. What does it do? What does it look like when it runs?)

### Skills I used
(List the Rust concepts you applied. Try to explain what each one does in plain English.)

### The hardest part
(What was the most challenging thing to build? How did you work through it?)

### What I'm most proud of
(What moment or feature felt most satisfying?)

### How this compares to where I started
(Session 1 for Milestone 1; for Milestones 2 and 3, compare to where you were at the previous milestone)
```

#### `dfe/assessor-briefing.md`
A document written for the assessor. Must be accessible to a non-technical reader. Should cover:
- What the DofE Skill section requires and the assessor's role
- What this course is and what Leo has been learning
- What to look for as evidence of genuine improvement (working projects, increasing complexity, Leo's own reflections in his own words)
- How to verify the work is Leo's (the git commit history timestamps activity; the session logs describe specific struggles and breakthroughs that indicate authentic engagement)
- The three questions the assessor should be able to answer yes to before signing off:
  1. Did Leo undertake this activity regularly over the required period?
  2. Has Leo demonstrably improved -- can he do things now he could not do at the start?
  3. Has Leo reflected meaningfully on his learning?
- The assessor sign-off section (name, relationship to participant, contact, signature, date)

#### `dfe/participant-statement-template.md`
A guided template for Leo's final personal statement, to be written after Session 24. This is submitted to DofE alongside the assessor report. Suggested structure:

```markdown
## Participant Statement: Learning Rust Programming

**Name:**
**Award level:**
**Skill chosen:** Rust programming
**Duration:** 12 weeks (24 sessions)
**Dates:** [start] to [end]

### Why I chose this skill
...

### What I could do at the start
...

### What I can do now
...

### The projects I built
(Describe each of the three projects briefly -- what they do, what you're proud of)

### The biggest challenge I overcame
...

### What surprised me
...

### What I would do next
...
```

#### `dfe/progress-summary.md`
Auto-generated by the agent as a static summary table -- Leo does not edit this. It maps each session to the concept learned and the DofE evidence it provides. Used to give the assessor a quick overview.

```markdown
| Session | Title | Concept | Evidence |
|---|---|---|---|
| 1 | Why Rust? History and Speed | Language fundamentals, performance | Python vs Rust Pi benchmark outputs |
| 2 | Variables and Types | Core types, mutability | Running frequency calculator |
| ... | | | |
| 8 | Project 1 Complete | Full project build | music-theory-cli working program |
| ... | | | |
```

The agent should populate this table for all 24 sessions.

---

## SETUP.md Specification

`SETUP.md` is the first thing any new learner reads before touching any session content. It must be clear, friendly, and complete. The agent must generate a full version covering all three platforms. Structure and content requirements:

### Opening section
A brief welcome paragraph explaining that Rust runs identically on Windows, macOS, and Linux, and that the course projects will work without modification on any of them. The learner only needs to follow their platform's section.

### Platform sections (one each for Windows, macOS, Linux)

Each platform section must cover the following steps in order, with exact commands and any screenshots described in alt-text:

#### Windows (primary -- most detailed)

1. **Install the MSVC Build Tools** -- This is the step most tutorials skip and it causes the most confusion on Windows. Direct the learner to the Visual Studio Build Tools installer (not full Visual Studio). They need the "Desktop development with C++" workload. Explain why Rust on Windows needs this (Rust's default Windows toolchain links against the MSVC linker).
   - Note the `winget` shortcut as an alternative: `winget install Microsoft.VisualStudio.2022.BuildTools`
   - Also note: the GNU toolchain (`x86_64-pc-windows-gnu`) is an alternative that avoids this step but is not recommended for this course because some crates behave differently.
2. **Install Rust via rustup** -- `winget install Rustlang.Rustup` or download from rustup.rs. Run the installer, accept defaults.
3. **Verify installation** -- Open a new terminal (Windows Terminal recommended) and run `rustc --version` and `cargo --version`.
4. **Install VS Code** -- `winget install Microsoft.VisualStudioCode`
5. **Install rust-analyzer extension** -- Open VS Code, go to Extensions, search `rust-analyzer`, install the official extension from the Rust Programming Language group.
6. **Install Python** (needed for the Session 1 demo only) -- `winget install Python.Python.3`
7. **Test everything** -- `cargo new hello_test && cd hello_test && cargo run`. Expected output: `Hello, world!`

#### macOS

1. **Install Xcode Command Line Tools** -- `xcode-select --install`. This is mandatory before any development tooling will work on macOS. The learner may see a pop-up dialog; click Install. This takes a few minutes.
2. **Install Rust via rustup** -- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Accept defaults. Then either open a new terminal or run `source "$HOME/.cargo/env"`.
3. **Verify installation** -- `rustc --version && cargo --version`
4. **Install VS Code** -- Download from code.visualstudio.com or `brew install --cask visual-studio-code` if Homebrew is installed.
5. **Install rust-analyzer extension** -- same as Windows.
6. **Install Python** -- macOS 13+ ships with Python 3. Run `python3 --version` to confirm. If missing: `brew install python3`.
7. **Test everything** -- `cargo new hello_test && cd hello_test && cargo run`

Note for macOS users on Apple Silicon (M1/M2/M3 Macs): `rustup` installs the `aarch64-apple-darwin` target by default, which is correct and will be faster than Rosetta. All course projects are compatible.

#### Linux (Ubuntu / Debian)

1. **Install build dependencies** -- `sudo apt update && sudo apt install -y build-essential curl`. On Arch: `sudo pacman -S base-devel curl`. On Fedora: `sudo dnf groupinstall "Development Tools" && sudo dnf install curl`.
2. **Install Rust via rustup** -- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Accept defaults. Then: `source "$HOME/.cargo/env"` or open a new terminal.
3. **Verify installation** -- `rustc --version && cargo --version`
4. **Install VS Code** -- Instructions vary by distro. For Ubuntu/Debian: download the `.deb` from code.visualstudio.com and run `sudo dpkg -i code_*.deb`. For Arch: `yay -S visual-studio-code-bin`.
5. **Install rust-analyzer extension** -- same as other platforms.
6. **Install Python** -- Most Linux distros ship Python 3. Run `python3 --version`. If missing: `sudo apt install python3`.
7. **Additional dependencies for Month 3 (audio/MIDI)** -- On Linux, `cpal` and `midir` require ALSA development headers. Run: `sudo apt install -y libasound2-dev`. Note this in the Month 3 README as well.
8. **Test everything** -- `cargo new hello_test && cd hello_test && cargo run`

### Common problems section

The agent must include a troubleshooting table covering the most frequent setup failures:

| Problem | Platform | Fix |
|---|---|---|
| `error: linker 'link.exe' not found` | Windows | MSVC build tools not installed or not on PATH. Re-run the Build Tools installer and ensure "Desktop development with C++" is checked. |
| `xcrun: error: invalid active developer path` | macOS | Run `xcode-select --install` |
| `error: could not find native static library 'c'` | Linux | Run `sudo apt install build-essential` |
| `alsa/asoundlib.h: No such file or directory` | Linux | Run `sudo apt install libasound2-dev` (needed for Month 3) |
| `rustc --version` not found after install | All | Close and reopen the terminal, or manually run `source "$HOME/.cargo/env"` |
| VS Code not showing rust-analyzer hints | All | Ensure the project is opened as a folder (`File > Open Folder`) not just a file. The extension needs a `Cargo.toml` in scope. |

---

## DofE Materials the Agent Must Generate

All DofE materials live in a `dfe/` folder at the repository root. The agent must generate all of the following:

```
rust-course-leo/
├── README.md                  # Course overview and how to use this repo
├── SETUP.md                   # Installing Rust, Cargo, VS Code -- Windows, macOS, Linux
├── dfe/
│   ├── README.md              # Plain-English guide to the DofE materials
│   ├── session-log.md         # Running log -- Leo fills this in after every session
│   ├── session-log-template.md
│   ├── session-log-printable.md  # Print-friendly version: one form per session, A4
│   ├── milestone-1-reflection.md
│   ├── milestone-2-reflection.md
│   ├── milestone-3-reflection.md
│   ├── assessor-briefing.md
│   ├── participant-statement-template.md
│   └── progress-summary.md    # Static table: all 24 sessions mapped to evidence
├── month-1/
│   ├── README.md              # Month 1 overview and project brief
│   ├── session-01/
│   │   ├── README.md          # Full session content
│   │   └── examples/          # Runnable code samples
│   │       ├── pi_python.py   # Python Pi demo (Session 1 only)
│   │       ├── pi_rust/       # Cargo project: Rust Pi demo
│   │       └── hello_world/   # Cargo project: Hello World
│   ├── session-02/ ...
│   ├── ...
│   └── project/
│       └── music-theory-cli/  # Complete mini-project starter + solution
├── month-2/
│   ├── README.md
│   ├── session-09/ ...
│   └── project/
│       └── world-generator/
├── month-3/
│   ├── README.md
│   ├── session-17/ ...
│   └── project/
│       └── midi-synth/
└── resources/
    ├── cheatsheet.md          # Rust syntax quick reference
    └── glossary.md            # Key terms
```

Each `session-XX/README.md` must follow the session template defined below.

---

## Session Template

Every session's `README.md` must contain all of the following sections, in this order:

```markdown
# Session N: [Title]

## What You'll Learn
A 2-3 sentence plain English summary of the session's concepts and why they matter.

## The Big Idea
A short, engaging framing of the core concept. Use an analogy, a surprising fact,
or a connection to music/maths/Minecraft. Max 1 paragraph.

## Concepts Covered
A list of the specific Rust concepts introduced or deepened in this session.

## Building Towards [Project Name]
One paragraph explaining how today's concepts connect to the current month's project.
Be specific -- "we'll use match statements to map MIDI note numbers to note names."

## Step-by-Step Walkthrough
The main teaching content. Introduce each concept with explanation, then immediately
show working Rust code. Do not introduce a concept without a code example.
All code should also exist as a runnable file in the examples/ directory.

## Common Mistakes
2-4 specific mistakes beginners make with today's concepts, with example broken code
and the corrected version. This is one of the most valuable sections.

## Session Challenge
An open-ended extension task. No solution is provided. Should be achievable
but require thinking beyond what was explicitly taught.

## Quick Reference
A mini cheat-sheet of the syntax introduced today. Useful for when Leo returns
to a session later as a reference.

## DofE Log Reminder
A one-line prompt at the end of every session reminding Leo to complete his
session log entry in `dfe/session-log.md`. Example:
> "You've finished Session N. Before you close the laptop, spend 5 minutes
> filling in your session log in `dfe/session-log.md` -- it's your DofE evidence
> and it only takes a few minutes while it's fresh."
```

---

## Month 1: Foundations

**Theme:** Learning the language through music theory.

**Month 1 Mini-Project:** `music-theory-cli` -- A command-line tool that takes a musical root note and a scale type as arguments and outputs the notes of that scale, their intervals, and a suggested chord progression. Example:

```
$ cargo run -- --root C --scale major
Scale: C Major
Notes: C  D  E  F  G  A  B
Intervals: W  W  H  W  W  W  H
Chords: Cmaj  Dmin  Emin  Fmaj  Gmaj  Amin  Bdim
```

This project is achievable with only the Month 1 concepts. It requires: string handling, enums, pattern matching, functions, and basic CLI argument parsing.

### Session Breakdown

#### Session 1: Why Rust? History, Setup, and the Speed Demo

**Special requirements for this session -- please read carefully.**

This is the first session and it sets the tone for the entire course. It must do three things:

1. **Give Leo a reason to care about Rust.** Keep the history brief (5-10 minutes of reading), but make it compelling. The angle is performance and reliability, framed for a teenager:
   - Programming languages have generations. C is the grandfather -- blazing fast but dangerous (one mistake and your whole program crashes or gets hacked). Python is the friendly modern option -- easy to read, but slow.
   - Rust is the breakthrough: it's as fast as C, but it has a system called the *borrow checker* that prevents entire categories of bugs at compile time. It literally won't let you write certain classes of broken code.
   - Rust has been voted the "most loved programming language" on Stack Overflow for 9 consecutive years.
   - Real-world uses: the Linux kernel, Windows kernel components, Firefox, Cloudflare, Amazon, Discord (rewrote performance-critical systems from Go to Rust), and the entire WebAssembly ecosystem.
   - The key message: "Other languages let you make mistakes. Rust makes the compiler your co-pilot."

2. **Show the speed difference immediately.** Provide both of the following as runnable files:

   **`pi_python.py`** -- Monte Carlo Pi estimation, 100 million iterations:
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

   **`pi_rust/src/main.rs`** -- Identical algorithm in Rust:
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
       let n = 100_000_000;
       let start = Instant::now();
       let result = estimate_pi(n);
       let elapsed = start.elapsed();
       println!("Pi ≈ {:.6}", result);
       println!("Time: {:.2?}", elapsed);
   }
   ```

   Use the `fastrand` crate for the Rust version (add to `Cargo.toml`). The expected results on a modern PC are approximately 35-60 seconds for Python and under 1 second for Rust. The agent should include a callout box noting the typical speedup factor and explaining *why* this happens (compiled vs interpreted, native types vs Python objects, no GIL overhead).

   The Monte Carlo method itself should be briefly explained: randomly sampling points inside a unit square and checking if they fall inside a quarter-circle. The ratio gives Pi. This is elegant maths and Leo will appreciate it.

3. **Get Leo writing Rust immediately.** After the speed demo, walk through installing Rust via `rustup`, then `cargo new hello_world`, then a first Hello World. Keep this brisk -- he's already motivated.

**Concepts introduced:** `cargo new`, `cargo run`, `fn main()`, `println!`, basic program structure. Do NOT go into ownership, types, or anything advanced -- just enough to run something.

---

#### Session 2: Variables, Types, and Mutability

**Concepts:** `let`, `let mut`, scalar types (`i32`, `u64`, `f64`, `bool`, `char`), type inference, shadowing, integer overflow in debug vs release builds.

**Music framing:** Use musical note frequencies as the example domain. The frequency of A4 is 440 Hz. The frequency of each semitone is the previous multiplied by the twelfth root of 2. Calculate and print a chromatic scale of frequencies.

**Common mistakes to cover:** Forgetting `mut`, confusing shadowing with mutation, integer type mismatches in arithmetic.

---

#### Session 3: Functions, Expressions, and Basic I/O

**Concepts:** Defining functions with `fn`, parameters and return types, expressions vs statements (Rust's implicit return), reading from stdin with `std::io`, converting strings to numbers with `.parse()`, `expect()` for basic error handling.

**Music framing:** Write a function that takes a MIDI note number (0-127) and returns the frequency in Hz. MIDI note 69 = A4 = 440 Hz. Formula: `440.0 * 2.0_f64.powf((note - 69) as f64 / 12.0)`. Prompt the user to enter a MIDI note number and print the frequency. This is genuinely useful -- and his MIDI keyboard sends exactly these numbers.

---

#### Session 4: Control Flow

**Concepts:** `if`/`else if`/`else`, `loop`, `while`, `for` with ranges, `break` and `continue`, returning values from `loop`.

**Music framing:** Print a piano keyboard ASCII diagram. Use loops to iterate over octaves and notes. Use `if` to determine whether a note is black or white (the pattern repeats: white keys are at semitone positions 0,2,4,5,7,9,11 in each octave).

---

#### Session 5: Pattern Matching and `match`

**Concepts:** The `match` expression, exhaustive matching, match guards, `_` wildcard, matching on integers and ranges, using `match` as an expression.

**Music framing:** Map MIDI note numbers to note names. The 12 semitones in an octave have names: C, C#, D, D#, E, F, F#, G, G#, A, A#, B. Use `match` on `note % 12` to return the name. This is a direct component of the Month 1 project.

---

#### Session 6: Enums and Strings

**Concepts:** Defining `enum`, using enums with `match`, `String` vs `&str`, string slices, common string methods (`.to_uppercase()`, `.contains()`, `.split()`), string formatting with `format!`.

**Music framing:** Define a `ScaleType` enum with variants `Major`, `NaturalMinor`, `PentatonicMajor`. Define a `NoteName` enum with all 12 chromatic notes. Write a function that takes a root `NoteName` and a `ScaleType` and returns a `Vec<NoteName>` (preview `Vec` briefly -- full treatment in Month 2). The student can hardcode the interval patterns for each scale type using arrays of semitone steps.

**Note to agent:** This session is doing double duty -- it completes the core knowledge needed for the mini-project AND introduces `Vec` briefly. Keep the `Vec` introduction minimal; just enough to use it as a return type. Full collections coverage is Month 2.

---

#### Session 7: Mini-Project Build Part 1 -- Scale Generator

This is a project session, not a teaching session. There is no new material.

**Goal:** Build the core of `music-theory-cli`. By the end of this session, the tool should:
- Accept command-line arguments for root note and scale type (use `std::env::args()` -- no external crate yet)
- Parse those arguments into `NoteName` and `ScaleType` enums
- Compute and print the correct notes of the scale

**Guidance the agent should provide:**
- A suggested project structure (single `main.rs` is fine)
- Worked approach for parsing CLI args from `std::env::args()`
- Reminder of the `match` and enum patterns from Sessions 5-6
- A complete working solution in a `solution/` subdirectory (clearly labelled as a spoiler)
- A checklist of what "done" looks like

---

#### Session 8: Mini-Project Build Part 2 -- Chord Progressions and Polish

**Goal:** Complete `music-theory-cli`. Add:
- Display of intervals (Whole/Half step pattern for the scale)
- Display of the diatonic chord quality for each scale degree (major, minor, diminished)
- Improved error handling for bad inputs (invalid note name, unknown scale type) -- use `eprintln!` and `std::process::exit(1)` for now
- Optional: colour output using the `colored` crate (introduce `Cargo.toml` dependencies briefly as a preview of Month 2)

**New concept (brief):** Introduce `Cargo.toml` `[dependencies]` and `cargo add` as a quick preview. Do not go deep -- this gets full treatment in Month 3.

---

## Month 2: Intermediate

**Theme:** Structuring data and building systems.

**Month 2 Mini-Project:** `world-generator` -- A seed-based procedural terrain generator that outputs an ASCII map to the terminal. The user provides a seed number; the generator deterministically produces a grid of terrain tiles (ocean, plains, forest, mountains, desert) using a simple noise algorithm. Different seeds produce different worlds.

```
$ cargo run -- --seed 42 --width 60 --height 30

Seed: 42  |  World: 60x30
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~~~~~▒▒▒▒▒▒▒▒▒~~~~~~~~~~~~~~~~~~~~▒▒▒▒~~~~~~~~~~~~~~~~~~~~
~~~▒▒▒▒▒▒▓▓▓▓▓▒▒▒~~~~~~~~~~~~~~~~▒▒▓▓▒▒~~~~~~~~~~~~~~~~~~~
~~▒▒▒▒▓▓▓▓▲▲▲▓▓▒▒~~~~~~~~~~~~~~~▒▒▒▒▓▓▒~~~~~~~~~~~~~~~~~~~
[... etc]

Legend: ~ Ocean  ▒ Plains  ▓ Forest  ▲ Mountains  . Desert
```

This project requires structs, enums with data, `Vec<Vec<T>>`, iterators, and a simple hash/noise function. It deliberately echoes Minecraft's world generation.

### Session Breakdown

#### Session 9: Structs and Methods

**Concepts:** Defining `struct`, creating instances, accessing fields, `impl` blocks, methods (`&self`, `&mut self`), associated functions (constructors like `::new()`), `#[derive(Debug)]`.

**Minecraft framing:** A `Block` struct with fields for block type, position (x, y, z), and hardness. Implement methods to check if a block is solid, to get its display character, and a `new()` constructor.

---

#### Session 10: Enums with Data and `Option<T>`

**Concepts:** Enum variants that carry data, `Option<T>`, `Some()` and `None`, pattern matching on `Option`, `.unwrap()`, `.unwrap_or()`, `.is_some()`, `.is_none()`.

**Framing:** A `Tile` enum where each variant carries different data: `Ocean` has depth, `Mountain` has height, `Forest` has tree density. Introduce `Option` by writing a function that searches a grid for the first mountain tile and returns `Option<(usize, usize)>`.

---

#### Session 11: Collections -- `Vec` and `HashMap`

**Concepts:** `Vec<T>` in depth (push, pop, indexing, slicing), iterating over `Vec`, `HashMap<K, V>` (insert, get, contains_key), when to use which.

**Framing:** Store a world grid as `Vec<Vec<Tile>>`. Use a `HashMap` to store biome statistics (how many tiles of each type). Build a function to initialise a flat world grid.

---

#### Session 12: Iterators and Closures (Introduction)

**Concepts:** The iterator trait, `.iter()`, `.map()`, `.filter()`, `.collect()`, `.for_each()`, `.enumerate()`, basic closure syntax `|x| x + 1`.

**Framing:** Use iterators to process the world grid: count tiles of each type, find all mountain tiles, transform a row of tiles into display characters for rendering.

---

#### Session 13: Error Handling

**Concepts:** `Result<T, E>`, the `?` operator, `unwrap()` vs proper error handling, `eprintln!`, creating simple custom error types with an enum, returning `Result` from `main()`.

**Framing:** Parse the seed from CLI args properly using `Result`. Handle the case where the user passes a non-numeric seed. Introduce the pattern of propagating errors up with `?`.

---

#### Session 14: Traits

**Concepts:** Defining a trait, implementing a trait for a type, `Display` from `std::fmt`, `Debug`, `Clone`, `Copy`, trait bounds in function signatures.

**Framing:** Implement `Display` for `Tile` so it renders as its ASCII character. Implement a custom `Describable` trait with a `describe() -> String` method on both `Tile` and a `World` struct.

---

#### Session 15: Mini-Project Build Part 1 -- World Core

**Goal:** Build the procedural generation engine. By the end of this session:
- A `World` struct exists with a 2D `Vec<Vec<Tile>>` grid
- Terrain is generated from a seed using a simple deterministic noise function

**Noise function guidance (agent must include this):** Provide a worked implementation of a simple hash-based noise function that doesn't require external crates. A Linear Congruential Generator (LCG) seeded with a combination of the world seed and tile coordinates works well:

```rust
fn hash(seed: u64, x: usize, y: usize) -> f64 {
    let mut h = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(x as u64)
        .wrapping_mul(2891336453)
        .wrapping_add(y as u64);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    (h as f64) / (u64::MAX as f64)
}
```

Explain this step by step -- it's a great opportunity to talk about hash functions, determinism, and why game world generation works the way it does.

---

#### Session 16: Mini-Project Build Part 2 -- Render and Polish

**Goal:** Complete `world-generator`:
- Render the world to the terminal with correct ASCII characters
- Add a legend
- Add multiple biome types (ocean, plains, forest, mountains, desert) with appropriate thresholds
- Show world statistics (biome counts)
- Optional: colour output per biome using the `colored` crate

**Complete working solution** provided in `solution/`.

---

## Month 3: Advanced Concepts and Final Project

**Theme:** The ecosystem, advanced patterns, and building something real.

**Final Project:** `midi-synth` -- A MIDI synthesiser that reads MIDI note data (from either a `.mid` file or live input from a connected MIDI keyboard) and either plays audio in real-time or renders a WAV file. Leo can plug in his MIDI keyboard and hear the notes as synthesised audio generated entirely by his own Rust code.

This project is ambitious but achievable in 4 sessions given the groundwork laid in Months 1 and 2. It uses real external crates and produces real, tangible output.

**Crates to use:**
- `hound` -- WAV file reading and writing
- `midir` -- MIDI input from hardware (cross-platform)
- `midly` -- MIDI file parsing
- `cpal` -- cross-platform audio output (for live playback, stretch goal)
- `fastrand` -- used in Session 1 demo

### Session Breakdown

#### Session 17: The Ecosystem -- Modules, Crates, and Cargo

**Concepts:** Organising code with `mod`, `pub`, `use`, multi-file projects (module in separate file), `Cargo.toml` in depth (dependencies, features, dev-dependencies), `cargo add`, `docs.rs`, searching crates on `crates.io`.

**Framing:** Refactor the `music-theory-cli` from Month 1 into a properly structured multi-module project. This is a great exercise in seeing how Rust scales. Create `src/notes.rs`, `src/scales.rs`, and `src/main.rs`.

---

#### Session 18: File I/O and Binary Data

**Concepts:** `std::fs` (read_to_string, write, create), `BufReader`, `BufWriter`, reading and writing binary files, understanding bytes and byte arrays, basic WAV file structure.

**Special content for this session -- agent must include this:**

Explain the WAV file format at a byte level. This is the foundation of the final project:

- WAV is a container format: a header followed by raw audio sample data
- Header fields: "RIFF" magic bytes, file size, "WAVE" marker, format chunk (PCM, sample rate, bit depth, channels), data chunk
- Provide a minimal hand-written WAV header generator in Rust (no external crates) that writes a 1-second sine wave at 440 Hz as a valid `.wav` file
- The student should be able to open this file in Windows Media Player, Audacity, or VLC and hear a tone

Provide the complete sample rate / frequency / amplitude / sample calculation walkthrough. This is genuine digital signal processing and Leo will find it fascinating.

---

#### Session 19: Closures and Iterators (Deep Dive)

**Concepts:** Closure captures (by reference, by value with `move`), `Fn`, `FnMut`, `FnOnce`, iterator adaptors in depth (`.zip()`, `.flat_map()`, `.chain()`, `.take()`, `.skip()`), lazy evaluation, `Iterator::collect()` into different types, writing a custom iterator with `impl Iterator`.

**Music framing:** Build an infinite iterator that generates audio samples for a given waveform type (sine, square, sawtooth). Use `move` closures to capture frequency and sample rate. Use `.take(sample_rate)` to get exactly one second of audio. This is a direct building block for the final project.

---

#### Session 20: Generics and Advanced Traits

**Concepts:** Generic functions and structs (`<T>`), trait bounds (`T: Display + Clone`), `impl Trait` in function arguments and return types, `Box<dyn Trait>` for dynamic dispatch, lifetime basics (just enough to understand `'a` when the compiler asks for it).

**Framing:** Build a generic `Oscillator<T>` struct where `T` constrains the output type. Use `Box<dyn Iterator<Item = f32>>` to store different waveform generators behind a common interface. This sets up the synthesiser architecture.

---

#### Session 21: Final Project Session 1 -- WAV Synthesis Engine

**Goal:** Build the audio synthesis core of `midi-synth`:
- `Waveform` enum: `Sine`, `Square`, `Sawtooth`, `Triangle`
- Function to generate a `Vec<f32>` of audio samples for a given note (frequency), duration (seconds), waveform type, and sample rate
- ADSR envelope function (Attack, Decay, Sustain, Release) -- apply a simple linear amplitude envelope to avoid clicks
- Write the output to a WAV file using the `hound` crate

By the end of this session, `cargo run -- --note 69 --duration 2 --waveform sine` should produce an `output.wav` file containing a 2-second A4 sine tone that plays correctly.

**ADSR explanation:** The agent must explain what an ADSR envelope is and why it's needed. A raw sine wave switched on and off instantly produces a click. An envelope shapes the amplitude over time. For a simple implementation:
- Attack: linear ramp from 0 to 1 over first N milliseconds
- Decay: ramp from 1 to sustain level
- Sustain: held amplitude while note is "held"
- Release: ramp from sustain to 0

---

#### Session 22: Final Project Session 2 -- MIDI File Parsing

**Goal:** Parse a MIDI file and render it to WAV:
- Use the `midly` crate to parse a `.mid` file
- Extract note-on and note-off events with timing information
- Convert MIDI ticks to seconds (requires reading the MIDI tempo)
- For each note event, generate audio samples and mix them into a single output buffer
- Write the complete mix to WAV

**Agent guidance:** MIDI timing is confusing the first time. Provide a clear worked explanation of:
- MIDI ticks and what "ticks per quarter note" (TPQN) means
- The relationship between tempo (microseconds per beat) and tick duration
- Simple mixing: adding two `f32` sample buffers together and clamping to [-1.0, 1.0]

Provide a sample `.mid` file in the session directory (a simple melody -- something recognisable like a C major scale or Twinkle Twinkle, 4-8 notes) for testing.

---

#### Session 23: Final Project Session 3 -- Live MIDI Input

**Goal:** Connect to Leo's MIDI keyboard and play synthesised audio in real-time:
- Use `midir` to enumerate MIDI input ports and connect to the keyboard
- Receive note-on and note-off events in a callback
- Use `cpal` to open an audio output stream
- When a note-on is received, start generating samples for that note; on note-off, release the envelope

**Agent note:** Real-time audio is harder than file output because it requires careful threading. The agent should:
- Explain the producer/consumer model (MIDI thread puts note events into a channel, audio thread reads from it)
- Use `std::sync::mpsc` for the channel
- Provide a complete working implementation -- this is the most complex code in the course and should be fully worked through, not left as an exercise

If real-time audio proves too complex for the session scope, the fallback is: capture MIDI input for N seconds to a log, then render to WAV. The agent should try the real-time path first.

---

#### Session 24: Final Project Session 4 -- Polish, Showcase, and What Next

**Goal:** Complete and polish `midi-synth` for showcase:
- Add a polished CLI (use `clap` crate -- introduce it briefly)
- Support multiple simultaneous notes (simple polyphony by mixing active oscillator outputs)
- Add a `--chord` mode that plays common chords (major, minor, 7th) from a root note
- Print a summary of notes played

**Celebration section (required):** The final section of this session must be a "What You've Built" retrospective that lists every meaningful Rust concept Leo has learned and every project he's completed. This is for his DofE record and his own satisfaction.

**What's next section (required):** Brief, exciting pointers to where Rust goes next:
- Web development with Axum or Actix
- Game development with Bevy
- Systems programming and OS development
- WebAssembly (Rust compiles to WASM -- it runs in browsers)
- Embedded systems (Rust runs on microcontrollers)
- Contributing to open source Rust projects

---

## Cross-Cutting Requirements for the Agent

### Code Quality Standards

All sample code must:
- Compile with `cargo build` on the stable Rust toolchain (1.75+)
- Pass `cargo clippy` with no warnings (include `#![allow(...)]` only if genuinely necessary with a comment explaining why)
- Use idiomatic Rust patterns -- do not write Java or Python in Rust syntax
- Include inline comments for non-obvious sections
- Use descriptive variable names

### Crate Usage Policy

Use external crates where they genuinely help (e.g. `hound`, `midir`, `midly`, `colored`, `clap`). Do not introduce crates just to avoid writing 10 lines of standard library code. Always provide the `Cargo.toml` entry when a new crate is first used.

### Musical Accuracy

The music theory in Sessions 1-8 and the final project must be accurate:
- Semitone intervals for common scales: major (2,2,1,2,2,2,1), natural minor (2,1,2,2,1,2,2), pentatonic major (2,2,3,2,3)
- Diatonic chord qualities in major: I maj, II min, III min, IV maj, V maj, VI min, VII dim
- Frequency formula for MIDI note number n: `440.0 * 2.0^((n - 69) / 12.0)`
- MIDI note numbers: middle C = 60, A4 = 69

### GitHub Repository Hygiene

The generated repository should include:
- `README.md` at root with: what the course is, who it's for, how to use it, prerequisites, licence
- `SETUP.md` with step-by-step Rust and tooling installation for Windows
- `LICENSE` file (MIT recommended for open source)
- `.gitignore` for Rust projects (standard `target/` exclusion)
- Each project should have its own `README.md` that works as a standalone brief

---

## Summary: The Three Projects

| Month | Project | Core Skills Demonstrated | "Wow Factor" |
|---|---|---|---|
| 1 | `music-theory-cli` | Enums, pattern matching, functions, strings | Types out a scale and chord progression in the terminal |
| 2 | `world-generator` | Structs, traits, collections, iterators, noise | Generates a different Minecraft-style world from every seed |
| 3 | `midi-synth` | Crates, file I/O, closures, traits, threads | Plug in a MIDI keyboard, hear your own synthesiser respond |
