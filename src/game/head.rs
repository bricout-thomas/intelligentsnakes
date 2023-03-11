use std::collections::VecDeque;

pub struct Head {
    pub position: (usize, usize),
    pub body: VecDeque<(usize, usize)>,
    pub direction: Direction,

    pub brainstate: Option<BrainState>,
    pub genome: Option<Genome>,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left, Right, Bottom, Top
}

#[derive(Clone)]
pub struct Synapse {
    input_index: u8,
    output_index: u8,
    weight: f32,
}

pub struct Genome {
    synapses: Vec<Synapse>,
}

impl Genome {
    pub fn copy(&self) -> Self {
        Self { 
            synapses: self.synapses.iter().map(
                    |s| {s.clone()} // TODO: add mutations
            ).collect(),
        }
    }
    pub fn new() -> Self {
        Self {
            synapses: vec!(),
        }
    }
}

pub type BrainState = [f32; 32];
// last four neurons are outputsout
// first sixteen neurons are inputs

impl Head {
    pub fn think(&mut self, sight: ()) {
        // unwrap brainstate and genome
        let Some(genome) = &self.genome else { return; };
        if let None = self.brainstate {
            self.brainstate = Some(Default::default());
        }
        let brainstate = &mut self.brainstate.unwrap(); // Safe unwrap
        // TODO read sight

        // use the genome
        for synapse in genome.synapses.iter() {
            brainstate[synapse.output_index as usize] += brainstate[synapse.input_index as usize] * synapse.weight;
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