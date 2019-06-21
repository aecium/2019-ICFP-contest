use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::app_core::{Direction, Point};
use crate::powerups::PowerUp;

pub struct Bot {
    pub powerups: Vec<PowerUp>,
    pub position: Point,
}

impl Bot {
    pub fn move_self(&mut self, direction: &Direction) {
        let position = &self.position;
        match direction {
            Direction::North => {
                self.position = Point {
                    x: position.x,
                    y: position.y + 1,
                }
            }
            Direction::East => {
                self.position = Point {
                    x: position.x + 1,
                    y: position.y,
                }
            }
            Direction::South => {
                self.position = Point {
                    x: position.x,
                    y: position.y - 1,
                }
            }
            Direction::West => {
                self.position = Point {
                    x: position.x - 1,
                    y: position.y,
                }
            }
        }
    }
}

pub trait ToChar {
    fn to_char(&self) -> char;
}

pub enum Action {
    Up,
    Right,
    Down,
    Left,
    Nop,
    RotClock,
    RotAnticlock,
    //Attach { dx:u8, dy:u8},
    Boost,
    Drill,
    Reset,
    //Shift {dest: &Point},
}

impl ToChar for Action {
    fn to_char(&self) -> char {
        match self {
            Up => 'W',
            Right => 'D',
            Down => 'S',
            Left => 'A',
            _ => panic!("unknown output char"),
        }
    }
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0, 4) {
            0 => Action::Up,
            1 => Action::Right,
            2 => Action::Down,
            _ => Action::Left,
        }
    }
}
