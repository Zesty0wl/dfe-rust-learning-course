// Session 13 example: Result<T, E>, custom errors, ? operator.

#[derive(Debug)]
enum SeedError {
    Empty,
    NotANumber(String),
    OutOfRange(u64),
}

fn parse_seed(s: &str) -> Result<u64, SeedError> {
    if s.is_empty() {
        return Err(SeedError::Empty);
    }
    let n: u64 = s
        .parse()
        .map_err(|_| SeedError::NotANumber(s.to_string()))?;
    if n > 1_000_000 {
        return Err(SeedError::OutOfRange(n));
    }
    Ok(n)
}

fn run() -> Result<(), SeedError> {
    let args: Vec<String> = std::env::args().collect();
    // Look for --seed VALUE, fall back to "" if missing
    let mut seed_str = "";
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--seed" && i + 1 < args.len() {
            seed_str = &args[i + 1];
            break;
        }
        i += 1;
    }
    let seed = parse_seed(seed_str)?;
    println!("Seed parsed successfully: {}", seed);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        match e {
            SeedError::Empty         => eprintln!("Error: please pass --seed N, e.g. --seed 42"),
            SeedError::NotANumber(s) => eprintln!("Error: '{}' is not a number", s),
            SeedError::OutOfRange(n) => eprintln!("Error: seed {} is too big (max 1,000,000)", n),
        }
        std::process::exit(1);
    }
}
