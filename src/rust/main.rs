use std::cmp;
use std::env;
use std::fmt;
use std::fs;

mod app_core;
use app_core::{Direction, Point};

mod powerups;
use powerups::PowerUp;

mod bot;
use bot::*;

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
        let bot_position: Vec<&str> = split_map.next()
            .expect("Ran out of parts.")
            .trim_matches('(')
            .trim_matches(')')
            .split(",").collect();
        let bot_position = Point {
            x:bot_position[0].parse::<usize>().unwrap(),
            y:bot_position[1].parse::<usize>().unwrap(),
        };
        let obstacles = split_map.next().expect("Need more tacos.")
            .split(";");
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

        Map::contour_map(&mut map, &mut points, MapSquare::Empty { power_up: None });

        for obstacle in obstacles {
            if obstacle.len() > 0 {
                let obstacle = obstacle.trim_matches('(').trim_matches(')').split("),(");
                let mut points: Vec<Point> = Vec::new();
                for point in obstacle {
                    let p: Vec<&str> = point.split(",").collect();
                    let x = p[0].parse::<usize>().unwrap();
                    let y = p[1].parse::<usize>().unwrap();
                    points.push(Point { x: x, y: y })
                }
                Map::contour_map(&mut map, &mut points, MapSquare::Blocked);
            }
        }

        let mut done: Vec<Vec<bool>> = Vec::new();
        for _y in 0..max_y + 1 {
            let mut row = Vec::new();
            for _x in 0..max_x + 1 {
                row.push(false);
            }
            done.push(row);
        }
        let mut todo: Vec<Point> = Vec::new();
        todo.push(bot_position);
        while todo.len() > 0 {
            let point = todo.pop().unwrap();
            let x = point.x;
            let y = point.y;
            let mt = MapSquare::Empty { power_up: None };
            let oob = MapSquare::OOB;
            map[y][x] = MapSquare::Empty { power_up: None };
            done[y][x] = true;
            if x > 0 && !done[y][x-1] && (map[y][x-1]==mt || map[y][x-1]==oob) {
                todo.push(Point { x: x-1, y: y });
            }
            if x < max_x && !done[y][x+1] && (map[y][x+1]==mt || map[y][x+1]==oob) {
                todo.push(Point { x: x+1, y: y });
            }
            if y > 0 && !done[y-1][x] && (map[y-1][x]==mt || map[y-1][x]==oob) {
                todo.push(Point { x: x, y: y-1 });
            }
            if y < max_y && !done[y+1][x] && (map[y+1][x]==mt || map[y+1][x]==oob) {
                todo.push(Point { x: x, y: y+1 });
            }
            //println!("{}, {}, {:?}", x, y, todo);
            //println!("{:?}", done);
        }

        Map {
            contour: points,
            squares: map,
        }
    }

    fn contour_map(map: &mut Vec<Vec<MapSquare>>, points: &mut Vec<Point>, square: MapSquare) {
        let mut last = Point { x: 0, y: 0 };
        let mut first = true;
        let mut ps = points.to_vec();
        let mt = MapSquare::Empty { power_up: None };
        let oob = MapSquare::OOB;
        let h = map.len()-1;
        let w = map[0].len()-1;
        ps.push(points[0].clone());
        for point in ps {
            if first {
                first = false;
            } else {
                let right = last.x < point.x;
                let left = last.x > point.x;
                let up = last.y < point.y;
                let down = last.y > point.y;
                let mut min_x = cmp::min(last.x, point.x);
                let mut max_x = cmp::max(last.x, point.x);
                let mut min_y = cmp::min(last.y, point.y);
                let mut max_y = cmp::max(last.y, point.y);
                if up || down {
                    max_y -= 1;
                    if up {
                        min_x -= 1;
                        max_x -= 1;
                    }
                }
                if left || right {
                    max_x -= 1;
                    if left {
                        min_y -= 1;
                        max_y -= 1;
                    }
                }
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        map[y][x] = square.clone();
                        if square==mt {
                            if up && x<w && map[y][x+1]==oob {
                                map[y][x+1] = MapSquare::OOB2;
                                if y>0 && map[y-1][x+1]!=mt {
                                    map[y-1][x+1] = MapSquare::OOB2;
                                }
                                if y < h && map[y+1][x+1]!=mt {
                                    map[y+1][x+1] = MapSquare::OOB2;
                                }
                            }
                            if down && x>0 && map[y][x-1]==oob {
                                map[y][x-1] = MapSquare::OOB2;
                                if y>0 && map[y-1][x-1]!=mt {
                                    map[y-1][x-1] = MapSquare::OOB2;
                                }
                                if y < h && map[y+1][x-1]!=mt {
                                    map[y+1][x-1] = MapSquare::OOB2;
                                }
                            }
                            if left && y<h && map[y+1][x]==oob {
                                map[y+1][x] = MapSquare::OOB2;
                                if x>0 && map[y+1][x-1]!=mt {
                                    map[y+1][x-1] = MapSquare::OOB2;
                                }
                                if x < w && map[y+1][x+1]!=mt {
                                    map[y+1][x+1] = MapSquare::OOB2;
                                }
                            }
                            if right && y>0 && map[y-1][x]==oob {
                                map[y-1][x] = MapSquare::OOB2;
                                if x>0 && map[y-1][x-1]!=mt {
                                    map[y-1][x-1] = MapSquare::OOB2;
                                }
                                if x < w && map[y-1][x+1]!=mt {
                                    map[y-1][x+1] = MapSquare::OOB2;
                                }
                            }
                        }
                    }
                }
            }
            last = point.clone();
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
        for y in (0..self.squares.len()).rev() {
            let row = &self.squares[y];
            let mut cols: Vec<char> = Vec::new();
            for x in 0..row.len() {
                cols.push(row[x].to_char());
            }
            let s: String = cols.into_iter().collect();
            map.push(s);
        }
        write!(f, "map:\n{}", map.join("\n"))
    }
}

