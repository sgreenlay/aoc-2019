use ascii;
use std::fs;

use crate::intcode::{VirtualMachine, VirtualMachineState};

fn load_program(filename: String) -> Vec<i128> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .split(',')
        .filter_map(|s| {
            let v = s.parse::<i128>();
            if v.is_err() {
                None
            } else {
                Some(v.unwrap())
            }
        })
        .collect()
}

fn run_program(program: &Vec<i128>, script: &Vec<&str>) {
    let mut vm = VirtualMachine::new(&program);

    for i in script {
        for ch in i.chars() {
            vm.add_input(ch as i128);
        }
        vm.add_input('\n' as i128);
    }

    let mut s: String = String::new();
    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                break;
            }
            VirtualMachineState::Output(v) => {
                let ch = ascii::AsciiChar::from_ascii(v as u8);
                if ch.is_ok() {
                    let ch = ch.unwrap();
                    s.push(ch.as_char());
                    if ch == '\n' {
                        print!("{}", s);
                        s = String::new();
                    }
                } else {
                    println!("{} damage", v);
                }
            }
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }
}

pub fn run() {
    let program = load_program("data/day21.txt".to_string());

    // There are only three instructions available in springscript:
    //   AND X Y sets Y to true if both X and Y are true
    //   OR X Y sets Y to true if at least one of X or Y is true
    //   NOT X Y sets Y to true if X is false

    // Part 1

    // @
    // #ABCD
    // T - Temp
    // J - Jump

    // @................
    // #####.###########

    // @................
    // #####...#########

    // @................
    // #####.#..########

    // (!C || !A) && D
    let script = vec!["NOT A T", "NOT C J", "OR T J", "AND D J", "WALK"];

    run_program(&program, &script);

    // Part 2

    // @
    // #ABCDEFGHI
    // T - Temp
    // J - Jump

    // @................
    // #####.#.#...#####

    // @................
    // #####..##.##.####

    // ((!C && H) || !B || !A) && D
    let script = vec![
        "NOT C T", "AND H T", "AND D T", "NOT B J", "AND D J", "OR J T", "NOT A J", "OR T J", "RUN",
    ];

    run_program(&program, &script);
}
