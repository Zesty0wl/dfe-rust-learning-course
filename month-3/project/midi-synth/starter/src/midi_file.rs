// midi_file.rs — parse a .mid file with `midly` and render to a buffer.
//
// Reference: ../../solution/src/midi_file.rs

use crate::synth::Waveform;

pub struct RenderedMidi {
    pub samples: Vec<f32>,
    pub events: usize,
    pub duration_s: f32,
}

pub fn render_midi_file(
    _bytes: &[u8],
    _waveform: Waveform,
    _sample_rate: u32,
) -> Result<RenderedMidi, Box<dyn std::error::Error>> {
    // TODO (session 22):
    //   1. parse with midly::Smf::parse
    //   2. read header timing (Metrical = ticks per quarter note)
    //   3. walk every track, tracking tempo and accumulating delta ticks
    //   4. on NoteOn → push an "active note" with start sample index
    //   5. on NoteOff → render render_note(...) for that note's duration and mix it into a buffer
    //   6. return the buffer
    Err("midi-file mode not implemented yet — see session 22.".into())
}
