use bracket_terminal::prelude::*;

mod grid; use grid::{Grid, Tile};

use self::head::Head;
mod head;

pub struct State {
    pub player: Option<Head>,
    pub grid: Grid,
    pub heads: Vec<Head>,
    pub eggs: Vec<()>,
}

impl State {
    pub fn new() -> Self {
        let mut state = Self {
            player: None,
            grid: Grid::new(),
            heads: vec!(),
            eggs: vec!()
        };
        let (x, y) = (40, 25);
        state.player = Some(Head { 
            position: (x, y),
            body: vec!(),
            brainstate: None,
            genome: None,
        });
        state.grid.field[x][y] = Tile::Head;
        state
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // TODO: make heads take their decisions and let eggs be born and copy everything from one to the other

        // display new grid to screen
        for (y, line) in self.grid.field.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                ctx.print(
                    x+1, y+1, 
                    match cell {
                        Tile::Empty => " ",
                        Tile::Apple => "@",
                        Tile::Egg => "o",
                        Tile::Body => "#",
                        Tile::Head => "*",
                    }
                );
            }
        }
    }
}