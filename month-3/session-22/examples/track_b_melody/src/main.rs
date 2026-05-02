// track_b_melody — Type a melody on the command line, get a .wav file.
//
// Usage:
//   cargo run -- "C4:0.5 D4:0.5 E4:0.5 F4:0.5 G4:1.0" twinkle.wav
//
// This is the Track B alternative to Session 22's `--midi-file` flag.
// Same end result — audio of a tune — but you write the tune yourself
// instead of parsing a binary `.mid` file.
//
// Concepts you still meet (and which carry into the project):
//   - midi_to_freq: MIDI note number → frequency in Hz
//   - oscillator: a sine wave at that frequency
//   - envelope: amplitude shape over time so notes don't click
//   - sequential mixing: place each note at the right offset in the buffer

use std::env;
use std::path::Path;

const SAMPLE_RATE: u32 = 44_100;

// ────────── 1. Parsing notes like "C4", "F#5", "Bb3" ──────────

/// Convert a note name like "C4" into a MIDI number (60 for middle C, 69 for A4).
/// Supports sharps (`#`), flats (`b`), and octaves 0–9.
fn parse_note(s: &str) -> Result<u8, String> {
    let s = s.trim();
    let mut chars = s.chars();
    let letter = chars.next().ok_or_else(|| format!("empty note in '{}'", s))?;
    let base = match letter.to_ascii_uppercase() {
        'C' => 0, 'D' => 2, 'E' => 4, 'F' => 5, 'G' => 7, 'A' => 9, 'B' => 11,
        _ => return Err(format!("unknown note letter: '{}'", letter)),
    };
    let rest: String = chars.collect();
    let (accidental, octave_str) = match rest.chars().next() {
        Some('#') => (1_i32, &rest[1..]),
        Some('b') => (-1_i32, &rest[1..]),
        _ => (0_i32, rest.as_str()),
    };
    let octave: i32 = octave_str
        .parse()
        .map_err(|_| format!("missing or invalid octave in '{}'", s))?;
    // MIDI 60 = C4, so MIDI for C{n} = 12 * (n + 1).
    let midi = 12 * (octave + 1) + base + accidental;
    if !(0..=127).contains(&midi) {
        return Err(format!("note '{}' is out of MIDI range 0..127", s));
    }
    Ok(midi as u8)
}

/// Parse the whole `--song` string into a Vec of (midi_note, duration_seconds).
/// Each token is `NOTE:DURATION`, e.g. `C4:0.5`. Tokens are separated by spaces.
fn parse_song(input: &str) -> Result<Vec<(u8, f32)>, String> {
    input
        .split_whitespace()
        .map(|tok| {
            let (note, dur) = tok
                .split_once(':')
                .ok_or_else(|| format!("token '{}' missing ':' (expected NOTE:DURATION)", tok))?;
            let midi = parse_note(note)?;
            let seconds: f32 = dur
                .parse()
                .map_err(|_| format!("bad duration '{}' in token '{}'", dur, tok))?;
            if seconds <= 0.0 {
                return Err(format!("duration must be > 0 in token '{}'", tok));
            }
            Ok((midi, seconds))
        })
        .collect()
}

// ────────── 2. Synth: oscillator + envelope ──────────

fn midi_to_freq(note: u8) -> f32 {
    440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
}

/// Render one note as a Vec<f32> of audio samples. Sine wave with a simple
/// attack/release envelope so each note fades in/out instead of clicking.
fn render_note(midi_note: u8, duration_s: f32) -> Vec<f32> {
    let freq = midi_to_freq(midi_note);
    let total = (duration_s * SAMPLE_RATE as f32) as usize;
    let attack = (0.01 * SAMPLE_RATE as f32) as usize; // 10 ms fade-in
    let release = (0.05 * SAMPLE_RATE as f32) as usize; // 50 ms fade-out
    let release_start = total.saturating_sub(release);

    let mut out = Vec::with_capacity(total);
    for n in 0..total {
        let t = n as f32 / SAMPLE_RATE as f32;
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let amp = if n < attack {
            n as f32 / attack as f32
        } else if n >= release_start {
            (total - n) as f32 / release as f32
        } else {
            1.0
        };
        out.push(phase.sin() * amp * 0.7); // 0.7 = velocity
    }
    out
}

// ────────── 3. Sequential mixing ──────────

/// Place each note end-to-end in a buffer.
/// (For chords or overlap, you'd use additive mixing — see Track A.)
fn render_song(song: &[(u8, f32)]) -> Vec<f32> {
    let mut buffer = Vec::new();
    for (note, dur) in song {
        let samples = render_note(*note, *dur);
        buffer.extend(samples);
    }
    buffer
}

// ────────── 4. WAV output ──────────

fn write_wav(path: &Path, samples: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for s in samples {
        let clipped = s.clamp(-1.0, 1.0);
        let i = (clipped * i16::MAX as f32) as i16;
        writer.write_sample(i)?;
    }
    writer.finalize()?;
    Ok(())
}

// ────────── 5. main ──────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: track_b_melody <SONG> [OUT.wav]");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  cargo run -- \"C4:0.5 D4:0.5 E4:0.5 F4:0.5 G4:1.0\"");
        eprintln!("  cargo run -- \"C4:0.5 C4:0.5 G4:0.5 G4:0.5 A4:0.5 A4:0.5 G4:1.0\" twinkle.wav");
        std::process::exit(1);
    }
    let song_str = &args[1];
    let out_path: &Path = if args.len() >= 3 {
        Path::new(&args[2])
    } else {
        Path::new("song.wav")
    };

    let song = parse_song(song_str)?;
    let total_seconds: f32 = song.iter().map(|(_, d)| d).sum();
    let buffer = render_song(&song);

    write_wav(out_path, &buffer)?;
    println!(
        "Rendered {} notes ({:.2}s) → {}",
        song.len(),
        total_seconds,
        out_path.display()
    );
    Ok(())
}

// ────────── tests ──────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_c_is_60() {
        assert_eq!(parse_note("C4").unwrap(), 60);
    }

    #[test]
    fn a4_is_69() {
        assert_eq!(parse_note("A4").unwrap(), 69);
    }

    #[test]
    fn sharps_and_flats() {
        assert_eq!(parse_note("C#4").unwrap(), 61);
        assert_eq!(parse_note("Db4").unwrap(), 61);
        assert_eq!(parse_note("F#5").unwrap(), 78);
    }

    #[test]
    fn parse_simple_song() {
        let song = parse_song("C4:0.5 D4:1.0 E4:0.25").unwrap();
        assert_eq!(song, vec![(60, 0.5), (62, 1.0), (64, 0.25)]);
    }

    #[test]
    fn rejects_bad_token() {
        assert!(parse_song("C4 D4:0.5").is_err());          // missing ':'
        assert!(parse_song("C4:abc").is_err());             // bad duration
        assert!(parse_song("Z4:0.5").is_err());             // bad letter
        assert!(parse_song("C4:-0.5").is_err());            // non-positive duration
    }
}
