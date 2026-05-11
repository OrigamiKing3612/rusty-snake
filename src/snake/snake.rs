use std::io::stdout;

use crossterm::{ExecutableCommand, cursor, style::Stylize};

use crate::{
    game::game::Game,
    types::{Direction, Position},
};

pub(crate) struct Snake {
    pub direction: Direction,
    pub body: Vec<Position>,
}

impl Snake {
    pub fn new(start_pos: Position) -> Self {
        Snake {
            direction: Direction::Up,
            body: vec![start_pos],
        }
    }
    pub fn step(&mut self) {
        let head = self.body[0].clone();
        let new_head = match self.direction {
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

        self.body.insert(0, new_head);
        self.body.pop(); // keeps same length for now
    }

    pub fn draw(&mut self, game: &Game) {
        let mut stdout = stdout();
        for (i, segment) in self.body.iter().enumerate() {
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
    }
}
