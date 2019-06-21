use std::env;
use std::fs;

struct Map {
    squares : Vec<Vec<MapSqare>>
}
impl Map {
    fn from_map_string(map_string: &str) -> Self {

        let split_map = map_string.split('#');
        let bounding_box = split_map.next();
        let bot_position = split_map.next();
        let obstacles = split_map.next();
        let boosters = split_map.next();

        let map_vec = Vec::new();
        for point_str in bounding_box.split(',')
        {

        }
}

enum MapSqare {
    Empty { powerUp : Option<PowerUp>},
    Wrapped { powerUp : Option<PowerUp>},
    Blocked { powerUp : Option<PowerUp>},
    OOB,
}

struct Bot {
    powerups : Vec<PowerUp>

}

enum PowerUp{
    Extension, //{code: 'B'},
    Boost,// {code: 'F'},
    Drill,// {code: 'L'},
}
 
pub trait ByCode {
    fn by_code(code: char) -> Self;
}

impl ByCode for PowerUp{
    fn by_code(code: char) -> Self {
        match code {
            'B' => PowerUp::Extension,
            'F' => PowerUp::Boost,
            'L' => PowerUp::Drill,
            _ => panic!("Unknown powerup code"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Failed to read.");

    println!("{}: {}", filename, contents);
    println!("🌮 Free Tacos! 🌮");
}
