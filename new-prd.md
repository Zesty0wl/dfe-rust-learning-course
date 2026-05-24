# PRD: Learn Rust with Leo
### A 24-Session Rust Programming Course for a 15-Year-Old with Interests in Chemistry, Maths, and Computer Games

---

## Document Purpose

This PRD is a brief for an AI course-generation agent. It defines the complete framework, learning philosophy, session structure, and project specifications for a 24-session Rust programming course. The agent's job is to fill in this framework with full session content: explanations, worked examples, sample code, exercises, and project guidance.

The course will live as an open-source GitHub repository so that others can use and adapt it.

---

## Learner Profile

- **Name:** Leo
- **Age:** 15
- **Experience:** Beginner programmer. Assume **zero prior knowledge** of any specific programming language. The generated course must stand on its own without Python, JavaScript, or any other language as a reference point.
- **Interests:** Chemistry, Mathematics, Computer Games
- **Hardware:** Powerful Windows PC
- **Additional context:** Leo has ADHD. Every session must deliver a visible, tangible dopamine hit -- something on screen that moves, reacts, or surprises. Concepts must never be taught in a vacuum. Engagement is not optional; it is the primary design constraint.
- **Goal:** Complete a structured, skill-building Rust course over 12 weeks as part of a Duke of Edinburgh Bronze Award skill requirement
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

5. **Make it fun, and make it visible.** Chemistry, maths, and games are the lenses through which every concept should be framed. A lesson on loops is more engaging when the example is simulating gravity than printing numbers 1 to 10. Every session must produce output that can be seen moving or changing on screen -- this is non-negotiable.

6. **Design for ADHD.** Every session must open with a single clear sentence stating what the learner will have built by the end. Concepts must be introduced mid-build, not before it. There must be at least one runnable intermediate state during the session -- not just at the end, and this first checkpoint should arrive in the **first third of the session**. Each session must have a named, specific "wow moment" that the agent calls out explicitly in the session content.

7. **Sample code must work.** Every code sample in every session must be complete, correct, and runnable with `cargo run`. Do not use pseudocode or omit imports. Include `Cargo.toml` snippets wherever external crates are introduced.

8. **Each session ends with a challenge.** A short, optional extension task that pushes slightly beyond the session content. No solutions provided for this one -- it's for exploration.

9. **Voice rule.** Generated session prose addresses the reader in the **second person** ("you"). Do **not** use the learner's name ("Leo") in generated session content -- it appears only in this PRD and in `dfe/README.md`. This keeps the published course usable by any DofE participant who forks the repo.

10. **Each session ships a `starter/` snapshot.** At the start of every session, the learner should be able to copy a known-good Cargo project from `month-X/session-NN/starter/` and pick up where the previous session ended. This is the safety net for a single-project arc: if Session N's refactor breaks the sim, the learner can roll back to `session-N/starter/` and try again. A matching `solution/` shows the finished state.

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
| 1 | A Window, a Grid, and Your First Pixel | macroquad window, Vec<Vec<u8>> grid, mouse input | Black window where clicking draws coloured pixels |
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
6. **Test everything** -- `cargo new hello_test && cd hello_test && cargo run`. Expected output: `Hello, world!`

#### macOS

1. **Install Xcode Command Line Tools** -- `xcode-select --install`. This is mandatory before any development tooling will work on macOS. The learner may see a pop-up dialog; click Install. This takes a few minutes.
2. **Install Rust via rustup** -- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Accept defaults. Then either open a new terminal or run `source "$HOME/.cargo/env"`.
3. **Verify installation** -- `rustc --version && cargo --version`
4. **Install VS Code** -- Download from code.visualstudio.com or `brew install --cask visual-studio-code` if Homebrew is installed.
5. **Install rust-analyzer extension** -- same as Windows.
6. **Test everything** -- `cargo new hello_test && cd hello_test && cargo run`

Note for macOS users on Apple Silicon (M1/M2/M3 Macs): `rustup` installs the `aarch64-apple-darwin` target by default, which is correct and will be faster than Rosetta. All course projects are compatible.

#### Linux (Ubuntu / Debian)

1. **Install build dependencies** -- `sudo apt update && sudo apt install -y build-essential curl`. On Arch: `sudo pacman -S base-devel curl`. On Fedora: `sudo dnf groupinstall "Development Tools" && sudo dnf install curl`.
2. **Install Rust via rustup** -- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Accept defaults. Then: `source "$HOME/.cargo/env"` or open a new terminal.
3. **Verify installation** -- `rustc --version && cargo --version`
4. **Install VS Code** -- Instructions vary by distro. For Ubuntu/Debian: download the `.deb` from code.visualstudio.com and run `sudo dpkg -i code_*.deb`. For Arch: `yay -S visual-studio-code-bin`.
5. **Install rust-analyzer extension** -- same as other platforms.
6. **Additional dependencies for macroquad (Month 1 onwards)** -- On Linux, `macroquad` is a windowed graphics application and needs the standard X11 + OpenGL development headers. Run: `sudo apt install -y libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev pkg-config`. (Equivalent on Arch: `sudo pacman -S libx11 libxi mesa alsa-lib pkgconf`. On Fedora: `sudo dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel pkgconf`.) `libasound2-dev` is also needed if sound effects are added in Month 2/3 milestones.
7. **Test everything** -- `cargo new hello_test && cd hello_test && cargo run`

