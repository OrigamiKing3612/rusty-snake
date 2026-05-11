use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event},
    style::{Attribute, PrintStyledContent, Stylize},
};

use crate::{
    game::game::Game,
    menu::{
        menu::{Action, draw_window},
        types::MenuOption,
    },
};

pub fn draw_game_over(game: &mut Game, message: &str) {
    draw_window(game);
    draw_game_over_text(game, message);
    loop {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    _ => break,
                }
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

pub fn draw_game_over_text(game: &Game, message: &str) {
    let mut stdout = stdout();

    let left = game.width / 4;
    let right = game.width * 3 / 4;

    let top = game.height / 4;
    let bottom = game.height * 3 / 4;

    let y = top + ((bottom - top) / 2);
    let x = left + ((right - left) / 2);

    stdout.execute(cursor::MoveTo(x, y)).unwrap();

    stdout
        .execute(PrintStyledContent(
            format!("{}", message).attribute(Attribute::Bold),
        ))
        .unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    stdout.flush().unwrap();
}
