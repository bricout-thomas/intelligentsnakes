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
                            ..= 4 => Tile::Apple,
                            _ => Tile::Empty,
                        }
                    }
                )}
            ),
        }
    }

    pub fn access_mut<'a>(&'a mut self, coord: (usize, usize)) -> &'a mut Tile {
        return &mut self.field[coord.0 % 50][coord.1 % 80];
    }
    pub fn access(&self, coord: (usize, usize)) -> Tile {
        return self.field[coord.0 % 50][coord.1 % 80];
    }
}

#[derive(Default, Clone, Copy)]
pub enum Tile {
    #[default]
    Empty,
    Apple,
    Body,
    Egg,
    Head,
}