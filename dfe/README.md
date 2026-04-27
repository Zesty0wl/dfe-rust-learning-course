# Duke of Edinburgh Award — Skill Section Pack

This folder turns the **Learn Rust with Leo** course into a complete evidence base for the **Duke of Edinburgh (DofE) Skill section** at any award level (Bronze, Silver, or Gold).

> If you are Leo, your parents, or your assessor: this is the folder to read first.

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
| `session-log.md` | The main running activity log (one entry per session). | **Leo**, after each session |
| `session-log-template.md` | A blank template for a single session entry. | — (reference only) |
| `session-log-printable.md` | Print-friendly A4 booklet, one form per session. | **Leo** (in pen, optional) |
| `milestone-1-reflection.md` | Reflection after the Month 1 mini-project. | **Leo**, after Session 8 |
| `milestone-2-reflection.md` | Reflection after the Month 2 mini-project. | **Leo**, after Session 16 |
| `milestone-3-reflection.md` | Reflection after the final project. | **Leo**, after Session 24 |
| `assessor-briefing.md` | Plain-English brief for the assessor. | — (the assessor reads this) |
| `participant-statement-template.md` | Template for the final personal statement. | **Leo**, after Session 24 |
| `progress-summary.md` | Pre-built mapping of every session to its evidence. | — (auto-generated) |

---

## What Leo needs to do

Three things, on a recurring rhythm.

1. **After every session** (5–10 minutes): open `session-log.md`, find that session's entry, and fill in the four short fields.
2. **After Sessions 8, 16, and 24**: complete the corresponding `milestone-N-reflection.md` (one page, 3–5 paragraphs).
3. **After Session 24**: write the final personal statement using `participant-statement-template.md`. Send it, with the completed log and the three milestone reflections, to the assessor for sign-off.

That's it. Each session ends with a small reminder so you don't forget.

---

## What the assessor needs to do

Two things, both small.

1. **Once at the start** (5 minutes): read `assessor-briefing.md` so you understand the setup.
2. **At the end** (about 30–45 minutes): review the session log, the three projects, and the participant statement. Sign the form at the bottom of `assessor-briefing.md` and return it to Leo.

The assessor does **not** need to know any Rust. The briefing explains what to look for.

---

## Who can be the assessor?

Per DofE rules, the assessor **must not** be a parent or guardian. Suitable people include:

- A teacher, sixth-form tutor, or computer science teacher
- A DofE leader at school
- A family friend or relative with a technical background

The assessor should be someone who can confirm the work is genuine and reflect on Leo's progress in their own words.

---

## File evidence trail

After 24 sessions you will have:

- **24 session log entries** (`dfe/session-log.md`)
- **3 milestone reflection documents** (`dfe/milestone-N-reflection.md`)
- **1 personal statement** (`dfe/participant-statement.md` — created from the template)
- **3 working software projects** (`month-N/project/`) with git commit history
- **1 assessor sign-off** (signed `dfe/assessor-briefing.md`)
- **24 commits' worth of timestamped activity** in git history (or in the repo's `.git` log)

That's a complete, self-contained evidence pack any DofE coordinator will recognise.
