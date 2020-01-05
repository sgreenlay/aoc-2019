use std::cmp;
use std::fmt;
use std::fs;

use crate::intcode::{VirtualMachine, VirtualMachineState};

use std::collections::HashMap;

use regex::Regex;

fn load_program(filename: String) -> Vec<i128> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .split(',')
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}

fn run_program(program: &Vec<i128>, input: &String) -> (String, i128) {
    let mut vm = VirtualMachine::new(program);

    if !input.is_empty() {
        // Force the vacuum robot to wake up by changing the value in your ASCII
        // program at address 0 from 1 to 2.
        vm.set_memory(0, 2);

        for ch in input.chars() {
            vm.add_input(ch as i128);
        }
    }

    let mut s: String = String::new();
    let mut last_output = 0;

    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                println!("Waiting for Input");
                break;
            }
            VirtualMachineState::Output(v) => {
                last_output = v;
                s.push((v as u8).into());
            }
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }
    (s, last_output)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Turn {
    Left,
    Right,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Turn::Left => write!(f, "L"),
            Turn::Right => write!(f, "R"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_to(&self, d: Direction) -> Turn {
        match self {
            Direction::North => match d {
                Direction::West => Turn::Left,
                Direction::East => Turn::Right,
                _ => {
                    panic!("Invalid turn");
                }
            },
            Direction::South => match d {
                Direction::West => Turn::Right,
                Direction::East => Turn::Left,
                _ => {
                    panic!("Invalid turn");
                }
            },
            Direction::East => match d {
                Direction::North => Turn::Left,
                Direction::South => Turn::Right,
                _ => {
                    panic!("Invalid turn");
                }
            },
            Direction::West => match d {
                Direction::North => Turn::Right,
                Direction::South => Turn::Left,
                _ => {
                    panic!("Invalid turn");
                }
            },
            _ => {
                panic!("Invalid turn");
            }
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::South => write!(f, "S"),
            Direction::East => write!(f, "E"),
            Direction::West => write!(f, "W"),
            _ => write!(f, "X"),
        }
    }
}

struct Robot {
    x: usize,
    y: usize,
    d: Direction,
}

impl Robot {
    fn move_forward(&mut self) {
        match self.d {
            Direction::North => {
                self.y -= 1;
            }
            Direction::South => {
                self.y += 1;
            }
            Direction::East => {
                self.x += 1;
            }
            Direction::West => {
                self.x -= 1;
            }
            _ => {
                panic!("Invalid robot orientation");
            }
        }
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.d)
    }
}

