
use std::io::BufRead;
use std::io;

use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {	
    x: i32,	
    y: i32,	
}

fn read_inputs(filename: String) -> Vec<Point> {
    let file_in = fs::File::open(filename).expect("Can't read file!");
    let file_reader = io::BufReader::new(file_in);
    let inputs: Vec<Vec<char>> = file_reader.lines().filter_map(io::Result::ok).map(|line| {
        let chars: Vec<char> = line.chars().collect();
        chars
    }).collect();

    let mut points: Vec<Point> = Vec::new();
    for y in 0..inputs.len() {
        for x in 0..inputs[y].len() {
            if inputs[y][x] == '#' {
                points.push(Point{ 
                    x: x as i32, 
                    y: y as i32
                });
            }
        }
    }
    points
}

fn is_point_between(a: Point, b: Point, c: Point) -> bool {
    let crossproduct = (c.y - a.y) * (b.x - a.x) - (c.x - a.x) * (b.y - a.y);
    if crossproduct != 0 {
        return false;
    }

    let dotproduct = (c.x - a.x) * (b.x - a.x) + (c.y - a.y) * (b.y - a.y);
    if dotproduct < 0 {
        return false;
    }

    let squaredlengthba = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
    if dotproduct > squaredlengthba {
        return false;
    }

    return true;
}

pub fn run() {
    let inputs = read_inputs("data/day10.txt".to_string());

    let mut max_count = 0;
    for i in 0..inputs.len() {
        let mut count = 0;
        let start = inputs[i];
        for j in 0..inputs.len() {
            if i == j {
                continue;
            }
            let end = inputs[j];
            let mut is_closest = true;
            for k in 0..inputs.len() {
                if (k == i) || (k == j) {
                    continue;
                }
                let p = inputs[k];

                if is_point_between(start, end, p) {
                    is_closest = false;
                    break;
                }
            }
            if is_closest {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
        }
    }
    println!("{}", max_count);
}