### Common problems section

The agent must include a troubleshooting table covering the most frequent setup failures:

| Problem | Platform | Fix |
|---|---|---|
| `error: linker 'link.exe' not found` | Windows | MSVC build tools not installed or not on PATH. Re-run the Build Tools installer and ensure "Desktop development with C++" is checked. |
| `xcrun: error: invalid active developer path` | macOS | Run `xcode-select --install` |
| `error: could not find native static library 'c'` | Linux | Run `sudo apt install build-essential` |
| `alsa/asoundlib.h: No such file or directory` | Linux | Run `sudo apt install libasound2-dev` (needed once audio is added) |
| `cannot find -lX11` / `cannot find -lGL` | Linux | Run `sudo apt install libx11-dev libxi-dev libgl1-mesa-dev pkg-config` (macroquad runtime deps) |
| Black window with no input on Wayland | Linux | macroquad uses X11 -- run under XWayland (set `WAYLAND_DISPLAY=` for the test) or install an X11 session |
| `rustc --version` not found after install | All | Close and reopen the terminal, or manually run `source "$HOME/.cargo/env"` |
| VS Code not showing rust-analyzer hints | All | Ensure the project is opened as a folder (`File > Open Folder`) not just a file. The extension needs a `Cargo.toml` in scope. |

---

## Proposed Repository Layout

All DofE materials live in a `dfe/` folder at the repository root. The full repository tree after generation:

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
│   ├── README.md              # Month 1 overview and milestone brief
│   ├── session-01/
│   │   ├── README.md          # Full session content
│   │   ├── starter/           # Cargo project: state at the START of this session
│   │   ├── solution/          # Cargo project: state at the END of this session
│   │   └── examples/          # Optional small focused samples (one concept each)
│   ├── session-02/ ...
│   ├── ...
│   └── milestone/
│       └── sand-sim-v0.1/     # Complete milestone build: starter + solution
├── month-2/
│   ├── README.md
│   ├── session-09/ ...
│   └── milestone/
│       └── sand-sim-v0.2/
├── month-3/
│   ├── README.md
│   ├── session-17/ ...
│   └── milestone/
│       └── sand-sim-v1.0/
├── CHEMISTRY-PRIMER.md        # Combustion, phase change, oxidation -- 10-min read
└── resources/
    ├── cheatsheet.md          # Rust syntax quick reference
    └── glossary.md            # Key terms including simulation and chemistry vocabulary
```

Each `session-XX/README.md` must follow the session template defined below.

---

## Session Template

Every session's `README.md` must contain all of the following sections, in this order:

```markdown
# Session N: [Title]

## The Goal
One sentence only. "By the end of this session you will have [specific visible thing]."
This appears at the very top, before any explanation. It is the dopamine contract.

## What You'll Learn
A 2-3 sentence plain English summary of the session's concepts and why they matter.

## The Big Idea
A short, engaging framing of the core concept. Use an analogy, a surprising fact,
or a connection to chemistry/maths/games. Max 1 paragraph.

## Concepts Covered
A list of the specific Rust concepts introduced or deepened in this session.

## Building Towards [Project Name]
One paragraph explaining how today's concepts connect to the current month's project.
Be specific -- "we'll use structs to represent each cell in the simulation grid."

## Step-by-Step Walkthrough
The main teaching content. Concepts are introduced mid-build, not before it.
Each concept is introduced immediately before it is needed. Every concept has a
working code example. There must be at least one intermediate runnable state --
a checkpoint where Leo can `cargo run` and see something happen before the
session is complete.

## Wow Moment
A explicitly called-out moment in the session designed to surprise or delight.
Named and described specifically. Examples: "This is the moment fire spreads for
the first time." "Run this now -- watch sand pile up naturally from diagonal spreading."
The agent must design this moment deliberately, not accidentally.

## Common Mistakes
2-4 specific mistakes beginners make with today's concepts, with example broken code
and the corrected version.

## Session Challenge
An open-ended extension task. No solution is provided.

## Quick Reference
A mini cheat-sheet of the syntax introduced today.

