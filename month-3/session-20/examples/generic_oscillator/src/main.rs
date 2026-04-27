// Session 20 example: generics vs trait objects, in the audio domain.

#[derive(Clone, Copy, Debug)]
pub enum Waveform { Sine, Square, Sawtooth, Triangle }

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
}

impl Iterator for Oscillator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
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
        Some(v)
    }
}

// --- Generic Voice (static dispatch) ---
pub struct Voice<O: Iterator<Item = f32>> {
    osc: O,
    gain: f32,
}

impl<O: Iterator<Item = f32>> Voice<O> {
    pub fn new(osc: O, gain: f32) -> Self { Self { osc, gain } }
}

impl<O: Iterator<Item = f32>> Iterator for Voice<O> {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.osc.next().map(|s| s * self.gain)
    }
}

// --- Dynamic Voice (trait object) ---
pub struct DynVoice {
    osc: Box<dyn Iterator<Item = f32>>,
    gain: f32,
}

impl DynVoice {
    pub fn new(osc: Box<dyn Iterator<Item = f32>>, gain: f32) -> Self { Self { osc, gain } }
}

impl Iterator for DynVoice {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.osc.next().map(|s| s * self.gain)
    }
}

fn main() {
    // Generic — concrete type Voice<Oscillator>
    let mut v_static = Voice::new(Oscillator::new(Waveform::Sine, 440.0, 44100), 0.5);
    let first5: Vec<f32> = (0..5).filter_map(|_| v_static.next()).collect();
    println!("Static Voice<Sine>:  first 5 = {:?}", first5);

    // Dynamic — heterogeneous collection
    let voices: Vec<DynVoice> = vec![
        DynVoice::new(Box::new(Oscillator::new(Waveform::Sine,     220.0, 44100)), 0.5),
        DynVoice::new(Box::new(Oscillator::new(Waveform::Square,   220.0, 44100)), 0.3),
        DynVoice::new(Box::new(Oscillator::new(Waveform::Sawtooth, 220.0, 44100)), 0.3),
        DynVoice::new(Box::new(Oscillator::new(Waveform::Triangle, 220.0, 44100)), 0.5),
    ];

    println!("\nFour DynVoices, first 4 samples each:");
    for (i, mut v) in voices.into_iter().enumerate() {
        let buf: Vec<f32> = (0..4).filter_map(|_| v.next()).collect();
        println!("  voice {}: {:?}", i, buf);
    }

    // Tiny perf comparison
    use std::time::Instant;
    const N: usize = 500_000;

    let t0 = Instant::now();
    let mut v = Voice::new(Oscillator::new(Waveform::Sine, 440.0, 44100), 0.5);
    let s: f32 = (0..N).filter_map(|_| v.next()).sum();
    println!("\nStatic  500k samples sum = {:.4}, took {:?}", s, t0.elapsed());

    let t0 = Instant::now();
    let mut v = DynVoice::new(Box::new(Oscillator::new(Waveform::Sine, 440.0, 44100)), 0.5);
    let s: f32 = (0..N).filter_map(|_| v.next()).sum();
    println!("Dynamic 500k samples sum = {:.4}, took {:?}", s, t0.elapsed());
}
