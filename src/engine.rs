use crate::common::{Action, GameState};

pub mod rl;

pub trait Engine {
    fn start(&self, state: &GameState);
    fn next(&self, state: &GameState) -> Action;
    fn end(&self, state: &GameState);
}
