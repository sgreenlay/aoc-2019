
use std::io::BufRead;
use std::io;

use std::fs;

fn read_inputs(filename: String) -> io::Result<Vec<i32>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).map(|line| {
        line.parse::<i32>().unwrap()
    }).collect())
}

pub fn run() {
    let inputs = read_inputs("data/day01.txt".to_string())
        .expect("Can't read file");
    
    // Part 1
    let mut sum = 0;
    for input in &inputs {
        // to find the fuel required for a module, take its mass, 
        // divide by three, round down, and subtract 2.

        let fuel = (input / 3) - 2;
        sum += fuel;
    }
    println!("{} fuel required", sum);

    // Part 2
    let mut sum = 0;
    for input in &inputs {
        // to find the fuel required for a module, take its mass, 
        // divide by three, round down, and subtract 2.

        let mut fuel = (input / 3) - 2;
        while fuel > 0 {
            sum += fuel;
            fuel = (fuel / 3) - 2;
        }
    }
    println!("{} fuel required", sum);
}
