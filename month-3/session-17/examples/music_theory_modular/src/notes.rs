// notes.rs — Note enum + helpers.

#[derive(Debug, Clone, Copy)]
pub enum Note {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}

impl Note {
    pub fn semitone(self) -> u8 {
        self as u8
    }

    pub fn from_semitone(n: u8) -> Self {
        let names = [
            Note::C, Note::CSharp, Note::D, Note::DSharp, Note::E,
            Note::F, Note::FSharp, Note::G, Note::GSharp,
            Note::A, Note::ASharp, Note::B,
        ];
        names[(n % 12) as usize]
    }

    pub fn name(self) -> &'static str {
        match self {
            Note::C => "C",   Note::CSharp => "C#",
            Note::D => "D",   Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",   Note::FSharp => "F#",
            Note::G => "G",   Note::GSharp => "G#",
            Note::A => "A",   Note::ASharp => "A#",
            Note::B => "B",
        }
    }
}
