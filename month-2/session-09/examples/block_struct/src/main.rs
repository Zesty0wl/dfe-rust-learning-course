// Session 9 example: structs and methods, modelling a Minecraft-style Block.

#[derive(Debug)]
struct Block {
    kind: u8,
    x: i32,
    y: i32,
    z: i32,
    hardness: f32,
}

impl Block {
    fn new(kind: u8, x: i32, y: i32, z: i32) -> Self {
        let hardness = match kind {
            0 => 0.0,  // air
            1 => 1.5,  // stone
            2 => 2.0,  // iron
            3 => 0.6,  // dirt
            _ => 0.5,
        };
        Self { kind, x, y, z, hardness }
    }

    fn is_solid(&self) -> bool {
        self.kind != 0
    }

    fn display_char(&self) -> char {
        match self.kind {
            0 => ' ',
            1 => '#',
            2 => '%',
            3 => '.',
            _ => '?',
        }
    }

    fn name(&self) -> &'static str {
        match self.kind {
            0 => "Air",
            1 => "Stone",
            2 => "Iron",
            3 => "Dirt",
            _ => "Unknown",
        }
    }

    fn break_it(&mut self) {
        self.kind = 0;
        self.hardness = 0.0;
    }
}

fn main() {
    let stone = Block::new(1, 10, 64, 200);
    let iron  = Block::new(2, 11, 64, 200);
    let air   = Block::new(0, 12, 64, 200);

    for b in [&stone, &iron, &air] {
        println!(
            "{:?} -> name={}, solid={}, char='{}'",
            b,
            b.name(),
            b.is_solid(),
            b.display_char(),
        );
    }

    let mut my_block = Block::new(1, 0, 0, 0);
    println!("\nBefore breaking: {:?}", my_block);
    my_block.break_it();
    println!("After breaking:  {:?}", my_block);
}
