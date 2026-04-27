// main.rs — CLI entry point.
// Modes (mutually exclusive):
//   --note N       single note → WAV
//   --midi-file F  render .mid → WAV
//   --chord ROOT   render a chord → WAV   (e.g. --chord C --chord-quality major)
//   --live         live MIDI keyboard input → audio output

mod synth;
mod wav;
mod midi_file;
mod live;

use clap::Parser;
use std::path::PathBuf;

use synth::{render_note, mix_into, normalise, Adsr, Waveform};

#[derive(Parser, Debug)]
#[command(name = "midi-synth", about = "Synthesise audio from notes, chords, MIDI files, or a live keyboard.")]
struct Cli {
    /// Single MIDI note number (0..127). 60 = middle C, 69 = A4.
    #[arg(long)]
    note: Option<u8>,

    /// Duration in seconds (single note / chord modes only).
    #[arg(long, default_value_t = 1.0)]
    duration: f32,

    /// Waveform: sine | square | sawtooth | triangle.
    #[arg(long, default_value = "sine")]
    waveform: String,

    /// Path to .mid file to render.
    #[arg(long)]
    midi_file: Option<PathBuf>,

    /// Chord root note name (C, C#, D, ...).
    #[arg(long)]
    chord: Option<String>,

    /// Chord quality: major | minor | dim | maj7 | min7 | dom7.
    #[arg(long, default_value = "major")]
    chord_quality: String,

    /// Live MIDI input mode (requires a connected MIDI keyboard).
    #[arg(long, default_value_t = false)]
    live: bool,

    /// Output WAV path (offline modes).
    #[arg(long, default_value = "output.wav")]
    out: PathBuf,

    /// Sample rate for offline rendering.
    #[arg(long, default_value_t = 44100)]
    sample_rate: u32,
}

fn note_name_to_midi(name: &str) -> Option<u8> {
    let n = name.trim();
    let (letter, accidental) = match n.chars().next()? {
        c if c.is_ascii_alphabetic() => (c.to_ascii_uppercase(), &n[1..]),
        _ => return None,
    };
    let base = match letter {
        'C' => 0, 'D' => 2, 'E' => 4, 'F' => 5, 'G' => 7, 'A' => 9, 'B' => 11,
        _ => return None,
    };
    let semis: i32 = match accidental {
        "" => 0,
        "#" | "♯" | "is" => 1,
        "b" | "♭" | "es" => -1,
        _ => return None,
    };
    Some((60 + base + semis).max(0).min(127) as u8)   // anchored at C4 (octave 4)
}

fn chord_intervals(quality: &str) -> Option<&'static [i32]> {
    Some(match quality.to_lowercase().as_str() {
        "major" | "maj"  => &[0, 4, 7],
        "minor" | "min"  => &[0, 3, 7],
        "dim"            => &[0, 3, 6],
        "maj7"           => &[0, 4, 7, 11],
        "min7"           => &[0, 3, 7, 10],
        "dom7" | "7"     => &[0, 4, 7, 10],
        _ => return None,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let waveform = Waveform::parse(&cli.waveform)
        .ok_or_else(|| format!("unknown waveform: '{}'", cli.waveform))?;

    if cli.live {
        live::run_live(waveform)?;
        return Ok(());
    }

    if let Some(path) = &cli.midi_file {
        let bytes = std::fs::read(path)?;
        let rendered = midi_file::render_midi_file(&bytes, waveform, cli.sample_rate)?;
        wav::write_wav(&cli.out, &rendered.samples, cli.sample_rate)?;
        println!(
            "Rendered {} note events from {} ({:.2}s) → {}",
            rendered.events,
            path.display(),
            rendered.duration_s,
            cli.out.display()
        );
        return Ok(());
    }

    if let Some(root_name) = &cli.chord {
        let root = note_name_to_midi(root_name)
            .ok_or_else(|| format!("unknown chord root: '{}'", root_name))?;
        let intervals = chord_intervals(&cli.chord_quality)
            .ok_or_else(|| format!("unknown chord quality: '{}'", cli.chord_quality))?;
        let adsr = Adsr::default();
        let mut buffer: Vec<f32> = Vec::new();
        let voices: Vec<u8> = intervals.iter().map(|i| (root as i32 + i) as u8).collect();
        for n in &voices {
            let buf = render_note(waveform, *n, cli.duration, cli.sample_rate, 0.7, adsr);
            mix_into(&mut buffer, &buf, 0);
        }
        normalise(&mut buffer);
        wav::write_wav(&cli.out, &buffer, cli.sample_rate)?;
        let names: Vec<String> = voices.iter().map(|n| n.to_string()).collect();
        println!("Wrote {} ({} {}: notes {}) → {}", cli.out.display(), root_name, cli.chord_quality, names.join(", "), cli.out.display());
        return Ok(());
    }

    if let Some(note) = cli.note {
        let buffer = render_note(waveform, note, cli.duration, cli.sample_rate, 0.7, Adsr::default());
        wav::write_wav(&cli.out, &buffer, cli.sample_rate)?;
        println!(
            "Wrote {} samples to {} ({:.2}s @ {} Hz, MIDI {}, {:?})",
            buffer.len(),
            cli.out.display(),
            cli.duration,
            cli.sample_rate,
            note,
            waveform
        );
        return Ok(());
    }

    eprintln!("No mode chosen. Try one of:");
    eprintln!("  --note 69                        # single note A4 → output.wav");
    eprintln!("  --chord C --chord-quality major  # chord → output.wav");
    eprintln!("  --midi-file song.mid             # render .mid → output.wav");
    eprintln!("  --live                           # live keyboard mode");
    eprintln!("Run with --help for full options.");
    std::process::exit(1);
}
