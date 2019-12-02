
use std::fs;

fn run_program(program : &Vec<usize>, noun : usize, verb : usize) -> usize {
    let mut output : Vec<usize> = program.clone();

    output[1] = noun;
    output[2] = verb;

    let mut pc = 0;
    loop {
        match output[pc] {
            1 => {
                // Opcode 1 adds together numbers read from two positions 
                // and stores the result in a third position. The three 
                // integers immediately after the opcode tell you these 
                // three positions - the first two indicate the positions 
                // from which you should read the input values, and the 
                // third indicates the position at which the output 
                // should be stored.

                let a = output[pc + 1];
                let b = output[pc + 2];
                let c = output[pc + 3];

                output[c] = output[a] + output[b];
            }
            2 => {
                // Opcode 2 works exactly like opcode 1, except it multiplies 
                // the two inputs instead of adding them. Again, the three 
                // integers after the opcode indicate where the inputs and 
                // outputs are, not their values.

                let a = output[pc + 1];
                let b = output[pc + 2];
                let c = output[pc + 3];

                output[c] = output[a] * output[b];
            }
            99 => {
                // 99 means that the program is finished and should immediately 
                // halt.
                break;
            }
            _ => {
                // Encountering an unknown opcode means something went wrong.
            }
        }
        // Once you're done processing an opcode, move to the next one by 
        // stepping forward 4 positions.
        pc += 4;
    }

    output[0]
}

pub fn run() {
    let input = fs::read_to_string("data/day02.txt")
        .expect("Can't read file");

    let values: Vec<&str> = input.split(',').collect();
    let program: Vec<usize> = values.iter().map(|x| x.parse::<i32>().unwrap() as usize).collect();

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
