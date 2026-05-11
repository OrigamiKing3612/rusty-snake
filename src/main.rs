mod food;
mod game;
mod input;
mod snake;
mod types;

use crossterm::ExecutableCommand;
use crossterm::event::Event;
use crossterm::terminal::{Clear, ClearType};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, terminal};
use std::io::{Write, stdout};
use std::{thread, time::Duration};

use crate::game::Game;
use crate::snake::Snake;
use crate::types::{Action, Direction, InputState, Position, Speed};

fn main() {
    let result = enable_raw_mode();
    if result.is_err() {
        eprintln!("Failed to enable raw mode: {:?}", result);
        return;
    }

    let mut stdout = stdout();

    let (width, height) = terminal::size().expect("could not get terminal size");
    let mut game = Game::new(width, height);

    let mut snake = Snake::new(Position {
        x: width / 2,
        y: height / 2,
    });

    let mut input = InputState { speed_boost: false };

    loop {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let Some(action) = input::key_to_action(key_event) {
                    match action {
                        Action::Up => snake.direction = Direction::Up,
                        Action::Down => snake.direction = Direction::Down,
                        Action::Left => snake.direction = Direction::Left,
                        Action::Right => snake.direction = Direction::Right,
                        Action::SpeedBoost => input.speed_boost = !input.speed_boost,
                        Action::Quit => break,
                    }
                }
            }
        }

        food::check_food(&mut game);
        snake.step();

        // if snake.body[0].x >= game.width || snake.body[0].y >= game.game_height {
        //     break; // Game over if snake goes out of bounds
        // }

        // if snake.body[1..].iter().any(|segment| segment.x == snake.body[0].x && segment.y == snake.body[0].y) {
        //     break; // Game over if snake collides with itself
        // }

        if let Some(index) = game
            .food
            .iter()
            .position(|food| food.x == snake.body[0].x && food.y == snake.body[0].y)
        {
            let tail = *snake.body.last().unwrap();
            snake.body.push(tail);
            game.food.remove(index);
            let new_food = food::make_food(&game);
            game.add_food(new_food);
            game.increment_score();
        }

        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        food::draw(&game);

        snake.draw(&game);
        game.draw_window();
        game.draw_score();

        stdout.flush().unwrap();
        let delay = if input.speed_boost {
            Speed::Fast.ms()
        } else {
            Speed::Medium.ms()
        };

        thread::sleep(Duration::from_millis(delay));
    }
    let result = disable_raw_mode();
    if result.is_err() {
        eprintln!("Failed to disable raw mode: {:?}", result);
    }
}