#[derive(Clone, PartialEq)]
enum MapSquare {
    Empty { power_up: Option<PowerUp> },
    Wrapped { power_up: Option<PowerUp> },
    Blocked,
    OOB,
    OOB2,
}
impl MapSquare {
    fn to_char(&self) -> char {
        match self {
            //MapSquare::Empty{  } => '.',
            //MapSquare::Wrapped => 'O',
            MapSquare::Blocked => '~',
            MapSquare::OOB => '.',
            MapSquare::OOB2 => ',',
            _ => '#',
        }
    }
}
impl fmt::Debug for MapSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
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

fn find_path(bot: &mut Bot, map: &mut Map) -> String {
    let mut solution: Vec<char> = Vec::new();
    while !map.is_complete() {
        let neighbors = map.find_neighbors(&bot.position);
        //let (north, east, south, west, my_square) = map.find_neighbors(&bot.position);
        let mut action: Action = rand::random();

        while !action_is_valid(&action, &neighbors) {
            action = rand::random();
        }
        solution.push(action.to_char());
        match action {
            Action::Up => bot.move_self(&Direction::North),
            Action::Right => bot.move_self(&Direction::East),
            Action::Down => bot.move_self(&Direction::South),
            Action::Left => bot.move_self(&Direction::North),
            _ => (),
        }
    }
    return solution.into_iter().collect();
}

fn action_is_valid(
    action: &Action,
    neighbors: &(
        Option<&MapSquare>,
        Option<&MapSquare>,
        Option<&MapSquare>,
        Option<&MapSquare>,
        &MapSquare,
    ),
) -> bool {
    let (north, east, south, west, _) = neighbors;
    match action {
        Up => (north.is_some() && is_valid_space(north.unwrap())),
        Right => (east.is_some() && is_valid_space(east.unwrap())),
        Down => (south.is_some() && is_valid_space(south.unwrap())),
        Left => (west.is_some() && is_valid_space(west.unwrap())),
    }
}

fn is_valid_space(space: &MapSquare) -> bool {
    match space {
        MapSquare::Empty { power_up } => true,
        MapSquare::Wrapped { power_up } => true,
        _ => false,
    }
}
