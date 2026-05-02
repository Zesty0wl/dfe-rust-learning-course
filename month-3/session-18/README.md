# Session 18: File I/O and Binary Data

> 📖 **Stuck on a term?** Words like *immutable*, *compiler*, *borrow*, *trait* etc. are all defined in plain English in the [GLOSSARY.md](../../GLOSSARY.md) at the repo root.

> 🎹 **New to music theory?** Notes, octaves, scales, MIDI numbers, frequencies — they're all explained from scratch in the [MUSIC-THEORY-PRIMER.md](../../MUSIC-THEORY-PRIMER.md) (10-minute read, has a labelled piano-keyboard diagram). You don't need to be a musician to do this course.

## What You'll Learn

How to read and write files in Rust, both text and binary. We'll go a step further than reading text — we'll **write a valid WAV audio file from scratch, using only the standard library**, that plays in any media player.

This is the foundation of the entire final project.

## The Big Idea

Files are just sequences of bytes. **Text files** happen to contain bytes that decode as UTF-8 characters. **Binary files** contain bytes that mean other things — image pixels, audio samples, ZIP-compressed data, MP4 video frames.

Writing a binary file is just writing bytes in the format the consumer expects. WAV files have a precise specification, which we'll implement byte-by-byte.

You'll generate a 1-second 440 Hz sine wave (concert A — pitch reference for orchestras worldwide) and produce a `.wav` file you can open in QuickTime, VLC, Audacity, or Windows Media Player. Bring headphones.

## Concepts Covered

- `std::fs::read_to_string`, `std::fs::write` — text-file convenience functions
- `std::fs::File`, `std::io::Write`, `std::io::Read` — the underlying traits
- `BufWriter` / `BufReader` — wrap a file for buffered (faster) I/O
- Writing raw bytes with `.write_all(&[u8])`
- Little-endian vs big-endian byte order
- The WAV file format (RIFF + format chunk + data chunk)
- Integer-to-bytes conversion: `u32::to_le_bytes()`, `i16::to_le_bytes()`
- Sample-rate, amplitude, and frequency as code

## Building Towards `midi-synth`

In Session 21 you'll use the `hound` crate (which does exactly what we'll do today, but with all the polish). Today's hand-rolled version teaches you what's *actually happening* under the hood. By the time `hound` enters the picture, the magic will already be gone.

---

> 💡 **How to run the examples in this session.** Every example below lives in its own folder under `month-3/session-18/examples/`. From a fresh terminal **at the root of the repo**, run:
>
> ```bash
> cd month-3/session-18/examples/<example-folder>
> cargo run
> ```
>
> Replace `<example-folder>` with the name shown in each section (e.g. `chromatic_scale`). Always start `cd`-ing from the repo root so you don't get lost.

## Step-by-Step Walkthrough

### 1. Reading and writing text files (warm-up)

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::write("hello.txt", "Hello, file!")?;
    let contents = fs::read_to_string("hello.txt")?;
    println!("Read back: {}", contents);
    Ok(())
}
```

Two helpers from `std::fs` cover 90% of text I/O:
- `fs::write(path, contents)` — overwrite a file with these contents (creates if needed).
- `fs::read_to_string(path)` — read the whole file into a `String`.

`std::io::Result<()>` is `Result<(), std::io::Error>` — the error type for file/network I/O.

### 2. The WAV file format

A WAV file is **a small header followed by raw audio samples**. The header itself is built of "chunks" — labelled blocks of bytes. Here is the layout we'll write:

```
Offset  Size  Field                  Value
------  ----  ---------------------  -----
0       4     "RIFF" magic           bytes 0x52 0x49 0x46 0x46
4       4     File size minus 8      u32 little-endian
8       4     "WAVE" marker          bytes 0x57 0x41 0x56 0x45

12      4     "fmt " chunk id        bytes 0x66 0x6D 0x74 0x20
16      4     fmt chunk size         u32 = 16 (for PCM)
20      2     audio format           u16 = 1 (PCM)
22      2     num channels           u16 = 1 (mono)
24      4     sample rate            u32 = 44100
28      4     byte rate              u32 = sample_rate * channels * (bits/8)
32      2     block align            u16 = channels * (bits/8)
34      2     bits per sample        u16 = 16

