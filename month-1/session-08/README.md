# Session 8 — Project Build Part 2: Polish and Milestone (v0.1 ships)

> **Stuck on a word?** Things like *FPS*, *toggle*, *legend*, *release*, *tag* are defined in plain English in the repo's [GLOSSARY.md](../../GLOSSARY.md).

---

## The Goal

By the end of this session **`sand-sim` v0.1 ships**. It has an FPS counter, a tap-to-pause toggle, a clear key, a right-click eraser, an on-screen legend, **a soft sand-pour sound**, and a commit tag `v0.1`.

---

## What you'll learn

- Polishing a working program — the difference between "it runs" and "it's nice"
- macroquad audio: loading and playing a WAV file
- Toggle vs hold (tap to pause, not press to pause)
- Tagging a git commit as a release point
- The small ceremonies that turn a project into evidence — milestone reflection, screenshot, repo tidy-up

---

## The big idea

Shipping is a skill. You can sit on "the cool prototype" for weeks, but the discipline of saying *this is the cut for v0.1, the README explains how to run it, and tomorrow I start fresh on v0.2* is what turns hobby code into a portfolio.

Today's job is small mechanical polish, then a release: a commit, a tag, a sentence in the DofE log saying you finished Milestone 1. The next eight sessions add chemistry; this version is the engine they're built on.

---

## Concepts covered

- `macroquad::audio::{load_sound, play_sound_once, PlaySoundParams}` (and where to find CC0 audio)
- Tap-to-toggle pattern with `is_key_pressed` + a boolean
- An on-screen legend showing keyboard shortcuts
- `cargo run --release` and why the milestone version should benchmark with it
- `git tag v0.1` and pushing a tag to GitHub

---

## Building towards `sand-sim`

This session **ships Milestone 1**. The folder `month-1/milestone/sand-sim-v0.1/` becomes the snapshot tagged `v0.1`. Month 2 starts from a copy of it as `month-2/milestone/sand-sim-v0.2/`. The session work, the milestone reflection, and the DofE log entry together make the assessor's life trivially easy when they review.

---

## Step-by-step walkthrough

> **Where you should be.** Session 7 ended with the brush, selector, scroll-wheel radius, hold-to-pause, and clear key all working in `month-1/milestone/sand-sim-v0.1/`. If anything from there is broken, today won't paper over it — fix Session 7 first.

### 1. Tap-to-pause toggle — 2 minutes

Replace the Session 7 `let paused = is_key_down(KeyCode::Space);` with a persistent toggle:

```rust
    let mut paused = false;

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        // ...
    }
```

Now Space *tap* pauses; Space *tap* again resumes. Hold-to-pause was fine for testing but feels wrong in a finished app.

### 2. FPS counter, top-right — 2 minutes

Just before `next_frame().await`, draw the frame rate where it's easy to glance at:

```rust
        let fps_label = format!("fps {}", get_fps());
        let fps_colour = if get_fps() < 30 { RED } else if get_fps() < 55 { YELLOW } else { GREEN };
        draw_text(&fps_label, screen_width() - 80.0, 20.0, 22.0, fps_colour);
```

`screen_width()` returns the current window width — so the label hugs the right edge even if you change the window size later. Colour code makes performance regressions visible without having to read numbers.

### 3. On-screen legend — 4 minutes

Below the brush radius readout from Session 7, add a multi-line key reference:

```rust
fn draw_legend() {
    let lines = [
        "1/2/3  select sand/water/stone",
        "scroll  brush size",
        "L-click  paint",
        "R-click  erase",
        "space   pause",
        "C       clear",
        "P       pour-sound test",
    ];
    let x: f32 = 8.0;
    let mut y: f32 = screen_height() - 16.0 * lines.len() as f32 - 8.0;
    for line in lines {
        draw_text(line, x, y, 16.0, LIGHTGRAY);
        y += 16.0;
    }
}
```

Call `draw_legend()` near the end of the per-frame block. The legend doubles as documentation for anyone you hand the binary to.

### 4. Audio — 6 minutes

Add the audio feature to `Cargo.toml`:

```toml
[dependencies]
macroquad = { version = "0.4", features = ["audio"] }
fastrand  = "2"
```

(If you already have `macroquad = "0.4"` plain, replace it with the `{ version = ..., features = [...] }` form so the audio backend compiles in.)

Linux only: install ALSA dev headers if you skipped them in `SETUP.md`:

```bash
sudo apt install -y libasound2-dev
```

Now, find a CC0 sand-pour WAV and drop it in `assets/sand.wav`. (Freesound.org filtered to CC0 is a good source; `freesound.org/search/?q=sand+pour&f=license%3A%22Creative+Commons+0%22`.) The file must be short — under a second — and looping isn't required because we'll re-trigger it while the mouse is held.

Record the source in `month-1/milestone/sand-sim-v0.1/assets/CREDITS.md`:

```markdown
# Audio credits — sand-sim v0.1

- `sand.wav` — short sand-pour SFX, CC0, sourced from <freesound.org/...> on YYYY-MM-DD.
```

