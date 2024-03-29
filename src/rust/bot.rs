use crate::app_core::{Direction, Offset, Point, Rotation};

#[derive(Clone)]
pub struct Bot {
    pub position: Point,
    pub facing: Direction,
    pub extension: usize,
    pub boost: usize,
    pub drill: usize,
    //pub mysterious_point: usize,
    pub teleports: usize,
    pub manipulators: Vec<Offset>,
}

impl Bot {
    pub fn new(pos: Point, dir: Direction) -> Self {
        Bot {
            position: pos,
            facing: dir,
            extension: 0,
            boost: 0,
            drill: 0,
            teleports: 0,
            manipulators: vec![
                Offset { x: 0, y: 0 },
                Offset { x: 1, y: 0 },
                Offset { x: 1, y: 1 },
                Offset { x: 1, y: -1 },
            ],
        }
    }

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

    pub fn rot(&mut self, dir : &Rotation) {
        let manip_list :&Vec<Offset> = &self.manipulators;
        match dir {
            Rotation::Clockwise => {
                self.manipulators = manip_list.into_iter().map(|old_offset| Offset{x: old_offset.y, y: old_offset.x * -1}).collect::<Vec<_>>();
            }
            Rotation::AntiClockwise => {
                self.manipulators = manip_list.into_iter().map(|old_offset| Offset{x: old_offset.y * -1, y: old_offset.x}).collect::<Vec<_>>();
            }
        }
    }
}

pub trait ToChar {
    fn to_char(&self) -> char;
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Action {
    Start,
    Up,
    Right,
    Down,
    Left,
    Nop,
    RotClock,
    RotAnticlock,
    Attach { dx: u8, dy: u8 },
    Boost,
    Drill,
    Reset,
    DropBeacon,
    Shift { x: usize, y: usize },
}

impl ToChar for Action {
    fn to_char(&self) -> char {
        match self {
            Action::Up => 'W',
            Action::Right => 'D',
            Action::Down => 'S',
            Action::Left => 'A',
            Action::RotClock => 'E',
            Action::RotAnticlock => 'Q',
            _ => panic!("unknown output char"),
        }
    }
}
