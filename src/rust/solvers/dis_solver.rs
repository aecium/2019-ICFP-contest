use crate::bot::*;
use crate::map::*;
use crate::app_core::*;

use pathfinding::prelude::bfs;

use std::{thread, time};

pub fn solve(map: &mut Map) -> Vec<Action> {
    map.perform(&Action::Start).unwrap();
    let mut action_list = Vec::new();
    while !map.is_complete() {
        //first take the move that fills the most spaces eagerly
        let mut move_options = Vec::new();
        let mut max_value = 1;
        while max_value != 0 {
            let moves = &mut move_options;
            for action in &[Action::Up,Action::Right,Action::Down,Action::Left] {
                moves.push( (action,roa(map, action)) );
            }
            let best = moves.into_iter().max_by_key(|x| x.1).unwrap();
            max_value = best.1;
            if max_value != 0 {
                if map.is_valid_action(best.0) {
                    map.perform(best.0);
                    action_list.push(best.0.clone());
                }
            }
            moves.clear();
        }

        if map.is_complete() {
            break;
        }
        //if these moves are all 0, do the bfs solve instead.
        let path = match bfs(&map.bot_position(),|p| map.find_reachable_neighbors(p),|p| !map.is_painted(*p)) {
            Some(x) => x,
            None => panic!("bfs couldn't find a path!\n action_list: {:?}\n bot_position: {:?}\n map: {:?}\n", action_list, &map.bot_position(), map),
        };
        let target_point = path.last().unwrap();
        let actions = convert_to_actions(&path);
        for a in actions {
            map.perform(&a);
            action_list.push(a);
            if map.is_painted(*target_point){
                break;
            }
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

fn roa(map: &mut Map, action: &Action) -> usize {
    map.push_undo();
    let before = map.get_remaining();
    map.perform(action);
    let after = map.get_remaining();
    map.pop_undo();
    return before - after;
}