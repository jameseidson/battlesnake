use super::Engine;
use crate::{Action, GameState};
use rand::{self, Rng};

#[derive(Clone)]
pub struct Dumb;

impl Engine for Dumb {
    fn start(&self, _: GameState) {}
    fn next(&self, _: GameState) -> Action {
        match rand::thread_rng().gen_range(0..=3) {
            0 => Action::Up,
            1 => Action::Down,
            3 => Action::Left,
            2 => Action::Right,
            _ => unreachable!(),
        }
    }
    fn end(&self, _: GameState) {}
}

impl Dumb {
    pub fn new() -> Self {
        Self {}
    }
}
