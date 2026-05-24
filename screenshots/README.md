# Screenshot Checklist

A list of moments worth screenshotting as you build `sand-sim`. Take the screenshot, drop the PNG in this folder using the suggested filename, then `git commit` it. By the end of the course you'll have a visual diary of the whole build — perfect for the DofE binder and for showing your mates.

> **How to take a screenshot:** Windows = `Win` + `Shift` + `S`. macOS = `Cmd` + `Shift` + `4` then drag a box. Linux (GNOME) = `Shift` + `PrtSc`. Most screenshot tools let you draw a rectangle around just the sim window — that's perfect.

> **Where to save:** drop the PNG straight into this `screenshots/` folder using the filename in the **File** column below. Then `git add screenshots/<filename>.png && git commit -m "session NN screenshot: <description>"`. Done.

---

## Month 1 — Sandbox

| Session | Moment to capture | Suggested file |
|---|---|---|
| **1** | Your first empty window. Black rectangle with the title bar. Proof you compiled and ran Rust. | `01-first-window.png` |
| **1** | A single click drawing a coloured pixel on screen. | `01-first-pixel.png` |
| **3** | **WOW MOMENT.** Sand piling up by itself after one click-and-hold from the top. A nice pyramid. | `03-sand-pile.png` |
| **5** | Sand pile next to a water reservoir held in a stone bowl. Three elements, all behaving correctly. | `05-three-elements.png` |
| **7** | Your element selector UI in the corner, and a big brush-radius pour of sand. | `07-brush-radius.png` |
| **8** | **Milestone v0.1.** Final scene with FPS counter visible. Make it pretty — pour sand into a stone funnel into a water tank. | `08-v0.1-shipping.png` |

---

## Month 2 — Chemistry

| Session | Moment to capture | Suggested file |
|---|---|---|
| **9** | Heat-map overlay turned on (`T`). Hot cells glowing brighter, the gradient visible. | `09-heatmap.png` |
| **11** | **WOW MOMENT.** Fire spreading along a wooden log. Catch it mid-spread for the best image. | `11-fire-spreads.png` |
| **12** | An oil explosion you triggered. Big orange flash mid-frame. | `12-oil-boom.png` |
| **13** | Water boiling. Steam clearly rising from a heated bowl. | `13-steam-rising.png` |
| **14** | The reactions HashMap living quietly in your code. Open VS Code, screenshot the relevant ~15 lines. (Code shots are nice variety in the binder.) | `14-reactions-code.png` |
| **15** | Lava poured into water. The moment of contact — steam, stone, chaos. | `15-lava-water.png` |
| **16** | **Milestone v0.2.** Big chain reaction. Oil on the floor, ignited at one end, water above boiling to steam, a stone bowl half-melted by lava. | `16-v0.2-shipping.png` |

---

## Month 3 — Alchemy Game

| Session | Moment to capture | Suggested file |
|---|---|---|
| **17** | Your VS Code file tree showing the new modules: `simulation.rs`, `elements.rs`, `rendering.rs`, `ui.rs`. Proof of refactor. | `17-modules.png` |
| **18** | Your save file (`save.json`) open in VS Code next to the sim showing the same world. Two screenshots side-by-side. | `18-save-load.png` |
| **20** | The codex screen partway through filling in — some tiles in colour, some `???`. | `20-codex-half.png` |
| **21** | A gunpowder explosion. Make it big. | `21-gunpowder.png` |
| **23** | **WOW MOMENT.** A codex tile flipping from `???` to discovered with the gold glow. Tricky to time — try a screen recorder set to single-frame export. | `23-codex-unlock.png` |
| **24** | **Milestone v1.0.** Title screen of your finished game. The one you'd put on a Steam page. | `24-v1.0-title.png` |
| **24** | The codex fully filled in. Every element discovered. The completionist shot. | `24-codex-full.png` |

---

## Optional bonus shots

These aren't required but they look great in a DofE binder:

- A `cargo run --release` terminal showing the sim launching, alongside the window. Proof it runs.
- Your `git log --oneline` showing 24+ commits dated across 12 weeks. Independent corroboration the work was spread out.
- A photo of your physical setup — the laptop, the open binder, the printed session log on the desk. Sets the scene for the assessor.
- A screen recording of v1.0 being played for 30 seconds. (Linux: `peek` or `byzanz`. macOS: `Cmd+Shift+5`. Windows: Xbox Game Bar with `Win+G`.) Save as `bonus-v1.0-demo.mp4`. GitHub renders short MP4s inline.

---

## Why bother?

Three reasons:

1. **It's evidence.** The DofE assessor can flip through the binder and *see* the project progressing. No coding knowledge required on their side.
2. **It's a dopamine bank.** When you hit a hard session in Month 3, scroll back through this folder. Twelve weeks ago you couldn't even open a window. Look at how far that is.
3. **It's a portfolio.** When you apply for an internship, a sixth-form computing course, or a degree, "here are 16 screenshots of the cellular automaton I built in Rust" is a better story than "I did some Codecademy."

---

*Filenames are suggestions — call them whatever you like. The important thing is that you commit them.*
