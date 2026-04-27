fn main() {
    println!("Hello, world!");
    println!("My name is Leo.");
    println!("I am learning Rust.");

    let semitones_per_octave = 12;
    let octaves = 4;
    let total_notes = semitones_per_octave * octaves;
    println!("A {}-octave keyboard has {} notes.", octaves, total_notes);
}
