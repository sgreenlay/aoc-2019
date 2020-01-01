
use std::cmp;
use std::fmt;
use std::fs;
use std::ops;

use std::collections::HashMap;

fn load_program(filename : String) -> Vec<i128> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .split(',')
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}

struct VirtualMachine {
    ip: usize,
    memory: HashMap<usize,i128>,
    input: Vec<i128>,
    relative_base: i128,
    debug: bool
}

#[derive(PartialEq)]
enum VirtualMachineState {
    Output(i128),
    WaitForInput,
    Terminated
}

impl VirtualMachine {
    fn new(program: &Vec<i128>) -> VirtualMachine {
        let mut vm = VirtualMachine {
            ip: 0,
            memory: HashMap::new(),
            input: Vec::new(),
            relative_base: 0,
            debug: false
        };
        for i in 0..program.len() {
            vm.memory.insert(i, program[i]);
        }
        vm
    }
    fn set_memory(&mut self, location: usize, value: i128) {
        self.memory.insert(location, value);
    }
    fn get_memory(&self, location: usize) -> i128 {
        match self.memory.get(&location) {
            Some(n) => *n,
            None => 0
        }
    }
    fn next_instruction(&self) -> (u32, Vec<u32>) {
        let i = self.get_memory(self.ip) as u32;

        if self.debug {
            print!("[{}] {}", self.ip, i);
        }

        // Parameter modes are stored in the same value as the instruction's opcode. 
        // The opcode is a two-digit number based only on the ones and tens digit of 
        // the value, that is, the opcode is the rightmost two digits of the first 
        // value in an instruction. Parameter modes are single digits, one per 
        // parameter, read right-to-left from the opcode: the first parameter's mode 
        // is in the hundreds digit, the second parameter's mode is in the thousands 
        // digit, the third parameter's mode is in the ten-thousands digit, and so on.
        // Any missing modes are 0.
    
        let mut digits: Vec<u32> = i.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        digits.reverse();
    
        let mut i: u32 = 0;
        let mut modes: Vec<u32> = Vec::new();
    
        let mut count = 0;
        for digit in digits {
            if count == 0 {
                i = digit;
            } else if count == 1 {
                i += digit * 10;
            } else {
                modes.push(digit);
            }
            count += 1;
        }
    
        (i, modes)
    }
    fn get_parameters(&self, rw: Vec<char>, mode: Vec<u32>) -> Vec<i128> {
        let ret: Vec<i128> = (0..rw.len()).map(|i| {
            let p = self.get_memory(self.ip + 1 + i);

            // Any missing modes are 0
            let m;
            if i < mode.len() {
                m = mode[i];
            } else {
                m = 0;
            }

            let ret;
            match m {
                0 => {
                    // mode 0, position mode, causes the parameter to be interpreted as a position

                    let pos = p as usize;
                    match rw[i] {
                        'r' => {
                            ret = self.get_memory(pos);
                        },
                        'w' => {
                            ret = pos as i128;
                        },
                        _ => {
                            panic!("Invalid rw mode");
                        }
                    }                    
                }
                1 => {
                    // mode 1, immediate mode, causes a parameter to be interpreted as a value
                    
                    match rw[i] {
                        'r' => {
                            ret = p;
                        },
                        _ => {
                            panic!("Invalid rw mode");
                        }
                    }
                }
                2 => {
                    // mode 2, relative mode, causes a parameter to be interpreted as a position 
                    // relative to the relative base

                    let pos = (self.relative_base + p) as usize;
                    match rw[i] {
                        'r' => {
                            ret = self.get_memory(pos);
                        },
                        'w' => {
                            ret = pos as i128;
                        }
                        _ => {
                            panic!("Invalid rw mode");
                        }
                    }
                }
                _ => {
                    panic!("Invalid mode");
                }
            }

            if self.debug {
                print!(" {} [mode:{}, value:{}]", p, m, ret);
            }
            ret
        }).collect();

        ret
    }
    pub fn run(&mut self) -> VirtualMachineState {
        let mut ret: Option<VirtualMachineState> = None;
        while ret.is_none() {
            let (i, mode) = self.next_instruction();
            let mut instruction_size = 0;
            match i {
                1 => {
                    // Opcode 1 adds together numbers read from two positions 
                    // and stores the result in a third position.
                    
                    // The three integers immediately after the opcode tell 
                    // you these three positions - the first two indicate the 
                    // positions from which you should read the input values, 
                    // and the third indicates the position at which the memory 
                    // should be stored.

                    if self.debug {
                        print!(" ADD");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r', 'w'], mode);
                    self.set_memory(p[2] as usize, p[0] + p[1]);

                    instruction_size = 4;
                }
                2 => {
                    // Opcode 2 works exactly like opcode 1, except it multiplies 
                    // the two inputs instead of adding them.

                    if self.debug {
                        print!(" MULT");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r', 'w'], mode);
                    self.set_memory(p[2] as usize, p[0] * p[1]);

                    instruction_size = 4;
                }
                3 => {
                    // Opcode 3 takes a single integer as input and saves it to the 
                    // position given by its only parameter.

                    if self.debug {
                        print!(" IN");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['w'], mode);

                    if self.input.is_empty() {
                        ret = Some(VirtualMachineState::WaitForInput)
                    } else {
                        let input = self.input.remove(0);

                        self.set_memory(p[0] as usize, input);

                        instruction_size = 2;
                    }
                }
                4 => {
                    // Opcode 4 outputs the value of its only parameter.

                    if self.debug {
                        print!(" OUT");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r'], mode);
                    ret = Some(VirtualMachineState::Output(p[0]));

                    instruction_size = 2;
                }
                5 => {
                    // Opcode 5 is jump-if-true: if the first parameter is non-zero, 
                    // it sets the instruction pointer to the value from the second 
                    // parameter. Otherwise, it does nothing.

                    if self.debug {
                        print!(" JMP");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r'], mode);
                    if p[0] != 0 {
                        self.ip = p[1] as usize;
                    } else {
                        instruction_size = 3;
                    }
                }
                6 => {
                    // Opcode 6 is jump-if-false: if the first parameter is zero, it 
                    // sets the instruction pointer to the value from the second 
                    // parameter. Otherwise, it does nothing.

                    if self.debug {
                        print!(" JMP!");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r'], mode);
                    if p[0] == 0 {
                        self.ip = p[1] as usize;
                    } else {
                        instruction_size = 3;
                    }
                }
                7 => {
                    // Opcode 7 is less than: if the first parameter is less than the 
                    // second parameter, it stores 1 in the position given by the 
                    // third parameter. Otherwise, it stores 0.

                    if self.debug {
                        print!(" LT");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r', 'w'], mode);
                    if p[0] < p[1] {
                        self.set_memory(p[2] as usize, 1);
                    } else {
                        self.set_memory(p[2] as usize, 0);
                    }

                    instruction_size = 4;
                }
                8 => {
                    // Opcode 8 is equals: if the first parameter is equal to the second 
                    // parameter, it stores 1 in the position given by the third 
                    // parameter. Otherwise, it stores 0.

                    if self.debug {
                        print!(" EQ");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r', 'w'], mode);
                    if p[0] == p[1] {
                        self.set_memory(p[2] as usize, 1);
                    } else {
                        self.set_memory(p[2] as usize, 0);
                    }

                    instruction_size = 4;
                }
                9 => {
                    // Opcode 9 adjusts the relative base by the value of its only 
                    // parameter.

                    if self.debug {
                        print!(" RB");
                    }

                    let p: Vec<i128> = self.get_parameters(vec!['r'], mode);

                    let offset = p[0];
                    self.relative_base += offset;

                    if self.debug {
                        print!(" -> [rb:{}]", self.relative_base);
                    }

                    instruction_size = 2;
                }
                99 => {
                    // Opcode 99 means that the program is finished and should 
                    // immediately halt.

                    if self.debug {
                        print!(" HALT");
                    }

                    ret = Some(VirtualMachineState::Terminated);

                    break;
                }
                _ => {
                    // Encountering an unknown opcode means something went wrong.
                    panic!("Unknown opcode")
                }
            }

            if self.debug {
                println!("");
            }

            // After an instruction finishes, the instruction pointer increases by 
            // the number of values in the instruction.
            self.ip += instruction_size;
        }
        ret.unwrap()
    }
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
        self.y.cmp(&other.y)	
            .then_with(|| self.x.cmp(&other.x))
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
    directions.insert(Direction::North, Point{x: 0, y: -1});
    directions.insert(Direction::South, Point{x: 0, y: 1});
    directions.insert(Direction::East, Point{x: 1, y: 0});
    directions.insert(Direction::West, Point{x: -1, y: 0});
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

    let mut current = Point{x: 0, y: 0};
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
                    },
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
                        vm.input.push(1);
                    },
                    Direction::South => {
                        vm.input.push(2);
                    },
                    Direction::East => {
                        vm.input.push(3);
                    },
                    Direction::West => {
                        vm.input.push(4);
                    },
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

fn bredth_first_search_map(start: &Point, map: &HashMap<Point, i128>, stop: &mut dyn FnMut(Point, i128) -> bool) {
    let mut frontier: Vec<(Point, i128)> = Vec::new();
    let mut visited: HashMap<Point, i128> = HashMap::new();

    if !map.contains_key(start) {
        panic!("Invalid start point");
    }

    visited.insert(*start, map[start]);
    frontier.push((*start, 0));

    while !frontier.is_empty() {
        let p = frontier.remove(0);

        for d in DIRECTIONS.iter() {
            let p_next = p.0 + *d.1;
            if !visited.contains_key(&p_next) && map.contains_key(&p_next) {
                let distance = p.1 + 1;

                let next = map[&p_next];
                if next == 0 {
                    continue;
                }

                frontier.push((p_next, distance));
                visited.insert(p_next, distance);

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
    let start = Point{x: 0, y: 0};
    let mut oxygen = Point{x: 0, y: 0};
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