## DofE Log Reminder
A one-line prompt at the end of every session reminding Leo to complete his
session log entry in `dfe/session-log.md`. Example:
> "You've finished Session N. Before you close the laptop, spend 5 minutes
> filling in your session log in `dfe/session-log.md` -- it's your DofE evidence
> and it only takes a few minutes while it's fresh."
```

---

## Month 1: The Engine

**Theme:** Build a real-time particle simulation from scratch. Every session adds a new element or behaviour. By the end of the month it looks and feels like a toy worth playing with.

**Framework:** macroquad -- a simple, beginner-friendly 2D game framework for Rust. Introduced in Session 1 alongside the language basics. `cargo add macroquad` is the only dependency for the entire month.

**Month 1 Milestone:** `sand-sim` v0.1 -- a real-time falling sand sandbox with three elements (sand, water, stone), a working physics engine, a brush tool for drawing, and an element selector. Runs at 60fps in a window. Visually impressive enough to show someone who knows nothing about programming.

**Rust concepts covered across Month 1:** `fn main`, `println!`, `let`/`let mut`, scalar types, type inference, `if`/`else`, `loop`/`while`/`for`, `match`, `enum`, `Vec`, 2D array indexing, basic structs, `cargo add`.

### Session Breakdown

---

#### Session 1: A Window, a Grid, and Your First Pixel

**The Goal:** By the end of this session you will have a black window open on your screen where clicking draws coloured dots.

**Special requirements -- agent must read carefully.**

Do **not** assume any prior programming experience or any other language as a reference point. The reader may never have written code before. Open Session 1 with a short hook (no benchmarks, no comparisons to other languages):

> **Today you open a window and start drawing on it.** Over the next 12 weeks you'll turn that window into a real-time simulation that runs sixty times every second -- sand falling, fire spreading, recipes to discover. Let's go.

Then move immediately into building something visual.

**Content:**

1. Brief framing (5 minutes max): what we're building over 12 weeks. Show a description or GIF of a falling sand alchemy game -- this is the destination. Name the three milestones: physics sandbox, chemistry sandbox, alchemy discovery game.

2. `cargo new sand-sim`, then `cargo add macroquad`. Show the `Cargo.toml` result. Explain what a crate is in one sentence.

3. Minimal macroquad window -- the smallest possible program that opens a window and clears it to black each frame:

```rust
use macroquad::prelude::*;

#[macroquad::main("Sand Sim")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
```

Explain `async`/`await` in one sentence: "macroquad uses this to hand control back to the OS each frame -- don't worry about it yet, just know it needs to be there."

4. Add a grid. Introduce a 2D `Vec<Vec<u8>>` to represent the simulation grid. Each cell is `0` (empty) for now. Walk through why we use `width * height` cells and how `grid[y][x]` indexing works. Connect this explicitly to the maths of 2D arrays.

5. Draw the grid. Iterate over all cells and call `draw_rectangle` for each non-zero cell. Nothing moves yet -- but clicking should draw sand-coloured rectangles.

6. Handle mouse input with `is_mouse_button_down(MouseButton::Left)` and convert mouse position to grid coordinates.

**Wow moment:** "Click on the window. You're drawing on a grid that Rust is rendering 60 times a second." Label this explicitly in the session.

**Concepts introduced:** `cargo new`, `cargo add`, `fn main`, `async fn main`, `loop`, basic `Vec<Vec<u8>>`, `draw_rectangle`, mouse input.

---

#### Session 2: Variables, Types, and Giving Sand a Colour

**The Goal:** By the end of this session the grid will store typed elements, each element will have its own colour, and clicking will draw the currently selected element.

**Concepts:** `let`, `let mut`, scalar types (`u8`, `usize`, `f32`), type inference, constants (`const`), named colours, basic `if`/`else` to branch on cell type.

**Simulation framing:** Replace the raw `u8` grid with named constants: `const EMPTY: u8 = 0`, `const SAND: u8 = 1`, `const WATER: u8 = 2`, `const STONE: u8 = 3`. Write a `cell_colour(cell: u8) -> Color` function using `if`/`else if` to return the right colour for each type. Clicking now draws whichever element is currently selected (hardcode `SAND` for now).

**Maths connection:** Explain why `usize` is used for grid indices and `u8` for cell types -- different domains need different numeric types. This is one of the things Rust insists on that many other languages don't, and the reason becomes clearer once you start seeing what the compiler can prove for you.

**Wow moment:** Change the `SAND` colour constant to something vivid and watch the whole grid update instantly. The colour is defined in one place.

---

#### Session 3: Sand Falls

**The Goal:** By the end of this session sand will fall under gravity and pile up naturally.

**Concepts:** Nested `for` loops, grid coordinate arithmetic, swapping values in a `Vec`, why we iterate bottom-to-top for gravity, `fastrand` for diagonal spread.

**Simulation framing:** The gravity update rule: for each sand cell, check if the cell directly below is empty. If so, swap them. Iterate the grid from bottom to top so falling sand doesn't get updated twice in one frame. Introduce `fastrand` to add diagonal spread: if the cell below is blocked, try bottom-left or bottom-right in random order.

**Maths connection:** Why bottom-to-top iteration matters -- walk through what goes wrong if you iterate top-to-bottom. This is a beautiful example of algorithm correctness depending on traversal order.

**Wow moment:** "Click and hold to pour sand. Watch it fall and pile. The pile has a natural angle of repose from the diagonal spreading. You just implemented emergent behaviour." Label this explicitly.

**Note to agent:** This is one of the most satisfying moments in the entire course. Give it room. Let the session breathe here.

---

#### Session 4: Control Flow and a Better Update Loop

**The Goal:** By the end of this session the simulation will have a clean, well-structured update function and sand will stop falling when it reaches the bottom edge.

**Concepts:** `if`/`else if`/`else` in depth, `match` preview, bounds checking with `if x > 0 && x < width - 1`, `continue` and `break`, refactoring the update logic into a separate `fn update_grid()`.

**Simulation framing:** The grid update loop is getting complex. Refactor it into a dedicated function. Add proper bounds checking so sand doesn't panic when it reaches the edges. Add a `match` on cell type as a preview -- the full treatment comes next session.

**Maths connection:** Boundary conditions -- a concept that appears in differential equations, cellular automata, and physics simulations. The grid edge is a boundary condition. Discuss briefly.

---

#### Session 5: Pattern Matching and Multiple Elements

**The Goal:** By the end of this session the simulation will have sand, water, and stone, each with different physics rules, all handled by a clean `match` statement.

**Concepts:** `match` expressions, exhaustive matching, `_` wildcard, match as an expression, matching on constants.

**Simulation framing:** Replace the `if`/`else if` chain in the update loop with a `match cell_type { SAND => ..., WATER => ..., STONE => ..., _ => {} }`. Water rule: flows sideways as well as down (tries below, then below-left, then below-right, then left, then right). Stone rule: does nothing (static).

**Wow moment:** "Add stone walls and pour water behind them. Watch it fill up. Pour sand on top of the water. Sand sinks through water because sand is 'heavier'." This emergent density behaviour comes naturally from update order.

---

#### Session 6: Enums -- Giving Elements Proper Names

**The Goal:** By the end of this session the simulation will use a proper `CellType` enum instead of magic numbers, and the code will be noticeably cleaner and safer.

**Concepts:** Defining `enum`, using enums with `match`, `#[derive(Debug, Clone, Copy, PartialEq)]`, why enums beat magic numbers (the compiler catches missing cases), converting between enum and integer for the grid storage.

