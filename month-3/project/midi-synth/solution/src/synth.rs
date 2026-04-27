// synth.rs — waveforms, oscillator, ADSR envelope, render_note.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Waveform { Sine, Square, Sawtooth, Triangle }

impl Waveform {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sine"     => Some(Waveform::Sine),
            "square"   => Some(Waveform::Square),
            "saw" | "sawtooth" => Some(Waveform::Sawtooth),
            "tri" | "triangle" => Some(Waveform::Triangle),
            _ => None,
        }
    }
}

pub fn midi_to_freq(note: u8) -> f32 {
    440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
}

pub struct Oscillator {
    waveform: Waveform,
    sample_rate: u32,
    frequency: f32,
    sample_index: u64,
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency: f32, sample_rate: u32) -> Self {
        Self { waveform, sample_rate, frequency, sample_index: 0 }
    }

    pub fn sample(&mut self) -> f32 {
        let t = self.sample_index as f32 / self.sample_rate as f32;
        let phase = 2.0 * std::f32::consts::PI * self.frequency * t;
        let cycles = self.frequency * t;
        let frac = cycles - cycles.floor();
        let v = match self.waveform {
            Waveform::Sine     => phase.sin(),
            Waveform::Square   => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },
            Waveform::Sawtooth => 2.0 * frac - 1.0,
            Waveform::Triangle => 4.0 * (frac - 0.5).abs() - 1.0,
        };
        self.sample_index += 1;
        v
    }
}

/// Linear ADSR envelope.
/// `release_start` is the sample index at which the note is released.
/// Times are in seconds.
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
    pub fn amplitude(&self, sample_index: u64, release_start: Option<u64>, sample_rate: u32) -> f32 {
        let t = sample_index as f32 / sample_rate as f32;
        match release_start {
            None => {
                if t < self.attack {
                    t / self.attack
                } else if t < self.attack + self.decay {
                    let dt = t - self.attack;
                    1.0 + (self.sustain - 1.0) * (dt / self.decay)
                } else {
                    self.sustain
                }
            }
            Some(rs) => {
                let release_t = rs as f32 / sample_rate as f32;
                let pre_release_amp = if release_t < self.attack {
                    release_t / self.attack
                } else if release_t < self.attack + self.decay {
                    let dt = release_t - self.attack;
                    1.0 + (self.sustain - 1.0) * (dt / self.decay)
                } else {
                    self.sustain
                };
                if t < release_t { return pre_release_amp; }
                let dt = t - release_t;
                if dt > self.release { 0.0 } else { pre_release_amp * (1.0 - dt / self.release) }
            }
        }
    }
}

/// Render a single note (full attack→release lifecycle) into a Vec<f32>.
pub fn render_note(
    waveform: Waveform,
    midi_note: u8,
    duration_s: f32,
    sample_rate: u32,
    velocity: f32,
    adsr: Adsr,
) -> Vec<f32> {
    let freq = midi_to_freq(midi_note);
    let total_samples = (duration_s * sample_rate as f32) as u64;
    // Hold for total_samples - release_samples, then release.
    let release_samples = (adsr.release * sample_rate as f32) as u64;
    let hold = total_samples.saturating_sub(release_samples);
    let mut osc = Oscillator::new(waveform, freq, sample_rate);
    let mut out: Vec<f32> = Vec::with_capacity(total_samples as usize);
    for n in 0..total_samples {
        let release_start = if n >= hold { Some(hold) } else { None };
        let amp = adsr.amplitude(n, release_start, sample_rate);
        let s = osc.sample() * amp * velocity;
        out.push(s);
    }
    out
}

/// Mix `src` into `dst` starting at `offset`. Resizes `dst` if needed.
pub fn mix_into(dst: &mut Vec<f32>, src: &[f32], offset: usize) {
    let needed = offset + src.len();
    if dst.len() < needed { dst.resize(needed, 0.0); }
    for (i, s) in src.iter().enumerate() {
        dst[offset + i] += *s;
    }
}

/// Soft clip to [-1.0, 1.0] to avoid harsh distortion.
pub fn normalise(buf: &mut [f32]) {
    let peak = buf.iter().cloned().fold(0.0_f32, |a, b| a.max(b.abs()));
    if peak > 1.0 {
        let g = 0.99 / peak;
        for s in buf.iter_mut() { *s *= g; }
    }
}
