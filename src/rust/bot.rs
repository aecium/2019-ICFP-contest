use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::app_core::{Direction, Point};

pub struct Bot {
    pub position: Point,
    pub facing: Direction,
    pub extension: usize,
    pub boost: usize,
    pub drill: usize,
    //pub mysterious_point: usize,
    pub teleports: usize,
    pub manipulators: Vec<Point>,
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
            Action::Up => 'W',
            Action::Right => 'D',
            Action::Down => 'S',
            Action::Left => 'A',
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
