use crate::common::{Action, GameState};

pub trait RewardSignal {
    fn reward_for(&self, state: &GameState, action: Action) -> f64;
}
