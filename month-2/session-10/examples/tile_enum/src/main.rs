// Session 10 example: enums with data + Option<T>.

#[derive(Debug)]
enum Tile {
    Ocean { depth: u8 },
    Plains,
    Forest { density: u8 },
    Mountain { height: u16 },
    Desert,
}

impl Tile {
    fn describe(&self) -> String {
        match self {
            Tile::Ocean { depth }     => format!("Ocean ({}m deep)", depth),
            Tile::Plains              => String::from("Grassy plains"),
            Tile::Forest { density }  => format!("Forest ({}% trees)", density),
            Tile::Mountain { height } => format!("Mountain ({}m tall)", height),
            Tile::Desert              => String::from("Hot, sandy desert"),
        }
    }
}

fn find_mountain(world: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    for (y, row) in world.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Tile::Mountain { .. } = tile {
                return Some((x, y));
            }
        }
    }
    None
}

fn main() {
    let world: Vec<Vec<Tile>> = vec![
        vec![Tile::Ocean { depth: 8 }, Tile::Plains, Tile::Forest { density: 60 }],
        vec![Tile::Plains, Tile::Mountain { height: 1500 }, Tile::Desert],
        vec![Tile::Forest { density: 80 }, Tile::Plains, Tile::Ocean { depth: 2 }],
    ];

    for row in &world {
        for tile in row {
            println!("- {}", tile.describe());
        }
    }

    println!();
    match find_mountain(&world) {
        Some((x, y)) => println!("Mountain found at ({}, {}).", x, y),
        None         => println!("No mountains in this world."),
    }

    let mountainless: Vec<Vec<Tile>> = vec![vec![Tile::Plains, Tile::Desert]];
    let result = find_mountain(&mountainless);
    println!("Mountainless world: is_some? {}", result.is_some());
    println!("Mountainless world: unwrap_or((99,99)) -> {:?}", result.unwrap_or((99, 99)));
}
