use crate::common::{Action, GameState};

pub trait RewardSignal {
    fn reward(&self, state: &GameState, action: Action) -> f64;
}
