# Duke of Edinburgh Award — Skill Section Pack

This folder turns the **DfE Rust Learning Course** into a complete evidence base for the **Duke of Edinburgh (DofE) Skill section** at any award level (Bronze, Silver, or Gold).

> If you are the participant, a parent or guardian, or the assessor: this is the folder to read first.

## Quick orientation by role

- **Participant** — there are two valid ways to keep your DofE log; both end with the same printed binder for the assessor.
  - **Path A — paper.** Print the booklet up front, fill it in by hand. Start with [`print-checklist.md`](./print-checklist.md) and [`session-log-printable.md`](./session-log-printable.md).
  - **Path B — fork-and-commit.** Use git as your working log; the GitHub commit history becomes a tamper-resistant timeline; print at the end. Start with [`github-workflow.md`](./github-workflow.md), and do the [Session 0 pre-flight](../month-1/session-00/) first.
  - Not sure which? Path A is simpler. Path B teaches a real industry skill *as well as* Rust. You can switch between them mid-course.
- **Parent / guardian** — this folder + the printed binder are the evidence pack. You don't need to do anything; you just need to know where it lives.
- **Assessor** — read [`assessor-briefing.md`](./assessor-briefing.md). The participant should hand you a physical A4 binder, not a link to a website. (If they also point you at a GitHub link with a commit timeline, that's optional supporting evidence — you don't need to look at it.)

## How this course is assessed

The intended workflow is **paper-first**:

1. The participant prints the session-log booklet at the start of the course.
2. They handwrite (or type-and-print) one short entry after each session.
3. Milestone reflections, the participant statement, and printed project READMEs are added to the binder as they're completed.
4. At the end, the assessor reviews the **physical binder** — not the GitHub repo — and signs the form on the last page.

This works because most assessors are teachers, tutors, or DofE leaders who would rather flip through a binder than navigate a code repository. It also makes the evidence harder to fake: 24 dated handwritten entries are not a thing you can knock out in an evening.

> **Going fully digital?** It's allowed. If your assessor is comfortable reviewing files on screen and you'd rather not print, use [`session-log.md`](./session-log.md) instead and have the assessor read everything in this folder online. But the default recommendation is **print**.

---

## What the DofE Skill section requires

To pass the Skill section you need to:

1. **Undertake a skill regularly** over a sustained period (Bronze: 3 months, Silver/Gold: longer).
2. **Show evidence of progressive improvement** — not just attendance, but visible growth.
3. Have an **assessor** (not a parent or guardian) confirm the activity took place and improvement was made.
4. Write a **personal statement** reflecting on what you learned.
5. Keep an **activity log** that can be reviewed.

This course is designed so that completing it *automatically* satisfies all five requirements.

---

## How this course meets each requirement

| Requirement | How this course meets it |
|---|---|
| Regular activity | 2 × 1-hour sessions per week × 12 weeks = **24 documented sessions**. |
| Evidence of progression | Session logs **plus** three milestone projects of increasing complexity, each in version control with timestamped commits. |
| Assessor sign-off | The assessor reads [`assessor-briefing.md`](./assessor-briefing.md), reviews the projects and logs, and signs off. |
| Personal statement | Use [`participant-statement-template.md`](./participant-statement-template.md). |
| Activity log | [`session-log.md`](./session-log.md), filled in after every session. A printable version is in [`session-log-printable.md`](./session-log-printable.md). |

---

## What's in this folder