**Simulation framing:** Replace `const EMPTY: u8 = 0` etc. with a `CellType` enum. Update the grid type to `Vec<Vec<CellType>>`. Update all `match` arms. The compiler will immediately flag any cases that weren't handled. This is the borrow checker being a co-pilot, not an enemy.

**Maths/chemistry connection:** An enum is a type with a finite, named set of values -- exactly like a chemical element symbol system. You can't accidentally create an element that doesn't exist.

---

#### Session 7: Project Build Part 1 -- Element Selector and Brush

**The Goal:** By the end of this session `sand-sim` will have a proper element selector UI and a variable brush size.

This is a project session. No new language concepts. The focus is on building, connecting existing knowledge, and polishing.

**Goals for this session:**
- An on-screen element selector (draw coloured squares in the corner for each element, highlight the selected one)
- Keyboard shortcuts to switch elements (1 = sand, 2 = water, 3 = stone)
- Variable brush size with scroll wheel (`mouse_wheel()` in macroquad)
- Draw a circle brush instead of a single cell using nested loops and the distance formula: `(dx*dx + dy*dy) <= radius*radius`

**Maths connection:** The circle brush uses the equation of a circle. Point this out -- this is `x² + y² ≤ r²` from geometry, running 60 times a second.

**Agent must provide:** A complete working solution in `solution/`, a checklist of what "done" looks like, and encouragement to experiment with brush sizes and build structures before Session 8.

---

#### Session 8: Project Build Part 2 -- Polish and Milestone

**The Goal:** By the end of this session `sand-sim` v0.1 is complete and ready to show someone.

**Goals for this session:**
- FPS counter displayed on screen
- Pause/unpause with spacebar
- Clear screen with `C` key
- Erase tool (right-click sets cells to `CellType::Empty`)
- A title and legend rendered on screen with `draw_text`
- Fix any remaining edge case bugs (sand escaping the grid, water behaving oddly at corners)

**Milestone moment (agent must include this explicitly):** End the session with a "You've built a real-time physics simulation" section. List everything it does. Tell Leo this is the engine that everything in Months 2 and 3 will be built on top of.

---

## Month 2: The Chemistry

**Theme:** Elements react with each other. Simple rules produce complex, emergent chemical behaviour. By the end of the month the sandbox feels like a chemistry set.

**Month 2 Milestone:** `sand-sim` v0.2 -- the same sandbox, but now with fire, oil, acid, steam, lava, and ice. Elements react in chains: oil ignites, heats water to steam, steam rises and cools. The whole thing is tunable and feels alive.

