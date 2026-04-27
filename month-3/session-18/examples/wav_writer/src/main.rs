// Session 18 example: hand-rolled WAV writer.
// Produces a440.wav: 1 second of 440 Hz sine, mono, 44.1 kHz, 16-bit PCM.
// No external crates required.

use std::fs::File;
use std::io::{BufWriter, Write};

const SAMPLE_RATE: u32 = 44100;
const CHANNELS:    u16 = 1;
const BITS:        u16 = 16;
const FREQUENCY:   f32 = 440.0;
const DURATION_S:  f32 = 1.0;
const AMPLITUDE:   f32 = 0.3;

fn main() -> std::io::Result<()> {
    let total_samples = (SAMPLE_RATE as f32 * DURATION_S) as u32;

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
    w.write_all(&16u32.to_le_bytes())?;
    w.write_all(&1u16.to_le_bytes())?;
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