36      4     "data" chunk id        bytes 0x64 0x61 0x74 0x61
40      4     data chunk size        u32 = num_samples * channels * (bits/8)
44      ...   raw samples            i16 little-endian, one after another
```

44 bytes of header, then your audio. **Little-endian** means low byte first — `0x12345678` is written as `0x78 0x56 0x34 0x12`. Rust gives us `.to_le_bytes()` on every integer type to do this conversion.

### 3. Generating a sine wave

The maths: a sine wave at frequency `f` has the formula

$$y(t) = A \cdot \sin(2\pi f t)$$

where `A` is amplitude (how loud) and `t` is time in seconds. To digitise this, we **sample** it at fixed intervals — `sample_rate` times per second. CD-quality audio uses 44,100 samples per second.

For each sample number `n`:
- `t = n / sample_rate`
- value = `A * sin(2π * f * t)`

Each sample is then converted from a float in `[-1.0, 1.0]` to a 16-bit signed integer in `[-32767, 32767]`.

### 4. The complete WAV writer

`examples/wav_writer/src/main.rs`:

```rust
use std::fs::File;
use std::io::{BufWriter, Write};

const SAMPLE_RATE: u32 = 44100;
const CHANNELS:    u16 = 1;
const BITS:        u16 = 16;
const FREQUENCY:   f32 = 440.0;
const DURATION_S:  f32 = 1.0;
const AMPLITUDE:   f32 = 0.3;     // 0.0 silent, 1.0 max — keep modest

fn main() -> std::io::Result<()> {
    let total_samples = (SAMPLE_RATE as f32 * DURATION_S) as u32;

    // Generate samples
    let mut samples: Vec<i16> = Vec::with_capacity(total_samples as usize);
    for n in 0..total_samples {
        let t = n as f32 / SAMPLE_RATE as f32;
        let v = AMPLITUDE * (2.0 * std::f32::consts::PI * FREQUENCY * t).sin();
        samples.push((v * i16::MAX as f32) as i16);
    }

    let file = File::create("a440.wav")?;
    let mut w = BufWriter::new(file);

    let data_size: u32 = total_samples * CHANNELS as u32 * (BITS / 8) as u32;
    let file_size: u32 = 36 + data_size;

    // RIFF header
    w.write_all(b"RIFF")?;
    w.write_all(&file_size.to_le_bytes())?;
    w.write_all(b"WAVE")?;

    // fmt chunk
    w.write_all(b"fmt ")?;
    w.write_all(&16u32.to_le_bytes())?;                     // chunk size
    w.write_all(&1u16.to_le_bytes())?;                      // PCM
    w.write_all(&CHANNELS.to_le_bytes())?;
    w.write_all(&SAMPLE_RATE.to_le_bytes())?;
    let byte_rate: u32 = SAMPLE_RATE * CHANNELS as u32 * (BITS / 8) as u32;
    w.write_all(&byte_rate.to_le_bytes())?;
    let block_align: u16 = CHANNELS * (BITS / 8);
    w.write_all(&block_align.to_le_bytes())?;
    w.write_all(&BITS.to_le_bytes())?;

    // data chunk
    w.write_all(b"data")?;
    w.write_all(&data_size.to_le_bytes())?;
    for s in &samples {
        w.write_all(&s.to_le_bytes())?;
    }

    w.flush()?;
    println!("Wrote a440.wav: 1 second of 440 Hz sine, mono, 44.1 kHz, 16-bit.");
    println!("Open it in any audio player. Headphones recommended; not loud.");
    Ok(())
}
```

### 5. Run it and listen

```bash
cargo run
open a440.wav     # macOS
xdg-open a440.wav # Linux
start a440.wav    # Windows
```

You should hear a clean, pure tone for one second. **You generated that from maths.** Every sample, every byte, every bit was assembled by code you wrote (or read carefully). No audio library. No framework. Just bytes.

If you have Audacity installed, drag the file in. You can see the waveform — it looks like a textbook sine wave because that's literally what it is.

### 6. Why `BufWriter`?

Without `BufWriter`, every `write_all` call could be a syscall — a round-trip to the operating system. For 44,100 separate 2-byte writes, that's a lot of overhead. `BufWriter` accumulates writes into a 8 KB buffer and flushes them in big chunks. Same correctness, much faster. Always wrap files with `BufWriter` when writing many small pieces.

`BufReader` is the dual: wrap a file you're reading byte-by-byte to read in big chunks under the hood.

### 7. `.flush()` matters

`BufWriter` only writes to the underlying file when its internal buffer fills up. `.flush()` forces it to write whatever's queued. Without it, the last few bytes of your file might never make it to disk — and the WAV header would be wrong (the file size would be too short, and the player might refuse to open it).

For non-`BufWriter` writes, `flush` is usually a no-op. For buffered writes, **always call `flush()` at the end**. Or let the `BufWriter` go out of scope cleanly — its `Drop` does it for you, but explicit is safer.

---

## Common Mistakes

- **Wrong endianness** — using `to_be_bytes()` (big-endian) instead of `to_le_bytes()`. WAV is little-endian; using big-endian produces a file that opens but plays garbage.
- **Wrong file size in header** — the RIFF size must be `36 + data_size`, not the total file length, not the data length. Off-by-eight is a common bug.
- **Forgetting `.flush()`** — a corrupt or truncated file. Always flush explicitly.
- **Forgetting the `b` prefix on byte literals** — `"RIFF"` is a `&str`, `b"RIFF"` is `&[u8; 4]`. The `write_all` for chunk IDs needs bytes.
- **Sample clipping** — multiplying a float by `i16::MAX` and casting works *if* the float is in `[-1.0, 1.0]`. Outside that range, `as i16` truncates and you get distortion. Keep amplitude well below 1.0.

---

## Session Challenge

1. Change the frequency to 220 Hz. Listen — it's an octave lower.
2. Change duration to 0.25 seconds. Listen — a short pip.
3. Add a second sine wave at 554 Hz (C#5) and **average** the two samples (sum / 2). You've just synthesised an A major third — a chord!
4. Write a stereo (2-channel) version: in stereo, samples interleave L, R, L, R, … Update `CHANNELS = 2`, `block_align`, and the data chunk size accordingly.

---

## Quick Reference

```rust
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