**Rust concepts covered across Month 2:** Structs, `impl` blocks, methods, `Option<T>`, `Vec` in depth, iterators, closures intro, `Result` and error handling basics, traits (`Display`, `Debug`, custom traits), `HashMap`.

### Session Breakdown

---

#### Session 9: Structs -- Giving Cells a Temperature

**The Goal:** By the end of this session every cell in the simulation will have a temperature property, and cells will glow warmer colours as temperature increases.

**Concepts:** Defining `struct`, fields, creating instances, `impl` blocks, methods with `&self` and `&mut self`, `#[derive(Debug, Clone, Copy)]`, associated functions (`::new()`).

**Simulation framing:** Replace `CellType` as the grid element with a `Cell` struct:
```rust
#[derive(Debug, Clone, Copy)]
struct Cell {
    cell_type: CellType,
    temperature: f32,  // 0.0 = ambient, 1.0 = very hot
}
```
Implement `Cell::new(cell_type: CellType) -> Cell` and `Cell::empty() -> Cell`. Update the rendering to blend the cell colour with a hot orange/red tint based on temperature. Nothing has a temperature yet -- but the infrastructure is there.

**Wow moment:** Temporarily set sand temperature to 0.8 in the constructor. The whole sandbox glows orange. Set it back. The infrastructure works.

---

#### Session 10: Enums with Data and Option -- Modelling Reactions

**The Goal:** By the end of this session the simulation will have a formal reaction system where elements can define what they turn into under different conditions.

**Concepts:** Enum variants with data, `Option<T>`, `Some()` and `None`, pattern matching on `Option`, `.unwrap_or()`.

**Simulation framing:** Add a `reaction_result(cell_type: CellType, neighbour: CellType) -> Option<CellType>` function that returns `Some(new_type)` if a reaction occurs or `None` if not. Start with one reaction: `WOOD` adjacent to `FIRE` → `Some(FIRE)` (wood ignites). This is the seed of the entire chemistry system.

**Chemistry connection:** Reactions have reactants and products. `Option` models the fact that not every pair of elements reacts.

---

#### Session 11: Fire

**The Goal:** By the end of this session fire will spread, burn out, and raise the temperature of nearby cells.

This is primarily a project/experimentation session. The new Rust concept is minimal -- the value is in tuning the fire behaviour until it feels right.

**New concept:** `fastrand::f32()` for probabilistic behaviour. Fire doesn't always spread -- it spreads with a probability. `if fastrand::f32() < 0.3 { spread }`. This is how you make simulations feel organic rather than mechanical.

**Simulation framing:**
- Fire rises (like a reverse sand -- moves upward preferentially)
- Fire has a lifetime stored in a `u8` field added to `Cell` -- decrements each frame, becomes `Empty` at zero
- Fire raises temperature of adjacent cells by a fixed amount per frame
- Wood cell adjacent to a cell above its ignition temperature (`temperature > 0.6`) becomes fire

**Wow moment:** "Draw a rectangle of wood. Light one corner with fire. Watch it burn." This must be called out explicitly as the session's wow moment.

---

#### Session 12: Oil and Explosive Reactions

**The Goal:** By the end of this session oil will flow like water, ignite explosively, and burn differently from wood.

**Concepts:** `Vec` in depth (push, indexing, slices), iterating over neighbours using a neighbours array pattern, why oil fire looks different from wood fire (different lifetime, different spread probability, higher temperature output).

**Simulation framing:** Oil is a liquid (flows like water) but ignites at a lower temperature than wood and burns faster and hotter. When oil ignites, it briefly raises all adjacent cells to very high temperature -- an explosive cascade. The key is tuning the numbers until the explosion looks satisfying.

**Chemistry connection:** Different fuels have different energy densities and ignition temperatures. Briefly mention this -- the simulation is a very rough model of real combustion chemistry.

**Wow moment:** "Pool oil under a stone ceiling. Drop a single fire particle in. Watch the explosion." Label it.

---

#### Session 13: Steam -- A State Change

**The Goal:** By the end of this session water will turn to steam when heated, rise, and cool back to water.

**Concepts:** Iterators intro -- `.iter()`, `.enumerate()`, `.map()`, `.filter()`. These are introduced to clean up the grid scanning code, not taught in isolation.

**Simulation framing:** Water with temperature above 0.8 becomes steam. Steam rises (like reverse sand, faster). Steam has a short lifetime and converts back to water when it expires. Steam cools surrounding cells slightly as it rises. This creates a visible convection cycle if you set up a heat source under water.

**Chemistry connection:** Phase transitions. Water → steam is an endothermic state change. The simulation models this as a temperature threshold, which is physically reasonable.

**Wow moment:** "Build a stone box. Fill it with water. Put a lava source under it. Watch it boil." Call it out.

---

#### Session 14: Acid and the Reactions Architecture

**The Goal:** By the end of this session acid will dissolve materials at different rates, AND the reaction system will be cleanly architected so that adding a new reaction takes 3 lines of code instead of scattered `if` statements.

