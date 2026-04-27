#[derive(Debug, Clone, Copy, PartialEq)]
enum NoteName {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}

impl NoteName {
    fn as_str(self) -> &'static str {
        match self {
            NoteName::C       => "C",
            NoteName::CSharp  => "C#",
            NoteName::D       => "D",
            NoteName::DSharp  => "D#",
            NoteName::E       => "E",
            NoteName::F       => "F",
            NoteName::FSharp  => "F#",
            NoteName::G       => "G",
            NoteName::GSharp  => "G#",
            NoteName::A       => "A",
            NoteName::ASharp  => "A#",
            NoteName::B       => "B",
        }
    }
}

enum ScaleType {
    Major,
    NaturalMinor,
    PentatonicMajor,
}

fn semitone_pattern(scale: &ScaleType) -> &'static [u8] {
    match scale {
        ScaleType::Major           => &[2, 2, 1, 2, 2, 2, 1],
        ScaleType::NaturalMinor    => &[2, 1, 2, 2, 1, 2, 2],
        ScaleType::PentatonicMajor => &[2, 2, 3, 2, 3],
    }
}

fn scale_notes(root: NoteName, scale: &ScaleType) -> Vec<NoteName> {
    let pattern = semitone_pattern(scale);
    let chromatic = [
        NoteName::C, NoteName::CSharp, NoteName::D, NoteName::DSharp,
        NoteName::E, NoteName::F, NoteName::FSharp, NoteName::G,
        NoteName::GSharp, NoteName::A, NoteName::ASharp, NoteName::B,
    ];

    let mut index = chromatic.iter().position(|&n| n == root).unwrap();
    let mut result = vec![root];
    for &step in pattern {
        index = (index + step as usize) % 12;
        result.push(chromatic[index]);
    }
    result
}

fn print_scale(label: &str, root: NoteName, scale: &ScaleType) {
    print!("{}: ", label);
    for n in scale_notes(root, scale) {
        print!("{} ", n.as_str());
    }
    println!();
}

fn main() {
    print_scale("C Major", NoteName::C, &ScaleType::Major);
    print_scale("A Natural Minor", NoteName::A, &ScaleType::NaturalMinor);
    print_scale("G Pentatonic Major", NoteName::G, &ScaleType::PentatonicMajor);
}
