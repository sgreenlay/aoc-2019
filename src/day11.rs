use std::cmp;
use std::fmt;
use std::fs;

use crate::intcode::{VirtualMachine, VirtualMachineState};

use std::collections::HashMap;

fn load_program(filename: String) -> Vec<i128> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .split(',')
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}

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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn rotate_left(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn rotate_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn move_forward(p: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::Up => Point { x: p.x, y: p.y - 1 },
        Direction::Down => Point { x: p.x, y: p.y + 1 },
        Direction::Left => Point { x: p.x - 1, y: p.y },
        Direction::Right => Point { x: p.x + 1, y: p.y },
    }
}

enum State {
    OutputColor,
    OutputOrientationChange,
}

fn run_robot(program: &Vec<i128>, starting: i128) -> HashMap<Point, i128> {
    let mut map: HashMap<Point, i128> = HashMap::new();
    let mut robot: (Point, Direction, State) =
        (Point { x: 0, y: 0 }, Direction::Up, State::OutputColor);
    map.insert(robot.0, starting);

    let mut vm = VirtualMachine::new(program);
    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                // The program uses input instructions to access the robot's camera:
                //     0 if the robot is over a black panel or
                //     1 if the robot is over a white panel

                if map.contains_key(&robot.0) {
                    vm.add_input(map[&robot.0]);
                } else {
                    // All of the panels are currently black.
                    vm.add_input(0);
                }
            }
            VirtualMachineState::Output(v) => {
                match robot.2 {
                    State::OutputColor => {
                        // First, it will output a value indicating the color to paint the
                        // panel the robot is over:
                        //     0 means to paint the panel black, and
                        //     1 means to paint the panel white
                        map.insert(robot.0, v);

                        robot.2 = State::OutputOrientationChange;
                    }
                    State::OutputOrientationChange => {
                        // Second, it will output a value indicating the direction the robot
                        // should turn:
                        //     0 means it should turn left 90 degrees, and
                        //     1 means it should turn right 90 degrees
                        // After the robot turns, it should always move forward exactly one panel.

                        match v {
                            0 => {
                                robot.1 = rotate_left(&robot.1);
                            }
                            1 => {
                                robot.1 = rotate_right(&robot.1);
                            }
                            _ => {
                                panic!("Invalid Orientation Change");
                            }
                        }

                        // After the robot turns, it should always move forward exactly one panel.
                        robot.0 = move_forward(&robot.0, &robot.1);

                        robot.2 = State::OutputColor;
                    }
                }
            }
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }
    map
}

pub fn run() {
    let program = load_program("data/day11.txt".to_string());

    // Part 1
    let part1 = run_robot(&program, 0);
    println!("{}", part1.keys().count());

    // Part 2
    let part2 = run_robot(&program, 1);
    let min: &Point = part2.keys().min_by_key(|&x| x).unwrap();
    let max: &Point = part2.keys().max_by_key(|&x| x).unwrap();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let p = Point { x: x, y: y };
            if part2.contains_key(&p) && (part2[&p] == 1) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