**Concepts:** `HashMap<(CellType, CellType), ReactionOutcome>`, structs with multiple fields, the architectural moment -- formalising the reaction lookup table.

**Agent note -- this is the most important session in Month 2.** This is where Leo goes from "hacking in new elements" to "building a system." The ADHD risk is highest here because it's less immediately visual. The agent must:

1. Start by showing the problem: "Look at our update function. Every new reaction means another `if` statement. By the time we have 20 elements this will be unreadable." Show the messy code.
2. Introduce the solution: a `HashMap` that maps `(CellType, CellType)` pairs to `ReactionOutcome`.
3. Define `ReactionOutcome`:
```rust
struct ReactionOutcome {
    replace_source: Option<CellType>,
    replace_target: Option<CellType>,
    probability: f32,
}
```
4. Show that adding acid-dissolves-sand is now: one `HashMap::insert()` call.
5. The wow moment: "Add 5 new reactions in 5 lines. Run it. They all work." This is the architectural payoff.

**Acid behaviour:** Acid dissolves sand, stone, and wood at different rates (different `probability` values). Acid is consumed in the reaction (becomes empty) with some probability. Acid does not dissolve metal (introduced in Month 3).

---

#### Session 15: Project Build Part 1 -- Lava, Ice, and Chain Reactions

**The Goal:** By the end of this session the simulation will have lava and ice, and chain reactions will produce visually complex emergent behaviour.

This is a project session. New elements are added using the reactions architecture from Session 14.

**Elements to add:**
- **Lava:** very high temperature, destroys most things on contact, cools to stone if it touches water
- **Ice:** cold, cools adjacent cells, melts to water if temperature gets high enough

**Chain reactions to verify work:**
- Lava + water → stone + steam
- Fire + ice → water (ice melts)
- Lava + ice → stone (rapid cooling)

**Agent must provide:** A full worked implementation of both elements and their reactions, using the `HashMap` architecture. Also provide a suggested "test scenario" for each reaction chain so Leo can verify them visually.

---

#### Session 16: Project Build Part 2 -- Polish and Milestone

**The Goal:** By the end of this session `sand-sim` v0.2 is complete. All reactions are balanced and feel satisfying.

**Goals for this session:**
- Balance all reaction probabilities until the sim feels physically plausible
- Add a temperature visualisation toggle (press `T` to see a heat map overlay)
- Add element count display (how many cells of each type are currently active)
- Fix any reaction bugs or edge cases found during testing
- Update the on-screen legend with all new elements

**Milestone moment (agent must include this):** End with a "You've built a chemistry sandbox" retrospective. List every element and every reaction. Note that the same architecture that handles 8 elements will handle 80 -- Leo has built a system, not just a program.

---

## Month 3: The Alchemy Game

**Theme:** Layer a discovery and progression system on top of the simulation. Transform the chemistry sandbox into a game with goals, unlocks, and secrets.

**Month 3 Milestone:** `sand-sim` v1.0 -- an alchemy discovery game. The player starts with four basic elements and a codex full of silhouettes. Experimenting unlocks new elements which fill in the codex. Hidden combinations produce surprising results. The game has a title screen, save/load, and enough content to play for 20-30 minutes.

**Rust concepts covered across Month 3:** Modules, multi-file projects, `Cargo.toml` in depth, file I/O, `serde` for serialisation, closures and iterators deep dive, generics, `Box<dyn Trait>`, `std::sync` basics.

### Session Breakdown

---

#### Session 17: Modules -- Taming the Codebase

**The Goal:** By the end of this session the project will be split into clean modules and navigating the code will feel effortless.

**Concepts:** `mod`, `pub`, `use`, splitting into multiple files (`src/simulation.rs`, `src/elements.rs`, `src/rendering.rs`, `src/ui.rs`), `pub struct` vs private fields, `super::`, module trees.

**Framing:** Open `main.rs`. It's probably 400+ lines. "This is unmanageable. Let's fix it." The refactor is the lesson. By the end, `main.rs` should be under 30 lines.

**Wow moment:** "`main.rs` goes from 400 lines to 25. Everything still works. The compiler told us exactly what to fix." Label it.

---

#### Session 18: File I/O -- Save and Load

**The Goal:** By the end of this session the simulation state will save to a file and reload correctly -- close the app, reopen it, the world is still there.

**Concepts:** `std::fs`, `serde` and `serde_json` for serialisation, `#[derive(Serialize, Deserialize)]`, `Result<T, E>`, the `?` operator, returning `Result` from functions.

**Framing:** Saving and loading is how games feel real. Without it, every session starts from scratch. Add `S` to save, `L` to load. The save file is human-readable JSON -- Leo can open it in VS Code and see the grid.

**Chemistry connection:** A save file is like a snapshot of a chemical system at a point in time. You can restore the exact state and continue the reaction.

**Wow moment:** "Build an elaborate structure. Press S. Close the app entirely. Reopen it. Press L. It's all still there." Label it.

