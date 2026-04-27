// Session 14 example: traits, Display, and trait bounds.

use std::fmt;

trait Describable {
    fn describe(&self) -> String;
    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Ocean { depth: u8 },
    Plains,
    Mountain { height: u16 },
}

impl Describable for Tile {
    fn describe(&self) -> String {
        match self {
            Tile::Ocean { depth }     => format!("ocean ({}m deep)", depth),
            Tile::Plains              => String::from("plains"),
            Tile::Mountain { height } => format!("mountain ({}m tall)", height),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Tile::Ocean { .. }    => '~',
            Tile::Plains          => '▒',
            Tile::Mountain { .. } => '▲',
        };
        write!(f, "{}", symbol)
    }
}

// Generic function bound on Describable
fn announce<T: Describable>(thing: &T) {
    println!(">> {}", thing.describe());
    println!("!! {}", thing.shout());
}

fn main() {
    let tiles = vec![
        Tile::Ocean { depth: 12 },
        Tile::Plains,
        Tile::Mountain { height: 2200 },
    ];

    for t in &tiles {
        announce(t);
    }

    println!("\nDisplay (one char each):");
    for t in &tiles {
        print!("{} ", t);
    }
    println!();

    println!("\nDebug (full structure):");
    for t in &tiles {
        println!("  {:?}", t);
    }

    // Demonstrating Clone (derived)
    let original = Tile::Mountain { height: 4000 };
    let cloned = original.clone();
    println!("\nOriginal: {:?}\nCloned:   {:?}", original, cloned);
}
