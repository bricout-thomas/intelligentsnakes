use std::collections::VecDeque;

pub struct Head {
    pub position: (usize, usize),
    pub body: VecDeque<(usize, usize)>,
    pub direction: Direction,

    pub brainstate: Option<BrainState>,
    pub genome: Option<Vec<Synapse>>,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left, Right, Bottom, Top
}

pub struct Synapse {
    input_index: u8,
    output_index: u8,
    weight: f32,
}

pub type BrainState = [f32; 32];
// last four neurons are outputs
// first sixteen neurons are inputs

impl Head {
    pub fn think(&mut self, sight: ()) {
        // unwrap brainstate and genome
        let Some(genome) = &self.genome else { return; };
        match self.brainstate {
            Some(_brainstate) => {},
            None => { self.brainstate = Default::default();  },
        }
        let brainstate = &mut self.brainstate.unwrap(); // Safe unwrap
        // TODO read sight

        // use the genome
        for synapse in genome.iter() {
            brainstate[synapse.output_index as usize] = brainstate[synapse.output_index as usize] * synapse.weight;
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
        self.direction = match neuron {
            28 => Direction::Top,
            29 => Direction::Bottom,
            30 => Direction::Left,
            31 => Direction::Right,
            _ => unreachable!()
        }
    }
}