---

#### Session 19: The Recipe System -- Unlocking Elements

**The Goal:** By the end of this session the game will have a recipe system where combining elements unlocks new ones, and the player won't know what to combine until they try.

**Concepts:** Closures intro -- `|x| x + 1`, passing closures to functions, `Fn` trait basics. Iterators in depth: `.map()`, `.filter()`, `.any()`, `.collect()`.

**Framing:** The recipe system is a `HashMap<(CellType, CellType), CellType>` of locked combinations. When a reaction occurs between two elements in the recipe map, the new element is added to the player's "discovered" set and unlocked in the element selector. Most recipes are hidden -- the player has to discover them through experimentation.

**Starting recipes (agent should include these):**
- Sand + fire → Glass
- Stone + water → Mud
- Wood + water → Rotting Wood
- Sand + water + time → Wet Sand (slows movement)

**Wow moment:** "Combine two elements and watch a new one appear in the selector for the first time." Label it. This is the core game loop clicking into place.

---

#### Session 20: The Codex UI

**The Goal:** By the end of this session there is an in-game codex that shows discovered elements in full colour and undiscovered ones as grey silhouettes with a question mark.

**Concepts:** Generics intro -- `fn draw_element<T: ElementInfo>(element: T)`, `Box<dyn Trait>` for dynamic dispatch, simple UI layout with macroquad's `draw_rectangle` and `draw_text`.

**Framing:** The codex is the visual reward for discovery. Design it like a Pokédex or chemistry periodic table -- a grid of element tiles, each showing the element name and colour when discovered. Undiscovered entries show a grey box and "???". Pressing `TAB` opens and closes it.

**Wow moment:** "Open the codex on a fresh save. Everything is grey. Unlock your first element. Open the codex again. One tile has filled in." Label it.

---

#### Session 21: New Elements -- Gunpowder and Glass

**The Goal:** By the end of this session gunpowder will explode and glass will refract the light (visually -- render differently to simulate it).

**Concepts:** Closures with `move`, iterator chaining, `flat_map`, writing clean element behaviour using the reactions architecture.

**Gunpowder:** Dry (sand-like physics). Extremely sensitive to fire -- ignites immediately and creates a large explosion (rapidly spawns fire and raises temperature in a radius). The explosion radius uses the circle equation from Session 7.

**Glass:** Created by heating sand to extreme temperature. Transparent (render as a semi-transparent overlay in macroquad). Melts back to sand at very high temperatures. Shatters (becomes sand + empty) if struck by a fast-moving heavy element.

**Maths connection:** The explosion radius calculation. The agent should call out explicitly: "This is the same `x² + y² ≤ r²` equation you used for the brush in Session 7. Maths you've already learned is doing new work."

**Wow moment:** "Pack gunpowder into a chamber. Seal it with stone. Drop one fire particle in through a gap. The explosion is contained by the stone walls -- mostly." Label it.

---

#### Session 22: New Elements -- Concrete and Rust

**The Goal:** By the end of this session concrete will set hard over time, and metal will rust slowly when in contact with water.

**Concepts:** The `cell.lifetime` field used for time-based state changes, `std::time` basics, iterators with state.

**Concrete:** Wet concrete (water + stone dust, a new sub-element) slowly solidifies over time. The lifetime field counts down. When it reaches zero, it becomes solid concrete (static, like stone but darker). If you don't let it set, it flows like a slow liquid.

**Rust (the chemical process, not the language -- the agent should absolutely make this joke):** Metal cells adjacent to water slowly accumulate a `rust_level: u8` field. As it increases, the cell colour shifts from silver to orange-brown. At maximum rust, the cell crumbles to sand. "Rust is happening to your metal. You're running Rust to simulate rust."

**Wow moment:** "Build a metal structure. Surround it with water. Come back in 30 seconds of sim-time. It's crumbling." Label it. And make the Rust/rust joke.

---

#### Session 23: Polish and Secrets

**The Goal:** By the end of this session the game has a title screen, easter eggs, and at least 3 hidden element combinations that aren't hinted at anywhere.

**Concepts:** Game state enum (`enum GameState { TitleScreen, Playing, Codex }`), state machine pattern, simple title screen rendering.

