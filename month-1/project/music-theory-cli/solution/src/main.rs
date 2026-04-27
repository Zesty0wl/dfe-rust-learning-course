// music-theory-cli — Month 1 SOLUTION
//
// Run examples:
//   cargo run -- --root C  --scale major
//   cargo run -- --root F# --scale minor
//   cargo run -- --root G  --scale pentatonic
//   cargo run -- --root Bb --scale major

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

impl ScaleType {
    fn label(&self) -> &'static str {
        match self {
            ScaleType::Major           => "Major",
            ScaleType::NaturalMinor    => "NaturalMinor",
            ScaleType::PentatonicMajor => "PentatonicMajor",
        }
    }
}

fn parse_note(s: &str) -> Option<NoteName> {
    // Normalise: trim, then keep first letter uppercase + remainder as-is.
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap().to_ascii_uppercase();
    let rest: String = chars.collect();
    let normalised = format!("{}{}", first, rest);

    match normalised.as_str() {
        "C"          => Some(NoteName::C),
        "C#" | "Db"  => Some(NoteName::CSharp),
        "D"          => Some(NoteName::D),
        "D#" | "Eb"  => Some(NoteName::DSharp),
        "E"          => Some(NoteName::E),
        "F"          => Some(NoteName::F),
        "F#" | "Gb"  => Some(NoteName::FSharp),
        "G"          => Some(NoteName::G),
        "G#" | "Ab"  => Some(NoteName::GSharp),
        "A"          => Some(NoteName::A),
        "A#" | "Bb"  => Some(NoteName::ASharp),
        "B"          => Some(NoteName::B),
        _            => None,
    }
}

fn parse_scale(s: &str) -> Option<ScaleType> {
    match s.trim().to_lowercase().as_str() {
        "major" | "maj"                          => Some(ScaleType::Major),
        "minor" | "min" | "natural-minor"        => Some(ScaleType::NaturalMinor),
        "pentatonic" | "pentatonic-major" | "pent" => Some(ScaleType::PentatonicMajor),
        _                                        => None,
    }
}

fn semitone_pattern(scale: &ScaleType) -> &'static [u8] {
    match scale {
        ScaleType::Major           => &[2, 2, 1, 2, 2, 2, 1],
        ScaleType::NaturalMinor    => &[2, 1, 2, 2, 1, 2, 2],
        ScaleType::PentatonicMajor => &[2, 2, 3, 2, 3],
    }
}

fn chord_qualities(scale: &ScaleType) -> &'static [&'static str] {
    match scale {
        ScaleType::Major           => &["maj", "min", "min", "maj", "maj", "min", "dim"],
        ScaleType::NaturalMinor    => &["min", "dim", "maj", "min", "min", "maj", "maj"],
        ScaleType::PentatonicMajor => &["maj", "maj", "maj", "maj", "maj"],
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
    // Drop the trailing octave so we have exactly N degrees.
    result.pop();
    result
}

fn intervals_for(scale: &ScaleType) -> Vec<&'static str> {
    semitone_pattern(scale)
        .iter()
        .map(|&n| if n == 1 { "H" } else { "W" })
        .collect()
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: music-theory-cli --root <NOTE> --scale <major|minor|pentatonic>");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  music-theory-cli --root C  --scale major");
    eprintln!("  music-theory-cli --root F# --scale minor");
    eprintln!("  music-theory-cli --root G  --scale pentatonic");
    std::process::exit(1);
}

fn parse_args() -> (NoteName, ScaleType) {
    let args: Vec<String> = std::env::args().collect();
    let mut root: Option<NoteName> = None;
    let mut scale: Option<ScaleType> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                if i + 1 >= args.len() { print_usage_and_exit(); }
                root = parse_note(&args[i + 1]);
                if root.is_none() {
                    eprintln!("Unknown note: {}", args[i + 1]);
                    print_usage_and_exit();
                }
                i += 2;
            }
            "--scale" => {
                if i + 1 >= args.len() { print_usage_and_exit(); }
                scale = parse_scale(&args[i + 1]);
                if scale.is_none() {
                    eprintln!("Unknown scale: {}", args[i + 1]);
                    print_usage_and_exit();
                }
                i += 2;
            }
            "-h" | "--help" => print_usage_and_exit(),
            other => {
                eprintln!("Unknown argument: {}", other);
                print_usage_and_exit();
            }
        }
    }

    match (root, scale) {
        (Some(r), Some(s)) => (r, s),
        _ => print_usage_and_exit(),
    }
}

fn main() {
    let (root, scale) = parse_args();
    let notes = scale_notes(root, &scale);
    let intervals = intervals_for(&scale);
    let qualities = chord_qualities(&scale);

    println!("Scale: {} {}", root.as_str(), scale.label());

    print!("Notes:    ");
    for n in &notes {
        print!(" {:<4}", n.as_str());
    }
    println!();

    print!("Intervals:");
    for iv in &intervals {
        print!(" {:<4}", iv);
    }
    println!();

    print!("Chords:   ");
    for (n, q) in notes.iter().zip(qualities.iter()) {
        let chord = format!("{}{}", n.as_str(), q);
        print!(" {:<4}", chord);
    }
    println!();
}
