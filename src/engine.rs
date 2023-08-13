use crate::{Direction, GameState};
pub use dumb::Dumb;

mod dumb;

pub trait Engine: 'static + Clone + Sync + Send {
    fn start(&self, state: GameState);
    fn next(&self, state: GameState) -> Direction;
    fn end(&self, state: GameState);
}
