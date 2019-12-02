
use std::fs;

fn load_program(filename : String) -> Vec<usize> {
    let input = fs::read_to_string(filename)
        .expect("Can't read file");

    let values: Vec<&str> = input.split(',').collect();
    let program: Vec<usize> = values.iter().map(|x| x.parse::<i32>().unwrap() as usize).collect();

    program
}

fn run_program(program : &Vec<usize>, noun : usize, verb : usize) -> usize {
    let mut output : Vec<usize> = program.clone();

    output[1] = noun;
    output[2] = verb;

    let mut ip = 0;
    loop {
        let instruction_size;
        match output[ip] {
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
                let store = output[ip + 3];

                output[store] = output[load1] + output[load2];

                instruction_size = 4;
            }
            2 => {
                // Opcode 2 works exactly like opcode 1, except it multiplies 
                // the two inputs instead of adding them.

                let load1 = output[ip + 1];
                let load2 = output[ip + 2];
                let store = output[ip + 3];

                output[store] = output[load1] * output[load2];

                instruction_size = 4;
            }
            99 => {
                // Opcode 99 means that the program is finished and should 
                // immediately halt.
                break;
            }
            _ => {
                // Encountering an unknown opcode means something went wrong.
                panic!("Unknown opcode")
            }
        }
        // After an instruction finishes, the instruction pointer increases by 
        // the number of values in the instruction.
        ip += instruction_size;
    }

    // Once the program has halted, its output is available at address 0.
    output[0]
}

pub fn run() {
    let program = load_program("data/day02.txt".to_string());

    // Part 1

    // before running the program, replace position 1 with the value 12 and 
    // replace position 2 with the value 2
    let part1 = run_program(&program, 12, 2);
    println!("{}", part1);

    // Part 2

    // Find the input noun and verb that cause the program to produce the 
    // output 19690720. What is 100 * noun + verb?
    let mut found = false;
    for noun in 1..99 {
        for verb in 1..99 {
            let part2 = run_program(&program, noun, verb);
            if part2 == 19690720 {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}
