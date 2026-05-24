# Milestone 2 Reflection — `sand-sim` v0.2

**Date completed:**
**Link to milestone folder:** [`month-2/milestone/sand-sim-v0.2/`](../month-2/milestone/sand-sim-v0.2/)

> Complete this after Session 16. Aim for 3–5 paragraphs.

---

### What I built

Describe what v0.2 does that v0.1 didn't. Walk through one good chain reaction — for example: oil on the floor, fire dropped in one corner, the whole pool ignites in a cascade, the heat boils nearby water to steam, the steam rises, condenses, and falls back as water. Paste a screenshot if you can.

---

### Skills I used

List the Rust concepts you applied. From this month, you'll likely have used:

- `struct` and `impl` blocks — how you modelled the `Cell` with a temperature and lifetime
- `Option<T>` — how the reaction function returns `Some(new_type)` or `None`
- Iterators (`.iter()`, `.enumerate()`, `.map()`, `.filter()`) — cleaner grid scanning
- `HashMap<(CellType, CellType), ReactionOutcome>` — the reactions architecture from Session 14
- `fastrand::f32()` — probabilistic behaviour so the sim feels organic
- (Optionally) `macroquad::audio` for the fire crackle and explosion thump in Session 16

The most important thing to call out is the **Session 14 refactor**: explain in your own words why moving from scattered `if` statements to a `HashMap`-based reaction table made the program easier to extend. This is the architectural lesson of the whole month.

---

### The hardest part

What was the most challenging thing to build or debug? Common candidates: getting fire to spread *and* burn out without staying forever, balancing oil's explosion radius so it's exciting but not instantly-game-ending, or untangling the order-of-operations bug where lava-hits-water turned everything to stone before the steam could spawn.

---

### What I'm most proud of

Which moment was most satisfying? The first chain reaction that worked end-to-end? The heat-map overlay revealing how heat actually flows through the sim? The Session 14 refactor when the messy `if`-chain turned into three clean lines and adding acid took 30 seconds?

---

### How this compares to Milestone 1

Look back at `milestone-1-reflection.md`. What feels easier now? What concepts that were new in Month 1 are now second nature? What new tools do you have in your kit — and which one are you most likely to reach for in the next project? (Hint: it's probably `HashMap`.)

---

*Signature (participant):* ______________________________
*Date:* ______________________________
