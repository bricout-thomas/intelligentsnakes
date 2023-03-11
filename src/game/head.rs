use std::collections::VecDeque;

use rand::random;

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

#[derive(Clone, Copy)]
pub struct Synapse {
    input_index: u8,
    output_index: u8,
    weight: f32,
}

impl Synapse {
    fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
    fn random() -> Self {
        Self {
            input_index: random::<u8>() % 32,
            output_index: random::<u8>() % 16 + 16,
            weight: random::<f32>() * match random() { true => 1., false => -1. },
        }
    }
}

pub struct Genome {
    synapses: Vec<Synapse>,
}

impl Genome {
    pub fn copy(&self) -> Self {
        Self { 
            synapses: {
                let mut ss = Vec::with_capacity(self.synapses.len() + 1);
                for s in self.synapses.iter().copied() {
                    let mu: u8 = random::<u8>() % 32;
                    if mu == 0 { // deletion
                        
                    } else if mu == 1 { // insertion
                        ss.push(s);
                        ss.push(Synapse::random());
                    } else if mu < 4 { // slight modification
                        ss.push(s.with_weight(random::<f32>() * match random() { true => 1., false => -1. }))
                    } else { // no mutation
                        ss.push(s);
                    }
                }
                ss
            },
        }
    }
    pub fn new() -> Self {
        Self {
            synapses: (0..10).map(|_| {Synapse::random()}).collect(),
        }
    }
}

pub type BrainState = [f32; 32];
// last four neurons are outputsout
// first sixteen neurons are inputs

impl Head {
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