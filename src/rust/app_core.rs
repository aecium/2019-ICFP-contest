use std::fmt;

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}
