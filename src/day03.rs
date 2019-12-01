
use std::io::BufRead;
use std::io;

use std::cmp;
use std::hash;
use std::fmt;
use std::fs;

use std::collections::HashMap;

use regex::Regex;

use lazy_static;

struct Patch {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", 
            self.id, self.x, self.y, self.w, self.h)
    }
}

struct Point {
    x: u32,
    y: u32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl hash::Hash for Point {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl cmp::Eq for Point {}

fn read_inputs(filename: String) -> io::Result<Vec<Patch>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).map(|line| {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
        }
        if LINE_RE.is_match(&line) {
            for line_cap in LINE_RE.captures_iter(&line) {
                let id: u32 = line_cap[1].parse().unwrap();
                let x: u32 = line_cap[2].parse().unwrap();
                let y: u32 = line_cap[3].parse().unwrap();
                let w: u32 = line_cap[4].parse().unwrap();
                let h: u32 = line_cap[5].parse().unwrap();
                
                return Patch { id: id, x: x, y: y, w: w, h: h }
            }
        }
        panic!("Invalid input");
    }).collect())
}

pub fn run() {
    let inputs = read_inputs("data/day03.txt".to_string())
        .expect("Can't read file");

    let mut fabric = HashMap::new();
    for input in &inputs {
        for x in 0..input.w {
            for y in 0..input.h {
                let p = Point{x: x + input.x, y: y + input.y};
                *fabric.entry(p).or_insert(0) += 1;
            }
        }
    }

    let mut more_than_one = 0;
    for (_, value) in fabric.iter() {
        if value > &1 {
            more_than_one += 1
        }
    }
    println!("Duplicates: {}", more_than_one);

    for input in &inputs {
        let mut has_overlap = false;
        for x in 0..input.w {
            for y in 0..input.h {
                let p = Point{x: x + input.x, y: y + input.y};
                if fabric.get(&p).unwrap() > &1 {
                    has_overlap = true;
                    break;
                }
            }
            if has_overlap {
                break;
            }
        }
        if !has_overlap {
            println!("No overlap: {}", input.id);
        }
    }
}