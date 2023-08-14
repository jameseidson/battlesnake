use crate::common::{Action, GameState, SnakeInfo};
use crate::engine::{Engine, Informative};
use rand::{self, Rng};

#[derive(Clone)]
pub struct Dumb;

impl Engine for Dumb {
    fn start(&self, _: &GameState) {}
    fn next(&self, _: &GameState) -> Action {
        (rand::thread_rng().gen_range(0..=3) as u8)
            .try_into()
            .unwrap()
    }
    fn end(&self, _: &GameState) {}
}

impl Informative for Dumb {
    fn info(&self) -> SnakeInfo {
        SnakeInfo::default()
    }
}

impl Dumb {
    pub fn new() -> Self {
        Self {}
    }
}
