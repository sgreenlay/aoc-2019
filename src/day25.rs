use std::collections::HashMap;
use std::io;
use std::fmt;

use regex::Regex;

use lazy_static;

use crate::intcode::{VirtualMachine, VirtualMachineState, load_program};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "north"),
            Direction::South => write!(f, "south"),
            Direction::East => write!(f, "east"),
            Direction::West => write!(f, "west"),
        }
    }
}

fn run_interactive(
    vm: &mut VirtualMachine,
    output: &mut String
) {
    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                if !output.is_empty() {
                    print!("{}", output);
                    *output = String::new();
                }
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        for ch in input.chars() {
                            if ch == '\r' {
                                continue;
                            }
                            vm.add_input(ch as i128);
                        }
                    }
                    Err(error) => println!("error: {}", error),
                }
            },
            VirtualMachineState::Output(v) => {
                let ch = (v as u8) as char;
                output.push(ch);
            },
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }
}

fn parse_output(
    output: &String
) -> (Option<String>, Vec<Direction>, Vec<String>) {
    let mut name: Option<String> = None;
    let mut directions: Vec<Direction> = Vec::new();
    let mut items: Vec<String> = Vec::new();

    let lines: Vec<String> = output.split('\n')
        .filter_map(|s: &str| {
            if s.len() == 0 {
                None
            } else {
                Some(s.chars().collect())
            }
        })
        .collect();

    for line in lines.iter() {
        lazy_static! {
            // == XXX ==
            static ref ROOM_RE: Regex = Regex::new(r"== (.*) ==").unwrap();
        }

        if ROOM_RE.is_match(&line) {
            for line_cap in ROOM_RE.captures_iter(&line) {
                let room = &line_cap[0];
                name = Some(room.to_string());
                break;
            }
        } else {
            let ch = line.chars().next().unwrap();
            if ch == '-' {
                if line.eq("- north") {
                    directions.push(Direction::North);
                } else if line.eq("- south") {
                    directions.push(Direction::South);
                } else if line.eq("- east") {
                    directions.push(Direction::East);
                } else if line.eq("- west") {
                    directions.push(Direction::West);
                } else {
                    items.push(line.split_at(2).1.to_string());
                }
            }
        }
    }

    (name, directions, items)
}

fn visit_all_the_rooms(
    vm: &mut VirtualMachine,
    output: &mut String,
    visit_room: &mut dyn FnMut(&String, &Vec<Direction>, &Vec<String>, &mut Vec<String>) -> bool
) {
    let mut frontier: Vec<Direction> = Vec::new();
    let mut inputs: Vec<String> = Vec::new();

    let mut current: String = String::new();
    let mut rooms_visited: HashMap<String, Vec<Direction>> = HashMap::new();

    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                if inputs.is_empty() {
                    if !output.is_empty() {
                        let (room, directions, items) = parse_output(&output);

                        let next = room.unwrap();
                        if next.eq(&current) {
                            frontier.pop();
                        } else {
                            current = next;
    
                            if !rooms_visited.contains_key(&current) {
                                if visit_room(&current, &directions, &items, &mut inputs) {
                                    break;
                                }
                                rooms_visited.insert(current.clone(), directions.clone());
                            }
                        }
                    }

                    if !rooms_visited.contains_key(&current) {
                        panic!("Haven't visited this room!");
                    }

                    let room = rooms_visited.get_mut(&current).unwrap();
                    let next: Direction;
                    if room.is_empty() {
                        if frontier.is_empty() {
                            break;
                        } else {
                            next = frontier.pop().unwrap().inverse();
                        }
                    } else {
                        next = room.pop().unwrap();
                        frontier.push(next);
                    }

                    let next_room = format!("{}\n", next).to_string();
                    inputs.push(next_room);

                    *output = String::new();
                }
                if !inputs.is_empty() {
                    let input = inputs.remove(0);
                    for ch in input.chars() {
                        vm.add_input(ch as i128);
                    }
                }
            },
            VirtualMachineState::Output(v) => {
                let ch = (v as u8) as char;

                output.push(ch);
            },
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }
}

fn run_with_input(
    vm: &mut VirtualMachine,
    output: &mut String,
    inputs: &mut Vec<String>
) {
    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                if inputs.is_empty() {
                    break;
                } else {
                    let input = inputs.remove(0);
                    for ch in input.chars() {
                        vm.add_input(ch as i128);
                    }
                    *output = String::new();
                }
            },
            VirtualMachineState::Output(v) => {
                let ch = (v as u8) as char;

                output.push(ch);
            },
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }
}

fn generate_all_combinations<T: Clone>(arr: Vec<T>) -> Vec<Vec<T>> {
    let mut output: Vec<Vec<T>> = Vec::new();

    let n = arr.len();
    let i_max = 2u32.pow(n as u32);

    for i in 0..i_max {
        let mut entry: Vec<T> = Vec::new();
        let mut m = 1;
        for j in 0..n {
            if (i & m) > 0 {		
                entry.push(arr[j].clone());
            }
            m = m << 1;
        }
        if !entry.is_empty() {
            output.push(entry);
        }
    }
    return output;
}

pub fn run() {
    let program = load_program("data/day25.txt".to_string());
    let mut vm = VirtualMachine::new(&program);

    let mut output: String = String::new();

    let interactive = false;
    if interactive {
        run_interactive(&mut vm, &mut output);
    } else {
        let mut rooms: HashMap<String, (Vec<Direction>, Vec<String>)> = HashMap::new();

        let mut cache_room = |name: &String, directions: &Vec<Direction>, items: &Vec<String>, _: &mut Vec<String>| -> bool {
            rooms.insert(name.clone(), (directions.clone(), items.clone()));
            false
        };
        visit_all_the_rooms(&mut vm, &mut output, &mut cache_room);

        let mut pick_up_all_things = |_: &String, _: &Vec<Direction>, items: &Vec<String>, inputs: &mut Vec<String>| -> bool {
            for i in items {
                let dont_pick_up = vec![
                    "escape pod",
                    "giant electromagnet",
                    "infinite loop",
                    "molten lava",
                    "photons",
                ];
                let mut pick_up = true;
                for d in dont_pick_up {
                    if i.eq(d) {
                        pick_up = false;
                        break;
                    }
                }

                if pick_up {
                    let take = format!("take {}\n", i).to_string();
                    inputs.push(take);
                }
            }
            false
        };
        visit_all_the_rooms(&mut vm, &mut output, &mut pick_up_all_things);

        let mut find_the_security_checkpoint = |name: &String, _: &Vec<Direction>, _: &Vec<String>, _: &mut Vec<String>| -> bool {
            name.eq(&"== Security Checkpoint ==")
        };
        visit_all_the_rooms(&mut vm, &mut output, &mut find_the_security_checkpoint);

        let (current, _, _) = parse_output(&output);

        let mut input: Vec<String> = vec![
            "inv\n".to_string()
        ];
        run_with_input(&mut vm, &mut output, &mut input);
        let (_, _, mut items) = parse_output(&output);
        items.sort();

        let item_combinations = generate_all_combinations(items);
        for items in item_combinations {
            println!("Trying:");
            for i in items {
                println!("{}", i);
            }
        }

        // TODO: Find the right combination of items...

        run_interactive(&mut vm, &mut output);
    }
}
