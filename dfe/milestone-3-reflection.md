# Milestone 3 Reflection — `midi-synth`

**Date completed:**
**Link to project folder:** [`month-3/project/midi-synth/`](../month-3/project/midi-synth/)

> Complete this after Session 24. Aim for 3–5 paragraphs. This is your final milestone — this is the one the assessor will weigh most heavily, so take your time.

---

### What I built

Describe the synthesiser. What can it do? Can it read `.mid` files and render WAVs? Did you get live MIDI input working? What does it sound like? (You can attach a short WAV file or describe it in words.)

---

### Skills I used

By now, your toolbox is large. List the most important Rust concepts you brought to bear:

- Multi-file modules (`mod`, `pub`, `use`)
- External crates: `hound`, `midly`, `midir`, `cpal`, `clap`
- Closures and iterators (waveform generators)
- Generics and traits (`Iterator`, `Display`, dyn dispatch)
- `Result<T, E>` and the `?` operator everywhere
- `std::sync::mpsc` for thread-to-thread communication
- File I/O — both text and binary

---

### The hardest part

This was the most ambitious project — what was the toughest thing you had to figure out? MIDI tick maths? Avoiding clicks with an envelope? Threading audio output? Be specific.

---

### What I'm most proud of

Which moment felt most rewarding? The first time you heard a tone you generated yourself? The first chord? The first time the keyboard worked end-to-end?

---

### How this compares to where I started in Month 1

Compare to Milestone 1 (or even to Session 1). What concepts were unimaginable to you then that are routine now? What kind of programs could you imagine writing next?

---

*Signature (participant):* ______________________________
*Date:* ______________________________
