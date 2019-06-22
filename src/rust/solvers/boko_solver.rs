use crate::bot::*;
use crate::map::*;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub fn solve(map: &mut Map) -> Vec<Action> {
    map.perform(&Action::Start).unwrap();
    let mut action_list = Vec::new();
    let mut rng = rand::thread_rng();
    while !map.is_complete() {
        let mut my_action = rng.gen();
        while !map.is_valid_action(&my_action)
        {
            my_action = rng.gen();
        }
        println!("My Selected Action is {:?}", my_action);
        map.perform(&my_action);
        action_list.push(my_action);
    }
    return action_list;
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0, 4) {
            0 => Action::Up,
            1 => Action::Right,
            2 => Action::Down,
            _ => Action::Left,
            //4 => Action::RotClock,
            //_ => Action::RotAnticlock,
        }
    }
}