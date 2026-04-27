# SETUP

Welcome. Before Session 1 you need Rust, Cargo, and a code editor. This guide covers Windows, macOS, and Linux.

> **Good news:** Rust is genuinely cross-platform. Once installed, every project in this course compiles and runs identically on all three operating systems. Pick your platform's section below and ignore the others.

---

## Windows (primary)

This is the most detailed section because Windows has one extra step that trips people up.

### 1. Install the MSVC Build Tools

This is the step most tutorials skip. Rust's default Windows toolchain links against Microsoft's linker (`link.exe`), which ships with the Visual Studio Build Tools. **You don't need full Visual Studio** — just the Build Tools.

Two ways to install:

- **Recommended (PowerShell):** `winget install Microsoft.VisualStudio.2022.BuildTools`
- **Manual:** Download the Build Tools installer from <https://visualstudio.microsoft.com/visual-cpp-build-tools/>. In the installer, tick **"Desktop development with C++"** under Workloads, then click Install.

> **Why?** Rust on Windows by default produces MSVC-style binaries that link with the same toolchain Microsoft uses. Without `link.exe`, Cargo can't finish a build.
>
> There is an alternative `x86_64-pc-windows-gnu` toolchain that avoids this step, but it's not recommended for this course because some crates behave differently. Stick with MSVC.

### 2. Install Rust via rustup

- **Recommended:** `winget install Rustlang.Rustup`
- **Manual:** Download from <https://rustup.rs> and run `rustup-init.exe`.

When prompted, accept the default options.

### 3. Verify the installation

Open a **new** Windows Terminal (PowerShell or Command Prompt). New, because PATH is only updated for new terminal sessions.

```powershell
rustc --version
cargo --version
```

You should see something like `rustc 1.78.0` and `cargo 1.78.0`. The version number must be **1.75 or higher**.

### 4. Install VS Code

```powershell
winget install Microsoft.VisualStudioCode
```

### 5. Install the rust-analyzer extension

Open VS Code → **Extensions** (the icon in the left sidebar, or `Ctrl+Shift+X`) → search `rust-analyzer` → install the official extension published by **The Rust Programming Language** group.

### 6. Install Python (Session 1 demo only)

```powershell
winget install Python.Python.3
```

### 7. Test the whole chain

```powershell
cargo new hello_test
cd hello_test
cargo run
```

Expected output:

```
   Compiling hello_test v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in ...s
     Running `target\debug\hello_test.exe`
Hello, world!
```

If you see `Hello, world!`, you're done. Move on to Session 1.

---

## macOS

### 1. Install the Xcode Command Line Tools

```bash
xcode-select --install
```

A pop-up dialog will appear — click **Install**. Takes a few minutes. This package provides `clang`, `make`, and other development essentials. **Mandatory** before any toolchain will work on macOS.

### 2. Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Accept defaults. Then either open a new terminal **or**:

```bash
source "$HOME/.cargo/env"
```

### 3. Verify

```bash
rustc --version && cargo --version
```

Should print `rustc 1.75` or higher and `cargo 1.75` or higher.

### 4. Install VS Code

Either download from <https://code.visualstudio.com> or, if you have Homebrew:

```bash
brew install --cask visual-studio-code
```

### 5. Install the rust-analyzer extension

Same as Windows: VS Code → Extensions → search `rust-analyzer` → install the one published by The Rust Programming Language group.

### 6. Python

macOS 13+ ships with Python 3:

```bash
python3 --version
```

If missing: `brew install python3`.

### 7. Test

```bash
cargo new hello_test && cd hello_test && cargo run
```

> **Apple Silicon (M1/M2/M3/M4) note:** `rustup` installs the `aarch64-apple-darwin` target by default. This is correct and faster than running x86 binaries through Rosetta. All course projects are compatible.

---

## Linux

### 1. Install build dependencies

| Distro | Command |
|---|---|
| Ubuntu / Debian | `sudo apt update && sudo apt install -y build-essential curl` |
| Arch / Manjaro | `sudo pacman -S --needed base-devel curl` |
| Fedora / RHEL | `sudo dnf groupinstall "Development Tools" && sudo dnf install -y curl` |

### 2. Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 3. Verify

```bash
rustc --version && cargo --version
```

### 4. Install VS Code

| Distro | Command |
|---|---|
| Ubuntu / Debian | Download `.deb` from <https://code.visualstudio.com>, then `sudo dpkg -i code_*.deb` |
| Arch | `yay -S visual-studio-code-bin` (or use the official `code` package) |
| Fedora | Follow the [official RPM repo instructions](https://code.visualstudio.com/docs/setup/linux) |

### 5. Install the rust-analyzer extension

Same as the other platforms.

### 6. Python

Most distros ship Python 3. Check with `python3 --version`. If missing on Debian/Ubuntu: `sudo apt install python3`.

### 7. Audio dependencies for Month 3

The `cpal` and `midir` crates used in Month 3 link against ALSA. On Debian/Ubuntu:

```bash
sudo apt install -y libasound2-dev pkg-config
```

On Fedora: `sudo dnf install -y alsa-lib-devel pkgconfig`.
On Arch: `sudo pacman -S --needed alsa-lib pkgconf`.

(You can install this later — you only need it before Month 3.)

### 8. Test

```bash
cargo new hello_test && cd hello_test && cargo run
```

---

## Common problems

| Problem | Platform | Fix |
|---|---|---|
| `error: linker 'link.exe' not found` | Windows | MSVC build tools missing or not on PATH. Re-run the Build Tools installer and ensure **"Desktop development with C++"** is ticked. Then open a new terminal. |
| `xcrun: error: invalid active developer path` | macOS | Run `xcode-select --install`. |
| `error: could not find native static library 'c'` | Linux | Run `sudo apt install build-essential`. |
| `alsa/asoundlib.h: No such file or directory` | Linux | Run `sudo apt install libasound2-dev pkg-config`. Needed for Month 3. |
| `rustc: command not found` after install | All | Close and reopen the terminal, or run `source "$HOME/.cargo/env"` (macOS/Linux). |
| VS Code shows no inline hints | All | Ensure you opened the project as a **folder** (`File → Open Folder…`), not just a single `.rs` file. `rust-analyzer` needs a `Cargo.toml` in scope. |
| `cargo` is very slow on first build | All | First builds download and compile dependencies; this is one-off. Subsequent builds are incremental and fast. |
| Permission denied on Linux when running compiled binary | Linux | `chmod +x target/debug/<binary>` or run with `cargo run`. |

---

## What good looks like

When you can run `cargo new test_project && cd test_project && cargo run` and see `Hello, world!`, your environment is ready.

Head to [`month-1/README.md`](./month-1/README.md) and begin Session 1.
