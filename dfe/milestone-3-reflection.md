# Milestone 3 Reflection — `sand-sim` v1.0

**Date completed:**
**Link to milestone folder:** [`month-3/milestone/sand-sim-v1.0/`](../month-3/milestone/sand-sim-v1.0/)

> Complete this after Session 24. Aim for 3–5 paragraphs. This is your final milestone — this is the one the assessor will weigh most heavily, so take your time.

---

### What I built

Describe v1.0 — the alchemy game on top of the chemistry sandbox. What does the title screen look like? How does the discovery system work? Which recipes are you most pleased with? Did you find all your own hidden recipes when you were testing, or did one of them surprise you? Did the easter egg fire when you expected? Paste a screenshot of the codex if you can — half-filled-in is the best demonstration.

---

### Skills I used

By now your toolbox is large. List the most important Rust concepts you brought to bear:

- Multi-file modules (`mod`, `pub`, `use`) — the Session 17 refactor that turned `main.rs` from ~500 lines back into ~25
- `serde` and `serde_json` — save and load to a human-readable JSON file
- `Result<T, E>` and the `?` operator — error handling everywhere file I/O happens
- Closures (`Fn`, `move`) and iterators in depth — used in the recipe system and the codex rendering
- Generics and `Box<dyn Trait>` — the element-info rendering trick from Session 20
- `enum GameState` and the state-machine pattern — title screen vs game vs codex
- `std::time` — concrete setting and metal rusting over wall-clock time

---

### The hardest part

This was the most ambitious month — what was the toughest thing you had to figure out? Common candidates: getting save/load to round-trip the whole grid correctly, the lifetime puzzle the first time you reached for closures with iterators, designing the codex tile layout so it scaled with the window, or chasing down the off-by-one in `flat_map` for the explosion radius.

---

### What I'm most proud of

Which moment felt most rewarding? The first time saving and re-loading actually worked? The codex tile flipping from grey "???" to full colour the first time you discovered an element? The title screen showing up looking properly like a game? The clippy pass coming back clean?

---

### How this compares to where I started in Month 1

Compare to Milestone 1 (or even to Session 1). What concepts were unimaginable to you then that are routine now? What kind of programs could you imagine writing next? (The "What's next" section of Session 24 has some pointers — multi-threading with `rayon`, WebAssembly, the Bevy game engine — but yours might be different. Whatever it is, write it down.)

---

*Signature (participant):* ______________________________
*Date:* ______________________________
