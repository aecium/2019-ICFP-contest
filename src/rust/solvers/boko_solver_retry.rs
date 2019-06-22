use std::cmp;
use crate::bot::*;
use crate::map::*;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub fn solve(map: &mut Map, max_moves: usize) -> Vec<Action> {
    let mut best = max_moves+1;
    let mut best_actions = Vec::new();
    let tries = 10000000 / max_moves;
    let mut has_solution = false;

    for try_num in 0..tries {
        if try_num > 1 && try_num % 100 == 0 {
            print!(".");
            if try_num > 100 && try_num % 8000 == 0 {
                println!("");
            }
        }
        let mut map = map.clone();
        map.perform(&Action::Start).unwrap();
        let mut action_list = Vec::new();
        let mut rng = rand::thread_rng();
        while !map.is_complete() && action_list.len() < best {
            let mut my_action = rng.gen();
            while !map.is_valid_action(&my_action)
            {
                my_action = rng.gen();
            }
            map.perform(&my_action);
            action_list.push(my_action);
        }
        if map.is_complete() && action_list.len() < best {
            has_solution = true;
            best_actions = action_list;
            best = best_actions.len();
            println!("... found {} length solution in {} tries.", best, try_num);
        }
    }
    if has_solution {
        println!("Found {} length solution in {} tries.", best, tries);
    } else {
        println!("Sorry, I didn't find a solution.");
    }
    return best_actions;
}
