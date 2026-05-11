use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{Attribute, Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};

use crate::{game::game::Game, menu::types::MenuOption};

pub enum Action {
    Up,
    Down,
    Quit,
    Enter,
}

pub fn draw_main_menu(game: &mut Game) -> MenuOption {
    let mut selected_option: MenuOption = MenuOption::StartGame;
    draw_window(game);
    loop {
        draw_menu_options(game, &selected_option);
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let Some(action) = key_to_action(key_event) {
                    match action {
                        Action::Up => selected_option = selected_option.previous(),
                        Action::Down => selected_option = selected_option.next(),
                        Action::Quit => break,
                        Action::Enter => return selected_option,
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
    return MenuOption::Quit;
}

pub fn key_to_action(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('w'), _) => Some(Action::Up),
        (KeyCode::Up, _) => Some(Action::Up),
        (KeyCode::Char('k'), _) => Some(Action::Up),

        (KeyCode::Char('a'), _) => Some(Action::Up),
        (KeyCode::Left, _) => Some(Action::Up),
        (KeyCode::Char('h'), _) => Some(Action::Up),

        (KeyCode::Char('s'), _) => Some(Action::Down),
        (KeyCode::Down, _) => Some(Action::Down),
        (KeyCode::Char('j'), _) => Some(Action::Down),

        (KeyCode::Char('d'), _) => Some(Action::Down),
        (KeyCode::Right, _) => Some(Action::Down),
        (KeyCode::Char('l'), _) => Some(Action::Down),

        (KeyCode::Char('q'), _) => Some(Action::Quit),
        (KeyCode::Enter, _) => Some(Action::Enter),

        _ => None,
    }
}

pub fn draw_window(game: &Game) {
    let mut stdout = stdout();

    let left = game.width / 4;

    let right = game.width * 3 / 4;

    let top = game.height / 4;

    let bottom = game.height * 3 / 4;

    stdout.execute(Clear(ClearType::All)).unwrap();

    for x in left + 1..right {
        stdout
            .execute(cursor::MoveTo(x, top))
            .unwrap()
            .execute(Print("─"))
            .unwrap();

        stdout
            .execute(cursor::MoveTo(x, bottom))
            .unwrap()
            .execute(Print("─"))
            .unwrap();
    }

    for y in top + 1..bottom {
        stdout
            .execute(cursor::MoveTo(left, y))
            .unwrap()
            .execute(Print("│"))
            .unwrap();

        stdout
            .execute(cursor::MoveTo(right, y))
            .unwrap()
            .execute(Print("│"))
            .unwrap();
    }

    stdout
        .execute(cursor::MoveTo(left, top))
        .unwrap()
        .execute(Print("┌"))
        .unwrap();

    stdout
        .execute(cursor::MoveTo(right, top))
        .unwrap()
        .execute(Print("┐"))
        .unwrap();

    stdout
        .execute(cursor::MoveTo(left, bottom))
        .unwrap()
        .execute(Print("└"))
        .unwrap();

    stdout
        .execute(cursor::MoveTo(right, bottom))
        .unwrap()
        .execute(Print("┘"))
        .unwrap();

    let title = " Rusty Snake ";

    let title_x = left + ((right - left) / 2) - (title.len() as u16 / 2);

    stdout
        .execute(cursor::MoveTo(title_x, top))
        .unwrap()
        .execute(PrintStyledContent(title.bold().with(Color::Green)))
        .unwrap();

    stdout.flush().unwrap();
}

pub fn draw_menu_options(game: &Game, selected_option: &MenuOption) {
    let mut stdout = stdout();

    let left = game.width / 4;

    let right = game.width * 3 / 4;

    let top = game.height / 4;

    let bottom = game.height * 3 / 4;

    let options = MenuOption::all();

    let menu_start_y = top + ((bottom - top) / 2) - (options.len() as u16 / 2);

    for (i, option) in options.iter().enumerate() {
        let label = option.label();

        let x = left + ((right - left) / 2) - (label.len() as u16 / 2);

        let y = menu_start_y + i as u16;

        stdout.execute(cursor::MoveTo(x, y)).unwrap();

        if option == selected_option {
            stdout
                .execute(PrintStyledContent(
                    format!("{}", label).attribute(Attribute::Bold),
                ))
                .unwrap();
        } else {
            stdout
                .execute(PrintStyledContent(label.dark_grey()))
                .unwrap();
        }
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    }

    stdout.flush().unwrap();
}
