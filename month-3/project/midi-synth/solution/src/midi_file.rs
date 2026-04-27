// midi_file.rs — parse a .mid file and render to a Vec<f32> audio buffer.

use crate::synth::{render_note, mix_into, normalise, Adsr, Waveform};
use midly::{Smf, MidiMessage, TrackEventKind, MetaMessage, Timing};

#[derive(Debug, Clone, Copy)]
struct ActiveNote {
    note: u8,
    velocity: u8,
    start_sample: u64,
}

pub struct RenderedMidi {
    pub samples: Vec<f32>,
    pub events: usize,
    pub duration_s: f32,
}

pub fn render_midi_file(
    bytes: &[u8],
    waveform: Waveform,
    sample_rate: u32,
) -> Result<RenderedMidi, Box<dyn std::error::Error>> {
    let smf = Smf::parse(bytes)?;

    // Extract ticks per quarter note.
    let tpqn: u32 = match smf.header.timing {
        Timing::Metrical(t) => t.as_int() as u32,
        Timing::Timecode(_, _) => return Err("SMPTE timing not supported".into()),
    };

    // Default tempo: 500_000 µs/qn = 120 BPM.
    let mut tempo_us_per_qn: u32 = 500_000;
    let mut buffer: Vec<f32> = Vec::new();
    let mut event_count = 0usize;
    let adsr = Adsr::default();

    // Iterate every track sequentially. For multi-track MIDI we treat each track's deltas
    // independently and mix all into the same output buffer.
    for track in &smf.tracks {
        let mut active: Vec<ActiveNote> = Vec::new();
        let mut tick: u64 = 0;
        for ev in track {
            tick += ev.delta.as_int() as u64;
            let time_s = ticks_to_seconds(tick, tpqn, tempo_us_per_qn);
            let sample_index = (time_s * sample_rate as f32) as u64;

            match ev.kind {
                TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                    tempo_us_per_qn = t.as_int();
                }
                TrackEventKind::Midi { message, .. } => {
                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            if vel.as_int() == 0 {
                                // NoteOn with velocity 0 = NoteOff (per MIDI spec)
                                finish_note(&mut active, &mut buffer, key.as_int(), sample_index, waveform, sample_rate, adsr);
                            } else {
                                active.push(ActiveNote {
                                    note: key.as_int(),
                                    velocity: vel.as_int(),
                                    start_sample: sample_index,
                                });
                                event_count += 1;
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                            finish_note(&mut active, &mut buffer, key.as_int(), sample_index, waveform, sample_rate, adsr);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // Any leftover active notes get flushed with a default 1-second duration.
        let last_tick_seconds = ticks_to_seconds(tick, tpqn, tempo_us_per_qn);
        let end_sample = (last_tick_seconds * sample_rate as f32) as u64 + sample_rate as u64;
        for n in active.drain(..) {
            let dur = (end_sample - n.start_sample) as f32 / sample_rate as f32;
            let note_buf = render_note(waveform, n.note, dur.max(0.05), sample_rate, n.velocity as f32 / 127.0, adsr);
            mix_into(&mut buffer, &note_buf, n.start_sample as usize);
        }
    }

    normalise(&mut buffer);
    let duration_s = buffer.len() as f32 / sample_rate as f32;
    Ok(RenderedMidi { samples: buffer, events: event_count, duration_s })
}

fn finish_note(
    active: &mut Vec<ActiveNote>,
    buffer: &mut Vec<f32>,
    note: u8,
    end_sample: u64,
    waveform: Waveform,
    sample_rate: u32,
    adsr: Adsr,
) {
    if let Some(pos) = active.iter().position(|n| n.note == note) {
        let n = active.remove(pos);
        let dur = (end_sample.saturating_sub(n.start_sample)) as f32 / sample_rate as f32;
        if dur < 0.001 { return; }
        let note_buf = render_note(waveform, n.note, dur, sample_rate, n.velocity as f32 / 127.0, adsr);
        mix_into(buffer, &note_buf, n.start_sample as usize);
    }
}

fn ticks_to_seconds(ticks: u64, tpqn: u32, tempo_us_per_qn: u32) -> f32 {
    // tempo_us_per_qn microseconds per quarter note
    // ticks / tpqn = number of quarter notes
    // → seconds = (ticks / tpqn) * tempo_us_per_qn * 1e-6
    (ticks as f64 / tpqn as f64) as f32 * (tempo_us_per_qn as f32 / 1_000_000.0)
}
