use crate::bot::*;
use crate::map::*;
use crate::app_core::*;

use pathfinding::prelude::bfs;

use std::{thread, time};

pub fn solve(map: &mut Map, _moves: usize) -> Vec<Action> {
    println!("Started solve!");
    map.perform(&Action::Start).unwrap();
    let mut action_list = Vec::new();
    while !map.is_complete() {
        //thread::sleep(time::Duration::from_secs(1));
        //println!("still not complete");
        let path = match bfs(&map.bot_position(),|p| map.find_reachable_neighbors(p),|p| !map.is_painted(*p)) {
            Some(x) => x,
            None => panic!("bfs couldn't find a path!\n action_list: {:?}\n bot_position: {:?}\n map: {:?}\n", action_list, &map.bot_position(), map),
        };
        //println!("Created path {:?}", path);
        let actions = convert_to_actions(&path);
        //println!("Created actions list {:?}", actions);
        for a in actions {
            //println!("Doing action {:?}", a);
            map.perform(&a);
            action_list.push(a);
        }
        //println!("Current Action List: {:?}", action_list);
    }
    return action_list;
}

fn convert_to_actions(points: &Vec<Point>) -> Vec<Action> {
    let mut action_list = Vec::new();
    for prev in 0..points.len()-1 {
        let next = prev + 1;
        let dx = points[next].x as i128 - points[prev].x as i128;
        let dy = points[next].y as i128 - points[prev].y as i128;
        match (dx, dy) {
            (1,0) => action_list.push(Action::Right),
            (0,1) => action_list.push(Action::Up),
            (0,-1) => action_list.push(Action::Down),
            (-1,0) => action_list.push(Action::Left),
            _ => panic!("BFS lied to me, it gave me point {:?}", (dx, dy)),
        };
    }
    return action_list;
}