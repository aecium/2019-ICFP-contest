use std::env;
use std::fs;
use std::fmt;

struct Map {

}

struct Bot {
    powerups : Vec<PowerUp>
}

enum PowerUp{
    Extension, //{code: 'B'},
    Boost,// {code: 'F'},
    Drill,// {code: 'L'},
}
 
pub trait ByCode {
    fn by_code(code: char) -> Self;
}

impl ByCode for PowerUp{
    fn by_code(code: char) -> Self {
        match code {
            'B' => PowerUp::Extension,
            'F' => PowerUp::Boost,
            'L' => PowerUp::Drill,
            _ => panic!("Unknown powerup code"),
        }
    }
}

struct Point {
    x: usize,
    y: usize,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Failed to read.");

    println!("{}: {}", filename, contents);

    let parts:Vec<&str> = contents.split("#").collect();
    let contour:Vec<&str> = parts[0]
        .trim_matches('(')
        .trim_matches(')')
        .split("),(")
        .collect();
    let mut points:Vec<Point> = Vec::new();
    for point in &contour {
        let p:Vec<&str> = point.split(",").collect();
        points.push(Point {
            x: p[0].parse::<usize>().unwrap(),
            y: p[1].parse::<usize>().unwrap(),
        })
    }

    println!("contour: {:?}", contour);
    println!("points: {:?}", points);


    println!("🌮 Free Tacos! 🌮");
}
