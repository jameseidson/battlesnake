use crate::common::Action;

type ActionDistribution = [f64; 4];

pub trait Decider: From<ActionDistribution> {
    fn decide(self) -> Action;
}

pub struct Greedy {
    distribution: ActionDistribution,
}

impl From<ActionDistribution> for Greedy {
    fn from(value: ActionDistribution) -> Self {
        return Self {
            distribution: value,
        };
    }
}

impl Decider for Greedy {
    fn decide(self) -> Action {
        let mut max = 0.0;
        let mut argmax: u8 = 0;

        for (i, p) in self.distribution.iter().enumerate() {
            if *p > max {
                max = *p;
                argmax = i as u8
            }
        }

        argmax.try_into().unwrap()
    }
}
