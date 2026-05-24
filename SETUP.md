# SETUP — Install Rust and everything else you need

> **No prior programming experience required.** You don't need Python, Node, Java, or any other toolchain on your machine. Everything in this course runs from `cargo` — Rust's package manager — and a code editor. This page lists every single thing you need.

Rust runs identically on Windows, macOS, and Linux. Every project in this course compiles and runs unmodified on all three. **Follow only your platform's section below** — you don't need to read the others.

The course's main dependency is [`macroquad`](https://github.com/not-fl3/macroquad), a small 2D game framework. It opens a window, reads mouse/keyboard input, draws rectangles and text, and plays sounds. On Windows and macOS the Rust toolchain installer takes care of everything macroquad needs. On Linux you'll need a few extra system packages — see the Linux section below.

---

## Windows (primary platform — most detailed)

### 1. Install the MSVC Build Tools

This is the step most Rust tutorials skip and it causes the most confusion. The default Rust toolchain on Windows (`x86_64-pc-windows-msvc`) links against Microsoft's C++ linker (`link.exe`), and that linker ships with Visual Studio's Build Tools — not with Rust itself.

You do **not** need the full Visual Studio IDE. Get the standalone Build Tools:

- **Quickest:** open PowerShell and run
  ```powershell
  winget install Microsoft.VisualStudio.2022.BuildTools
  ```
- **Or download manually:** <https://visualstudio.microsoft.com/visual-cpp-build-tools/> → run the installer → tick **"Desktop development with C++"** → install.

> **Note on the GNU toolchain alternative.** Rust also ships a `x86_64-pc-windows-gnu` target that uses the MinGW linker and avoids the MSVC step. We don't recommend it for this course — some crates behave subtly differently and the error messages are worse. Stick with MSVC.

### 2. Install Rust via rustup

```powershell
winget install Rustlang.Rustup
```

…or download `rustup-init.exe` from <https://rustup.rs> and run it. Accept all the defaults (host triple: `x86_64-pc-windows-msvc`, default toolchain: `stable`).

### 3. Verify the installation

