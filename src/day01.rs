
use std::io::BufRead;
use std::io;

use std::fs;

use std::collections::HashSet;

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
        sum += input;
    }
    println!("Sum is: {}", sum);

    // Part 2
    let mut sum = 0;
    let mut sums = HashSet::new();
    let mut found = false;

    while !found {
        for input in &inputs {
            sum += input;
            if sums.contains(&sum) {
                println!("First duplicate sum is: {}", sum);
                found = true;
                break;
            }
            sums.insert(sum);
        }   
    }
}
