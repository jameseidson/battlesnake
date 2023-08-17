use crate::common::{Action, GameState};
use rand::{distributions::Standard, prelude::*};
use reward::RewardSignal;
use valuator::Valuator;

pub mod reward;
pub mod valuator;

pub trait Agent<V, R>
where
    V: Valuator<R>,
    R: RewardSignal,
{
    fn act(&self, state: &GameState) -> ActionSelector;
    fn evaluate(&self, state: &GameState, valuator: V);
    fn improve(&self, state: &GameState, better: Action);
}

pub struct ActionSelector {
    probabilities: [f64; 4],
}

impl ActionSelector {
    pub fn e_greedy(self, e: f64) -> Action {
        let mut rng = SmallRng::from_entropy();
        if e > rng.sample::<f64, Standard>(Standard) {
            self.greedy()
        } else {
            (rng.gen::<u8>() % 4).try_into().unwrap()
        }
    }

    pub fn greedy(&self) -> Action {
        let mut max = 0.0;
        let mut argmax: u8 = 0;

        for (i, p) in self.probabilities.iter().enumerate() {
            if *p > max {
                max = *p;
                argmax = i as u8
            }
        }

        argmax.try_into().unwrap()
    }
}
