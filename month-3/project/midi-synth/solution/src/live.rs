// live.rs — live MIDI input + cpal audio output via mpsc channel.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::synth::{midi_to_freq, Adsr, Oscillator, Waveform};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use midir::{MidiInput, Ignore};

#[derive(Clone, Copy, Debug)]
pub enum NoteEvent {
    On  { note: u8, velocity: u8 },
    Off { note: u8 },
}

struct Voice {
    note: u8,
    osc: Oscillator,
    velocity: f32,
    sample_index: u64,
    release_start: Option<u64>,
    sample_rate: u32,
    adsr: Adsr,
}

impl Voice {
    fn new(note: u8, velocity: u8, waveform: Waveform, sample_rate: u32, adsr: Adsr) -> Self {
        let freq = midi_to_freq(note);
        Self {
            note,
            osc: Oscillator::new(waveform, freq, sample_rate),
            velocity: velocity as f32 / 127.0,
            sample_index: 0,
            release_start: None,
            sample_rate,
            adsr,
        }
    }

    fn release(&mut self) {
        if self.release_start.is_none() {
            self.release_start = Some(self.sample_index);
        }
    }

    fn done(&self) -> bool {
        if let Some(rs) = self.release_start {
            let elapsed = (self.sample_index.saturating_sub(rs)) as f32 / self.sample_rate as f32;
            elapsed > self.adsr.release
        } else {
            false
        }
    }

    fn next_sample(&mut self) -> f32 {
        let amp = self.adsr.amplitude(self.sample_index, self.release_start, self.sample_rate);
        let s = self.osc.sample() * amp * self.velocity;
        self.sample_index += 1;
        s
    }
}

pub fn run_live(waveform: Waveform) -> Result<(), Box<dyn std::error::Error>> {
    // --- MIDI input ---
    let mut midi_in = MidiInput::new("midi-synth")?;
    midi_in.ignore(Ignore::None);
    let ports = midi_in.ports();
    if ports.is_empty() {
        return Err("No MIDI input ports found. Plug in a keyboard and try again.".into());
    }
    let port = &ports[0];
    let port_name = midi_in.port_name(port)?;
    println!("Listening on MIDI port: {}", port_name);

    let (tx, rx) = mpsc::channel::<NoteEvent>();

    // midir requires the connection to live for the duration of the program.
    let _conn = midi_in.connect(
        port,
        "midi-synth-port",
        move |_stamp, message, _| {
            if message.len() < 3 { return; }
            let status = message[0] & 0xF0;
            let note = message[1];
            let velocity = message[2];
            match status {
                0x90 if velocity > 0 => { let _ = tx.send(NoteEvent::On  { note, velocity }); }
                0x80 | 0x90          => { let _ = tx.send(NoteEvent::Off { note }); }
                _ => {}
            }
        },
        (),
    )?;

    // --- Audio output via cpal ---
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("no output audio device")?;
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;
    println!("Audio device: {} ({} Hz, {} ch)", device.name().unwrap_or("?".into()), sample_rate, channels);

    let voices: Arc<Mutex<Vec<Voice>>> = Arc::new(Mutex::new(Vec::new()));
    let voices_audio = voices.clone();
    let adsr = Adsr::default();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                // First, drain any pending events.
                while let Ok(ev) = rx.try_recv() {
                    let mut v = voices_audio.lock().unwrap();
                    match ev {
                        NoteEvent::On { note, velocity } => {
                            v.push(Voice::new(note, velocity, waveform, sample_rate, adsr));
                        }
                        NoteEvent::Off { note } => {
                            for voice in v.iter_mut() {
                                if voice.note == note { voice.release(); }
                            }
                        }
                    }
                }

                // Then synthesise.
                let mut v = voices_audio.lock().unwrap();
                for frame in data.chunks_mut(channels) {
                    let mut s = 0.0_f32;
                    for voice in v.iter_mut() { s += voice.next_sample(); }
                    let s = s.clamp(-1.0, 1.0);
                    for sample in frame.iter_mut() { *sample = s; }
                }
                v.retain(|voice| !voice.done());
            },
            move |err| eprintln!("stream error: {}", err),
            None,
        )?,
        other => return Err(format!("Unsupported sample format: {:?}", other).into()),
    };

    stream.play()?;
    println!("Press Ctrl-C to stop.");
    loop { std::thread::sleep(std::time::Duration::from_secs(60)); }
}
