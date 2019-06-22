use std::env;
use std::fs;

mod app_core;
use app_core::Direction;

mod powerups;

mod map;
use map::{Map, MapSquare};

mod bot;
use bot::*;
mod solvers;
use solvers::boko_solver;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read.");

    println!("{}: {}", filename, contents);

    let map = Map::from_map_string(&contents);

    println!("map: {:?}", map);
    println!("complete?: {}", map.is_complete());

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
        Action::Up => (north.is_some() && is_valid_space(north.unwrap())),
        Action::Right => (east.is_some() && is_valid_space(east.unwrap())),
        Action::Down => (south.is_some() && is_valid_space(south.unwrap())),
        Action::Left => (west.is_some() && is_valid_space(west.unwrap())),
        _ => panic!("I don't know how to handle non-movement actions!"),
    }
}

fn is_valid_space(space: &MapSquare) -> bool {
    match space {
        MapSquare::Empty { power_up: _ } => true,
        MapSquare::Wrapped { power_up: _ } => true,
        _ => false,
    }
}
