use crate::{food::make_food, types::Position};

pub struct Game {
    pub width: u16,
    pub height: u16,
    pub food: Vec<Position>,
    pub max_food: u16,
    pub score: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        let max_food = width / 10 * 2;
        let mut game = Game {
            width,
            height,
            food: Vec::new(),
            score: 0,
            game_over: false,
            max_food: max_food,
        };
        while game.food.len() < max_food as usize {
            let food = make_food(&game);
            game.food.push(food);
        }
        return game;
    }
    pub fn add_food(&mut self, pos: Position) {
        self.food.push(pos);
    }
}
