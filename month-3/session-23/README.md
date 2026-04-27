# Session 23 — Live MIDI: `midir`, `cpal`, and Threads with `mpsc`

> *"This week the program plays in real time. Press a key on a real MIDI keyboard, hear it through real speakers. Welcome to systems programming."*

## What You'll Learn

- Cross-platform live MIDI input with [`midir`](https://docs.rs/midir).
- Cross-platform audio output with [`cpal`](https://docs.rs/cpal).
- Multi-threading with `std::sync::mpsc` channels — the producer/consumer pattern.
- Why audio callbacks must be **lock-light and allocation-free**.

## The Big Idea

There are three threads in a live synth:

```
   ┌──────────────┐  NoteOn/Off  ┌──────────────┐
   │ MIDI thread  ├─────────────→│  mpsc::tx    │
   └──────────────┘              └─────┬────────┘
                                       │
                                       ▼
   ┌──────────────┐  fill samples ┌──────────────┐
   │ Audio thread │←──────────────│   rx + voices│
   └──────────────┘               └──────────────┘
```

The MIDI thread fires whenever your keyboard sends a byte. The audio thread fires every few milliseconds asking for more samples. They talk through a channel.

If the audio thread blocks for more than a few ms, the speaker glitches. So no `println!`, no `Mutex` contention, no allocation in the hot path. (We *do* take a `Mutex` here — but very briefly. For a real product you'd use a lock-free ring buffer.)

## Concepts Covered

- `midir::MidiInput`, ports, `connect()` callback.
- `cpal::Host → Device → Stream` pattern.
- `std::sync::mpsc::channel()` for thread-safe message passing.
- `Arc<Mutex<Vec<Voice>>>` shared between callbacks (the *only* shared state).
- Why the audio callback **must not** allocate or block (real-time constraint).

## Building Towards `midi-synth`

You'll implement `live.rs` and add a `--live` flag. After today:

```bash
cargo run -- --live --waveform sawtooth
```

…starts listening. Plug in a USB MIDI keyboard (even a cheap £30 one), press keys, hear sound.

> 💡 **No keyboard?** Free virtual MIDI keyboards exist for every OS:
> - macOS: built-in via Audio MIDI Setup → "IAC Driver" + a free app like **VMPK**
> - Windows: **loopMIDI** + **VMPK**
> - Linux: `aconnect` + **vkeybd**

## Step-by-Step Walkthrough

### 1. Open MIDI input

```rust
let mut midi_in = MidiInput::new("midi-synth")?;
midi_in.ignore(Ignore::None);
let ports = midi_in.ports();
let port = &ports[0];      // first port — list and pick if multiple
```

### 2. Make a channel

```rust
let (tx, rx) = mpsc::channel::<NoteEvent>();
```

The `tx` is `Send` and `Clone` — perfect for moving into the MIDI callback.

### 3. Connect with a callback

```rust
let _conn = midi_in.connect(port, "midi-synth-port",
    move |_stamp, message, _| {
        if message.len() < 3 { return; }
        let status = message[0] & 0xF0;
        let (note, vel) = (message[1], message[2]);
        match status {
            0x90 if vel > 0 => { let _ = tx.send(NoteEvent::On  { note, velocity: vel }); }
            0x80 | 0x90     => { let _ = tx.send(NoteEvent::Off { note }); }
            _ => {}
        }
    },
    (),
)?;
```

> **Why the underscore on `_conn`?** If we drop the connection, the callback stops. Binding it keeps it alive for the lifetime of `main`. Don't shadow it.

### 4. Open audio output

```rust
let host = cpal::default_host();
let device = host.default_output_device().ok_or("no output device")?;
let config = device.default_output_config()?;
let sample_rate = config.sample_rate().0;
let channels = config.channels() as usize;
```

### 5. Build the stream

The `data` callback runs on the audio thread. It receives a `&mut [f32]` slice of length `frames * channels` and you must fill it.

```rust
let stream = device.build_output_stream(
    &config.into(),
    move |data: &mut [f32], _| {
        // Drain MIDI events into the voice list.
        while let Ok(ev) = rx.try_recv() { /* push or release Voice */ }
        // Fill samples.
        for frame in data.chunks_mut(channels) {
            let mut s = 0.0;
            for v in voices.iter_mut() { s += v.next_sample(); }
            for sample in frame { *sample = s.clamp(-1.0, 1.0); }
        }
        voices.retain(|v| !v.done());
    },
    |err| eprintln!("stream error: {}", err),
    None,
)?;
stream.play()?;
```

### 6. Sleep forever

```rust
loop { std::thread::sleep(Duration::from_secs(60)); }
```

`Ctrl-C` to stop.

## Common Mistakes

- **No sound, no error**: your default device might be muted, or `cpal` picked the wrong one. Print `device.name()`.
- **Sound stutters / pops**: you're doing too much work in the audio callback. No I/O, no allocation, no `println!`.
- **Stuck notes**: forgot to handle the velocity-0 NoteOn → NoteOff case. Same as session 22.
- **Build fails on Linux**: `sudo apt install libasound2-dev pkg-config`.

## Session Challenge

1. Add a `--port N` flag to pick which MIDI input port to use.
2. Print every received MIDI byte to a *separate* logging thread (use a second channel — never print from the audio callback).
3. Make the synth respond to MIDI Control Change CC74 (filter cutoff) by changing the waveform on-the-fly.

## Quick Reference

```rust
// MIDI status bytes:
0x80 = NoteOff (channel low nibble)
0x90 = NoteOn  (vel > 0); NoteOn vel=0 == NoteOff
0xB0 = ControlChange
0xE0 = PitchBend

// cpal sample formats:
SampleFormat::F32 | SampleFormat::I16 | SampleFormat::U16
// You handled F32; add the others if your device requires them.
```

## DofE Log Reminder

Open `dfe/session-log.md`, find row 23, and write 1–3 sentences about what you built. Mention: what was the hardest part of the threading model? Did you see a "stuck note" bug? How did you fix it?
