// Session 19 example: custom Iterator for an audio oscillator.

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
            // sawtooth: ramps from -1 to 1 over each cycle
            Waveform::Sawtooth => 2.0 * frac - 1.0,
            // triangle: 0 -> 1 -> 0 -> -1 -> 0 over each cycle
            Waveform::Triangle => 4.0 * (frac - 0.5).abs() - 1.0,
        };

        self.sample_index += 1;
        Some(v)
    }
}

fn main() {
    let osc = Oscillator::new(Waveform::Sine, 440.0, 44100);
    let one_second: Vec<f32> = osc.take(44100).collect();
    println!("Generated {} samples; first 5 = {:?}", one_second.len(), &one_second[..5]);

    println!();
    for wf in [Waveform::Sine, Waveform::Square, Waveform::Sawtooth, Waveform::Triangle] {
        let osc = Oscillator::new(wf, 220.0, 44100);
        let buf: Vec<f32> = osc.take(22050).collect();
        let max = buf.iter().cloned().fold(f32::MIN, f32::max);
        let min = buf.iter().cloned().fold(f32::MAX, f32::min);
        println!("{:?}: {} samples, range [{:.2}, {:.2}]", wf, buf.len(), min, max);
    }

    // Demonstrating zip + map: two oscillators averaged ("chord-lite")
    println!("\nChord-lite (440 Hz + 554 Hz, 4 samples):");
    let a = Oscillator::new(Waveform::Sine, 440.0, 44100);
    let b = Oscillator::new(Waveform::Sine, 554.0, 44100);
    let mixed: Vec<f32> = a.zip(b).map(|(x, y)| (x + y) * 0.5).take(4).collect();
    println!("{:?}", mixed);
}
