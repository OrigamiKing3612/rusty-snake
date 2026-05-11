use std::io::stdout;

use crossterm::{ExecutableCommand, cursor};

use crate::food::make_food;
use shared::Position;

pub(crate) struct Game {
    pub width: u16,
    pub height: u16,
    pub game_height: u16,
    pub food: Vec<Position>,
    pub max_food: u16,
    pub score: u32,
}

const MAX_GAME_WINDOW: u16 = 3;

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        let max_food = width / 10 * 2;
        let mut game = Game {
            width,
            height,
            game_height: height - MAX_GAME_WINDOW,
            food: Vec::new(),
            score: 0,
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
    pub fn draw_window(&mut self) {
        let mut stdout = stdout();
        self.game_height = self.height - MAX_GAME_WINDOW;

        for x in 0..self.width {
            stdout
                .execute(cursor::MoveTo(x, self.game_height))
                .unwrap()
                .execute(crossterm::style::Print("─"))
                .unwrap()
                .execute(cursor::MoveTo(x, self.height))
                .unwrap()
                .execute(crossterm::style::Print("─"))
                .unwrap();
        }

        for y in (self.game_height + 1)..=(self.height - 1) {
            stdout
                .execute(cursor::MoveTo(0, y))
                .unwrap()
                .execute(crossterm::style::Print("│"))
                .unwrap()
                .execute(cursor::MoveTo(self.width - 1, y))
                .unwrap()
                .execute(crossterm::style::Print("│"))
                .unwrap();
        }

        stdout
            .execute(cursor::MoveTo(0, self.game_height))
            .unwrap()
            .execute(crossterm::style::Print("┌"))
            .unwrap()
            .execute(cursor::MoveTo(self.width, self.game_height))
            .unwrap()
            .execute(crossterm::style::Print("┐"))
            .unwrap()
            .execute(cursor::MoveTo(0, self.height))
            .unwrap()
            .execute(crossterm::style::Print("└"))
            .unwrap()
            .execute(cursor::MoveTo(self.width, self.height))
            .unwrap()
            .execute(crossterm::style::Print("┘"))
            .unwrap();

        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    }
    pub fn draw_score(&mut self) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveTo(1, self.game_height + 1))
            .unwrap();
        print!("Score: {}", self.score);
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    }
    pub fn increment_score(&mut self) {
        self.score += 1;
        self.draw_score();
    }
}
