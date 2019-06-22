use std::cmp;
use std::fmt;

use crate::app_core::{Point,Direction};
use crate::bot::*;
use crate::powerups::PowerUp;

pub type Neighbors<'a> = (
    Option<&'a MapSquare>,
    Option<&'a MapSquare>,
    Option<&'a MapSquare>,
    Option<&'a MapSquare>,
    &'a MapSquare,
);
pub struct Map {
    remaining: usize,
    contour: Vec<Point>,
    squares: Vec<Vec<MapSquare>>,
    bot: Bot,
}
impl Map {
    pub fn from_map_string(map_string: &str) -> Self {
        let mut split_map = map_string.split('#');
        let contour = split_map
            .next()
            .expect("Ran out of parts.")
            .trim_matches('(')
            .trim_matches(')')
            .split("),(");
        let bot_position: Vec<&str> = split_map
            .next()
            .expect("Ran out of parts.")
            .trim_matches('(')
            .trim_matches(')')
            .split(",")
            .collect();
        let bot_position = Point {
            x: bot_position[0].parse::<usize>().unwrap(),
            y: bot_position[1].parse::<usize>().unwrap(),
        };
        let obstacles = split_map.next().expect("Need more tacos.").split(";");
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

        let mut obstacle_starts: Vec<Point> = Vec::new();
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
                obstacle_starts.push(points[0].clone());
                Map::contour_map(&mut map, &mut points, MapSquare::Blocked);
            }
        }

        Map::fill_map(&mut map, bot_position.clone(), MapSquare::Empty { power_up: None }, MapSquare::Empty { power_up: None });
        for point in obstacle_starts {
            Map::fill_map(&mut map, point, MapSquare::Blocked, MapSquare::Blocked);
        }

        Map {
            contour: points,
            squares: map,
            remaining: 1,
            bot: Bot::new(bot_position.clone(), Direction::East),
        }
    }

    fn fill_map(map: &mut Vec<Vec<MapSquare>>, start: Point, search: MapSquare, replace: MapSquare) {
        let h = map.len() - 1;
        let w = map[0].len() - 1;
        let mut done: Vec<Vec<bool>> = Vec::new();
        for _y in 0..h + 1 {
            let mut row = Vec::new();
            for _x in 0..w + 1 {
                row.push(false);
            }
            done.push(row);
        }
        let mut todo: Vec<Point> = Vec::new();
        todo.push(start);
        while todo.len() > 0 {
            let point = todo.pop().unwrap();
            let x = point.x;
            let y = point.y;
            let oob = MapSquare::OOB;
            map[y][x] = replace.clone();
            done[y][x] = true;
            if x > 0 && !done[y][x - 1] && (map[y][x - 1] == search || map[y][x - 1] == oob) {
                todo.push(Point { x: x - 1, y: y });
            }
            if x < w && !done[y][x + 1] && (map[y][x + 1] == search || map[y][x + 1] == oob) {
                todo.push(Point { x: x + 1, y: y });
            }
            if y > 0 && !done[y - 1][x] && (map[y - 1][x] == search || map[y - 1][x] == oob) {
                todo.push(Point { x: x, y: y - 1 });
            }
            if y < h && !done[y + 1][x] && (map[y + 1][x] == search || map[y + 1][x] == oob) {
                todo.push(Point { x: x, y: y + 1 });
            }
            //println!("{}, {}, {:?}", x, y, todo);
            //println!("{:?}", done);
        }
    }

    fn contour_map(map: &mut Vec<Vec<MapSquare>>, points: &mut Vec<Point>, square: MapSquare) {
        let mut last = Point { x: 0, y: 0 };
        let mut first = true;
        let mut ps = points.to_vec();
        let mt = MapSquare::Empty { power_up: None };
        let oob = MapSquare::OOB;
        let h = map.len() - 1;
        let w = map[0].len() - 1;
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
                        if square == mt {
                            if up && x < w && map[y][x + 1] == oob {
                                map[y][x + 1] = MapSquare::Boundry;
                                if y > 0 && map[y - 1][x + 1] != mt {
                                    map[y - 1][x + 1] = MapSquare::Boundry;
                                }
                                if y < h && map[y + 1][x + 1] != mt {
                                    map[y + 1][x + 1] = MapSquare::Boundry;
                                }
                            }
                            if down && x > 0 && map[y][x - 1] == oob {
                                map[y][x - 1] = MapSquare::Boundry;
                                if y > 0 && map[y - 1][x - 1] != mt {
                                    map[y - 1][x - 1] = MapSquare::Boundry;
                                }
                                if y < h && map[y + 1][x - 1] != mt {
                                    map[y + 1][x - 1] = MapSquare::Boundry;
                                }
                            }
                            if left && y < h && map[y + 1][x] == oob {
                                map[y + 1][x] = MapSquare::Boundry;
                                if x > 0 && map[y + 1][x - 1] != mt {
                                    map[y + 1][x - 1] = MapSquare::Boundry;
                                }
                                if x < w && map[y + 1][x + 1] != mt {
                                    map[y + 1][x + 1] = MapSquare::Boundry;
                                }
                            }
                            if right && y > 0 && map[y - 1][x] == oob {
                                map[y - 1][x] = MapSquare::Boundry;
                                if x > 0 && map[y - 1][x - 1] != mt {
                                    map[y - 1][x - 1] = MapSquare::Boundry;
                                }
                                if x < w && map[y - 1][x + 1] != mt {
                                    map[y - 1][x + 1] = MapSquare::Boundry;
                                }
                            }
                        }
                    }
                }
            }
            last = point.clone();
        }
    }

    pub fn find_neighbors(&self, pos: &Point) -> Neighbors {
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

    pub fn is_complete(&self) -> bool {
        let mut remaining = 0;
        for y in (0..self.squares.len()) {
            let row = &self.squares[y];
            for x in 0..row.len() {
                match self.squares[y][x] {
                    MapSquare::Empty { power_up: _ } => remaining+=1,
                    MapSquare::Wrapped { power_up: _ } | MapSquare::Blocked | MapSquare::OOB | MapSquare::Boundry => {},
                };
            }
        }
        println!("remaining: {}", remaining);
        return remaining == 0;
    }

    pub fn is_valid_action(&self, action: &Action) -> bool {
        let pos = &self.bot.position;
        let neighbors = self.find_neighbors(&pos);
        match action {
            Action::Right => match &neighbors.3 {
                Some(square) => match square {
                    MapSquare::Empty { power_up: _ } | MapSquare::Wrapped { power_up: _ } => true,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    pub fn perform(&mut self, action: &Action) -> Result<(),String> {
        if !self.is_valid_action(action) {
            return Result::Err("Action is invalid".to_string());
        }
        match action {
            Action::Right => {
                self.bot.move_self(&Direction::East);
                return Result::Ok(());
            }
            _ => panic!("I'm sorry, I can't do that Dave"),
        }
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
pub enum MapSquare {
    Empty { power_up: Option<PowerUp> },
    Wrapped { power_up: Option<PowerUp> },
    Blocked,
    OOB,
    Boundry,
}
impl MapSquare {
    fn to_char(&self) -> char {
        match self {
            //MapSquare::Empty{  } => '.',
            MapSquare::Wrapped { power_up: _ } => 'O',
            MapSquare::Blocked => '~',
            MapSquare::OOB => '.',
            MapSquare::Boundry => ',',
            _ => '#',
        }
    }
}
impl fmt::Debug for MapSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
