use crate::bot::*;
use crate::map::*;

pub fn solve(map: &mut Map) -> Vec<Action> {
    let mut action_list = Vec::new();
    map.perform(&Action::Start).unwrap();
    map.perform(&Action::Up).unwrap();
    action_list.push(Action::Up);
    map.perform(&Action::Up).unwrap();
    action_list.push(Action::Up);
    map.perform(&Action::Down).unwrap();
    action_list.push(Action::Down);

    while !map.is_complete() {
        let my_action = Action::Right;

        if map.is_valid_action(&my_action) {
            match map.perform(&my_action) {
                Err(msg) => panic!(
                    "The Map wouldn't let me Right, it said {0}", /*,my_action*/
                    msg
                ),
                _ => action_list.push(my_action),
            }
        } else {
            println!("{:?}", map);
            panic!(
                "Solver has run out of ideas... Here's so far:{}",
                (action_list
                    .into_iter()
                    .map(|action| action.to_char())
                    .collect::<String>())
            );
        }

        let my_action = Action::Up;

                if map.is_valid_action(&my_action) {
            match map.perform(&my_action) {
                Err(msg) => panic!(
                    "The Map wouldn't let me Right, it said {0}", /*,my_action*/
                    msg
                ),
                _ => action_list.push(my_action),
            }
        } else {
            println!("{:?}", map);
            panic!(
                "Solver has run out of ideas... Here's so far:{}",
                (action_list
                    .into_iter()
                    .map(|action| action.to_char())
                    .collect::<String>())
            );
        }
    }
    return action_list;
}
