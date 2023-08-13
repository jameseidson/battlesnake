use crate::{Action, GameState};
pub use dumb::Dumb;

mod dumb;

pub trait SnakeEngine {
    fn start(&self, state: &GameState);
    fn next(&self, state: &GameState) -> Action;
    fn end(&self, state: &GameState);
}