Update `main` to load and play it (the `// ...` lines show where this audio code slots into the existing main loop — not a standalone program):

```rust,ignore
use macroquad::audio::{load_sound, play_sound_once};

#[macroquad::main(window_conf)]
async fn main() {
    let sand_sound = load_sound("assets/sand.wav").await.unwrap();
    // ... rest of setup ...

    let mut sand_playing_cooldown_frames: u32 = 0;

    loop {
        // ... input + simulation ...

        // Audio: trigger a soft sand-pour when sand is selected AND the mouse is held.
        if selected == CellType::Sand && is_mouse_button_down(MouseButton::Left) {
            if sand_playing_cooldown_frames == 0 {
                play_sound_once(&sand_sound);
                sand_playing_cooldown_frames = 18;   // ~ 0.3s at 60fps
            }
        }
        if sand_playing_cooldown_frames > 0 {
            sand_playing_cooldown_frames -= 1;
        }

        // ... rendering ...
    }
}
```

The cooldown avoids re-triggering 60 times per second (would be deafening). Tune the `18` to taste.

**Save. Run.** Hold the mouse to pour sand — gentle whisper of sound. Switch to water — silence. **First runnable checkpoint.**

#### Audio on Ubuntu — the full story

This is the biggest Linux gotcha of Month 1, so it gets its own subsection. Read this even if your audio worked first time, because the failure modes are common and the fixes are quick.

**1. The package is `libasound2-dev`** (not `libasound-dev`, not `alsa-dev`). Install with:

```bash
sudo apt install -y libasound2-dev pkg-config
```

**2. PipeWire vs PulseAudio vs raw ALSA.** Ubuntu 22.04+ ships **PipeWire** as the default audio server, with PulseAudio and ALSA compat layers on top. macroquad talks to ALSA, which on a stock Ubuntu install is routed through PipeWire's `pipewire-alsa` plugin. To confirm:

```bash
systemctl --user status pipewire pipewire-pulse
# Should show "active (running)" for both on 22.04+.
```

If either is inactive, audio output will be silent without an error. Fix:

```bash
sudo apt install -y pipewire pipewire-pulse wireplumber
systemctl --user --now enable pipewire pipewire-pulse wireplumber
```

**3. Headphones plugged in but no sound?** Open *Settings → Sound → Output Device* and confirm the right device is selected. PipeWire sometimes routes everything to HDMI when a monitor is connected.

**4. Crackling on slow machines.** Pure Linux thing — increase the ALSA buffer:

```bash
mkdir -p ~/.config/pipewire/pipewire.conf.d
cat > ~/.config/pipewire/pipewire.conf.d/buffer.conf <<EOF
context.properties = {
    default.clock.quantum = 2048
}
EOF
systemctl --user restart pipewire
```

**5. CI / headless Ubuntu (e.g. building in a Docker container, no audio device).** Run with the environment variable `MACROQUAD_DISABLE_AUDIO=1` or compile without the `audio` feature for that build. The sand-pour code path above handles missing audio gracefully via `unwrap()` — if you want a non-crashing build, change to:

```rust
let sand_sound = load_sound("assets/sand.wav").await.ok();
// ... and ...
if let Some(snd) = &sand_sound {
    if cooldown == 0 { play_sound_once(snd); cooldown = 18; }
}
```

That returns `Option<Sound>` and skips playback when audio is unavailable.

### 5. Cargo manifest tidy-up — 2 minutes

Open `Cargo.toml`. Fill in the metadata:

```toml
[package]
name        = "sand-sim"
version     = "0.1.0"
edition     = "2021"
authors     = ["Your Name"]
description = "A real-time falling-sand sandbox in Rust + macroquad."
license     = "MIT OR Apache-2.0"

[dependencies]
macroquad = { version = "0.4", features = ["audio"] }
fastrand  = "2"

[profile.release]
opt-level = 3
lto       = true
```

The `[profile.release]` section adds Link-Time Optimisation, which makes `--release` builds ~10–20% faster. Worth it for a milestone build.

Run a release build:

```bash
cargo run --release
```

Compile takes longer (a minute or two from scratch); the resulting simulation feels noticeably smoother — try a brush of 12, full of water, and check the FPS counter.

### 6. README for the milestone project — 4 minutes

Replace `month-1/milestone/sand-sim-v0.1/README.md` with the real thing:

```markdown
# sand-sim v0.1

A real-time falling-sand sandbox in Rust. Three elements (sand, water, stone), brush painting, basic UI.

## Run

```bash
cargo run --release
```

## Controls

- **1 / 2 / 3** — select sand, water, stone
- **L-click drag** — paint
- **R-click drag** — erase
- **Scroll** — brush size
- **Space** — pause / unpause
- **C** — clear

## Credits

Audio: see `assets/CREDITS.md` (alongside this README inside the milestone folder).

## What's next

Month 2 (see `month-2/README.md` at the repo root) adds fire, water-boils-to-steam, oil, lava, ice, acid, and a reactions architecture.
```

