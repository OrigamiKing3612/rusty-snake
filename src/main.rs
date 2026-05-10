use crossterm::ExecutableCommand;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, terminal};
use rand::Rng;
use std::io::{Write, stdout};
use std::{thread, time::Duration};

#[derive(Debug, Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}

struct Snake {
    direction: Direction,
    body: Vec<Position>,
}

#[derive(Debug, Clone, Copy)]
enum Speed {
    Medium = 100,
    Fast = 250,
}

impl Speed {
    fn ms(self) -> u64 {
        self as u64
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

enum Action {
    Up,
    Down,
    Left,
    Right,
    Quit,
    SpeedBoost,
}

struct Game {
    width: u16,
    height: u16,
    food: Option<Position>,
}

struct InputState {
    speed_boost: bool,
}

fn main() {
    let result = enable_raw_mode();
    if result.is_err() {
        eprintln!("Failed to enable raw mode: {:?}", result);
        return;
    }

    let mut stdout = stdout();

    let (width, height) = terminal::size().expect("could not get terminal size");
    let mut game = Game {
        width,
        height,
        food: None,
    };

    let mut snake = Snake {
        direction: Direction::Up,
        body: vec![Position {
            x: width / 2,
            y: height / 2,
        }],
    };
    let mut input = InputState { speed_boost: false };

    game.food = Some(make_food(&game));

    loop {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                input.speed_boost = key_event.modifiers.contains(KeyModifiers::SHIFT);
                if let Some(action) = key_to_action(key_event) {
                    match action {
                        Action::Up => snake.direction = Direction::Up,
                        Action::Down => snake.direction = Direction::Down,
                        Action::Left => snake.direction = Direction::Left,
                        Action::Right => snake.direction = Direction::Right,
                        Action::SpeedBoost => input.speed_boost = true,
                        Action::Quit => break,
                    }
                }
            }
        }

        step(&mut snake);

        // if snake.body[0].x >= game.width || snake.body[0].y >= game.height {
        //     break; // Game over if snake goes out of bounds
        // }

        // if snake.body[1..].iter().any(|segment| segment.x == snake.body[0].x && segment.y == snake.body[0].y) {
        //     break; // Game over if snake collides with itself
        // }

        if let Some(food) = game.food {
            if snake.body[0].x == food.x && snake.body[0].y == food.y {
                snake.body.push(snake.body[snake.body.len() - 1]);
                game.food = Some(make_food(&game));
            }
        }

        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        if let Some(food) = game.food {
            let err = stdout.execute(cursor::MoveTo(food.x, food.y));
            if err.is_ok() {
                print!("{}", "*".red());
            }
        }
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        for (i, segment) in snake.body.iter().enumerate() {
            stdout
                .execute(cursor::MoveTo(segment.x, segment.y))
                .unwrap();
            if segment.x >= game.width || segment.y >= game.height {
                continue; // skip segments that are out of bounds
            }
            if i == 0 {
                print!("{}", "█".yellow());
            } else {
                print!("{}", "█".green());
            }
        }
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

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

fn key_to_action(key: KeyEvent) -> Option<Action> {
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

fn step(snake: &mut Snake) {
    let head = snake.body[0].clone();
    let new_head = match snake.direction {
        Direction::Up => Position {
            x: head.x,
            y: head.y.saturating_sub(1),
        },
        Direction::Down => Position {
            x: head.x,
            y: head.y + 1,
        },
        Direction::Left => Position {
            x: head.x.saturating_sub(1),
            y: head.y,
        },
        Direction::Right => Position {
            x: head.x + 1,
            y: head.y,
        },
    };

    snake.body.insert(0, new_head);
    snake.body.pop(); // keeps same length for now
}

fn make_food(game: &Game) -> Position {
    let mut rng = rand::rng();

    let x = rng.random_range(0..game.width);
    let y = rng.random_range(0..game.height);

    if let Some(food) = game.food {
        if food.x == x && food.y == y {
            return make_food(game); // avoid placing food on top of existing food
        }
    }

    return Position { x, y };
}
