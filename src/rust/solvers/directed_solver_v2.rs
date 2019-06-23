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
        let mut action_backlog: Vec<Action> = Vec::new();;
        let mut d_count = 0;
        let mut last_actions_real: Vec<Action> = Vec::new();
        let mut last_actions: &mut Vec<Action> = &mut last_actions_real;
        let mut plan_map = map.clone();
        let mut starting = true;
        loop {
            //thread::sleep(time::Duration::from_secs(2));
            actions.clear();
            let path = match bfs(&plan_map.bot_position(),|p| plan_map.find_reachable_neighbors(p),|p| !plan_map.is_painted(*p)) {
            Some(x) => x,
            None => panic!("bfs couldn't find a path!\n action_list: {:?}\n bot_position: {:?}\n map: {:?}\n", action_list, &plan_map.bot_position(), map),
            };
            //println!("Created path {:?}", path);

            actions = convert_to_actions(&path);
            //println!("Created actions list {:?}", actions);
            if actions.len() > 1 && d_count < 3 {
                action_backlog.append(&mut actions);
                last_actions.clear();
                //println!("exit acctions.len() > 1");
                break;
            };

            //let action_to_perform = actions.first()
            plan_map.perform(actions.first().unwrap());
            action_backlog.append(&mut actions.clone());

            // actions is a single step action
            if &actions == last_actions {
                d_count = d_count + 1;
            // println!("1action:{:?}", actions);
            // println!("1last_actions:{:?}", last_actions);
            // println!("repeat count:{:?}", d_count);
            } else {
                //*last_actions = actions.clone();
                //println!("`count:{:?}", d_count);
                if d_count > 2 {
                    println!("1action:{:?}", actions);
                    println!("1last_actions:{:?}", last_actions);
                    match actions[0] {
                        Action::Down | Action::Up => match last_actions[0] {
                            Action::Left | Action::Right => {
                                action_backlog.insert(0, Action::RotAnticlock);
                                println!("did rotate 1");
                            }
                            _ => println!("did not rotate 1"),
                        },
                        Action::Left | Action::Right => match last_actions[0] {
                            Action::Down | Action::Up => {
                                action_backlog.insert(0, Action::RotClock);
                                println!("did rotate 2");
                            }
                            _ => println!("did not rotate 2"),
                        },
                        _ => println!("did nothing"),
                    }

                    //map.perform(&Action::RotAnticlock);
                    //action_list.push(Action::RotAnticlock);
                    //println!("Doing action {:?}", Action::RotAnticlock);
                }
                *last_actions = actions.clone();
                if d_count > 0 {
                    //println!("exit d_count > 0");
                    d_count = 0;
                    break;
                }
            }

            //println!("action_list: {:?}", action_list);
            //println!("action_backlog: {:?}", action_backlog);
        }
        for a in action_backlog {
            //println!("Doing action {:?}", a);
            map.perform(&a);
            action_list.push(a);
        }
        //println!(
        //    "Action Backlog applied!\n New Action List: {:?}",
        //    action_list
        //);
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
