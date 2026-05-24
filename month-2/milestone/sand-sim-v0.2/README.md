# `sand-sim` v0.2 — Month 2 Milestone

> 🚧 **This folder is a placeholder.** It'll fill up as you work through Month 2.

This is where your **`sand-sim` v0.2** lives. It starts as a copy of the v0.1 project from Month 1, then grows through eight sessions of chemistry: fire, oil, lava, water boiling to steam, ice melting, acid, and the reactions HashMap that ties them all together.

## How it gets built

| Session | What goes into this folder |
|---|---|
| 9–14 | Small per-session experiments in their own `starter/` / `solution/` folders; concepts like `struct`, `Option<T>`, iterators, `HashMap`. |
| **15** | Cargo project is copied from `sand-sim-v0.1`. Lava + Ice + chain reactions added via the reactions table. |
| **16** | Reaction balancing, heat-map overlay, element counts, audio effects (fire crackle, lava sizzle, oil-ignition thump). **v0.2 ships.** |

## How you run it (once it exists)

```bash
cd month-2/milestone/sand-sim-v0.2
cargo run --release
```

## `assets/`

The Session 16 audio (CC0 WAV files) lives in [`assets/`](./assets/) once you reach that session, with attributions in [`assets/CREDITS.md`](./assets/CREDITS.md). Until then this folder is intentionally bare.

---

Back to [`month-2/README.md`](../../README.md) · previous milestone: [`month-1/milestone/sand-sim-v0.1/`](../../../month-1/milestone/sand-sim-v0.1/) · next: [`month-3/milestone/sand-sim-v1.0/`](../../../month-3/milestone/sand-sim-v1.0/).
