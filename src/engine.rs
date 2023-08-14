use crate::common::{Action, GameState, SnakeInfo};
pub use dumb::Dumb;

mod dumb;
pub mod rl;

pub trait Engine {
    fn start(&self, state: &GameState);
    fn next(&self, state: &GameState) -> Action;
    fn end(&self, state: &GameState);
}

pub trait Informative {
    fn info(&self) -> SnakeInfo;
}
