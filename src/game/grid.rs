pub struct Grid {
    pub field: Vec<Vec<Tile>>,
    x_size: usize,
    y_size: usize,
    taurus: bool,
}

impl Grid {
    pub fn new(x_size: usize, y_size: usize, taurus: bool ) -> Self {
        use rand::random;
        Self {
            field: {
                (0..x_size).map(|_| {
                    (0..y_size).map(|_| {
                        let r: u8 = random();
                        match r {
                            ..= 32 => Tile::Apple,
                            _ => Tile::Empty,
                        }
                    }).collect()
                }).collect()
            },
            x_size,
            y_size,
            taurus,
        }
    }
    // deprecated
    pub fn access_mut<'a>(&'a mut self, coord: (usize, usize)) -> &'a mut Tile {
        return &mut self.field[coord.0 % self.y_size][coord.1 % self.x_size];
    }
    pub fn set_tile(&mut self, coord: (usize, usize), new_tile: Tile) {
        self.field[coord.0 % self.y_size][coord.1 % self.x_size] = new_tile;
    }
    pub fn access(&self, coord: (usize, usize)) -> Tile {
        return if self.taurus {
            self.field[coord.0 % self.y_size][coord.1 % self.x_size]
        } else {
            if coord.0 >= self.x_size || coord.1 >= self.y_size { Tile::Void }
            else { self.field[coord.0][coord.1] }
        }
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
    Void,
}

impl Tile {
    pub fn match_val(&self) -> f32 {
        match self {
            Tile::Apple | Tile::Egg => 1.,                  // good things
            Tile::Body | Tile::Head | Tile::Void => -1.,    // usually bad things
            _ => 0.,
        }
    }
}