// Session 12 example: iterators and closures.

fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("doubled: {:?}", doubled);

    let evens: Vec<i32> = nums.iter().copied().filter(|n| n % 2 == 0).collect();
    println!("evens:   {:?}", evens);

    let total: i32 = (1..=10)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("Sum of squares of evens 1..=10 = {}", total);

    let names = vec!["alpha", "beta", "gamma"];
    for (i, name) in names.iter().enumerate() {
        println!("{}: {}", i, name);
    }

    let world: Vec<Vec<&str>> = vec![
        vec!["~", "~", "~", "▒", "▒"],
        vec!["~", "~", "▒", "▓", "▓"],
        vec!["▒", "▒", "▓", "▲", "▓"],
    ];

    let ocean_count = world.iter().flatten().filter(|c| **c == "~").count();
    let mountain_count = world.iter().flatten().filter(|c| **c == "▲").count();
    println!("\nOcean tiles:    {}", ocean_count);
    println!("Mountain tiles: {}", mountain_count);

    // Map a row to a single string
    let row_strings: Vec<String> = world
        .iter()
        .map(|row| row.iter().copied().collect::<String>())
        .collect();
    println!("\nWorld:");
    for line in &row_strings {
        println!("{}", line);
    }

    // Closure capturing
    let threshold = 3;
    let big = nums.iter().copied().filter(|n| *n > threshold).count();
    println!("\nNumbers > {} = {}", threshold, big);

    // .find returns Option
    let first_big = nums.iter().find(|n| **n > 3);
    println!("first > 3 = {:?}", first_big);

    // .any / .all
    println!("any > 4? {}", nums.iter().any(|n| *n > 4));
    println!("all > 0? {}", nums.iter().all(|n| *n > 0));
}
