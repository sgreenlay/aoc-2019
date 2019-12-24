
use std::cmp;
use std::fmt;
use std::fs;

use std::collections::HashMap;

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
        self.y.cmp(&other.y)	
            .then_with(|| self.x.cmp(&other.x))
    }	
}	

impl PartialOrd for Point {	
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {	
        Some(self.cmp(other))	
    }	
}

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

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn rotate_left(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => {
            Direction::Left
        }
        Direction::Down => {
            Direction::Right
        }
        Direction::Left => {
            Direction::Down
        }
        Direction::Right => {
            Direction::Up
        }
    }
}

fn rotate_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => {
            Direction::Right
        }
        Direction::Down => {
            Direction::Left
        }
        Direction::Left => {
            Direction::Up
        }
        Direction::Right => {
            Direction::Down
        }
    }
}

fn move_forward(p: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::Up => {
            Point{x: p.x, y: p.y - 1 }
        }
        Direction::Down => {
            Point{x: p.x, y: p.y + 1 }
        }
        Direction::Left => {
            Point{x: p.x - 1, y: p.y }
        }
        Direction::Right => {
            Point{x: p.x + 1, y: p.y }
        }
    }
}

enum State {
    OutputColor,
    OutputOrientationChange
}

fn run_robot(program: &Vec<i128>, starting: i128) -> HashMap<Point, i128> {
    let mut map: HashMap<Point, i128> = HashMap::new();
    let mut robot: (Point, Direction, State) = (
        Point{ x: 0, y: 0 },
        Direction::Up,
        State::OutputColor
    );
    map.insert(robot.0, starting);

    let mut vm = VirtualMachine::new(program);
    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                // The program uses input instructions to access the robot's camera: 
                //     0 if the robot is over a black panel or 
                //     1 if the robot is over a white panel

                if map.contains_key(&robot.0) {
                    vm.input.push(map[&robot.0]);
                } else {
                    // All of the panels are currently black.
                    vm.input.push(0);
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
            let p = Point{x: x, y: y};
            if part2.contains_key(&p) && (part2[&p] == 1) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}