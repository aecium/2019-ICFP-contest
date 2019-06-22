use std::fmt;

#[derive(Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn offset_by(&self, offset: &Offset) -> Result<Self, String> {
        let mut x_result = self.x as i64 + offset.x;
        let mut y_result = self.y as i64 + offset.y;

        //if either are less than 0 set the offset to the origin
        if x_result < 0 || y_result < 0 {
            return Err("invalid point offset".to_string());
        }

        let x_result = x_result as usize;
        let y_result = y_result as usize;

        Ok(Point {
            x: x_result,
            y: y_result,
        })
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
