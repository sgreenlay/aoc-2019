
#[derive(Clone)]
pub struct VirtualMachine {
    ip: usize,
    memory: Vec<i128>,
    input: Vec<i128>,
    relative_base: i128
}

#[derive(PartialEq)]
pub enum VirtualMachineState {
    Output(i128),
    WaitForInput,
    Terminated
}

impl VirtualMachine {
    pub fn new(program: &Vec<i128>) -> VirtualMachine {
        VirtualMachine {
            ip: 0,
            memory: program.clone(),
            input: Vec::new(),
            relative_base: 0
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
    fn next_instruction(&self) -> (u32, Vec<u32>) {
        let op = self.get_memory(self.ip) as u32;

        // Parameter modes are stored in the same value as the instruction's opcode. 
        // The opcode is a two-digit number based only on the ones and tens digit of 
        // the value, that is, the opcode is the rightmost two digits of the first 
        // value in an instruction. Parameter modes are single digits, one per 
        // parameter, read right-to-left from the opcode: the first parameter's mode 
        // is in the hundreds digit, the second parameter's mode is in the thousands 
        // digit, the third parameter's mode is in the ten-thousands digit, and so on.
        // Any missing modes are 0.
    
        let mut i: u32 = 0;
        let mut modes: Vec<u32> = Vec::new();
        
        let mut mult = 1;
        for idx in 0..5 {
            let digit = (op / mult) % 10;
            
            match idx {
                0 => {
                    i = digit;
                },
                1 => {
                    i += digit * 10;
                },
                _ => {
                    modes.push(digit);
                }
            }

            mult *= 10;
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

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r', 'w'], mode);
                    self.set_memory(p[2] as usize, p[0] + p[1]);

                    instruction_size = 4;
                }
                2 => {
                    // Opcode 2 works exactly like opcode 1, except it multiplies 
                    // the two inputs instead of adding them.

                    let p: Vec<i128> = self.get_parameters(vec!['r', 'r', 'w'], mode);
                    self.set_memory(p[2] as usize, p[0] * p[1]);

                    instruction_size = 4;
                }
                3 => {
                    // Opcode 3 takes a single integer as input and saves it to the 
                    // position given by its only parameter.

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

                    let p: Vec<i128> = self.get_parameters(vec!['r'], mode);
                    ret = Some(VirtualMachineState::Output(p[0]));

                    instruction_size = 2;
                }
                5 => {
                    // Opcode 5 is jump-if-true: if the first parameter is non-zero, 
                    // it sets the instruction pointer to the value from the second 
                    // parameter. Otherwise, it does nothing.

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

                    let p: Vec<i128> = self.get_parameters(vec!['r'], mode);

                    let offset = p[0];
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