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

The participant has been working through a 24-session course called the **DfE Rust Learning Course** — a structured introduction to the **Rust programming language**.

- Two ~1-hour sessions per week, for 12 weeks
- Each session has a written walkthrough and runnable code
- The course builds **one** project across the 12 weeks — a real-time sandbox simulation that runs in a window — and ships it three times, each release more ambitious than the last:
  - **Month 1 (`sand-sim` v0.1):** a real-time falling-sand sandbox with three elements (sand, water, stone) and a brush UI. Click to draw, watch physics happen at 60 frames per second.
  - **Month 2 (`sand-sim` v0.2):** the same sandbox upgraded to a chemistry set — fire spreads, oil ignites, water boils to steam, lava cools to stone, acid eats things.
  - **Month 3 (`sand-sim` v1.0):** the chemistry set wrapped in a game — title screen, save and load, a discovery system where players unlock elements by experimentation, and a Pokédex-style codex.

Rust is a modern, industrial programming language used at companies including Microsoft, Amazon, Discord, Cloudflare, and inside the Linux kernel itself. Choosing it as a Skill demonstrates initiative — Python or JavaScript would be easier choices.

---

## What you'll find in this binder

> **Note to the assessor:** this folder is designed to be reviewed as a **printed A4 binder**, not on screen. The participant should have handed you a physical ring binder containing the documents below. If they've handed you a USB stick or a link, ask them to print it — most of these documents are explicitly designed to be printed (e.g. the session log has hand-fillable form fields). You do **not** need to open a computer to do this assessment.

| Section in the binder | What it is |
|---|---|
| **Session Log** | A running log — one short form per session, 24 in total. Dates and short hand-written or typed notes. |
| **Milestone Reflections (× 3)** | One longer reflection (3–5 paragraphs) after each of the three `sand-sim` releases. |
| **Milestone READMEs (× 3)** | A printed description of each release — v0.1, v0.2, v1.0 — so you can see what was made without running code. |
| **Participant Statement** | The participant's final personal statement (1–2 pages). |
| **Progress Summary** | A pre-built one-page table mapping each session to the concept and evidence. Useful as a map. |
| **This briefing + sign-off form** | The last page is the form for you to sign at the end. |
| **Commit history printout** *(optional)* | If the participant chose the GitHub workflow, a one-page, dated, machine-generated list of every commit they made. Independent corroboration that the work was spread across the required period. |

If you want to see the actual code, the participant can demo it on their laptop in 5 minutes — but you don't need to.

> **If you'd like online corroboration (entirely optional).** Some participants will have used the *GitHub workflow* and will have included a printed commit-history page in the binder, plus a short URL like `github.com/<their-username>/dfe-rust-learning-course`. If you want to verify, click the link and look at the **commits** tab — you should see ~24+ commits dated across the required period. You don't need to read the code; the dates alone tell you the work was real and spread out. **You absolutely do not need to do this** — the binder is sufficient evidence on its own.

---

## What to look for as evidence

You're checking three things. **All three answers can come from the printed binder alone** — flip through the pages.

### 1. Was the activity genuinely undertaken regularly?

- **Flip through the 24 session-log pages.** Are there 24 of them? Are the **dates** plausibly spread across 12 weeks (i.e. roughly two per week, not all batched into one weekend)?
- **Flip through the three milestone reflections.** Are they written in a personal voice with specific details a generic template wouldn't contain?
- **(Optional)** Ask the participant to open one of the project folders on their laptop and show you the code is real (multiple files, not a single page of pseudocode).

### 2. Has the participant demonstrably improved?

- Compare the **first session log page** (Session 1) with one of the **later ones** (e.g. Session 20+). The vocabulary, complexity of what's described, and confidence should be visibly different.
- Compare the **three milestone reflections**. Each release is more ambitious than the last:
  - v0.1: a few hundred lines, single file, two external libraries
  - v0.2: hundreds of lines with structured data, lookup tables, and tuned probabilistic behaviour
  - v1.0: a multi-module program using file I/O, serialisation, generics, and a state machine — a real game
- Each milestone reflection contains a section explicitly comparing where the participant is **now** to where they were **previously**.

### 3. Has the participant reflected meaningfully?

- Read 3–4 random session-log pages. Are they thoughtful? Do they describe specific struggles ("I couldn't get the borrow checker to accept this") rather than vague summaries ("It was good")?
- Read the **participant statement** at the end. Is it in the participant's own voice? Does it describe genuine learning?

If all three answers are *yes*, you have everything you need to sign off.

---

## How to verify the work is the participant's own

Three signals together are very strong evidence:

- **Handwritten dated entries** in the session log. Faking 24 dated entries by hand, with believable variation in handwriting, mood, and content, is far more effort than just doing the work.
- **Specific, concrete struggles in the session logs.** A faked log says "I learned about loops". A genuine log says "I forgot that `for` doesn't need parentheses, like Python — wasted ten minutes on a syntax error."
- **The work itself.** The three releases increase in difficulty. The final release is non-trivial — a real-time simulation with save/load, generics, modules, and a discovery system is real engineering.

If you want, ask the participant to **demo the sim to you live** (5 minutes — they open a terminal, type `cargo run --release` inside `month-3/milestone/sand-sim-v1.0/`, a window opens, you watch them play with fire and acid). That's the most direct verification and works well as a closing conversation.

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
