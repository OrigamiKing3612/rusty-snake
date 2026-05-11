mod food;
mod game;
mod input;
mod menu;
mod snake;
mod types;

use crossterm::terminal;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::game::game::Game;
use crate::snake::snake::Snake;
use crate::types::Position;

fn main() {
    let result = enable_raw_mode();
    if result.is_err() {
        eprintln!("Failed to enable raw mode: {:?}", result);
        return;
    }

    let (width, height) = terminal::size().expect("could not get terminal size");
    let mut game = Game::new(width, height);

    let mut snake = Snake::new(Position {
        x: width / 2,
        y: height / 2,
    });

    let option = menu::menu::draw_main_menu(&mut game);
    match option {
        menu::types::MenuOption::StartGame => {
            game.game_loop(&mut snake);
        }
        menu::types::MenuOption::Settings => {}
        menu::types::MenuOption::Quit => {
            quit();
            return;
        }
    }

    quit();
}

fn quit() {
    let result = disable_raw_mode();
    if result.is_err() {
        eprintln!("Failed to disable raw mode: {:?}", result);
    }
}