// Text — convenience
fs::write("file.txt", "data")?;
let s = fs::read_to_string("file.txt")?;

// Bytes — convenience
fs::write("file.bin", &[0u8, 1, 2, 3])?;
let bytes = fs::read("file.bin")?;            // Vec<u8>

// Streaming
let f = File::create("out.bin")?;
let mut w = BufWriter::new(f);
w.write_all(b"hello")?;
w.write_all(&42u32.to_le_bytes())?;
w.flush()?;

let f = File::open("in.bin")?;
let mut r = BufReader::new(f);
let mut buf = [0u8; 4];
r.read_exact(&mut buf)?;
let n = u32::from_le_bytes(buf);
```

---

## Further Reading

Curated extra material on the topics covered in this session (File I/O and Binary Data). All free; all current as of writing.

- [**The Rust Book** — *Reading a File* (12.2)](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html) — The smallest possible end-to-end file-reading example.
- [**`std::fs` documentation**](https://doc.rust-lang.org/std/fs/) — All the file-system primitives. Read about `File`, `OpenOptions`, and `read_to_string`.
- [**`byteorder` crate** — endianness-aware binary reads/writes](https://docs.rs/byteorder/latest/byteorder/) — Indispensable when you start parsing binary formats.
- [**Wikipedia — *Endianness***](https://en.wikipedia.org/wiki/Endianness) — Why MIDI is big-endian, WAV is little-endian, and why anyone should care.

---

## Stuck?

You're not the first. Three places that work when you're properly stuck:

- [**Rust Discord** — `#beginners`](https://discord.gg/rust-lang-community) (fastest; people are friendly)
- [**`/r/learnrust`**](https://www.reddit.com/r/learnrust/) (paste your code + the error; usually answered within hours)
- [**`users.rust-lang.org`**](https://users.rust-lang.org/) (slower; thorough; answers stay searchable for years)

When the compiler error is the thing confusing you, [`resources/compiler-errors.md`](../../resources/compiler-errors.md) translates the most common ones into plain English.

Asking for help isn't cheating — real Rust developers do it daily. Search first; if no luck, post a [minimal reproducible example](https://stackoverflow.com/help/minimal-reproducible-example).

---
## DofE Log Reminder

Row 18. Note the moment you heard your own audio — that's a memorable session.
