// scales.rs — Scale patterns and generation.

use crate::notes::Note;

pub const MAJOR_PATTERN:         [u8; 7] = [2, 2, 1, 2, 2, 2, 1];
pub const NATURAL_MINOR_PATTERN: [u8; 7] = [2, 1, 2, 2, 1, 2, 2];

fn build(root: Note, pattern: &[u8]) -> Vec<Note> {
    let mut out = vec![root];
    let mut current = root.semitone();
    for step in pattern {
        current = (current + step) % 12;
        out.push(Note::from_semitone(current));
    }
    out
}

pub fn major(root: Note) -> Vec<Note> {
    let mut s = build(root, &MAJOR_PATTERN);
    s.pop();   // drop the octave repeat
    s
}

pub fn natural_minor(root: Note) -> Vec<Note> {
    let mut s = build(root, &NATURAL_MINOR_PATTERN);
    s.pop();
    s
}
