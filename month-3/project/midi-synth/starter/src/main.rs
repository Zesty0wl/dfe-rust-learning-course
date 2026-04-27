// main.rs — start with --note mode, then add --midi-file, --chord, --live.
// Reference solution: ../../solution/src/main.rs

mod synth;
mod wav;
mod midi_file;
mod live;

use clap::Parser;
use std::path::PathBuf;

use synth::{render_note, Adsr, Waveform};

#[derive(Parser, Debug)]
#[command(name = "midi-synth")]
struct Cli {
    #[arg(long)]
    note: Option<u8>,

    #[arg(long, default_value_t = 1.0)]
    duration: f32,

    #[arg(long, default_value = "sine")]
    waveform: String,

    #[arg(long, default_value = "output.wav")]
    out: PathBuf,

    #[arg(long, default_value_t = 44100)]
    sample_rate: u32,

    // TODO sessions 22-24: add --midi-file, --chord, --chord-quality, --live flags.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let waveform = Waveform::parse(&cli.waveform)
        .ok_or_else(|| format!("unknown waveform: '{}'", cli.waveform))?;

    if let Some(note) = cli.note {
        let buffer = render_note(waveform, note, cli.duration, cli.sample_rate, 0.7, Adsr::default());
        wav::write_wav(&cli.out, &buffer, cli.sample_rate)?;
        println!("Wrote {} samples → {}", buffer.len(), cli.out.display());
        return Ok(());
    }

    eprintln!("Usage: midi-synth --note 69 --duration 1 --waveform sine --out a4.wav");
    std::process::exit(1);
}
