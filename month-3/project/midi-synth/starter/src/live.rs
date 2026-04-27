// live.rs — live MIDI input + cpal audio output.
//
// Reference: ../../solution/src/live.rs

use crate::synth::Waveform;

pub fn run_live(_waveform: Waveform) -> Result<(), Box<dyn std::error::Error>> {
    // TODO (session 23):
    //   1. open a midir MidiInput, list ports, pick port 0
    //   2. create an mpsc::channel<NoteEvent>
    //   3. inside the midir callback, parse the 3-byte message and send NoteOn/NoteOff
    //   4. open the cpal default output stream (F32 format)
    //   5. inside the audio callback, drain rx, update Voice list, fill `data` with samples
    //   6. stream.play() and sleep forever
    Err("live mode not implemented yet — see session 23.".into())
}
