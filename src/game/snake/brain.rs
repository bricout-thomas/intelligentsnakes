use rand::random;

#[derive(Clone, Copy)]
pub struct Synapse {
    input_index: u8,
    output_index: u8,
    weight: f32,
}

impl Synapse {
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
    pub fn random() -> Self {
        Self {
            input_index: random::<u8>() % 32,
            output_index: random::<u8>() % 16 + 16,
            weight: random::<f32>() * match random() { true => 1., false => -1. },
        }
    }
    pub fn act(&self, brainstate: &mut BrainState) {
        let outindex = self.output_index as usize;
        brainstate[outindex] += brainstate[self.input_index as usize] * self.weight;
        brainstate[outindex] = if brainstate[outindex] > 1. {
            1.
        } else if brainstate[outindex]< -1. {
            -1.
        } else { brainstate[outindex] }
    }
}

pub struct Genome {
    pub synapses: Vec<Synapse>,
}

impl Genome {
    pub fn copy(&self) -> Self {
        Self { 
            synapses: {
                let mut ss = Vec::with_capacity(self.synapses.len() + 1);
                for s in self.synapses.iter().copied() {
                    let mu: u8 = random::<u8>() % 33;
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
            synapses: (0..30).map(|_| {Synapse::random()}).collect(),
        }
    }
}

pub type BrainState = [f32; 32];
// last four neurons are outputsout
// first sixteen neurons are inputs