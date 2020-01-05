use std::io;
use std::io::BufRead;

use std::fs;

fn read_inputs(filename: String) -> io::Result<Vec<i32>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader
        .lines()
        .filter_map(io::Result::ok)
        .map(|line| line.parse::<i32>().unwrap())
        .collect())
}

pub fn run() {
    let inputs = read_inputs("data/day01.txt".to_string()).expect("Can't read file");

    // Part 1
    let sum = inputs.iter().fold(0, |acc, x| acc + (x / 3) - 2);
    println!("{} fuel required", sum);

    // Part 2
    let sum = inputs.iter().fold(0, |acc, x| {
        let mut local_sum = 0;
        let mut fuel = (x / 3) - 2;
        while fuel > 0 {
            local_sum += fuel;
            fuel = (fuel / 3) - 2;
        }
        acc + local_sum
    });
    println!("{} fuel required", sum);
}
