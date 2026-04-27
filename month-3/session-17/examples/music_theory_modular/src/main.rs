// main.rs — entry point. Composes the notes and scales modules.

mod notes;
mod scales;

use notes::Note;

fn main() {
    let c_maj = scales::major(Note::C);
    let names: Vec<&str> = c_maj.iter().map(|n| n.name()).collect();
    println!("C Major: {}", names.join(" "));

    let a_min = scales::natural_minor(Note::A);
    let mnames: Vec<&str> = a_min.iter().map(|n| n.name()).collect();
    println!("A Minor: {}", mnames.join(" "));

    let g_maj = scales::major(Note::G);
    let gnames: Vec<&str> = g_maj.iter().map(|n| n.name()).collect();
    println!("G Major: {}", gnames.join(" "));
}
