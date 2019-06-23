use crate::bot::*;
use crate::map::*;
use crate::app_core::*;

use pathfinding::prelude::bfs;

fn combo(moves: usize) -> Vec<Vec<Action>> {
    let actions = &[Action::Up,Action::Right,Action::Down,Action::Left,Action::RotClock,Action::RotAnticlock];
    let mut combinations: Vec<Vec<Action>> = Vec::new();
    for a1 in actions {
        let mut moves: Vec<Action> = Vec::new();
        moves.push(a1.clone());
        combinations.push(moves);
    }
    for i in 1..moves {
        let num = combinations.len();
        for i in 0..num {
            for a1 in actions {
                let mut moves: Vec<Action> = combinations[i].clone();
                moves.push(a1.clone());
                combinations.push(moves);
            }
        }
    }
    combinations
}

pub fn solve(map: &mut Map) -> Vec<Action> {
    map.perform(&Action::Start).unwrap();
    let mut action_list: Vec<Action> = Vec::new();
    //let actions = &[Action::Up,Action::Right,Action::Down,Action::Left,Action::RotClock,Action::RotAnticlock];
    //let actions = &[Action::Up,Action::Right,Action::Down,Action::Left];
    //let actions = &[Action::Up];
    let combinations: Vec<Vec<Action>> = combo(3);

    //for combination in &combinations {
    //    println!("{:?}", combination);
    //    println!("{:?}", combination.len());
    //}

    while !map.is_complete() {
        //println!("Before: {:?}", map);

        //first take the move that fills the most spaces eagerly
        let mut max_score = 1;
        while max_score != 0 {

            max_score = 0;
            let mut best: Vec<Action> = Vec::new();
            for moves in &combinations {
                let score = roa(map, moves);
                if score > max_score {
                    best = moves.clone();
                    max_score = score;
                }
            }

            //println!("{:?}\n Max: {}", map, max_value);
            if max_score != 0 {
                for action in best {
                    if map.is_valid_action(&action) {
                        //println!("{:?}", action);
                        map.perform(&action);
                        action_list.push(action);
                    }
                }
            }
            //println!("After: {:?}", map);
            //println!("Max score: {}", max_score);
            //break;
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

fn roa(map: &mut Map, actions: &Vec<Action>) -> usize {
    map.push_undo();
    let before = map.get_remaining();
    for action in actions {
        if map.is_valid_action(&action) {
            map.perform(action);
        } else {
            map.pop_undo();
            return 0;
        }
    }
    let after = map.get_remaining();
    map.pop_undo();
    let mut score = before - after;
    let moves = actions.len();
    return score / moves;
    if score > moves {
        score -= moves;
    } else {
        score = 1;
    }
    return score;
}