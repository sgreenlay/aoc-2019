
use std::io::BufRead;
use std::io;

use std::fs;

use std::collections::HashMap;

fn read_inputs(filename: String) -> HashMap<String, String> {
    let file_in = fs::File::open(filename).expect("Couldn't open file");
    let file_reader = io::BufReader::new(file_in);
    let inputs: Vec<Vec<String>> = file_reader.lines().filter_map(io::Result::ok).map(|line| {
        line.split(')').map(|s| {
            String::from(s)
        }).collect()
    }).collect();

    let mut orbits = HashMap::new();
    for input in inputs {
        let key: String = input[1].to_string();
        let value: String = input[0].to_string();
        orbits.entry(key).or_insert(value);
    }
    orbits
}

fn path_to_root(p: &String, planets: &HashMap<String, String>) -> Vec<String> {
    let mut current_planet: &String = p;
    let mut path: Vec<String> = Vec::new();
    while planets.contains_key(current_planet) {
        let next_planet = &planets[current_planet];
        path.push(next_planet.to_string());
        current_planet = &next_planet;
    }
    path
}

pub fn run() {
    let inputs = read_inputs("data/day06.txt".to_string());

    let mut orbit_total = 0;
    let planets = inputs.keys();
    for p in planets {
        let path = path_to_root(p, &inputs);
        orbit_total += path.len();
    }
    println!("{}", orbit_total);
}