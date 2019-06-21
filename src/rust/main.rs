use std::cmp;
use std::env;
use std::fmt;
use std::fs;

struct Map {
    contour: Vec<Point>,
    squares: Vec<Vec<MapSquare>>,
}
impl Map {
    fn from_map_string(map_string: &str) -> Self {
        let mut split_map = map_string.split('#');
        let contour = split_map
            .next()
            .expect("Ran out of parts.")
            .trim_matches('(')
            .trim_matches(')')
            .split("),(");
        let bot_position = split_map.next();
        let obstacles = split_map.next();
        let boosters = split_map.next();

        let mut points: Vec<Point> = Vec::new();

        let mut max_x = 0;
        let mut max_y = 0;
        for point in contour {
            let p: Vec<&str> = point.split(",").collect();
            let x = p[0].parse::<usize>().unwrap();
            let y = p[1].parse::<usize>().unwrap();
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            points.push(Point { x: x, y: y })
        }

        let mut map: Vec<Vec<MapSquare>> = Vec::new();
        for _y in 0..max_y + 1 {
            let mut row = Vec::new();
            for _x in 0..max_x + 1 {
                row.push(MapSquare::OOB);
            }
            map.push(row);
        }

        let mut last = Point { x: 0, y: 0 };
        let mut first = true;
        let mut ps = points.to_vec();
        ps.push(points[0].clone());
        for point in ps {
            map[point.y][point.x] = MapSquare::Empty { power_up: None };
            if first {
                first = false;
            } else {
                for x in cmp::min(last.x, point.x)..=cmp::max(last.x, point.x) {
                    for y in cmp::min(last.y, point.y)..=cmp::max(last.y, point.y) {
                        println!("{}, {}", x, y);
                        map[y][x] = MapSquare::Empty { power_up: None };
                    }
                }
            }
            last = Point {
                x: point.x,
                y: point.y,
            };
        }

        Map {
            contour: points,
            squares: map,
        }
    }
    fn find_neighbors(
        &self,
        pos: &Point,
    ) -> (
        Option<&MapSquare>,
        Option<&MapSquare>,
        Option<&MapSquare>,
        Option<&MapSquare>,
        &MapSquare,
    ) // (north, east, south, west, center)
    {
        let squares = &self.squares;

        //let mySquare = &self.squares.get(index: I)[pos.x]
        let my_square = match squares.get(pos.y) {
            Some(x) => match x.get(pos.y) {
                Some(square) => square,
                _ => panic!("invalid"),
            },
            _ => panic!("invalid"),
        };

        let north = squares.get(pos.y + 1).and_then(|row| row.get(pos.x));
        let east = squares.get(pos.y).and_then(|row| row.get(pos.x + 1));
        let south = squares.get(pos.y - 1).and_then(|row| row.get(pos.x));
        let west = squares.get(pos.y).and_then(|row| row.get(pos.x - 1));

        return (north, east, south, west, my_square);
    }

    fn is_complete(&self) -> bool {
        return false;
    }
}
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map: Vec<String> = Vec::new();
        for y in 0..self.squares.len() {
            let row = &self.squares[y];
            let mut cols: Vec<char> = Vec::new();
            for x in 0..row.len() {
                cols.push(row[x].to_char());
            }
            let s: String = cols.into_iter().collect();
            map.push(s);
        }
        write!(f, "contour: {:?}", self.contour);
        write!(f, "map:\n{}", map.join("\n"))
    }
}

enum MapSquare {
    Empty { power_up: Option<PowerUp> },
    Wrapped { power_up: Option<PowerUp> },
    Blocked { power_up: Option<PowerUp> },
    OOB,
}
impl MapSquare {
    fn to_char(&self) -> char {
        match self {
            //MapSquare::Empty{  } => '.',
            //MapSquare::Wrapped => 'O',
            //MapSquare::Blocked => 'X',
            MapSquare::OOB => '#',
            _ => '.',
        }
    }
}
impl fmt::Debug for MapSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct Bot {
    powerups: Vec<PowerUp>,
    position: Point,
}

pub trait ByCode {
    fn by_code(code: char) -> Self;
}
enum PowerUp {
    Extension, //{code: 'B'},
    Boost,     // {code: 'F'},
    Drill,     // {code: 'L'},
}

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl ByCode for PowerUp {
    fn by_code(code: char) -> Self {
        match code {
            'B' => PowerUp::Extension,
            'F' => PowerUp::Boost,
            'L' => PowerUp::Drill,
            _ => panic!("Unknown powerup code"),
        }
    }
}
pub trait ToChar {
    fn to_char(&self) -> char;
}
enum Action {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read.");

    println!("{}: {}", filename, contents);

    let map = Map::from_map_string(&contents);

    println!("map: {:?}", map);

    println!("🌮 Free Tacos! 🌮");
}

fn find_path(bot: &mut Bot, map: &mut Map) {
    let path: Vec<char> = Vec::new();
    while (!map.is_complete()) {
        let (north, east, south, west, my_square) = map.find_neighbors(&bot.position);
    }
}
