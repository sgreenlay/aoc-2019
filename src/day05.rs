use std::fs;
use std::io;
use std::io::BufRead;

pub fn load_program(filename: String) -> Vec<i32> {
    let file_in = fs::File::open(filename).expect("Can't read file");
    let file_reader = io::BufReader::new(file_in);
    let line: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();

    line[0]
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn decode_instruction(i: u32) -> (u32, Vec<u32>) {
    // Parameter modes are stored in the same value as the instruction's opcode.
    // The opcode is a two-digit number based only on the ones and tens digit of
    // the value, that is, the opcode is the rightmost two digits of the first
    // value in an instruction. Parameter modes are single digits, one per
    // parameter, read right-to-left from the opcode: the first parameter's mode
    // is in the hundreds digit, the second parameter's mode is in the thousands
    // digit, the third parameter's mode is in the ten-thousands digit, and so on.
    // Any missing modes are 0.

    let mut digits: Vec<u32> = i
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
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
            if (digit != 0) && (digit != 1) {
                panic!("Invalid mode");
            }
            modes.push(digit);
        }
        count += 1;
    }

    (i, modes)
}

fn get_parameter(p: i32, m: u32, program: &Vec<i32>) -> i32 {
    match m {
        0 => {
            // mode 0, position mode, causes the parameter to be interpreted as a position
            let pos = p as usize;
            program[pos]
        }
        1 => {
            // mode 1, immediate mode, causes a parameter is interpreted as a value
            p
        }
        _ => {
            panic!("Invalid mode");
        }
    }
}

fn get_mode(mode: &Vec<u32>, i: usize) -> u32 {
    if i >= mode.len() {
        return 0;
    }
    mode[i]
}

fn run_program(program: &Vec<i32>, inputs: &Vec<i32>) {
    let mut output: Vec<i32> = program.clone();
    let mut input: Vec<i32> = inputs.clone();

    let mut ip = 0;
    loop {
        let mut instruction_size = 0;

        print!("[{}] ", ip);

        let raw_i = output[ip] as u32;
        let (i, mode) = decode_instruction(raw_i);
        match i {
            1 => {
                // Opcode 1 adds together numbers read from two positions
                // and stores the result in a third position.

                // The three integers immediately after the opcode tell
                // you these three positions - the first two indicate the
                // positions from which you should read the input values,
                // and the third indicates the position at which the output
                // should be stored.

                let load1 = output[ip + 1];
                let load2 = output[ip + 2];
                let store = output[ip + 3] as usize;

                println!("ADD ({}) {} {} {}", raw_i, load1, load2, store);

                let p1 = get_parameter(load1, get_mode(&mode, 0), &output);
                let p2 = get_parameter(load2, get_mode(&mode, 1), &output);

                output[store] = p1 + p2;

                instruction_size = 4;
            }
            2 => {
                // Opcode 2 works exactly like opcode 1, except it multiplies
                // the two inputs instead of adding them.

                let load1 = output[ip + 1];
                let load2 = output[ip + 2];
                let store = output[ip + 3] as usize;

                println!("MULT ({}) {} {} {}", raw_i, load1, load2, store);

                let p1 = get_parameter(load1, get_mode(&mode, 0), &output);
                let p2 = get_parameter(load2, get_mode(&mode, 1), &output);

                output[store] = p1 * p2;

                instruction_size = 4;
            }
            3 => {
                // Opcode 3 takes a single integer as input and saves it to the
                // position given by its only parameter.

                let store = output[ip + 1] as usize;

                println!("IN ({}) {}", raw_i, store);

                output[store] = input.remove(0);

                instruction_size = 2;
            }
            4 => {
                // Opcode 4 outputs the value of its only parameter.

                let get = output[ip + 1] as usize;

                println!("OUT ({}) {}", raw_i, get);

                println!("{}", output[get]);

                instruction_size = 2;
            }
            5 => {
                // Opcode 5 is jump-if-true: if the first parameter is non-zero,
                // it sets the instruction pointer to the value from the second
                // parameter. Otherwise, it does nothing.

                let load1 = output[ip + 1];
                let load2 = output[ip + 2];

                println!("JUMPIF ({}) {} {}", raw_i, load1, load2);

                let p1 = get_parameter(load1, get_mode(&mode, 0), &output);
                let p2 = get_parameter(load2, get_mode(&mode, 1), &output);

                if p1 != 0 {
                    ip = p2 as usize;
                } else {
                    instruction_size = 3;
                }
            }
            6 => {
                // Opcode 6 is jump-if-false: if the first parameter is zero, it
                // sets the instruction pointer to the value from the second
                // parameter. Otherwise, it does nothing.

                let load1 = output[ip + 1];
                let load2 = output[ip + 2];

                println!("JUMPIF! ({}) {} {}", raw_i, load1, load2);

                let p1 = get_parameter(load1, get_mode(&mode, 0), &output);
                let p2 = get_parameter(load2, get_mode(&mode, 1), &output);

                if p1 == 0 {
                    ip = p2 as usize;
                } else {
                    instruction_size = 3;
                }
            }
            7 => {
                // Opcode 7 is less than: if the first parameter is less than the
                // second parameter, it stores 1 in the position given by the
                // third parameter. Otherwise, it stores 0.

                let load1 = output[ip + 1];
                let load2 = output[ip + 2];
                let store = output[ip + 3] as usize;

                println!("LT ({}) {} {}", raw_i, load1, load2);

                let p1 = get_parameter(load1, get_mode(&mode, 0), &output);
                let p2 = get_parameter(load2, get_mode(&mode, 1), &output);

                if p1 < p2 {
                    output[store] = 1;
                } else {
                    output[store] = 0;
                }

                instruction_size = 4;
            }
            8 => {
                // Opcode 8 is equals: if the first parameter is equal to the second
                // parameter, it stores 1 in the position given by the third
                // parameter. Otherwise, it stores 0.

                let load1 = output[ip + 1];
                let load2 = output[ip + 2];
                let store = output[ip + 3] as usize;

                println!("EQ ({}) {} {}", raw_i, load1, load2);

                let p1 = get_parameter(load1, get_mode(&mode, 0), &output);
                let p2 = get_parameter(load2, get_mode(&mode, 1), &output);

                if p1 == p2 {
                    output[store] = 1;
                } else {
                    output[store] = 0;
                }

                instruction_size = 4;
            }
            99 => {
                // Opcode 99 means that the program is finished and should
                // immediately halt.

                println!("HALT");

                break;
            }
            _ => {
                // Encountering an unknown opcode means something went wrong.

                println!("UNKNOWN ({}) ({})", raw_i, i);

                panic!("Unknown opcode")
            }
        }
        // After an instruction finishes, the instruction pointer increases by
        // the number of values in the instruction.
        ip += instruction_size;
    }
    /*
    // Debugging
    for i in 0..output.len() {
        println!("{}: {}", i, output[i]);
    }
    */
}

pub fn run() {
    let program = load_program("data/day05.txt".to_string());

    // Part 1
    let part1 = vec![1];
    run_program(&program, &part1);

    // Part 1
    let part2 = vec![5];
    run_program(&program, &part2);
}
