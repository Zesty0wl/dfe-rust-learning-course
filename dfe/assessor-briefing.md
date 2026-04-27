# Assessor Briefing — Duke of Edinburgh Skill Section

> **For the assessor.** You don't need to know any Rust. This document explains what the participant has been doing, what evidence they've collected, and how to sign off the Skill section. Reading time: about 5 minutes. Review time at the end: 30–45 minutes.

---

## What the DofE Skill section asks

The Skill section of the Duke of Edinburgh Award asks the participant to:

1. **Practise a skill regularly** over a sustained period (3 months for Bronze; longer for Silver/Gold).
2. **Show genuine improvement** — not just attendance, but visible growth.
3. Have an assessor (you) confirm that both happened.

Your role is **not** to grade the technical work. It's to confirm that:

- The participant did the activity for the required period.
- They genuinely improved.
- They reflected meaningfully on what they learned.

That's it. You're a witness, not an examiner.

---

## What the participant has been doing

The participant has been working through a 24-session course called **Learn Rust with Leo** — a structured introduction to the **Rust programming language**.

- Two ~1-hour sessions per week, for 12 weeks
- Each session has a written walkthrough and runnable code
- Every month ends with a project:
  - **Month 1:** a command-line music-theory tool that prints scales and chord progressions
  - **Month 2:** a procedural ASCII world generator (Minecraft-inspired)
  - **Month 3:** a MIDI synthesiser that turns MIDI files (or live keyboard input) into audio

Rust is a modern, industrial programming language used at companies including Microsoft, Amazon, Discord, Cloudflare, and inside the Linux kernel itself. Choosing it as a Skill demonstrates initiative — Python or JavaScript would be easier choices.

---

## What you'll find in this `dfe/` folder

| File | What it is |
|---|---|
| `session-log.md` | A running log — one short entry per session, 24 in total. The participant fills these in within a day of each session. |
| `milestone-1-reflection.md` <br>`milestone-2-reflection.md` <br>`milestone-3-reflection.md` | One longer reflection (3–5 paragraphs) after each of the three projects. |
| `participant-statement.md` | The participant's final personal statement, written from the template. |
| `progress-summary.md` | A pre-built table mapping each session to the concept and evidence. Useful if you want a quick map of the whole course. |

You'll also find three project folders elsewhere in the repository — `month-1/project/`, `month-2/project/`, `month-3/project/` — each containing the working code for that milestone.

---

## What to look for as evidence

You're checking three things.

### 1. Was the activity genuinely undertaken regularly?

- **Open `session-log.md`.** Are there 24 entries with plausible dates spread across the period?
- **Open the three milestone reflections.** Are they written in a personal voice, with specific details a generic person couldn't fake?
- **Open the project folders** (e.g. `month-1/project/music-theory-cli/`). Is there code there? Does it look like a real project, with multiple files?
- **(Optional, if you're comfortable.)** In a terminal, run `git log --oneline` inside the project folder. Each commit is a timestamped piece of activity.

### 2. Has the participant demonstrably improved?

- Compare the **first session log** (Session 1) with one of the **later ones** (e.g. Session 20+). The vocabulary, complexity of what's described, and confidence should be visibly different.
- Compare the **three milestone reflections**. Each project is more ambitious than the last:
  - Milestone 1: a few hundred lines, single file, no external libraries
  - Milestone 2: hundreds of lines across multiple types, with structured data
  - Milestone 3: a multi-module program using real external libraries, including audio output and possibly hardware MIDI input
- Each milestone reflection contains a section explicitly comparing where the participant is **now** to where they were **previously**.

### 3. Has the participant reflected meaningfully?

- Read 3–4 random session log entries. Are they thoughtful? Do they describe specific struggles ("I couldn't get the borrow checker to accept this") rather than vague summaries ("It was good")?
- Read the **participant statement** at the end. Is it in the participant's own voice? Does it describe genuine learning?

If all three answers are *yes*, you have everything you need to sign off.

---

## How to verify the work is the participant's own

Three signals together are very strong evidence:

- **Git commit history.** In the project folders, `git log` shows timestamped activity over the 12-week period. Faking this is far more effort than just doing the work.
- **Specific, concrete struggles in the session logs.** A faked log says "I learned about loops". A genuine log says "I forgot that `for` doesn't need parentheses, like Python — wasted ten minutes on a syntax error."
- **The work itself.** The three projects increase in difficulty. The final project is non-trivial — generating audio from raw bytes is real engineering.

If you want, ask the participant to **demo one project to you live** (e.g. open a terminal, run `cargo run`, type a few inputs). That's the most direct verification.

---

## Three questions to answer before you sign

1. Did the participant undertake this activity regularly over the required period? **Yes / No**
2. Has the participant demonstrably improved — can they do things now they could not do at the start? **Yes / No**
3. Has the participant reflected meaningfully on their learning? **Yes / No**

If you can answer **Yes** to all three, please complete the sign-off below.

---

## Assessor sign-off

> By signing below I confirm that, to the best of my knowledge, the named participant has completed this Skill activity regularly over the required period, demonstrated genuine improvement, and reflected meaningfully on their learning.

| Field | Value |
|---|---|
| Participant name | |
| Award level (Bronze / Silver / Gold) | |
| Skill | Rust programming |
| Activity period (start → end) | |
| Assessor name | |
| Assessor relationship to participant | |
| Assessor contact (email or phone) | |
| Assessor signature | |
| Date | |

Thank you. Please return this signed document to the participant for upload to the DofE eDofE system.
