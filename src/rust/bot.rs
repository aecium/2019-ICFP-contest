use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::app_core::{Direction, Point, Offset};

pub struct Bot {
    pub Position: Point,
    pub Facing: Direction,
    pub Extension: usize,
    pub Boost: usize,
    pub Drill: usize,
    //pub Mysterious_Point: usize,
    pub Teleports: usize,
    pub Manipulators: Vec<Offset>,
}

impl Bot {
    pub fn new(tPosition: Point, tDirection: Direction)->Self{
        Bot{Position: tPosition,
        Facing: tDirection,
        Extension: 0,
        Boost: 0,
        Drill: 0,
        Teleports: 0,
        Manipulators: vec![Offset{x:0,y:0},Offset{x:1,y:0},Offset{x:1,y:1},Offset{x:1,y:-1}], 
        }
    }

    pub fn move_self(&mut self, direction: &Direction) {
        let position = &self.Position;
        match direction {
            Direction::North => {
                self.Position = Point {
                    x: position.x,
                    y: position.y + 1,
                }
            }
            Direction::East => {
                self.Position = Point {
                    x: position.x + 1,
                    y: position.y,
                }
            }
            Direction::South => {
                self.Position = Point {
                    x: position.x,
                    y: position.y - 1,
                }
            }
            Direction::West => {
                self.Position = Point {
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
