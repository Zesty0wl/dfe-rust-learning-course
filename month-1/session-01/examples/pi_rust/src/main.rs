use std::time::Instant;

fn estimate_pi(n: u64) -> f64 {
    let mut rng = fastrand::Rng::new();
    let mut inside: u64 = 0;
    for _ in 0..n {
        let x = rng.f64();
        let y = rng.f64();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    4.0 * inside as f64 / n as f64
}

fn main() {
    let n: u64 = 100_000_000;
    println!("Estimating Pi using {} samples...", n);
    let start = Instant::now();
    let result = estimate_pi(n);
    let elapsed = start.elapsed();
    println!("Pi ≈ {:.6}", result);
    println!("Time: {:.2?}", elapsed);
    println!("(Run with `cargo run --release` for the real speed test.)");
}
