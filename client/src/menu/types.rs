#[derive(PartialEq)]
pub enum MenuOption {
    StartGame,
    JoinGame,
    Settings,
    Quit,
}

impl MenuOption {
    pub fn label(&self) -> &'static str {
        match self {
            MenuOption::StartGame => "Start Game",
            MenuOption::JoinGame => "Join Game",
            MenuOption::Settings => "Settings",
            MenuOption::Quit => "Quit",
        }
    }
    pub fn all() -> [MenuOption; 4] {
        return [
            MenuOption::StartGame,
            MenuOption::JoinGame,
            MenuOption::Settings,
            MenuOption::Quit,
        ];
    }
    pub fn next(&self) -> MenuOption {
        match self {
            MenuOption::StartGame => MenuOption::JoinGame,
            MenuOption::JoinGame => MenuOption::Settings,
            MenuOption::Settings => MenuOption::Quit,
            MenuOption::Quit => MenuOption::StartGame,
        }
    }
    pub fn previous(&self) -> MenuOption {
        match self {
            MenuOption::StartGame => MenuOption::Quit,
            MenuOption::JoinGame => MenuOption::StartGame,
            MenuOption::Settings => MenuOption::JoinGame,
            MenuOption::Quit => MenuOption::JoinGame,
        }
    }
}
