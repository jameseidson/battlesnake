use super::{decider::Decider, reward::RewardSignal, valuator::Valuator};
use crate::common::{Action, GameState};

pub trait Policy<V, D, R>
where
    V: Valuator<R>,
    D: Decider,
    R: RewardSignal,
{
    fn enact(&self, state: &GameState) -> D;
    fn evaluate(&self, state: &GameState, evaluator: V);
    fn improve(&self, state: &GameState, better: Action);
}