| File | What it's for | Who fills it in |
|---|---|---|
| `README.md` | (You are here.) Plain-English overview. | — |
| `print-checklist.md` | Path A: what to print, when, and in what binder order. | **Participant** (Path A) |
| `github-workflow.md` | Path B: how to fork, clone, and use git as your log. | **Participant** (Path B) |
| `session-log-printable.md` | Print-friendly A4 booklet, one form per session. | **Participant** (Path A) |
| `session-log.md` | Markdown log; edit-and-commit version of the same data. | **Participant** (Path B) |
| `scripts/print-commit-history.sh` | Shell script that produces a printable, dated commit-history page from your fork. | **Participant** (Path B), at the end |
| `session-log-template.md` | A blank template for a single session entry. | — (reference only) |
| `milestone-1-reflection.md` | Reflection after the Month 1 mini-project. | **Participant**, after Session 8 |
| `milestone-2-reflection.md` | Reflection after the Month 2 mini-project. | **Participant**, after Session 16 |
| `milestone-3-reflection.md` | Reflection after the final project. | **Participant**, after Session 24 |
| `assessor-briefing.md` | Plain-English brief for the assessor. **Print this last; it goes at the front of the binder.** | — (the assessor reads this) |
| `participant-statement-template.md` | Template for the final personal statement. | **Participant**, after Session 24 |
| `progress-summary.md` | Pre-built mapping of every session to its evidence. | — (reference only; print and bind for the assessor) |
| [`../GLOSSARY.md`](../GLOSSARY.md) | Plain-English definitions for every technical term used in the course (compiled, immutable, borrow, cellular automaton, emergent behaviour, …). | — (reference only) |
| [`../CHEMISTRY-PRIMER.md`](../CHEMISTRY-PRIMER.md) | Chemistry from scratch (combustion, phase change, oxidation) in a 10-minute read. Read before Month 2 if you'd like the reaction names to mean something in real life. | **Participant** (if needed) |
| [`../resources/compiler-errors.md`](../resources/compiler-errors.md) | The most common Rust compiler errors translated into plain English, with fixes. Linked from every session's "Stuck?" footer. | **Participant** (when stuck) |

---

## What the participant needs to do

Three things, on a recurring rhythm.

1. **Before Session 1**: read [`print-checklist.md`](./print-checklist.md), print the booklet, and put it in a slim A4 ring binder.
2. **After every session** (5–10 minutes): open the binder, find that session's page, and fill in the four short fields by hand. Always write the date.
3. **After Sessions 8, 16, and 24**: complete the corresponding `milestone-N-reflection.md` (one page, 3–5 paragraphs), print it, and add it to the binder.
4. **After Session 24**: write the final personal statement using `participant-statement-template.md`, print it, and add it to the binder along with the printed assessor briefing. Hand the binder to the assessor for review and sign-off.

That's it. Each session ends with a small reminder so you don't forget step 2.

---

## What the assessor needs to do

Two things, both small.

1. **Once at the start** (5 minutes): read `assessor-briefing.md` so you understand the setup. The participant will print this and put it at the front of the binder.
1. **At the end** (about 30–45 minutes): flip through the binder — session log, the three milestone reflections, the three milestone READMEs (`sand-sim` v0.1, v0.2, v1.0), and the participant statement. Sign the form at the bottom of the printed `assessor-briefing.md` and return the binder (or just the signed form) to the participant.

The assessor does **not** need to know any Rust, **does not need a computer**, and does not need to run any code. The briefing explains exactly what to look for.

---

## Who can be the assessor?

Per DofE rules, the assessor **must not** be a parent or guardian. Suitable people include:

- A teacher, sixth-form tutor, or computer science teacher
- A DofE leader at school
- A family friend or relative with a technical background

The assessor should be someone who can confirm the work is genuine and reflect on the participant's progress in their own words.

---

## Evidence trail — what's in the binder at the end

After 24 sessions you will have a single A4 ring binder containing:

- **Cover page** with participant name, dates, award level
- **24 session-log forms**, dated and filled in by hand (or typed and printed)
- **3 milestone reflection pages** — one after each `sand-sim` release (v0.1, v0.2, v1.0)
- **3 printed milestone READMEs** — one per release
- **1 participant personal statement** — printed
- **1 progress summary** — one-page index of the whole course
- **1 assessor briefing + signed sign-off form** — at the back

Plus, on your laptop and on GitHub: the actual working code for the three `sand-sim` releases, ready to demo if the assessor wants to see something running.

That's a complete, self-contained evidence pack any DofE coordinator will recognise — and one your assessor can review in a single sitting without ever opening a computer.
