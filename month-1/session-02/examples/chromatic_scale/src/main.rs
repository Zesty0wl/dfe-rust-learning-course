fn main() {
    let a4 = 440.0_f64;
    let ratio = 2.0_f64.powf(1.0 / 12.0);
    let mut frequency = a4;

    let names = [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ];

    println!("Chromatic scale starting at A4 (440 Hz):");
    println!();
    for name in names {
        println!("  {:>2}: {:>7.2} Hz", name, frequency);
        frequency *= ratio;
    }
    println!();
    println!("After 12 semitones: {:.2} Hz (should be ~880, an octave above A4)", frequency);
}
