use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPacket {}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Input { direction: Direction },

    Join { name: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Welcome { id: u32 },
    State { players: Vec<PlayerState> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerState {
    pub id: u32,
    pub x: i32,
    pub y: i32,
}
