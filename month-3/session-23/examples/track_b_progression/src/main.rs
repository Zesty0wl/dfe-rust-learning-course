// track_b_progression — Type a chord progression on the command line, get a .wav file.
//
// Usage:
//   cargo run -- "Cmaj:2 Fmaj:2 Gmaj:2 Cmaj:2" cadence.wav
//
// This is the Track B alternative to Session 23's `--midi-file` flag for
// rendering chords. Same end result — audio of a chord progression — but
// you write the progression yourself instead of editing a .mid file.
//
// New concepts beyond Track B for Session 22:
//   - additive mixing: many notes overlap into the same time slot
//   - chord_intervals: how chord names map to semitone offsets
//   - normalisation: stop the sum of voices clipping past ±1.0

use std::env;
use std::path::Path;

const SAMPLE_RATE: u32 = 44_100;

// ────────── 1. Note-name parsing (same as Session 22) ──────────

fn parse_note_letter(s: &str) -> Result<(u8, &str), String> {
    // returns (semitone offset from C, remaining-after-letter-and-accidental)
    let mut chars = s.chars();
    let letter = chars.next().ok_or_else(|| format!("empty note in '{}'", s))?;
    let base: i32 = match letter.to_ascii_uppercase() {
        'C' => 0, 'D' => 2, 'E' => 4, 'F' => 5, 'G' => 7, 'A' => 9, 'B' => 11,
        _ => return Err(format!("unknown note letter: '{}'", letter)),
    };
    let rest = &s[letter.len_utf8()..];
    let (accidental, after_accidental) = if rest.starts_with('#') {
        (1_i32, &rest[1..])
    } else if rest.starts_with('b') && !rest.is_empty() {
        // careful: 'b' could be a flat OR the start of a quality like "bmaj".
        // For chord roots we treat lone leading-b after a letter as flat.
        (-1_i32, &rest[1..])
    } else {
        (0_i32, rest)
    };
    let semis = base + accidental;
    Ok((((semis + 12) % 12) as u8, after_accidental))
}

// ────────── 2. Chord parsing: "Cmaj", "Am", "G7", "F#dim" ──────────

fn chord_intervals(quality: &str) -> Result<&'static [i32], String> {
    Ok(match quality.to_lowercase().as_str() {
        "" | "maj" | "major" => &[0, 4, 7],
        "m" | "min" | "minor" => &[0, 3, 7],
        "dim"                => &[0, 3, 6],
        "maj7"               => &[0, 4, 7, 11],
        "m7" | "min7"        => &[0, 3, 7, 10],
        "7" | "dom7"         => &[0, 4, 7, 10],
        other => return Err(format!("unknown chord quality: '{}'", other)),
    })
}

/// Parse a chord name like "Cmaj", "Am", "G7", "F#dim" into a list of MIDI notes
/// rooted at octave 4 (root in the 60–71 range).
fn parse_chord(name: &str) -> Result<Vec<u8>, String> {
    let (root_offset, rest) = parse_note_letter(name)?;
    let intervals = chord_intervals(rest)?;
    let root_midi: i32 = 60 + root_offset as i32; // anchor in octave 4
    Ok(intervals.iter().map(|i| (root_midi + i) as u8).collect())
}

/// Parse the whole `--progression` string into a Vec of (chord_notes, duration_seconds).
fn parse_progression(input: &str) -> Result<Vec<(Vec<u8>, f32)>, String> {
    input
        .split_whitespace()
        .map(|tok| {
            let (name, dur) = tok
                .split_once(':')
                .ok_or_else(|| format!("token '{}' missing ':' (expected CHORD:DURATION)", tok))?;
            let chord = parse_chord(name)?;
            let seconds: f32 = dur
                .parse()
                .map_err(|_| format!("bad duration '{}' in token '{}'", dur, tok))?;
            if seconds <= 0.0 {
                return Err(format!("duration must be > 0 in token '{}'", tok));
            }
            Ok((chord, seconds))
        })
        .collect()
}

// ────────── 3. Synth: oscillator + envelope + render_note ──────────

fn midi_to_freq(note: u8) -> f32 {
    440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
}

fn render_note(midi_note: u8, duration_s: f32) -> Vec<f32> {
    let freq = midi_to_freq(midi_note);
    let total = (duration_s * SAMPLE_RATE as f32) as usize;
    let attack = (0.01 * SAMPLE_RATE as f32) as usize;
    let release = (0.05 * SAMPLE_RATE as f32) as usize;
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
        out.push(phase.sin() * amp * 0.7);
    }
    out
}

