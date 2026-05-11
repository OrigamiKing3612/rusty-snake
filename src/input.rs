use crossterm::event::{KeyCode, KeyEvent};

use crate::types::Action;

pub fn key_to_action(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('w'), _) => Some(Action::Up),
        (KeyCode::Up, _) => Some(Action::Up),
        (KeyCode::Char('k'), _) => Some(Action::Up),

        (KeyCode::Char('a'), _) => Some(Action::Left),
        (KeyCode::Left, _) => Some(Action::Left),
        (KeyCode::Char('h'), _) => Some(Action::Left),

        (KeyCode::Char('s'), _) => Some(Action::Down),
        (KeyCode::Down, _) => Some(Action::Down),
        (KeyCode::Char('j'), _) => Some(Action::Down),

        (KeyCode::Char('d'), _) => Some(Action::Right),
        (KeyCode::Right, _) => Some(Action::Right),
        (KeyCode::Char('l'), _) => Some(Action::Right),

        (KeyCode::Char('q'), _) => Some(Action::Quit),
        (KeyCode::Esc, _) => Some(Action::Quit),
        (KeyCode::Char(' '), _) => Some(Action::SpeedBoost),
        _ => None,
    }
}
