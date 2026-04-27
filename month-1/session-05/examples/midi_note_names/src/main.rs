fn note_name(midi: u8) -> &'static str {
    match midi % 12 {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => unreachable!(),
    }
}

fn full_note_name(midi: u8) -> String {
    let name = note_name(midi);
    let octave = (midi as i32 / 12) - 1;
    format!("{}{}", name, octave)
}

fn main() {
    println!("Some notable MIDI notes:\n");
    for midi in [21u8, 60, 69, 72, 108] {
        println!("  MIDI {:>3} = {}", midi, full_note_name(midi));
    }

    println!("\nThe full chromatic scale around middle C:\n");
    for midi in 60u8..=72 {
        println!("  MIDI {:>3} = {}", midi, full_note_name(midi));
    }
}
