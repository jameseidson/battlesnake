use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    pub height: u32,
    pub width: u32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Snake>,
    pub hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: u32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: u32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize)]
pub struct SnakeInfo {
    apiversion: String,
    author: String,
    color: Option<String>,
    head: Option<String>,
    tail: Option<String>,
    version: String,
}

impl Default for SnakeInfo {
    fn default() -> Self {
        Self {
            apiversion: String::from("1"),
            author: String::from("jameseidson"),
            version: String::from(env!("CARGO_PKG_VERSION")),
            color: None,
            head: None,
            tail: None,
        }
    }
}

impl SnakeInfo {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn color(&mut self, color: String) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn head(&mut self, head: String) -> &mut Self {
        self.head = Some(head);
        self
    }

    pub fn tail(&mut self, tail: String) -> &mut Self {
        self.tail = Some(tail);
        self
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Action {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => return Err(()),
        })
    }
}

impl From<Action> for u8 {
    fn from(value: Action) -> Self {
        match value {
            Action::Up => 0,
            Action::Down => 1,
            Action::Left => 2,
            Action::Right => 3,
        }
    }
}
