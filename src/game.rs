use rand::Rng;

use crate::{snake::Snake, types::Position};

pub struct Game {
    pub width: u16,
    pub height: u16,
    pub food: Vec<Position>,
}

impl Game {
    pub fn make_food(&mut self, snake: &Snake) -> Position {
        let mut rng = rand::rng();

        loop {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);

            let pos = Position { x, y };
            let on_snake = snake.body.iter().any(|p| p.x == x && p.y == y);
            let on_food = self.food.iter().any(|f| f.x == x && f.y == y);
            if !on_snake && !on_food {
                return pos;
            }
        }
    }
}
