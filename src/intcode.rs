use std::fs;
use std::io;
use std::io::BufRead;

pub fn load_program(filename: String) -> Vec<i128> {
    let file_in = fs::File::open(filename).expect("Can't read file");
    let file_reader = io::BufReader::new(file_in);
    let line: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();

    line[0]
        .split(',')
        .map(|line| line.parse::<i128>().unwrap())
        .collect()
}

#[derive(Clone)]
pub struct VirtualMachine {
    ip: usize,
    memory: Vec<i128>,
    input: Vec<i128>,
    relative_base: i128,
}

#[derive(PartialEq)]
pub enum VirtualMachineState {
    Output(i128),
    WaitForInput,
    Terminated,
}

impl VirtualMachine {
    pub fn new(program: &Vec<i128>) -> VirtualMachine {
        VirtualMachine {
            ip: 0,
            memory: program.clone(),
            input: Vec::new(),
            relative_base: 0,
        }
    }
    pub fn add_input(&mut self, input: i128) {
        self.input.push(input);
    }
    pub fn set_memory(&mut self, location: usize, value: i128) {
        if location >= self.memory.len() {
            self.memory.resize(location + 1, 0);
        }
        self.memory[location] = value;
    }
    pub fn get_memory(&self, location: usize) -> i128 {
        if self.memory.len() <= location {
            0
        } else {
            self.memory[location]
        }
    }
    fn get_parameter(&self, i: usize, rw: char) -> i128 {
        let op = self.get_memory(self.ip);
        let p = self.get_memory(self.ip + i);

        // Any missing modes are 0
        let m;
        match i {
            1 => {
                m = (op / 100) % 10;
            }
            2 => {
                m = (op / 1000) % 10;
            }
            3 => {
                m = (op / 10000) % 10;
            }
            _ => {
                panic!("Invalid parameter");
            }
        }

        let ret;
        match m {
            0 => {
                // mode 0, position mode, causes the parameter to be interpreted as a position

                let pos = p as usize;
                match rw {
                    'r' => {
                        ret = self.get_memory(pos);
                    }
                    'w' => {
                        ret = pos as i128;
                    }
                    _ => {
                        panic!("Invalid rw mode");
                    }
                }
            }
            1 => {
                // mode 1, immediate mode, causes a parameter to be interpreted as a value

                match rw {
                    'r' => {
                        ret = p;
                    }
                    _ => {
                        panic!("Invalid rw mode");
                    }
                }
            }
            2 => {
                // mode 2, relative mode, causes a parameter to be interpreted as a position
                // relative to the relative base

                let pos = (self.relative_base + p) as usize;
                match rw {
                    'r' => {
                        ret = self.get_memory(pos);
                    }
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
        ret
    }
    pub fn run(&mut self) -> VirtualMachineState {
        let mut ret: Option<VirtualMachineState> = None;
        while ret.is_none() {
            let op = (self.get_memory(self.ip) as u32) % 100;
            let mut instruction_size = 0;
            match op {
                1 => {
                    // Opcode 1 adds together numbers read from two positions
                    // and stores the result in a third position.

                    // The three integers immediately after the opcode tell
                    // you these three positions - the first two indicate the
                    // positions from which you should read the input values,
                    // and the third indicates the position at which the memory
                    // should be stored.

                    self.set_memory(
                        self.get_parameter(3, 'w') as usize,
                        self.get_parameter(1, 'r') + self.get_parameter(2, 'r'),
                    );

                    instruction_size = 4;
                }
                2 => {
                    // Opcode 2 works exactly like opcode 1, except it multiplies
                    // the two inputs instead of adding them.

                    self.set_memory(
                        self.get_parameter(3, 'w') as usize,
                        self.get_parameter(1, 'r') * self.get_parameter(2, 'r'),
                    );

                    instruction_size = 4;
                }
                3 => {
                    // Opcode 3 takes a single integer as input and saves it to the
                    // position given by its only parameter.

                    if self.input.is_empty() {
                        ret = Some(VirtualMachineState::WaitForInput)
                    } else {
                        let input = self.input.remove(0);

                        self.set_memory(self.get_parameter(1, 'w') as usize, input);

                        instruction_size = 2;
                    }
                }
                4 => {
                    // Opcode 4 outputs the value of its only parameter.

                    ret = Some(VirtualMachineState::Output(self.get_parameter(1, 'r')));

                    instruction_size = 2;
                }
                5 => {
                    // Opcode 5 is jump-if-true: if the first parameter is non-zero,
                    // it sets the instruction pointer to the value from the second
                    // parameter. Otherwise, it does nothing.

                    if self.get_parameter(1, 'r') != 0 {
                        self.ip = self.get_parameter(2, 'r') as usize;
                    } else {
                        instruction_size = 3;
                    }
                }
                6 => {
                    // Opcode 6 is jump-if-false: if the first parameter is zero, it
                    // sets the instruction pointer to the value from the second
                    // parameter. Otherwise, it does nothing.

                    if self.get_parameter(1, 'r') == 0 {
                        self.ip = self.get_parameter(2, 'r') as usize;
                    } else {
                        instruction_size = 3;
                    }
                }
                7 => {
                    // Opcode 7 is less than: if the first parameter is less than the
                    // second parameter, it stores 1 in the position given by the
                    // third parameter. Otherwise, it stores 0.

                    if self.get_parameter(1, 'r') < self.get_parameter(2, 'r') {
                        self.set_memory(self.get_parameter(3, 'w') as usize, 1);
                    } else {
                        self.set_memory(self.get_parameter(3, 'w') as usize, 0);
                    }

                    instruction_size = 4;
                }
                8 => {
                    // Opcode 8 is equals: if the first parameter is equal to the second
                    // parameter, it stores 1 in the position given by the third
                    // parameter. Otherwise, it stores 0.

                    if self.get_parameter(1, 'r') == self.get_parameter(2, 'r') {
                        self.set_memory(self.get_parameter(3, 'w') as usize, 1);
                    } else {
                        self.set_memory(self.get_parameter(3, 'w') as usize, 0);
                    }

                    instruction_size = 4;
                }
                9 => {
                    // Opcode 9 adjusts the relative base by the value of its only
                    // parameter.

                    let offset = self.get_parameter(1, 'r');
                    self.relative_base += offset;

                    instruction_size = 2;
                }
                99 => {
                    // Opcode 99 means that the program is finished and should
                    // immediately halt.

                    ret = Some(VirtualMachineState::Terminated);

                    break;
                }
                _ => {
                    // Encountering an unknown opcode means something went wrong.
                    panic!("Unknown opcode")
                }
            }

            // After an instruction finishes, the instruction pointer increases by
            // the number of values in the instruction.
            self.ip += instruction_size;
        }
        ret.unwrap()
    }
}
