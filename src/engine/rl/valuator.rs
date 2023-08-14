use super::reward::RewardSignal;
use crate::common::{Action, GameState};

pub trait Valuator<R>
where
    R: RewardSignal,
{
    fn action_value(&self, state: &GameState, action: Action) -> f32;
}
