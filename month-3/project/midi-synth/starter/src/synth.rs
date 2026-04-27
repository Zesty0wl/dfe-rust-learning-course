// synth.rs — your audio engine.
//
// You will build out:
//   - Waveform enum (Sine, Square, Sawtooth, Triangle)
//   - midi_to_freq helper
//   - Oscillator struct with .sample() method
//   - ADSR envelope
//   - render_note(...) that produces a Vec<f32>
//
// Reference solution is in ../../solution/src/synth.rs if you get stuck.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Waveform { Sine, Square, Sawtooth, Triangle }

impl Waveform {
    pub fn parse(s: &str) -> Option<Self> {
        // TODO: match on lowercase string and return the right variant.
        let _ = s;
        Some(Waveform::Sine)
    }
}

pub fn midi_to_freq(note: u8) -> f32 {
    // TODO: implement 440 * 2^((note - 69) / 12)
    let _ = note;
    440.0
}

pub struct Oscillator {
    // TODO: store waveform, sample_rate, frequency, sample_index
}

impl Oscillator {
    pub fn new(_waveform: Waveform, _frequency: f32, _sample_rate: u32) -> Self {
        // TODO
        Self { }
    }

    pub fn sample(&mut self) -> f32 {
        // TODO: compute the next sample value, advance sample_index.
        0.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Adsr {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl Default for Adsr {
    fn default() -> Self {
        Self { attack: 0.01, decay: 0.05, sustain: 0.7, release: 0.1 }
    }
}

impl Adsr {
    pub fn amplitude(&self, _sample_index: u64, _release_start: Option<u64>, _sample_rate: u32) -> f32 {
        // TODO: return amplitude in [0.0, 1.0] following the ADSR curve.
        1.0
    }
}

pub fn render_note(
    _waveform: Waveform,
    _midi_note: u8,
    _duration_s: f32,
    _sample_rate: u32,
    _velocity: f32,
    _adsr: Adsr,
) -> Vec<f32> {
    // TODO: build a Vec<f32> by pulling samples from an Oscillator and applying ADSR.
    Vec::new()
}

pub fn mix_into(dst: &mut Vec<f32>, src: &[f32], offset: usize) {
    let needed = offset + src.len();
    if dst.len() < needed { dst.resize(needed, 0.0); }
    for (i, s) in src.iter().enumerate() {
        dst[offset + i] += *s;
    }
}

pub fn normalise(buf: &mut [f32]) {
    let peak = buf.iter().cloned().fold(0.0_f32, |a, b| a.max(b.abs()));
    if peak > 1.0 {
        let g = 0.99 / peak;
        for s in buf.iter_mut() { *s *= g; }
    }
}
