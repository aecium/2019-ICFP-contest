use std::env;
use std::fs;
use std::fmt;

struct Map {
    contour: Vec<Point>,
    squares : Vec<Vec<MapSquare>>
}
impl Map {
    fn from_map_string(map_string: &str) -> Self {

        let mut split_map = map_string.split('#');
        let contour = split_map.next().expect("Ran out of parts.")
            .trim_matches('(').trim_matches(')')
            .split("),(");
        let bot_position = split_map.next();
        let obstacles = split_map.next();
        let boosters = split_map.next();

        let mut points:Vec<Point> = Vec::new();

        let mut max_x = 0;
        let mut max_y = 0;
        for point in contour {
            let p:Vec<&str> = point.split(",").collect();
            let x = p[0].parse::<usize>().unwrap();
            let y = p[1].parse::<usize>().unwrap();
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            points.push(Point {
                x: x,
                y: y,
            })
        }

        let mut map: Vec<Vec<MapSquare>> = Vec::new();
        for _y in 0..max_x {
            let mut row = Vec::new();
            for _x in 0..max_y {
                row.push(MapSquare::OOB);
            }
            map.push(row);
        }

        Map {
            contour: points,
            squares: map,
        }
    }
}
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "contour: {:?}", self.contour)
    }
}

enum MapSquare {
    Empty { powerUp : Option<PowerUp>},
    Wrapped { powerUp : Option<PowerUp>},
    Blocked { powerUp : Option<PowerUp>},
    OOB,
}
impl MapSquare {
    fn to_char(&self) -> char {
        match self {
            //MapSquare::Empty => '.',
            //MapSquare::Wrapped => 'O',
            //MapSquare::Blocked => 'X',
            MapSquare::OOB => '~',
            _ => '.',
        }
    }
}
impl fmt::Debug for MapSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

struct Bot {
    powerups : Vec<PowerUp>

}

enum PowerUp{
    Extension, //{code: 'B'},
    Boost,// {code: 'F'},
    Drill,// {code: 'L'},
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


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Failed to read.");

    println!("{}: {}", filename, contents);

    let map = Map::from_map_string(&contents);

    println!("map: {:?}", map);

    println!("🌮 Free Tacos! 🌮");
}
