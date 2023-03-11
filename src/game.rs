use std::collections::VecDeque;

use bracket_terminal::prelude::*;

mod grid; use grid::{Grid, Tile};
use rand::random;

use crate::game::head::{Direction, Genome};

use self::head::Head;
mod head;

pub struct State {
    pub player: Option<Head>,
    pub grid: Grid,
    pub heads: Vec<Head>,
    pub eggs: VecDeque<((usize, usize), Genome)>,
}

impl State {
    pub fn new(player: bool) -> Self {
        let mut state = Self {
            player: None,
            grid: Grid::new(),
            heads: vec!(),
            eggs: VecDeque::new()
        };

        let number_of_snakes = 10;
        for _ in 0..number_of_snakes {
            let x: usize = random::<usize>() % 50;
            let y: usize = random::<usize>() % 80;
            *state.grid.access_mut((x, y)) = Tile::Head;
            state.heads.push(Head {
                position: (x, y),
                body: VecDeque::new(),
                direction: Direction::Top,
                brainstate: None,
                genome: Some(Genome::new())
            })
        }

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
        for head in self.heads.iter_mut() {
            let close_positions: [(isize, isize); 8] = [
                (-1, -1), (0, -1), (1, -1),
                (-1, 0),           (1, 0),
                (-1, 1), (0, 1), (1, 1)];
            let mut sight = Vec::<f32>::with_capacity(16);
            for (x, y) in close_positions.into_iter() {
                sight.push(self.grid.access(
                    ((x+head.position.0 as isize) as usize, (y+head.position.1 as isize) as usize)
                ).match_val());
            }
            head.think(sight);
        }
        // Open second loop to avoid heads modifying game state conflicting with heads accessing sight
        let mut kill_list = vec!();
        for (i, head) in self.heads.iter_mut().enumerate() {
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
            } else {
                if random::<u8>() <= 64 {
                    let tail_end = head.body.pop_front().unwrap(); // unwrap safe because we just pushed something
                    *self.grid.access_mut(tail_end) = Tile::Egg;
                    self.eggs.push_back((tail_end,
                        match &head.genome {
                            Some(genome) => genome.copy(),
                            None => Genome::new(),
                        })
                    )
                }
            }
            *self.grid.access_mut(head.position) = Tile::Head;
            if kill_snake {
                *self.grid.access_mut(head.position) = Tile::Apple;
                for pos in &head.body {
                    *self.grid.access_mut(*pos) = Tile::Apple;
                }
            }
            if kill_snake { kill_list.push(i) } // here because of borrow checker
        }
        while let Some(i) = kill_list.pop() { // so that the index of snakes don't change before we try to kill them
            self.heads.swap_remove(i);
        }

        {   // player movement ( direction change and movement )
            // I should find a cleaner way to do that, not just repeat every piece of logic
            // but it works so yas
            // TODO: rewrite to use access_mut instead of indexing
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

        // birth eggs
        let n_birth = random::<u8>() as usize * (self.eggs.len() + 1) / 256;
        for _ in 0..n_birth {
            if let Some((pos, genome)) = self.eggs.pop_front() {
                *self.grid.access_mut(pos) = Tile::Head;
                self.heads.push(Head {
                    position: pos,
                    body: VecDeque::new(),
                    direction: Direction::Top,
                    brainstate: None,
                    genome: Some(genome),
                })
            }
        }

        // display new grid to screen
        // Would it be more optimized to do that from eggs and heads
        // Or to use store the string in a buffer before calling ctx.print to print everything at once?
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