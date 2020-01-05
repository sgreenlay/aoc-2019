use std::cmp;
use std::fmt;
use std::fs;
use std::ops;

use crate::intcode::{VirtualMachine, VirtualMachineState};

use std::collections::HashMap;
use std::collections::HashSet;

fn load_program(filename: String) -> Vec<i128> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .split(',')
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i128,
    y: i128,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn inverse(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn generate_direction_table() -> HashMap<Direction, Point> {
    let mut directions: HashMap<Direction, Point> = HashMap::new();
    directions.insert(Direction::North, Point { x: 0, y: -1 });
    directions.insert(Direction::South, Point { x: 0, y: 1 });
    directions.insert(Direction::East, Point { x: 1, y: 0 });
    directions.insert(Direction::West, Point { x: -1, y: 0 });
    directions
}

lazy_static! {
    static ref DIRECTIONS: HashMap<Direction, Point> = generate_direction_table();
}

fn run_program(program: &Vec<i128>) -> HashMap<Point, i128> {
    let mut vm = VirtualMachine::new(program);

    // The remote control program executes the following steps in a loop forever:
    //   - Accept a movement command via an input instruction.
    //   - Send the movement command to the repair droid.
    //   - Wait for the repair droid to finish the movement operation.
    //   - Report on the status of the repair droid via an output instruction.

    let mut current = Point { x: 0, y: 0 };
    let mut path: Vec<(Direction, Point)> = Vec::new();

    let mut map: HashMap<Point, i128> = HashMap::new();
    map.insert(current, 0);

    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                // Only four movement commands are understood:
                //   north (1),
                //   south (2),
                //   west (3), and
                //   east (4).

                let mut next: Option<(Direction, Point)> = None;
                for d in DIRECTIONS.iter() {
                    let p_next = current + *d.1;
                    if !map.contains_key(&p_next) {
                        next = Some((*d.0, p_next));
                        break;
                    }
                }

                let direction;
                match next {
                    Some(n) => {
                        let prev = current;
                        direction = n.0;
                        current = n.1;
                        path.push((direction, prev));
                    }
                    None => {
                        if path.is_empty() {
                            break;
                        }
                        let backtrack = path.pop().unwrap();
                        direction = backtrack.0.inverse();
                        current = backtrack.1;
                    }
                }

                match direction {
                    Direction::North => {
                        vm.add_input(1);
                    }
                    Direction::South => {
                        vm.add_input(2);
                    }
                    Direction::East => {
                        vm.add_input(3);
                    }
                    Direction::West => {
                        vm.add_input(4);
                    }
                }
            }
            VirtualMachineState::Output(v) => {
                // The repair droid can reply with any of the following status codes:
                //    0: The repair droid hit a wall. Its position has not changed.
                //    1: The repair droid has moved one step in the requested direction.
                //    2: The repair droid has moved one step in the requested direction;
                //       its new position is the location of the oxygen system.
                if !map.contains_key(&current) {
                    map.insert(current, v);
                }

                if v == 0 {
                    let backtrack = path.pop().unwrap();
                    current = backtrack.1;
                }
            }
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }

    map
}

fn bredth_first_search_map(
    start: &Point,
    map: &HashMap<Point, i128>,
    stop: &mut dyn FnMut(Point, i128) -> bool,
) {
    let mut frontier: Vec<(Point, i128)> = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();

    if !map.contains_key(start) {
        panic!("Invalid start point");
    }

    visited.insert(*start);
    frontier.push((*start, 0));

    while !frontier.is_empty() {
        let p = frontier.remove(0);

        for d in DIRECTIONS.iter() {
            let p_next = p.0 + *d.1;
            if !visited.contains(&p_next) && map.contains_key(&p_next) {
                let distance = p.1 + 1;

                let next = map[&p_next];
                if next == 0 {
                    continue;
                }

                frontier.push((p_next, distance));
                visited.insert(p_next);

                if stop(p_next, distance) {
                    break;
                }
            }
        }
    }
}

pub fn run() {
    let program = load_program("data/day15.txt".to_string());
    let map = run_program(&program);

    // Part 1
    let start = Point { x: 0, y: 0 };
    let mut oxygen = Point { x: 0, y: 0 };
    let mut min_distance = 0;
    bredth_first_search_map(&start, &map, &mut |p, distance| -> bool {
        if map[&p] == 2 {
            oxygen = p.clone();
            min_distance = distance;
            true
        } else {
            false
        }
    });
    println!("Shortest path to oxygen @ {} = {}", oxygen, min_distance);
    
    // Part 2
    let mut max_distance = 0;
    bredth_first_search_map(&oxygen, &map, &mut |_, distance| -> bool {
        if distance > max_distance {
            max_distance = distance;
        }
        false
    });
    println!("Max distance from oxygen is {}", max_distance);
}
