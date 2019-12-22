
use std::fs;

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
                    let input = self.input.remove(0);

                    self.set_memory(p[0] as usize, input);

                    instruction_size = 2;
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

pub fn run() {
    let program = load_program("data/day09.txt".to_string());

    let mut vm = VirtualMachine::new(&program);
    vm.debug = true;

    vm.input.push(1);
    
    loop {
        match vm.run() {
            VirtualMachineState::Output(v) => {
                println!("{}", v);
            }
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }
}