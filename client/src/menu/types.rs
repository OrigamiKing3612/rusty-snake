#[derive(PartialEq)]
pub enum MenuOption {
    StartGame,
    Settings,
    Quit,
}

impl MenuOption {
    pub fn label(&self) -> &'static str {
        match self {
            MenuOption::StartGame => "Start Game",
            MenuOption::Settings => "Settings",
            MenuOption::Quit => "Quit",
        }
    }
    pub fn all() -> [MenuOption; 3] {
        return [
            MenuOption::StartGame,
            MenuOption::Settings,
            MenuOption::Quit,
        ];
    }
    pub fn next(&self) -> MenuOption {
        match self {
            MenuOption::StartGame => MenuOption::Settings,
            MenuOption::Settings => MenuOption::Quit,
            MenuOption::Quit => MenuOption::StartGame,
        }
    }
    pub fn previous(&self) -> MenuOption {
        match self {
            MenuOption::StartGame => MenuOption::Quit,
            MenuOption::Settings => MenuOption::StartGame,
            MenuOption::Quit => MenuOption::Settings,
        }
    }
}
