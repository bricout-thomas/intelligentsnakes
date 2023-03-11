use std::collections::VecDeque;

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
    pub fn new(player: bool) -> Self {
        let mut state = Self {
            player: None,
            grid: Grid::new(),
            heads: vec!(),
            eggs: vec!()
        };
        if player {
            let (x, y) = (40, 25);
            state.player = Some(Head { 
                direction: head::Direction::Top,
                position: (x, y),
                body: VecDeque::new(),
                brainstate: None,
                genome: None,
            });
            state.grid.field[x][y] = Tile::Head;
        }
        state
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        use head::Direction;
        for head in self.heads.iter_mut() {
            head.think(());
        }
        // Open second loop to avoid heads modifying game state conflicting with heads accessing sight
        for head in self.heads.iter_mut() {
            let mut kill_snake = false;
            *self.grid.access_mut(head.position) = Tile::Body;
            head.body.push_back(head.position);
            
            match head.direction {
                Direction::Top => { head.position.0 -= 1 },
                Direction::Bottom => { head.position.0 += 1 },
                Direction::Left => { head.position.1 -= 1 },
                Direction::Right => { head.position.1 += 1 },
            }
            let mut apple = false;
            match self.grid.access(head.position) {
                Tile::Apple => apple = true,
                Tile::Egg => { apple = true; /* might need to remove from self.eggs */ },
                Tile::Body => { kill_snake = true },
                Tile::Head => { kill_snake = true /* might kill the other snake too IDK */ },
                Tile::Empty => {},
            }

            if !apple {
                let tail_end = head.body.pop_front().unwrap(); // unwrap safe because we just pushed something
                *self.grid.access_mut(tail_end) = Tile::Empty;
            }
            *self.grid.access_mut(head.position) = Tile::Head;
            if kill_snake {
                *self.grid.access_mut(head.position) = Tile::Apple;
                for pos in &head.body {
                    *self.grid.access_mut(*pos) = Tile::Apple;
                }
            }
            if kill_snake { self.player = None; } // here because of borrow checker
        }

        {   // player movement ( direction change and movement )
            // I should find a cleaner way to do that, not just repeat every piece of logic
            // but it works so yas
            let mut kill_player = false;
            if let Some(player_head) = &mut self.player {
                if let Some(keypress) = ctx.key {
                    player_head.direction = match keypress {
                        VirtualKeyCode::Up => Direction::Top,
                        VirtualKeyCode::Down => Direction::Bottom,
                        VirtualKeyCode::Left => Direction::Left,
                        VirtualKeyCode::Right => Direction::Right,
                        _ => player_head.direction,
                    };
                }
                self.grid.field[player_head.position.0][player_head.position.1] = Tile::Body;
                player_head.body.push_back(player_head.position);
                
                match player_head.direction {
                    Direction::Top => { player_head.position.0 -= 1 },
                    Direction::Bottom => { player_head.position.0 += 1 },
                    Direction::Left => { player_head.position.1 -= 1 },
                    Direction::Right => { player_head.position.1 += 1 },
                }
                let mut apple = false;
                match self.grid.field[player_head.position.0][player_head.position.1] {
                    Tile::Apple => apple = true,
                    Tile::Egg => { apple = true; /* might need to remove from self.eggs */ },
                    Tile::Body => { kill_player = true },
                    Tile::Head => { kill_player = true /* might kill the other snake too IDK */ },
                    Tile::Empty => {},
                }

                if !apple {
                    let tail_end = player_head.body.pop_front().unwrap(); // unwrap safe because we just pushed something
                    self.grid.field[tail_end.0][tail_end.1] = Tile::Empty;
                }
                self.grid.field[player_head.position.0][player_head.position.1] = Tile::Head;
                if kill_player {
                    self.grid.field[player_head.position.0][player_head.position.1] = Tile::Apple;
                    for (x, y) in &player_head.body {
                        self.grid.field[*x][*y] = Tile::Apple;
                    }
                }
            }
            if kill_player { self.player = None; } // here because of borrow checker
        }

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