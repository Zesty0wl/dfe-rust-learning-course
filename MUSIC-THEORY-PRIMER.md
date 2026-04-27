# Music Theory Primer

You don't need to be a musician to do this course — but Month 1 and the `music-theory-cli` project lean on a few music ideas. This page explains everything you'll need, in plain English, in about 10 minutes of reading. Come back any time you hit a music word you've forgotten.

If you already play an instrument or have done music GCSE, you can skip this page. For everyone else: start here.

---

## 1. Notes and the alphabet that runs out at G

Western music uses **seven note letters**: **A B C D E F G**. After G it loops back to A — there is no H. So as you go up: …F G A B C D E F G A… forever.

Each repetition lands you on a higher copy of the same note. The "higher A" still sounds like A, just brighter. We'll explain why in a moment.

> **Why those seven letters?** Pure historical accident — they're the white keys on a piano, which got their layout in roughly the 16th century from medieval church music. Other musical traditions divide the octave differently (Indian classical music uses 22 microtones, for example).

---

## 2. The piano keyboard — the picture worth a thousand words

Every piece of music theory you'll meet in this course is easier to see on a piano keyboard.

![Annotated piano keyboard showing two octaves (C3 to C5) with each white and black key labelled with its note name (e.g. C4, F#3), MIDI number (e.g. 60, 54), and frequency in Hz (e.g. 261.6 Hz). Middle C and A4 = 440 Hz are highlighted. Below the keyboard, the C major scale is shown as eight numbered dots connected by labelled "WHOLE" (2 semitones) and "HALF" (1 semitone) steps in the pattern W-W-H-W-W-W-H.](./diagrams/piano-keyboard.svg)

A few things to notice:

- **White keys** are the seven letter-named notes: C, D, E, F, G, A, B, then back to C.
- **Black keys** are the **sharps** (♯) and **flats** (♭) — the notes *between* the white keys.
- The black keys appear in groups of **2, then 3, then 2, then 3…** That visual pattern is how pianists know which key is which without labels.
- The whole 7-white + 5-black pattern (12 keys total) **repeats**. Every repetition is one **octave**.

---

## 3. Octaves and middle C

Every time you go from one C up to the next C (or any note up to its next same-name copy) you've gone up one **octave**. There are 12 piano keys (white + black) in an octave.

To tell octaves apart we add a number: `C4`, `C5`, `A3`, etc. The numbers count from `C0` (very low) upward. **`C4` is "middle C"** — the C closest to the middle of a standard piano keyboard, and the natural starting point for most beginners.

`A4` is one of the most famous notes in music. It's the note an orchestra tunes to before a concert, and it's the reference pitch for the whole modern Western tuning system.

---

## 4. Semitones — the smallest step

A **semitone** (also called a **half step**) is the distance from any key to the very next key on the piano — white *or* black. So:

- C → C♯ is a semitone.
- C♯ → D is a semitone.
- E → F is a semitone (notice — there's no black key between E and F).

A **whole step** (or **whole tone**) is two semitones. So C → D is a whole step (C → C♯ → D).

There are **12 semitones in an octave**.

> **Trivia.** B → C and E → F are the two pairs of adjacent white keys with no black key between them. They are still semitones apart. This single fact explains why scales feel the way they feel.

---

## 5. MIDI note numbers — music as integers

Computers don't think in "F♯ above middle C". They think in numbers. **MIDI** (Musical Instrument Digital Interface, 1983) gives every semitone a number from **0 to 127**:

- MIDI **0** is the lowest note (`C-1`, way below human hearing).
- MIDI **60** is **middle C** (`C4`).
- MIDI **69** is `A4`, the orchestra tuning note.
- MIDI **127** is the highest (`G9`, above the top of any real piano).

To go up one semitone, add 1. To go up one octave, add 12. That's the whole numbering system.

This is why our `music-theory-cli` and `midi-synth` projects work in MIDI numbers internally — they're just integers, the friendliest possible format.

---

## 6. Frequencies — the actual physics

Sound is air vibrating. The **frequency** of a sound is how many times per second the air vibrates, measured in **Hertz (Hz)**. Higher frequency = higher pitch.

The Western tuning system fixes one note: **A4 = 440 Hz**. Every other note's frequency is calculated from that with one formula:

```
freq(midi) = 440 × 2^((midi − 69) / 12)
```

What this says: each semitone multiplies the frequency by **2^(1/12)** (≈ 1.0595). After 12 semitones (one octave), you've multiplied by `2^(12/12) = 2` — the frequency exactly doubles. That's why an A one octave above A4 is `880 Hz`, and an A one octave below is `220 Hz`.

You'll implement this exact formula in **Session 3**. It's three lines of Rust.

---

## 7. Scales — recipes for picking notes

A **scale** is a chosen subset of the 12 semitones that "go together" musically. The most important by far is the **major scale**.

### The major scale

Pick a starting note (the **root**). Then walk up the keyboard following this pattern of steps:

> **W − W − H − W − W − W − H**
>
> (Whole − Whole − Half − Whole − Whole − Whole − Half)

That gives you 8 notes (counting back to the root one octave higher).

Starting from C:
| Step | Pattern | Note | MIDI |
|---|---|---|---|
| 1 | start | C | 60 |
| 2 | + W (2 semitones) | D | 62 |
| 3 | + W | E | 64 |
| 4 | + H (1 semitone) | F | 65 |
| 5 | + W | G | 67 |
| 6 | + W | A | 69 |
| 7 | + W | B | 71 |
| 8 | + H | C | 72 |

That's the **C major scale**. Notice it lands exactly on the white keys! That's why C major is taught first — it's the easiest to play.

The key insight: **the same W-W-H-W-W-W-H pattern starting on any note gives you a major scale** — you just end up using different black/white keys. G major has one sharp (F♯), D major has two sharps (F♯, C♯), etc.

### The minor scale

Same idea, different pattern: **W − H − W − W − H − W − W**. Sounds darker, sadder. Most pop songs in a minor key use this pattern.

### The pentatonic scale

Five notes instead of seven (penta = five). Used everywhere in folk, blues, rock, pop. **Major pentatonic** = the major scale with the 4th and 7th notes removed.

The `music-theory-cli` project (Sessions 7–8) lets the user pick any of these three scales from any of the 12 root notes.

---

## 8. Intervals — distances between notes

An **interval** is the distance between two notes, measured in semitones, but given a name:

| Semitones | Interval name | Example |
|---|---|---|
| 0 | Unison | C → C |
| 1 | Minor second | C → D♭ |
| 2 | Major second | C → D |
| 3 | Minor third | C → E♭ |
| 4 | Major third | C → E |
| 5 | Perfect fourth | C → F |
| 7 | Perfect fifth | C → G |
| 8 | Minor sixth | C → A♭ |
| 9 | Major sixth | C → A |
| 12 | Octave | C → C |

You can hear an interval just from the gap. The first two notes of "Twinkle Twinkle Little Star" form a **perfect fifth** (C → G). The first two notes of "Here Comes the Bride" are a **perfect fourth** (C → F). Music theory is largely about how intervals stack into chords and progressions.

---

## 9. Chords — stacks of notes played together

Hold down three notes from a scale at once and you've made a **chord**. The most basic chord is a **triad** — root + third + fifth.

A **C major chord** is C + E + G (the 1st, 3rd, and 5th notes of the C major scale).
A **C minor chord** is C + E♭ + G — same root and fifth, but a *flatter* third. That single half-step difference is what makes a chord sound "sad" instead of "happy".

The `music-theory-cli` Session 8 stretch goal lets the user generate triads on top of any scale.

---

## 10. Putting it all together

When you write `let a4_freq: f64 = 440.0;` in Session 2, you're storing the reference frequency of MIDI note 69 in cycles per second.

When `music-theory-cli` (Sessions 7–8) takes `--root C --scale major` and prints `C D E F G A B C`, it's using the W-W-H-W-W-W-H pattern + the 12-semitone-per-octave rule to walk from MIDI 60 upward.

When `midi-synth` (Sessions 21–24) reads a MIDI file or live keyboard input, it converts each note number to a frequency with the formula in section 6, then synthesises a sine/square/sawtooth wave at that frequency to produce sound.

Everything in Months 1 and 3 is built on the seven ideas in this primer. Bookmark this page; you'll come back to it.

---

## Further reading (with diagrams)

If you want to go deeper, all of these have generous illustrations:

- [**musictheory.net** — *The Basics* lessons](https://www.musictheory.net/lessons) — Free, interactive, animated. The single best place to *practice* reading music theory. Lessons 1–10 cover everything in this primer.
- [**Wikipedia: Piano key frequencies**](https://en.wikipedia.org/wiki/Piano_key_frequencies) — Full table of MIDI numbers, frequencies, and key names from C0 to C8.
- [**Wikipedia: Equal temperament**](https://en.wikipedia.org/wiki/Equal_temperament) — Why we divide the octave into 12 equal semitones (it's a relatively recent decision — Bach was a fan).
- [**Wikipedia: Major scale**](https://en.wikipedia.org/wiki/Major_scale) — Full diagrams of every major scale and the circle of fifths.
- [**Wikipedia: MIDI**](https://en.wikipedia.org/wiki/MIDI) — Background on the 1983 standard you're using to talk to synths.
- [**Michael New — Music Theory for Beginners (YouTube)**](https://www.youtube.com/playlist?list=PLD1bPPUH1FRC5DkfEZAXQjdg5FnP_a-x6) — Friendly 10-minute videos. Episodes 1–4 cover everything above with audio examples.
- [**Andrew Huang — Music Theory in 16 Minutes (YouTube)**](https://www.youtube.com/watch?v=rgaTLrZGlk0) — A faster, breezier overview if you prefer one big video.
- [**Sound on Sound — *Synth Secrets* series**](https://www.soundonsound.com/series/synth-secrets-sound-sound) — Once you start the Month 3 synth project, this is the canonical reference for *how* sound synthesis actually works.

---

## Glossary of music words used in this course

| Word | What it means |
|---|---|
| **Note** | A single musical pitch — e.g. "C4", "A♯3". |
| **Pitch** | How high or low a note sounds. Determined by frequency. |
| **Octave** | The interval between a note and the next note with the same letter name. 12 semitones; doubling of frequency. |
| **Semitone** / **half step** | The smallest step on a piano. 1/12 of an octave. |
| **Whole step** | Two semitones. |
| **Sharp (♯)** | One semitone higher. C♯ = "C sharp". |
| **Flat (♭)** | One semitone lower. D♭ = "D flat". (D♭ and C♯ are the same key on a piano.) |
| **MIDI number** | Integer 0–127 naming every semitone. Middle C = 60, A4 = 69. |
| **Frequency (Hz)** | Vibrations per second. A4 = 440 Hz. |
| **Scale** | An ordered set of notes inside an octave that "go together". |
| **Root** | The starting note of a scale or chord — what the scale is "in". |
| **Major scale** | 7-note scale with the W-W-H-W-W-W-H pattern. Sounds bright. |
| **Minor scale** | 7-note scale with the W-H-W-W-H-W-W pattern. Sounds dark. |
| **Pentatonic scale** | 5-note scale, very common in folk/rock/pop. |
| **Interval** | Distance between two notes, measured in semitones (and given a name like "perfect fifth"). |
| **Chord** | Three or more notes played together. |
| **Triad** | A 3-note chord — root + third + fifth. |
| **Major chord** | Triad with a major third — sounds happy. |
| **Minor chord** | Triad with a minor third — sounds sad. |
| **Tuning** | Choosing the exact frequencies for each note. We use *equal temperament* (every semitone = 2^(1/12)). |
