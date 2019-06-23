use crate::app_core::*;
use crate::bot::*;
use crate::map::*;

use pathfinding::prelude::bfs;

use std::{thread, time};

pub fn solve(map: &mut Map, _moves: usize) -> Vec<Action> {
    println!("Started solve!");
    map.perform(&Action::Start).unwrap();
    let mut action_list = Vec::new();
    while !map.is_complete() {
        //thread::sleep(time::Duration::from_secs(1));
        //println!("still not complete");
        let mut actions: Vec<Action> = Vec::new();;
        let mut d_count = 0;
        let mut last_actions: Vec<Action> = Vec::new();
        loop {
            println!("0action:{:?}", actions);
            println!("0last_actions:{:?}", last_actions);
            if actions == last_actions {
                d_count = d_count + 1;
                println!("count:{:?}", d_count);
            } else {
                last_actions = actions.clone();
                actions.clear();
                println!("1action:{:?}", actions);
                println!("1last_actions:{:?}", last_actions);

                if d_count > 3 {
                    map.perform(&Action::RotAnticlock);
                    action_list.push(Action::RotAnticlock);
                    println!("Doing action {:?}", Action::RotAnticlock);
                }

                break;
            }
            println!("21action:{:?}", actions);
            println!("2last_actions:{:?}", last_actions);
            let path = match bfs(&map.bot_position(),|p| map.find_reachable_neighbors(p),|p| !map.is_painted(*p)) {
            Some(x) => x,
            None => panic!("bfs couldn't find a path!\n action_list: {:?}\n bot_position: {:?}\n map: {:?}\n", action_list, &map.bot_position(), map),
            };
            println!("3action:{:?}", actions);
            println!("3last_actions:{:?}", last_actions);
            //println!("Created path {:?}", path);
            actions = convert_to_actions(&path);
            println!("Created actions list {:?}", actions);
            if actions.len() > 1 {
                break;
            }
            println!("4action:{:?}", actions);
            println!("4last_actions:{:?}", last_actions);
        }
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
    for prev in 0..points.len() - 1 {
        let next = prev + 1;
        let dx = points[next].x as i128 - points[prev].x as i128;
        let dy = points[next].y as i128 - points[prev].y as i128;
        match (dx, dy) {
            (1, 0) => action_list.push(Action::Right),
            (0, 1) => action_list.push(Action::Up),
            (0, -1) => action_list.push(Action::Down),
            (-1, 0) => action_list.push(Action::Left),
            _ => panic!("BFS lied to me, it gave me point {:?}", (dx, dy)),
        };
    }
    return action_list;
}
