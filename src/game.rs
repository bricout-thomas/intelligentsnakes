use std::collections::VecDeque;

use bracket_terminal::prelude::*;

mod grid; use grid::{Grid, Tile};
use rand::random;

use crate::game::snake::{Direction, Genome};

use self::snake::Snake;
mod snake;

pub struct State {
    pub display_size: (usize, usize),
    pub campos: (usize, usize),
    pub player: Option<Snake>,
    pub grid: Grid,
    pub snakes: Vec<Snake>,
    pub eggs: VecDeque<((usize, usize), Genome)>,
}

impl State {
    pub fn new( player: bool, wh: usize, ww: usize, sh: usize, sw: usize, taurus: bool ) -> Self {
        let midusize = usize::MAX / 2;
        let mut state = Self {
            display_size: (sw, sh),
            campos: (1, 1),
            player: None,
            grid: Grid::new(ww, wh, taurus),
            snakes: vec!(),
            eggs: VecDeque::new()
        };

        let number_of_snakes = 400;
        for _ in 0..number_of_snakes {
            // + usize to avoid buffer overflow or underflow
            let x: usize = random::<usize>() % ww + if taurus { midusize } else { 0 };
            let y: usize = random::<usize>() % wh + if taurus { midusize } else { 0 };
            state.grid.set_tile((x, y), Tile::Head);
            state.snakes.push(Snake {
                head_pos: (x, y),
                tail: VecDeque::new(),
                facing: Direction::random(),
                brainstate: None,
                genome: Some(Genome::new())
            })
        }

        if player {
            let (x, y) = (40, 25);
            state.player = Some(Snake { 
                facing: snake::Direction::random(),
                head_pos: (x, y),
                tail: VecDeque::new(),
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
        for snake in self.snakes.iter_mut() {
            let close_positions: [(isize, isize); 8] = [
                (-1, -1), (0, -1), (1, -1),
                (-1, 0),           (1, 0),
                (-1, 1), (0, 1), (1, 1)];
            let mut sight = Vec::<f32>::with_capacity(16);
            for (x, y) in close_positions.into_iter() {
                sight.push(self.grid.access(
                    ((x+snake.head_pos.0 as isize) as usize, (y+snake.head_pos.1 as isize) as usize)
                ).match_val());
            }
            for (x, y) in close_positions.into_iter() {
                let mut big_square_val = 0.;
                for (sx, sy) in close_positions.into_iter() {
                    let (cx, cy) = ((x*3+sx+snake.head_pos.0 as isize) as usize,(y*3+sy+snake.head_pos.1 as isize) as usize);
                    big_square_val += self.grid.access((cx, cy)).match_val();
                }
                sight.push(big_square_val);
            }
            snake.think(sight);
        }
        // Open second loop to avoid heads modifying game state conflicting with heads accessing sight
        let mut kill_list = vec!();
        for (i, head) in self.snakes.iter_mut().enumerate() {
            let mut kill_snake = false;
            self.grid.set_tile(head.head_pos, Tile::Body);
            head.tail.push_back(head.head_pos);
            
            match head.facing {
                Direction::Top => { head.head_pos.0 -= 1 },
                Direction::Bottom => { head.head_pos.0 += 1 },
                Direction::Left => { head.head_pos.1 -= 1 },
                Direction::Right => { head.head_pos.1 += 1 },
            }
            let mut apple = false;
            match self.grid.access(head.head_pos) {
                Tile::Apple => apple = true,
                Tile::Egg => { apple = true; /* might need to remove from self.eggs */ },
                Tile::Body | Tile::Void => { kill_snake = true },
                Tile::Head => { kill_snake = true /* might kill the other snake too IDK */ },
                Tile::Empty => {},
            }

            if !apple {
                let tail_end = head.tail.pop_front().unwrap(); // unwrap safe because we just pushed something
                *self.grid.access_mut(tail_end) = Tile::Empty;
                if random::<u8>() == 0 {
                    if let Some(tail_end) = head.tail.pop_front() {
                        *self.grid.access_mut(tail_end) = Tile::Egg;
                        self.eggs.push_back((tail_end,
                            match &head.genome {
                                Some(genome) => genome.copy(),
                                None => Genome::new(),
                            })
                        )
                    } else {
                        kill_snake = true;
                    }
                }
            } else {
                if random::<u8>() <= 64 {
                    let tail_end = head.tail.pop_front().unwrap(); // unwrap safe because we just pushed something
                    *self.grid.access_mut(tail_end) = Tile::Egg;
                    self.eggs.push_back((tail_end,
                        match &head.genome {
                            Some(genome) => genome.copy(),
                            None => Genome::new(),
                        })
                    )
                }
            }
            *self.grid.access_mut(head.head_pos) = Tile::Head;
            if kill_snake {
                *self.grid.access_mut(head.head_pos) = Tile::Apple;
                for pos in &head.tail {
                    *self.grid.access_mut(*pos) = Tile::Apple;
                }
            }
            if kill_snake { kill_list.push(i) } // here because of borrow checker
        }
        while let Some(i) = kill_list.pop() { // so that the index of snakes don't change before we try to kill them
            self.snakes.swap_remove(i);
        }

        {   // player movement ( direction change and movement )
            // I should find a cleaner way to do that, not just repeat every piece of logic
            // but it works so yas
            // TODO: rewrite to use access_mut instead of indexing
            let mut kill_player = false;
            if let Some(player_head) = &mut self.player {
                if let Some(keypress) = ctx.key {
                    player_head.facing = match keypress {
                        VirtualKeyCode::Up => Direction::Top,
                        VirtualKeyCode::Down => Direction::Bottom,
                        VirtualKeyCode::Left => Direction::Left,
                        VirtualKeyCode::Right => Direction::Right,
                        _ => player_head.facing,
                    };
                }
                self.grid.field[player_head.head_pos.0][player_head.head_pos.1] = Tile::Body;
                player_head.tail.push_back(player_head.head_pos);
                
                match player_head.facing {
                    Direction::Top => { player_head.head_pos.0 -= 1 },
                    Direction::Bottom => { player_head.head_pos.0 += 1 },
                    Direction::Left => { player_head.head_pos.1 -= 1 },
                    Direction::Right => { player_head.head_pos.1 += 1 },
                }
                let mut apple = false;
                match self.grid.field[player_head.head_pos.0][player_head.head_pos.1] {
                    Tile::Apple => apple = true,
                    Tile::Egg => { apple = true; /* might need to remove from self.eggs */ },
                    Tile::Body | Tile::Void => { kill_player = true },
                    Tile::Head => { kill_player = true /* might kill the other snake too IDK */ },
                    Tile::Empty => {},
                }

                if !apple {
                    let tail_end = player_head.tail.pop_front().unwrap(); // unwrap safe because we just pushed something
                    *self.grid.access_mut(tail_end) = Tile::Empty;
                }
                self.grid.field[player_head.head_pos.0][player_head.head_pos.1] = Tile::Head;
                if kill_player {
                    self.grid.field[player_head.head_pos.0][player_head.head_pos.1] = Tile::Apple;
                    for pos in &player_head.tail {
                        *self.grid.access_mut(*pos) = Tile::Apple;
                    }
                }
            }
            if kill_player { self.player = None; } // here because of borrow checker
        }

        // birth eggs
        let n_birth = random::<u8>() as usize * (self.eggs.len()) / 256;
        for _ in 0..n_birth {
            if let Some((pos, genome)) = self.eggs.pop_front() {
                *self.grid.access_mut(pos) = Tile::Head;
                self.snakes.push(Snake {
                    head_pos: pos,
                    tail: VecDeque::new(),
                    facing: Direction::random(),
                    brainstate: None,
                    genome: Some(genome),
                })
            }
        }

        // move camera pos
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => { self.campos.0 -= 1; },
                VirtualKeyCode::Down => { self.campos.0 += 1; },
                VirtualKeyCode::Left => { self.campos.1 -= 1; },
                VirtualKeyCode::Right => { self.campos.1 += 1; },
                _ => {},
            }
        }

        // Would it be more optimized to do that from eggs and heads ?
        for y in 0..self.display_size.1 {
            let mut buffer = String::with_capacity(self.display_size.0);
            for x in 0..self.display_size.0 {
                buffer.push(
                    match self.grid.access((x + self.campos.0, y + self.campos.1)) {
                        Tile::Empty => ' ',
                        Tile::Apple => '@',
                        Tile::Egg => 'o',
                        Tile::Body => '#',
                        Tile::Head => '*',
                        Tile::Void => unreachable!(),
                    }
                );
            }
            ctx.print(1, y, buffer);
        }
    }
}