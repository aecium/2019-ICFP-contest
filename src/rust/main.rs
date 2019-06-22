use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate clap;
use clap::App;

mod app_core;

mod powerups;

mod map;
use map::Map;

mod bot;
use bot::*;
mod solvers;
use solvers::turn_right_solver;

mod test;

fn main() {
    test::test();

    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let visualize = matches.is_present("visualize");

    for filename in matches.values_of("INPUT").unwrap() {
        let contents = fs::read_to_string(filename).expect("Failed to read.");

        println!("Processing file: {}", filename);
        if visualize {
            println!("{}", contents);
        }

        let mut map = Map::from_map_string(&contents);
        map.visualize = visualize;
        println!("map: {:?}", map);

        let solution = turn_right_solver::solve(&mut map);
        let solution_string = solution
            .into_iter()
            .map(|a| a.to_char())
            .collect::<String>();
        println!("solution: {}", solution_string);
        println!("solution length: {}", solution_string.len());
        println!("complete?: {}", map.is_complete());

        // Store the solutions
        let path = Path::new("solutions");
        if !path.exists() {
            fs::create_dir(path);
        }
        let mut solution_filename = Path::new(filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        solution_filename.push_str(".sol");
        println!("{:?}", solution_filename);
        let file_path = path.join(Path::new(&solution_filename));
        let mut file = File::create(file_path).unwrap();
        file.write_all(&solution_string.into_bytes()).unwrap();
    }

    println!("🌮 Free Tacos! 🌮");
}
