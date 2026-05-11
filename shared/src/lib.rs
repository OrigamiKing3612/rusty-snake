use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPacket {}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}