**Content:**
- `GameState` enum drives which screen is rendered and which inputs are active
- Title screen: game name, "Press Enter to play", brief flavour text
- Three hidden recipes that produce surprising results (agent's choice -- be creative, keep them chemistry-flavoured)
- An easter egg triggered by a specific key combination (agent's choice)

**Framing for the agent:** The hidden recipes should be genuinely surprising and not guessable from the element names alone. The joy of discovery is the point.

---

#### Session 24: Showcase, Retrospective, and What Next

**The Goal:** `sand-sim` v1.0 is finished, documented, and ready to present to the DofE assessor.

**Content:**

**Final build tasks:**
- Ensure save/load works reliably
- Update the on-screen legend and codex with all elements
- Write a `README.md` for the project that Leo's teacher could read and understand
- Final `cargo clippy` pass -- fix any warnings

**Celebration section (required):** A "What You've Built" retrospective. List every Rust concept Leo has learned. List every element in the simulation. List every reaction. List every session. This is the DofE evidence summary and it should feel genuinely impressive -- because it is.

**What's next section (required):** Brief, exciting pointers to where to go from here:
- Make it faster: multi-threading the simulation with `rayon`
- Make it bigger: a larger grid, chunk-based storage for infinite worlds
- Make it shareable: compile to WebAssembly so it runs in a browser (`wasm32-unknown-unknown` target)
- Go further with games: the Bevy game engine for 3D
- Go deeper with chemistry: actual fluid dynamics, reaction-diffusion systems (Turing patterns)
- Go into systems: Rust in the Linux kernel, embedded Rust on microcontrollers

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

Use external crates where they genuinely help. The primary crates for this course are:

- `macroquad` -- 2D rendering and input throughout all three months
- `fastrand` -- probabilistic element behaviour (spread, reaction probability)
- `serde` + `serde_json` -- save/load in Month 3

Do not introduce crates just to avoid writing 10 lines of standard library code. Always provide the `Cargo.toml` entry when a new crate is first used.

### Simulation Accuracy

The simulation does not need to be physically accurate -- it needs to feel physically plausible. When a chemistry concept is modelled (combustion, phase change, dissolution, oxidation), include a brief callout explaining the real chemistry behind it. Leo has an interest in chemistry and will appreciate the connection. The callout should be clearly labelled as "The Real Chemistry" and kept to 2-3 sentences.

### GitHub Repository Hygiene

The generated repository should include:
- `README.md` at root with: what the course is, who it's for, how to use it, prerequisites, licence
- `SETUP.md` with step-by-step Rust and tooling installation for Windows, macOS, and Linux
- `LICENSE` file (MIT recommended for open source)
- `.gitignore` for Rust projects (standard `target/` exclusion)
- Each project milestone should have its own `README.md` that works as a standalone brief

### Audio Polish (Milestones)

Silent simulations are flat. Sound is a powerful, low-cost dopamine vector and the agent must add audio in two specific places:

- **Session 8 (Month 1 milestone polish):** add a soft sand-pour whoosh that plays while the left mouse button is held down. Use `macroquad::audio` -- a single short looping WAV is enough.
- **Session 16 (Month 2 milestone polish):** add fire crackle, lava sizzle, and a low explosion thump for oil ignition. Three short WAVs, triggered when the relevant cell type is first spawned in a frame.

Provide the WAV files in `month-X/milestone/sand-sim-vX.Y/assets/` with attribution. CC0 sources only (freesound.org with CC0 licence, opengameart.org CC0 section).

### Migration From The Existing Repo

This PRD replaces an earlier course design (music-theory-cli / world-generator / midi-synth across three separate projects). The agent must **delete the old content** before generating new content. Specifically:

- Delete `month-1/project/music-theory-cli/`, `month-2/project/world-generator/`, `month-3/project/midi-synth/` (and the empty `project/` folders).
- Delete the `examples/` folder under every `month-*/session-*/` (they contain music/Minecraft examples that are no longer relevant).
- Delete `MUSIC-THEORY-PRIMER.md` at the repo root.
- Regenerate `GLOSSARY.md` to remove music terms (MIDI, frequency, scale, chord) and add simulation/chemistry terms (cellular automaton, phase change, ignition, viscosity, emergent behaviour).
- Regenerate `dfe/progress-summary.md` and `dfe/session-log.md` -- both are pre-populated with the old session titles and must be rebuilt against the 24 sand-sim session titles in this PRD.
- Update `dfe/README.md` and `dfe/session-log-printable.md` to reflect the new session titles.
- Keep `dfe/assessor-briefing.md`, `dfe/participant-statement-template.md`, the three `milestone-N-reflection.md` files, `dfe/github-workflow.md`, `dfe/print-checklist.md`, and `dfe/scripts/` -- they are project-agnostic.

The agent should make these deletions and the new structure in a single migration commit so the repo history clearly shows the pivot.

---

## Summary: The Three Milestones

| Month | Version | What it does | Core Rust skills | Wow factor |
|---|---|---|---|---|
| 1 | `sand-sim` v0.1 | Real-time physics sandbox: sand, water, stone | Types, enums, loops, match, Vec, macroquad | Sand piles naturally from the first particle drop |
| 2 | `sand-sim` v0.2 | Chemistry sandbox: fire, oil, acid, steam, lava, ice | Structs, HashMap, iterators, Option, traits | Lava hitting water solidifies to stone while steam rises |
| 3 | `sand-sim` v1.0 | Alchemy discovery game: codex, recipes, secrets | Modules, file I/O, serde, closures, generics | The codex fills in as hidden combinations are discovered |