**Close and reopen your terminal** (the installer adds Cargo to `PATH` and existing terminals won't see it). Then:

```powershell
rustc --version
cargo --version
```

You should see something like `rustc 1.78.0 (9b00956e5 2024-04-29)` and `cargo 1.78.0`. Any version `1.75` or newer is fine.

### 4. Install VS Code

```powershell
winget install Microsoft.VisualStudioCode
```

### 5. Install the rust-analyzer extension

Open VS Code → click the Extensions icon in the left sidebar (or press `Ctrl+Shift+X`) → search `rust-analyzer` → install the one published by **"The Rust Programming Language"**. Don't install any of the older `Rust` extensions — they're deprecated.

### 6. Test everything

```powershell
cargo new hello_test
cd hello_test
cargo run
```

Expected output: `Hello, world!` after a brief compile. If you see this, you're done.

---

## macOS

### 1. Install Xcode Command Line Tools

Mandatory before any development tooling will work on macOS. Open Terminal:

```bash
xcode-select --install
```

A system dialog will pop up — click **Install** and wait a few minutes.

### 2. Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Accept the defaults. After install, either open a new terminal **or** run:

```bash
source "$HOME/.cargo/env"
```

On Apple Silicon (M1/M2/M3/M4), `rustup` installs the `aarch64-apple-darwin` target by default — which is correct, native, and faster than Rosetta. All course projects are compatible.

### 3. Verify the installation

```bash
rustc --version && cargo --version
```

Any version `1.75` or newer is fine.

### 4. Install VS Code

Either download the `.dmg` from <https://code.visualstudio.com> and drag to `/Applications`, or — if you have Homebrew — `brew install --cask visual-studio-code`.

### 5. Install the rust-analyzer extension

Open VS Code → Extensions → search `rust-analyzer` → install the **"The Rust Programming Language"** one.

### 6. Test everything

```bash
cargo new hello_test && cd hello_test && cargo run
```

Expected output: `Hello, world!`.

---

## Linux (Ubuntu / Debian / Arch / Fedora)

### 1. Install build dependencies

macroquad needs a working C toolchain (for linking) and the X11 + OpenGL development headers (for windowing). The ALSA headers are also useful — they're needed if/when you add sound effects in the Month 1 and Month 2 milestones.

**Ubuntu / Debian:**
```bash
sudo apt update
sudo apt install -y build-essential curl pkg-config \
    libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

**Arch / Manjaro:**
```bash
sudo pacman -S --needed base-devel curl pkgconf libx11 libxi mesa alsa-lib
```

**Fedora:**
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install curl pkgconf libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel
```

> **Wayland note.** macroquad uses X11. On a Wayland desktop (GNOME, KDE Plasma 6 default) it runs under XWayland, which is installed by default on every mainstream distro. If you see a black window with no input, log out and log in on an X11 session, or check that XWayland is running (`pgrep Xwayland` should print a PID).

### 2. Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Accept the defaults. Then either open a new terminal or run:

```bash
source "$HOME/.cargo/env"
```

### 3. Verify

```bash
rustc --version && cargo --version
```

### 4. Install VS Code

- **Ubuntu/Debian:** download the `.deb` from <https://code.visualstudio.com/download> → `sudo dpkg -i code_*.deb` (then `sudo apt -f install` if needed).
- **Arch:** `yay -S visual-studio-code-bin` (AUR), or use the open-source VSCodium: `sudo pacman -S code`.
- **Fedora:** Microsoft's RPM repo — see <https://code.visualstudio.com/docs/setup/linux>.

### 5. Install the rust-analyzer extension

Same as Windows/macOS — Extensions → `rust-analyzer` → install the **"The Rust Programming Language"** one.

### 6. Test everything

```bash
cargo new hello_test && cd hello_test && cargo run
```

Expected output: `Hello, world!`.

---

## Common problems

| Problem | Platform | Fix |
|---|---|---|
| `error: linker 'link.exe' not found` | Windows | MSVC build tools not installed (or not on PATH). Re-run the Build Tools installer and ensure **"Desktop development with C++"** is checked. |
| `xcrun: error: invalid active developer path` | macOS | Run `xcode-select --install` |
| `error: could not find native static library 'c'` | Linux | Run `sudo apt install build-essential` |
| `cannot find -lX11` / `cannot find -lGL` | Linux | Run `sudo apt install libx11-dev libxi-dev libgl1-mesa-dev pkg-config` — these are macroquad's runtime deps |
| `alsa/asoundlib.h: No such file or directory` | Linux | Run `sudo apt install libasound2-dev` (needed once sound effects are added in Sessions 8 / 16) |
| Black window with no input on Wayland | Linux | macroquad uses X11; XWayland should handle this. If it doesn't, log in on an X11 session or check `pgrep Xwayland` |
| `rustc --version` not found after install | All | Close and reopen the terminal, or run `source "$HOME/.cargo/env"` (Linux/macOS) |
| VS Code not showing rust-analyzer hints | All | Open the project as a **folder** (`File → Open Folder`), not just a single file. rust-analyzer needs a `Cargo.toml` in scope. |
| `cargo run` opens a window then immediately exits | All | Make sure `main` is `async fn main()` annotated with `#[macroquad::main("…")]` and ends with a `loop { … next_frame().await; }` — without the loop, the program exits after one frame |
| `error[E0432]: unresolved import macroquad` | All | You forgot `cargo add macroquad`, or you're running from the wrong directory — `cd` into the folder with `Cargo.toml` |

---

## Once you're done

Head back to the [main README](./README.md) and start with [`month-1/session-00/`](./month-1/session-00/) for the pre-flight checklist, then [`month-1/README.md`](./month-1/README.md) for the course proper.
