use std::io::stdout;

use crossterm::{ExecutableCommand, cursor, style::Stylize};
use rand::Rng;

use crate::{game::Game, types::Position};

pub fn make_food(game: &Game) -> Position {
    let mut rng = rand::rng();

    loop {
        let x = rng.random_range(0..game.width);
        let y = rng.random_range(0..game.game_height);

        let pos = Position { x, y };
        let on_food = game.food.iter().any(|f| f.x == x && f.y == y);
        if !on_food {
            return pos;
        }
    }
}
pub fn check_food(game: &mut Game) {
    while game.food.len() < game.max_food as usize {
        let food = make_food(&game);
        game.add_food(food);
    }
}

pub fn draw(game: &Game) {
    let mut stdout = stdout();
    for food in &game.food {
        stdout.execute(cursor::MoveTo(food.x, food.y)).unwrap();

        print!("{}", "*".red());
    }
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}
