# `sand-sim` v1.0 — Month 3 Milestone (Final Release)

> 🚧 **This folder is a placeholder.** It'll fill up as you work through Month 3.

This is where your **`sand-sim` v1.0** lives. The same engine + chemistry from v0.2, now wrapped in a game: title screen, save/load to JSON, a recipe-discovery system, a Pokédex-style codex, six new elements unlocked through experimentation, and a few easter eggs.

## How it gets built

| Session | What goes into this folder |
|---|---|
| **17** | `sand-sim-v0.2` is copied here and the code is split across `simulation.rs`, `elements.rs`, `rendering.rs`, `ui.rs`. `main.rs` shrinks from ~500 lines to ~25. |
| 18–23 | New features land per session: save/load, recipes, codex, gunpowder + glass, concrete + rust, state-machine title screen + hidden recipes. |
| **24** | Final polish, README pass, clippy clean, "what you built" summary. **v1.0 ships.** |

## How you run it (once it exists)

```bash
cd month-3/milestone/sand-sim-v1.0
cargo run --release
```

## `save.json` and `assets/`

Your in-game save lives in `save.json` (created the first time you press `S` in the running game). Audio + sprite assets live in `assets/` with attributions in `assets/CREDITS.md`. Until you reach the relevant sessions, this folder is bare.

---

Back to [`month-3/README.md`](../../README.md) · previous milestone: [`month-2/milestone/sand-sim-v0.2/`](../../../month-2/milestone/sand-sim-v0.2/).