Now the milestone is *legible* to someone who finds the folder cold.

### 7. Complete the DofE milestone reflection — 5 minutes

Open [`dfe/milestone-1-reflection.md`](../../dfe/milestone-1-reflection.md). Answer the four prompts in your own words. **Be specific** — the assessor wants to see growth, not perfection.

### 8. Commit and tag — 2 minutes

From the repo root:

```bash
git add -A
git commit -m "Ship sand-sim v0.1 — three elements, brush, audio"
git tag -a v0.1 -m "Month 1 milestone: working falling-sand sandbox"
git push origin main
git push origin v0.1
```

The `-a` makes it an **annotated tag** (carries a message and a timestamp) rather than a lightweight tag. Annotated tags show up as releases on GitHub.

> **The Wow Moment.** Open your GitHub repo in the browser. Look at the right-hand side under **Releases** — `v0.1` is there. **You shipped software.** A real, runnable, downloadable, *yours*. Send the link to one person. Their reaction is the moment you start being a software developer.

---

## Common mistakes

### `error: could not find native library 'asound'` (Linux)

You enabled macroquad's `audio` feature but ALSA headers aren't installed. `sudo apt install -y libasound2-dev` and re-run `cargo build`. Re-check [SETUP.md](../../SETUP.md) if other platform-specific things break.

### `Error opening sand.wav: NoSuchFile`

Working directory issue. `cargo run` runs from the directory containing `Cargo.toml`, so `assets/sand.wav` must be at `sand-sim-v0.1/assets/sand.wav` (relative to the manifest). Quickest sanity check: `ls assets/sand.wav` from the project directory.

### Pause toggle pauses *and* unpauses on a single keystroke

You used `is_key_down` instead of `is_key_pressed`. `is_key_down` returns `true` every frame the key is held — so it toggles many times per second. `is_key_pressed` fires exactly once per press.

### Audio crackles or stutters

Calling `play_sound_once` 60 times per second is too many overlapping playbacks. Add the cooldown shown in step 4. Or run with `--release`, which sometimes resolves audio jitter caused by debug-mode CPU usage.

### `cargo run --release` is *slower* than debug

You probably ran it once and saw the *compile* time, not the runtime. Run it twice — the second run is the real measurement. (LTO compile is genuinely slow; runtime is fast.)

### Ubuntu-specific: "No such file or directory: assets/sand.wav" but the file is there

You ran `cargo run` from a different folder. On Ubuntu the working-dir-is-Cargo.toml-folder rule still holds, but some shells (especially nested terminals inside tmux) keep an old `pwd`. Run `pwd` to confirm you're in `sand-sim-v0.1/`. If you launched via `./target/release/sand-sim` instead of `cargo run`, the binary's working directory is *your shell's* cwd — `cd` into `sand-sim-v0.1/` first, then run the binary, or move `assets/` next to wherever you store the binary.

### `git tag` already exists

Annotated tags can't be silently overwritten. To replace one: `git tag -d v0.1 && git push origin :refs/tags/v0.1 && git tag -a v0.1 ...` Or pick a new tag (`v0.1.1`) and ship that instead.

---

## Session challenge

Pick one — no solution provided. (These are stretch ideas; the milestone is the main deliverable today.)

1. **Window-resize handling.** Right now constants assume 720×480. Read `screen_width()` / `screen_height()` each frame and resize the grid dynamically. Tricky: what do you do with cells that fall outside the new bounds? (Crop? Push back?)
2. **Mute toggle.** Press `M` to set a `let mut muted = false;` to `true`. Wrap the `play_sound_once` call in `if !muted { ... }`.
3. **A second sound effect.** Add a *plink* WAV for when stone is placed. Record the source in CREDITS.
4. **Screenshot key.** `P` → `get_screen_data().export_png("screenshot.png");`. Drop the screenshot in your DofE log as evidence of v0.1.

---

## Quick reference

| What | Code |
|---|---|
| Enable audio | `macroquad = { version = "0.4", features = ["audio"] }` |
| Load a sound | `let s = load_sound("assets/x.wav").await.unwrap();` |
| Play it once | `play_sound_once(&s);` |
| Window width | `screen_width()` |
| Frame rate | `get_fps()` |
| Tap-to-toggle | `if is_key_pressed(K::Space) { paused = !paused; }` |
| Release build | `cargo run --release` |
| Annotated tag | `git tag -a v0.1 -m "message"` |
| Push the tag | `git push origin v0.1` |

---

## DofE log reminder

This is the **biggest** log entry of Month 1. Open [`dfe/session-log.md`](../../dfe/session-log.md) and fill in **Session 8**, then also fill in **[`dfe/milestone-1-reflection.md`](../../dfe/milestone-1-reflection.md)**. Worth capturing:

- A screenshot of `sand-sim` v0.1 (use the snapshot key, or your OS screenshot shortcut)
- The link to the v0.1 release on your GitHub
- A two-sentence answer to *"what does this prove you can do that you couldn't do in Session 1?"*

→ Onwards to [Month 2: The Chemistry](../../month-2/README.md).
