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
use solvers::*;

mod test;

fn main() {
    test::test();

    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let visualize = matches.is_present("visualize");

    for filename in matches.values_of("INPUT").unwrap() {
        let contents = fs::read_to_string(filename).expect("Failed to read.");
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
        let mut has_old = false;
        let mut old_len = 0;
        if file_path.exists() {
            let file = File::open(&file_path).unwrap();
            old_len = file.metadata().unwrap().len() as usize;
            if old_len > 0 {
                has_old = true;
            }
        }

        println!("Processing file: {}", filename);
        if visualize {
            println!("{}", contents);
        }

        let mut map = Map::from_map_string(&contents);
        map.visualize = visualize;
        println!("map: {:?}", map);

        let mut max_moves =  map.get_remaining() * 100;
        if has_old && old_len>0 {
            max_moves = old_len;
        }
        let solution = match matches.value_of("solver").unwrap() {
            "boko_retry" => boko_solver_retry::solve(&mut map, max_moves),
            "boko" => boko_solver::solve(&mut map),
            "right" => right_solver::solve(&mut map),
            "spiral_right" => spiral_right_solver::solve(&mut map),
            "directed" => directed_solver::solve(&mut map, max_moves),
            "directed_v2" => directed_solver_v2::solve(&mut map, max_moves),
            "eager" => eager_solver::solve(&mut map),
            _ => panic!("Unknown solver."),
        };

        let new_len = solution.len();
        let solution_string = solution
            .into_iter()
            .map(|a| a.to_char())
            .collect::<String>();
        println!("solution: {}", solution_string);
        println!("solution length: {}", new_len);
        println!("map size: {}", map.w*map.h);

        if new_len>0 {
            if !has_old || solution_string.len() < old_len {
                let mut file = File::create(file_path).unwrap();
                println!("Better than previous best {} vs. {}", new_len, old_len);
                file.write_all(&solution_string.into_bytes()).unwrap();
            } else {
                println!("Not better than previous best {} vs. {}", new_len, old_len);
            }
        } else {
            println!("No improved solution found.");
        }
    }

    println!("🌮 Free Tacos! 🌮");
}
