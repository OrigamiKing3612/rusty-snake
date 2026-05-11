#[derive(Debug, Clone, Copy)]
pub enum Speed {
    Medium = 100,
    Fast = 50,
}

impl Speed {
    pub fn ms(self) -> u64 {
        self as u64
    }
}

pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Quit,
    SpeedBoost,
}

pub struct InputState {
    pub speed_boost: bool,
}

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