pub fn run() {
    let program = load_program("data/day17.txt".to_string());

    let output = run_program(&program, &"".to_string());
    let scaffold: Vec<Vec<char>> = output
        .0
        .split("\n")
        .filter_map(|s| {
            if s.len() > 0 {
                Some(s.chars().collect())
            } else {
                None
            }
        })
        .collect();
    let (height, width) = (scaffold.len(), scaffold[0].len());
    // Part 1
    let is_intersection = |x: usize, y: usize| -> bool {
        (scaffold[y][x] == '#')
            && (scaffold[y - 1][x] == '#')
            && (scaffold[y + 1][x] == '#')
            && (scaffold[y][x - 1] == '#')
            && (scaffold[y][x + 1] == '#')
    };

    let mut sum = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if is_intersection(x, y) {
                let alignment_parameter = x * y;
                sum += alignment_parameter;
            }
        }
    }
    println!("Sum of alignment parameters: {}", sum);

    // Part 2
    let get_tile = |x: i32, y: i32| -> char {
        if (x < 0) || (x >= (width as i32)) || (y < 0) || (y >= (height as i32)) {
            '.'
        } else {
            match scaffold[y as usize][x as usize] {
                '.' => '.',
                _ => '#',
            }
        }
    };
    let get_junction_unique_directions = |x: usize, y: usize| -> Option<(Direction, Direction)> {
        let ix = x as i32;
        let iy = y as i32;

        if (get_tile(ix, iy) != '.')
            && ((get_tile(ix, iy - 1) != get_tile(ix, iy + 1))
                || (get_tile(ix - 1, iy) != get_tile(ix + 1, iy)))
        {
            let vertical;
            if get_tile(ix, iy - 1) != '.' {
                vertical = Direction::North;
            } else if get_tile(ix, iy + 1) != '.' {
                vertical = Direction::South;
            } else {
                vertical = Direction::None;
            }

            let horizontal;
            if get_tile(ix - 1, iy) != '.' {
                horizontal = Direction::West;
            } else if get_tile(ix + 1, iy) != '.' {
                horizontal = Direction::East;
            } else {
                horizontal = Direction::None;
            }

            Some((horizontal, vertical))
        } else {
            None
        }
    };
    let get_robot = |x: usize, y: usize| -> Option<Robot> {
        let tile = scaffold[y][x];

        match tile {
            '^' => Some(Robot {
                x: x,
                y: y,
                d: Direction::North,
            }),
            'v' => Some(Robot {
                x: x,
                y: y,
                d: Direction::South,
            }),
            '<' => Some(Robot {
                x: x,
                y: y,
                d: Direction::West,
            }),
            '>' => Some(Robot {
                x: x,
                y: y,
                d: Direction::East,
            }),
            _ => None,
        }
    };

    let mut find_robot: Option<Robot> = None;
    for y in 0..height {
        for x in 0..width {
            find_robot = get_robot(x, y);
            if find_robot.is_some() {
                break;
            }
        }
        if find_robot.is_some() {
            break;
        }
    }
    let mut robot = find_robot.unwrap();

    let mut distance = 0;
    let mut turn: Option<Turn> = None;
    let mut path: Vec<(Turn, usize)> = Vec::new();

    // Orient the robot in the correct direction
    let start = get_junction_unique_directions(robot.x, robot.y).unwrap();
    if &start.0 != &Direction::None {
        match &start.0 {
            Direction::East | Direction::West => {
                if robot.d != start.0 {
                    turn = Some(robot.d.turn_to(start.0));
                    robot.d = start.0;
                }
                distance += 1;
                robot.move_forward();
            }
            _ => panic!("Invalid"),
        }
    } else if &start.1 != &Direction::None {
        match &start.1 {
            Direction::North | Direction::South => {
                if robot.d != start.1 {
                    turn = Some(robot.d.turn_to(start.1));
                    robot.d = start.1;
                }
                distance += 1;
                robot.move_forward();
            }
            _ => panic!("Invalid"),
        }
    } else {
        panic!("Invalid start");
    }

    // Find a path for the robot from start to end
    loop {
        let is_j = get_junction_unique_directions(robot.x, robot.y);
        if is_j.is_some() {
            let j = is_j.unwrap();

            path.push((turn.unwrap(), distance));
            distance = 0;

            match robot.d {
                Direction::East | Direction::West => match &j.1 {
                    Direction::North | Direction::South => {
                        turn = Some(robot.d.turn_to(j.1));
                        robot.d = j.1;
                    }
                    Direction::None => {
                        break;
                    }
                    _ => {
                        panic!("Invalid junction");
                    }
                },
                Direction::North | Direction::South => match &j.0 {
                    Direction::East | Direction::West => {
                        turn = Some(robot.d.turn_to(j.0));
                        robot.d = j.0;
                    }
                    Direction::None => {
                        break;
                    }
                    _ => {
                        panic!("Invalid junction");
                    }
                },
                _ => {
                    panic!("Invalid junction");
                }
            }
        }

        distance += 1;
        robot.move_forward();
    }

    // Find the unique unique_directions in the path
    let mut unique_directions: HashMap<(Turn, usize), char> = HashMap::new();
    let mut next_unique_directions = 'a';
    for &p in &path {
        if !unique_directions.contains_key(&p) {
            unique_directions.insert(p, next_unique_directions);
            next_unique_directions = ((next_unique_directions as u8) + 1) as char;
        }
    }
    let main: String = path.iter().map(|p| unique_directions[p]).collect();

    // Find 3 unique substrings that can be used to construct main
    let split_by_and_remove = |b: &String, strings: &Vec<String>| -> Vec<String> {
        strings
            .iter()
            .filter_map(|s| {
                if s.cmp(&b) == cmp::Ordering::Equal {
                    None
                } else {
                    let remaining: Vec<String> = s
                        .split(b)
                        .filter_map(|s| {
                            if s.len() > 0 {
                                Some(s.chars().collect())
                            } else {
                                None
                            }
                        })
                        .collect();
                    if remaining.len() == 0 {
                        None
                    } else {
                        Some(remaining)
                    }
                }
            })
            .flatten()
            .collect::<Vec<_>>()
    };

    let mut is_abc: Option<(String, String, String)> = None;
    for i in 2..=main.len() {
        let a: String = main[0..i].to_string();
        let remaining_a = split_by_and_remove(&a, &vec![main.clone()]);
        if (remaining_a.len() == 0) || (remaining_a[0].len() < 2) {
            continue;
        }
        for j in 2..=remaining_a[0].len() {
            let b: String = remaining_a[0][0..j].to_string();
            let remaining_b = split_by_and_remove(&b, &remaining_a);
            if (remaining_b.len() == 0) || (remaining_b[0].len() < 2) {
                continue;
            }
            for k in 2..=remaining_b[0].len() {
                let c: String = remaining_b[0][0..k].to_string();
                let remaining_c = split_by_and_remove(&c, &remaining_b);
                if remaining_c.len() == 0 {
                    is_abc = Some((a.clone(), b.clone(), c.clone()));
                    break;
                }
            }
            if is_abc.is_some() {
                break;
            }
        }
        if is_abc.is_some() {
            break;
        }
    }

    if is_abc.is_none() {
        panic!("Couldn't find a set of unique substrings!");
    }

    // Generate Main
    let (a, b, c) = is_abc.unwrap();

    let replace_a = Regex::new(&format!("{}", a).to_string()).unwrap();
    let result = replace_a.replace_all(&main, "A");

    let replace_b = Regex::new(&format!("{}", b).to_string()).unwrap();
    let result = replace_b.replace_all(&result, "B");

    let replace_c = Regex::new(&format!("{}", c).to_string()).unwrap();
    let result = replace_c.replace_all(&result, "C");

    let main: String = result
        .chars()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // Generate Functions
    let (mut fn_a, mut fn_b, mut fn_c) = (a.clone(), b.clone(), c.clone());
    for d in unique_directions {
        let replace_d = Regex::new(&format!("{}", d.1).to_string()).unwrap();

        let direction = d.0;
        let replacement_d: String = format!("{},{},", direction.0, direction.1);

        fn_a = replace_d.replace_all(&fn_a, &*replacement_d).to_string();
        fn_b = replace_d.replace_all(&fn_b, &*replacement_d).to_string();
        fn_c = replace_d.replace_all(&fn_c, &*replacement_d).to_string();
    }

    // Remove trailing ','
    fn_a.pop();
    fn_b.pop();
    fn_c.pop();

    // Run the program
    let input = vec![
        /* Main: */ main,
        /* Function A: */ fn_a,
        /* Function B: */ fn_b,
        /* Function C: */ fn_c,
        /* Continuous video feed? */ "n".to_string(),
        "".to_string(),
    ]
    .join("\n");

    let output = run_program(&program, &input);
    println!("{}", output.1);
}
