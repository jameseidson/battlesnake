use super::Engine;
use crate::{Direction, GameState};
use rand::{self, Rng};

#[derive(Clone)]
pub struct Dumb;

impl Engine for Dumb {
    fn start(&self, _: GameState) {}
    fn next(&self, _: GameState) -> Direction {
        match rand::thread_rng().gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            3 => Direction::Left,
            2 => Direction::Right,
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
