use std::time::Duration;
use std::{
    io::{Write, stdout},
    thread,
};

use crossterm::event::Event;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, cursor, event};

use crate::game::game::Game;
use crate::snake::snake::Snake;
use crate::types::{Action, InputState, Speed};
use crate::{food, input};
use shared::Direction;

impl Game {
    pub fn game_loop(&mut self, snake: &mut Snake) {
        let mut stdout = stdout();
        let mut input = InputState { speed_boost: false };

        loop {
            if event::poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if let Some(action) = input::key_to_action(key_event) {
                        match action {
                            Action::Up => {
                                if snake.direction != Direction::Down {
                                    snake.direction = Direction::Up
                                }
                            }
                            Action::Down => {
                                if snake.direction != Direction::Up {
                                    snake.direction = Direction::Down
                                }
                            }
                            Action::Left => {
                                if snake.direction != Direction::Right {
                                    snake.direction = Direction::Left
                                }
                            }
                            Action::Right => {
                                if snake.direction != Direction::Left {
                                    snake.direction = Direction::Right
                                }
                            }
                            Action::SpeedBoost => input.speed_boost = !input.speed_boost,
                            Action::Quit => break,
                        }
                    }
                }
            }

            food::check_food(self);
            snake.step();

            if snake.body[0].x >= self.width || snake.body[0].y >= self.game_height {
                println!("Game Over! Final Score: {}", self.score);
                break; // Game over if snake goes out of bounds
            }

            if snake.body[1..]
                .iter()
                .any(|segment| segment.x == snake.body[0].x && segment.y == snake.body[0].y)
            {
                println!("Game Over! Final Score: {}", self.score);
                break; // Game over if snake collides with itself
            }

            if let Some(index) = self
                .food
                .iter()
                .position(|food| food.x == snake.body[0].x && food.y == snake.body[0].y)
            {
                let tail = *snake.body.last().unwrap();
                snake.body.push(tail);
                self.food.remove(index);
                let new_food = food::make_food(&self);
                self.add_food(new_food);
                self.increment_score();
            }

            stdout.execute(Clear(ClearType::All)).unwrap();
            stdout.execute(cursor::MoveTo(0, 0)).unwrap();

            food::draw(&self);

            snake.draw(&self);
            self.draw_window();
            self.draw_score();

            stdout.flush().unwrap();
            let delay = if input.speed_boost {
                Speed::Fast.ms()
            } else {
                Speed::Medium.ms()
            };

            thread::sleep(Duration::from_millis(delay));
        }
    }
}
