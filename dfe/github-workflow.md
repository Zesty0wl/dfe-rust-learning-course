# Optional: Use GitHub as Your Working Log

This course can be completed two ways. Both end with the **same printed binder** for the assessor:

- **Path A — Pen-and-paper.** Print the booklet up front, fill it in by hand after every session. Simple, offline, robust. Detailed in [`print-checklist.md`](./print-checklist.md).
- **Path B — Fork-and-commit (this document).** Fork the course on GitHub, treat the repo as your working log, and print the binder at the end (or at milestones). You get an automatic dated commit history as a second layer of evidence — and you learn the version-control workflow real developers use.

Both paths are equally valid for DofE. Pick the one that fits how you like to work.

> **You don't have to choose now.** You can start on Path A and switch to Path B later (or vice versa). The session log files have the same numbering in both formats.

---

## Why this is a useful path

- **The repo itself becomes evidence.** Every time you `git commit`, GitHub records the date, time, and exactly what you changed. Twenty-four commits spread across twelve weeks is a tamper-resistant timeline that's far harder to fake than typed log entries created in one evening.
- **You learn the real-world workflow.** Professional Rust developers all use git and GitHub. Doing your DofE skill *with* the tools the skill is about is a nice piece of authenticity.
- **One link covers everything.** If your assessor *is* technical and would rather click than flip pages, a single repo URL contains the code, the log, and the timestamps.

---

## The workflow in one paragraph

Fork the course repo to your own GitHub account, clone it to your laptop, work through a session, edit `dfe/session-log.md` to fill in that session's row, then commit and push with a message like `session 5: ownership and borrowing`. After Sessions 8, 16, and 24, fill in the matching `milestone-N-reflection.md` and commit. After Session 24, write `participant-statement-template.md`, commit, then print everything for the assessor binder. Done.

---

## Prerequisites

You need a **GitHub account** (free). If you're under 13 you cannot have one; if you're 13–17 read GitHub's terms and check with your parent or guardian. For everyone, the [official GitHub docs](https://docs.github.com/get-started/start-your-journey/creating-an-account-on-github) cover account setup in five minutes.

You also need **git** installed. The course's [`SETUP.md`](../SETUP.md) walks you through it; the short version on macOS is `xcode-select --install`, on Windows it's [git-scm.com/download/win](https://git-scm.com/download/win), on Linux it's `sudo apt install git` (or your distro's equivalent).

---

## Step-by-step setup

### 1. Fork the repo

Open the course in your browser:

> <https://github.com/Zesty0wl/dfe-rust-learning-course>

Click the **Fork** button (top right). Make sure the owner is *your* account, leave the name as-is, and click **Create fork**. You now have your own copy at `https://github.com/<your-username>/dfe-rust-learning-course`.

### 2. Clone your fork to your laptop

In a terminal:

```bash
git clone https://github.com/<your-username>/dfe-rust-learning-course.git
cd dfe-rust-learning-course
```

### 3. Tell git who you are (one-time)

```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

Use the same email as your GitHub account so commits are linked to your profile.

### 4. Confirm everything works

```bash
cd month-1/session-01/examples/hello_world
cargo run
```

You should see a Hello World message. If not, fix your Rust install before continuing — see `SETUP.md`.

---

## The per-session loop

After every session (about 5 extra minutes on top of the session itself):

```bash
# 1. Make sure you're up to date
git pull

# 2. Open dfe/session-log.md in your editor and fill in this session's row.
#    (Or open dfe/session-log-template.md, copy a block, and paste it into a
#    notes file inside the session folder if you'd rather keep them separate.)

# 3. Stage and commit
git add dfe/session-log.md
git commit -m "session 5: ownership and borrowing - log entry"

# 4. Push to GitHub
git push
```

Use the date of the session in the commit. Even if you write the log the same evening, the commit timestamp is what the assessor sees.

---

## Suggested commit-message convention

Consistent commit messages make your history easy to scan. Pick a pattern and stick to it. A good one:

```
session NN: <topic from the session README> - <what you did>
```

Examples:

```
session 01: hello world - first run, edited message
session 02: variables and types - finished examples, log
session 08: milestone - sand-sim v0.1 scaffolded
session 08: milestone - sand and water working
milestone 1: reflection committed
session 16: project - ascii world generator complete
milestone 3: participant statement
```

A clean history of about **24–40 commits** across 12 weeks is a stronger artefact than 200 noisy `wip` commits.

---

## Generating a printable commit history

When you put the binder together, include a one-page printout of your commit history. This is the killer page for a technical assessor and a nice "wow" page for a non-technical one.

A helper script is provided:

```bash
bash dfe/scripts/print-commit-history.sh > my-commit-history.txt
```

(or run the underlying command directly:)

```bash
git log --reverse --pretty=format:"%ad  %h  %s" --date=short > my-commit-history.txt
```

Open `my-commit-history.txt` in any text editor and print it. It's a dated, line-per-commit summary of everything you did.

For a richer one-pager that includes session totals, the script also prints a header with your name, the date range, and a count of commits.

---

## What the assessor sees

Your assessor still gets the **printed binder** — they don't need to know git or open GitHub. But you'll add **two extra pages** at the back:

1. **Commit history printout** (`my-commit-history.txt`, printed) — 24+ dated lines proving regular activity.
2. **A single short URL** at the bottom of your participant statement: "Source on GitHub: github.com/&lt;your-username&gt;/dfe-rust-learning-course". A technical assessor can click; a non-technical one can ignore it.

The session log itself can be either:

- printed and **typed-and-bound** (you fill `session-log.md` digitally and print at the end), or
- printed and **handwritten** (you print `session-log-printable.md` blank, fill it in by hand, and ignore the digital version).

Handwritten still has the highest "this is genuine" signal. If you can manage both — typed log in git *and* a handwritten copy in the binder — you've left no room for doubt.

---

## Branching, pull requests, etc.

You **don't need** to use branches, pull requests, or any of git's more advanced features. A single `main` branch with linear commits is fine and is what most beginners use.

If you want to stretch yourself, an optional pattern is to use a branch per milestone:

```bash
git switch -c milestone/sand-sim-v0.1
# ... work ...
git push -u origin milestone/sand-sim-v0.1
# Open a pull request on github.com from this branch into main
# Review your own PR, then merge it
```

This mimics the workflow used in real software teams and gives you something extra to talk about in your participant statement. It's not required.

---

## Common questions

**My commits show up under a different account / no profile picture on GitHub.**
Check `git config --global user.email` matches your GitHub email. You can also add the email under [github.com/settings/emails](https://github.com/settings/emails).

**I forgot to commit after a session — is that a problem?**
No. Commit when you next open the project, but use the original session date in the commit message body so the timeline is honest:

```bash
git commit -m "session 7: error handling - log entry

Session held on 2026-02-14; log written the next day."
```

**Should I make my fork public or private?**
Public is simpler and lets the assessor click straight in. Private is fine too — you'd need to add the assessor as a collaborator if they want to view it. If unsure, public is the default and contains nothing sensitive.

**What if I break my repo?**
You can always re-fork from `Zesty0wl/dfe-rust-learning-course`. As long as your `dfe/session-log.md` is somewhere safe (commit early, commit often), you can reconstruct the rest.

---

## Whichever path you choose

Path A and Path B both produce the same printed binder for the assessor. Path B just adds a second, automatic, tamper-resistant layer of evidence — and teaches you a real industry skill along the way.

If you've got the time and access, **do Path B**. If you're short on either, **do Path A**. Neither will cost you DofE marks.
