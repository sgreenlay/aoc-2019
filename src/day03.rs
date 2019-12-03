
use std::io::BufRead;
use std::io;

use std::cmp;	
use std::hash;	
use std::fmt;
use std::ops;
use std::fs;

use std::collections::HashMap;

fn read_inputs(filename: String) -> io::Result<Vec<Vec<Direction>>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).map(|line| {
        line.split(',').map(|s| {
            let (direction, distance) = s.split_at(1);
            let magnitude = distance.parse::<i32>().unwrap();
            match direction {
                "U" => Direction::Up(magnitude),
                "D" => Direction::Down(magnitude),
                "L" => Direction::Left(magnitude),
                "R" => Direction::Right(magnitude),
                _ => panic!("Unknown direction"),
            }
        }).collect()
    }).collect())
}

enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Copy, Clone)]
struct Point {	
    x: i32,	
    y: i32,	
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

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn manhatten_distance(a : Point, b : Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn path(directions : &Vec<Direction>) -> HashMap<Point, i32> {
    let mut visited = HashMap::new();
    let mut p = Point {x: 0, y: 0};
    let mut steps = 0;

    for d in directions {
        let dp;
        let len;

        match d {
            Direction::Up(distance) => {
                dp = Point{x: 0, y: -1};
                len = *distance;
            },
            Direction::Down(distance) => {
                dp = Point{x: 0, y: 1};
                len = *distance;
            },
            Direction::Left(distance) => {
                dp = Point{x: -1, y: 0};
                len = *distance;
            },
            Direction::Right(distance) => {
                dp = Point{x: 1, y: 0};
                len = *distance;
            }
        }
        
        for _ in 1..=len {
            p = p + dp;
            steps += 1;
            visited.entry(p).or_insert(steps);
        }
    }
    visited
}

pub fn run() {
    let inputs = read_inputs("data/day03.txt".to_string())
        .expect("Can't read file");

    if inputs.len() != 2 {
        panic!("Bad file length");
    }

    let path_a = path(&inputs[0]);
    let path_b = path(&inputs[1]);

    let intersections : Vec<(&Point, &i32)> = path_a.iter()
        .filter(|p| path_b.contains_key(p.0))
        .collect();

    let start = Point{x: 0, y: 0};
    let mut min_intersection = None;
    let mut min_distance : i32 = 0;

    for intersection in &intersections {
        if min_intersection == None {
            min_intersection = Some(*intersection.0);
            min_distance = manhatten_distance(start, min_intersection.unwrap());
        } else {
            let distance = manhatten_distance(start, *intersection.0);
            if distance < min_distance {
                min_intersection = Some(*intersection.0);
                min_distance = distance;
            }
        }
    }

    println!("{}", min_distance);

    min_intersection = None;
    min_distance = 0;

    for intersection in &intersections {
        if min_intersection == None {
            min_intersection = Some(*intersection.0);
            min_distance = path_a[intersection.0] + path_b[intersection.0];
        } else {
            let distance = path_a[intersection.0] + path_b[intersection.0];
            if distance < min_distance {
                min_intersection = Some(*intersection.0);
                min_distance = distance;
            }
        }
    }

    println!("{}", min_distance);
}
