fn main() {
    let names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let octaves = 2;
    let total_semitones = octaves * 12;

    print!("Keys:  ");
    for i in 0..total_semitones {
        let in_octave = i % 12;
        let is_black = matches!(in_octave, 1 | 3 | 6 | 8 | 10);
        if is_black {
            print!("B  ");
        } else {
            print!("W  ");
        }
    }
    println!();

    print!("Notes: ");
    for i in 0..total_semitones {
        let in_octave = i % 12;
        print!("{:<3}", names[in_octave]);
    }
    println!();
}
