# Milestone 1 Reflection — `sand-sim` v0.1

**Date completed:**
**Link to milestone folder:** [`month-1/milestone/sand-sim-v0.1/`](../month-1/milestone/sand-sim-v0.1/)

> Complete this after Session 8. Aim for 3–5 paragraphs. This is more substantial than a session log — it's the document that demonstrates real reflection on a chunk of work, which is exactly what the DofE Skill section asks for.

---

### What I built

Describe the sandbox in your own words. What does it look like when it's running? What are the three elements and how does each behave? What's the most fun thing to do with it? (You can paste a screenshot if you like — a sand pyramid next to a half-full water reservoir is a great image.)

---

### Skills I used

List the Rust concepts you applied to build this. Try to explain what each one does in plain English. Examples to draw from:

- `cargo new` and `cargo add` — how you set up a project and pulled in `macroquad` and `fastrand`
- `loop` and `async fn main` — how the simulation runs frame after frame at 60fps
- `Vec<Vec<u8>>` — how you stored the grid of cells
- `enum CellType` — why naming the element types beat using magic numbers
- `match` — how you wrote different physics for each element
- `if`/`else` and bounds checking — how you stopped sand falling off the edge of the world
- Mouse input — how a click in the window became a cell on the grid

---

### The hardest part

What was the most challenging thing to build or debug? How did you work through it? What did the compiler tell you, and how did you respond? (Common answers: the bottom-to-top iteration order in Session 3, the brush radius maths in Session 7, or getting water to stop sloshing through walls.)

---

### What I'm most proud of

What moment or feature felt most satisfying? Which line of code, output, or behaviour felt like "yes, I built that"? (The Session 3 "sand pile forms naturally" moment is a popular answer.)

---

### How this compares to where I started

Look back at your Session 1 log. What could you do at the start of the course? What can you do now that you couldn't do then? Be specific. A sentence about how `match`, `enum`, and `Vec` together feel obvious now is a great way to demonstrate progression.

---

*Signature (participant):* ______________________________
*Date:* ______________________________
