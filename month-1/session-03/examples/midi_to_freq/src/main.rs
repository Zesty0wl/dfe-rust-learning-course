use std::io;

fn midi_to_frequency(note: u8) -> f64 {
    440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0)
}

fn main() {
    println!("Enter a MIDI note number (0-127):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let note: u8 = input
        .trim()
        .parse()
        .expect("Please enter a number between 0 and 127");

    let freq = midi_to_frequency(note);
    println!("MIDI {} = {:.2} Hz", note, freq);
}
