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

impl Point{
    fn offset_by(&self,offset: &Offset) -> Self {
        
        Point{x:0,y:0}
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct Offset {
    pub x: i64,
    pub y: i64,
}

impl fmt::Debug for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Offset {{ x: {}, y: {} }}", self.x, self.y)
    }
}