// ────────── 4. Additive mixing (the new bit for chords) ──────────

/// Mix `src` into `dst` starting at sample `offset`. Resizes `dst` if needed.
/// This is *additive* — overlapping notes sum together, which is how chords work.
fn mix_into(dst: &mut Vec<f32>, src: &[f32], offset: usize) {
    let needed = offset + src.len();
    if dst.len() < needed {
        dst.resize(needed, 0.0);
    }
    for (i, s) in src.iter().enumerate() {
        dst[offset + i] += *s;
    }
}

/// Render the whole progression: each chord's voices mixed together, then
/// chords laid end-to-end.
fn render_progression(prog: &[(Vec<u8>, f32)]) -> Vec<f32> {
    let mut buffer: Vec<f32> = Vec::new();
    let mut cursor: usize = 0;
    for (chord, dur) in prog {
        for note in chord {
            let samples = render_note(*note, *dur);
            mix_into(&mut buffer, &samples, cursor);
        }
        cursor += (dur * SAMPLE_RATE as f32) as usize;
    }
    normalise(&mut buffer);
    buffer
}

/// Scale the buffer so the loudest sample sits at 0.99. Stops 4-voice chords
/// from clipping past ±1.0 when mixed.
fn normalise(buf: &mut [f32]) {
    let peak = buf.iter().cloned().fold(0.0_f32, |a, b| a.max(b.abs()));
    if peak > 1.0 {
        let g = 0.99 / peak;
        for s in buf.iter_mut() {
            *s *= g;
        }
    }
}

// ────────── 5. WAV output ──────────

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

// ────────── 6. main ──────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: track_b_progression <PROGRESSION> [OUT.wav]");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  cargo run -- \"Cmaj:2 Fmaj:2 Gmaj:2 Cmaj:2\"          # I–IV–V–I in C");
        eprintln!("  cargo run -- \"Am:2 F:2 C:2 G:2\" pop-progression.wav # vi–IV–I–V");
        eprintln!("  cargo run -- \"Cmaj7:2 Am7:2 Dm7:2 G7:2\"             # ii–V–I jazz");
        std::process::exit(1);
    }
    let prog_str = &args[1];
    let out_path: &Path = if args.len() >= 3 {
        Path::new(&args[2])
    } else {
        Path::new("progression.wav")
    };

    let prog = parse_progression(prog_str)?;
    let total_seconds: f32 = prog.iter().map(|(_, d)| d).sum();
    let buffer = render_progression(&prog);

    write_wav(out_path, &buffer)?;
    println!(
        "Rendered {} chords ({:.2}s) → {}",
        prog.len(),
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
    fn c_major_is_c_e_g() {
        assert_eq!(parse_chord("Cmaj").unwrap(), vec![60, 64, 67]);
    }

    #[test]
    fn a_minor_is_a_c_e() {
        // A4 = 69, C5 = 72, E5 = 76 — but we anchor roots inside octave 4 (60–71),
        // so A is 69. Wait — our parse_note_letter returns (semitone_offset_mod_12),
        // and we add 60. For 'A': base 9, no accidental, mod 12 = 9, root midi = 69.
        assert_eq!(parse_chord("Am").unwrap(), vec![69, 72, 76]);
    }

    #[test]
    fn dominant_seventh_has_four_notes() {
        let g7 = parse_chord("G7").unwrap();
        assert_eq!(g7.len(), 4);
        // G4 = 67, B4 = 71, D5 = 74, F5 = 77
        assert_eq!(g7, vec![67, 71, 74, 77]);
    }

    #[test]
    fn parse_simple_progression() {
        let prog = parse_progression("Cmaj:2 G7:1").unwrap();
        assert_eq!(prog.len(), 2);
        assert_eq!(prog[0].1, 2.0);
        assert_eq!(prog[1].1, 1.0);
    }

    #[test]
    fn rejects_unknown_quality() {
        assert!(parse_chord("Cwobble").is_err());
    }

    #[test]
    fn additive_mix_resizes_destination() {
        let mut dst = vec![0.5];
        mix_into(&mut dst, &[0.25, 0.25, 0.25], 0);
        assert_eq!(dst, vec![0.75, 0.25, 0.25]);
    }
}
