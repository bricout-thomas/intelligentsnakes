use std::collections::VecDeque;

use rand::random;
mod brain; pub use brain::{BrainState, Genome};

pub struct Snake {
    pub head_pos: (usize, usize),
    pub tail: VecDeque<(usize, usize)>,
    pub facing: Direction,

    pub brainstate: Option<BrainState>,
    pub genome: Option<Genome>,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left, Right, Bottom, Top
}

impl Direction {
    pub fn random() -> Self {
        match random::<u8>() % 4 {
            0 => Direction::Top,
            1 => Direction::Bottom,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!()
        }
    }
}



impl Snake {
    pub fn think(&mut self, sight: Vec<f32>) {
        // unwrap brainstate and genome
        let Some(genome) = &self.genome else { return; };
        if let None = self.brainstate {
            self.brainstate = Some(Default::default());
        }
        let brainstate = &mut self.brainstate.unwrap(); // Safe unwrap
        // read sight
        for (i, val) in sight.into_iter().enumerate() {
            brainstate[i] = val;
        }
        // set seventeenth neuron to one to avoid one unitt length snakes to be stuck in randomness
        brainstate[16] = 1.;
        // use the genome
        for synapse in genome.synapses.iter() {
            synapse.act(brainstate);
        }
        // actualize self.direction according to output neurons
        let mut greater_v = f32::MIN;
        let mut neuron = 28;
        for i in 28..32 {
            if brainstate[i] > greater_v {
                neuron = i;
                greater_v = brainstate[i];
            }
        }
        if greater_v != 0. { self.facing = match neuron {
            28 => Direction::Top,
            29 => Direction::Bottom,
            30 => Direction::Left,
            31 => Direction::Right,
            _ => unreachable!()
        }} else { self.facing = Direction::random() }; // avoid the snakes always going in the same direction
    }
}