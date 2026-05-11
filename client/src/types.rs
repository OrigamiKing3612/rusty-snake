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
