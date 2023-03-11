use super::head::Head;

pub struct Grid {
    pub field: [[Tile; 80]; 50],
}

impl Grid {
    pub fn new() -> Self {
        use rand::random;
        Self {
            field: [[(); 80]; 50].map(
                |col| { col.map(
                    |_| {
                        let r: u8 = random();
                        match r {
                            ..= 4 => Tile::Egg,
                            _ => Tile::Empty,
                        }
                    }
                )}
            ),
        }
    }
}

#[derive(Default)]
pub enum Tile {
    #[default]
    Empty,
    Apple,
    Body,
    Egg,
    Head,
}