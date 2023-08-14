use crate::engine::{Engine, Informative};

mod common;
pub mod engine;
pub mod server;

pub trait BattleSnake: Engine + Informative {}

impl<E> BattleSnake for E where E: Engine + Informative {}
