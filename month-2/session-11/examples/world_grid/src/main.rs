// Session 11 example: Vec, Vec<Vec<T>>, HashMap.

use std::collections::HashMap;

fn main() {
    // --- Vec basics ---
    let mut nums: Vec<i32> = Vec::new();
    nums.push(10);
    nums.push(20);
    nums.push(30);
    println!("Vec: {:?}, len={}", nums, nums.len());

    let names: Vec<&str> = vec!["Alice", "Bob", "Carol"];
    for name in &names {
        println!("hello {}", name);
    }

    println!("names[1] = {}", names[1]);
    println!("names.get(99) = {:?}", names.get(99));

    // --- 2D grid ---
    let width = 8;
    let height = 4;
    let mut grid: Vec<Vec<&str>> = Vec::with_capacity(height);
    for _y in 0..height {
        let row = vec!["~"; width];
        grid.push(row);
    }
    grid[1][3] = "▲";
    grid[1][4] = "▲";
    grid[2][2] = "▓";
    grid[2][5] = "▓";

    println!("\nGrid:");
    for row in &grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }

    // --- HashMap counting ---
    let world_tiles = vec![
        "Ocean", "Ocean", "Plains", "Mountain",
        "Plains", "Plains", "Forest", "Ocean",
        "Mountain", "Plains", "Forest", "Ocean",
    ];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for tile in &world_tiles {
        *counts.entry(tile).or_insert(0) += 1;
    }

    println!("\nBiome counts:");
    let mut entries: Vec<_> = counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for (biome, count) in entries {
        println!("  {:<10} {}", biome, count);
    }

    let n = counts.get("Ocean").copied().unwrap_or(0);
    println!("\nLooked up Ocean directly: {}", n);
    let m = counts.get("Volcano").copied().unwrap_or(0);
    println!("Looked up Volcano (not present): {}", m);
}
