// music-theory-cli — STARTER
//
// Your job: complete the TODOs below so this matches the README target output.
// Reference: month-1/session-06/examples/scales_intro/ has the engine.
// Reference: month-1/session-07/README.md has the suggested approach.

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

#[derive(Debug)]
enum ScaleType {
    Major,
    NaturalMinor,
    PentatonicMajor,
}

fn parse_note(s: &str) -> Option<NoteName> {
    // TODO Session 7: handle both # and b notation, and case-insensitivity.
    match s {
        "C"  => Some(NoteName::C),
        "C#" | "Db" => Some(NoteName::CSharp),
        // TODO: complete the rest
        _ => None,
    }
}

fn parse_scale(s: &str) -> Option<ScaleType> {
    // TODO Session 7
    None
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

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: music-theory-cli --root <NOTE> --scale <major|minor|pentatonic>");
    std::process::exit(1);
}

fn main() {
    // TODO Session 7: parse args, find --root and --scale values, then print scale.
    // TODO Session 8: also print intervals (W/H) and chord progression.

    let args: Vec<String> = std::env::args().collect();
    println!("You passed {} args. Now wire them up.", args.len() - 1);
}
