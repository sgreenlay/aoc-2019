
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
    let _inputs = read_inputs("data/day01.txt".to_string())
        .expect("Can't read file");
    
    // TODO
}
