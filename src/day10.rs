use std::io;
use std::io::BufRead;

use std::fmt;
use std::fs;

use std::collections::HashSet;
use std::f64::consts;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn read_inputs(filename: String) -> Vec<Point> {
    let file_in = fs::File::open(filename).expect("Can't read file!");
    let file_reader = io::BufReader::new(file_in);
    let inputs: Vec<Vec<char>> = file_reader
        .lines()
        .filter_map(io::Result::ok)
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            chars
        })
        .collect();

    let mut points: Vec<Point> = Vec::new();
    for y in 0..inputs.len() {
        for x in 0..inputs[y].len() {
            if inputs[y][x] == '#' {
                points.push(Point {
                    x: x as i32,
                    y: y as i32,
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

fn get_closest(start: Point, inputs: &Vec<Point>) -> Vec<usize> {
    let mut closest: Vec<usize> = Vec::new();
    for j in 0..inputs.len() {
        let end = inputs[j];
        if start == end {
            continue;
        }
        let mut is_closest = true;
        for k in 0..inputs.len() {
            let p = inputs[k];
            if (p == start) || (p == end) {
                continue;
            }
            if is_point_between(start, end, p) {
                is_closest = false;
                break;
            }
        }
        if is_closest {
            closest.push(j);
        }
    }

    closest
}

fn angle_to(start: Point, end: Point) -> f64 {
    let dx = (end.x as f64) - (start.x as f64);
    let dy = (end.y as f64) - (start.y as f64);

    let mut deg = -dy.atan2(dx) * 180.0 / consts::PI;

    if (deg <= 90.0) && (deg >= 0.0) {
        deg = (deg - 90.0).abs();
    } else if deg < 0.0 {
        deg = deg.abs() + 90.0;
    } else {
        deg = 450.0 - deg;
    }

    return deg;
}

pub fn run() {
    let mut inputs = read_inputs("data/day10.txt".to_string());

    // Part 1
    let mut max_count = 0;
    let mut max_asteroid = 0;
    for i in 0..inputs.len() {
        let closest = get_closest(inputs[i], &inputs);
        if closest.len() > max_count {
            max_count = closest.len();
            max_asteroid = i;
        }
    }

    let station = inputs[max_asteroid];
    println!("{} @ {}", max_count, station);

    // Part 2
    let mut remaining_asteroids = 200;
    loop {
        let closest = get_closest(station, &inputs);
        if closest.len() < remaining_asteroids {
            remaining_asteroids -= closest.len();
            let remove: HashSet<Point> = closest.iter().map(|&i| inputs[i]).collect();
            inputs.retain(|p| !remove.contains(p));
        } else {
            let mut angles: Vec<(usize, Point, f64)> = closest
                .iter()
                .map(|&i| {
                    let p = inputs[i];
                    let angle = angle_to(station, p);
                    (i, p, angle)
                })
                .collect();
            angles.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            let two_hundredth = angles[remaining_asteroids - 1].0;
            println!("{}", inputs[two_hundredth]);
            break;
        }
    }
}
