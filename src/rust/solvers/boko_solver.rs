use crate::bot::*;
use crate::map::*;

pub fn solve(map: &mut Map) -> Vec<Action> {
    let mut action_list = Vec::new();
    while !map.is_complete()  {
        let my_action = Action::Right;

        if map.is_valid_action(&my_action)  {
            match map.perform(&my_action) {
                Err(msg) => panic!("The Map wouldn't let me Right, it said {0}"/*,my_action*/,msg),
                _ => action_list.push(my_action),
            }
        } else {
            println!("{:?}", map);
            panic!("Solver has run out of ideas...");
        }
    }
    return action_list;
}
