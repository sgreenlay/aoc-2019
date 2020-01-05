use std::fs;

use crate::intcode::{VirtualMachine, VirtualMachineState, load_program};

pub fn run() {
    let program = load_program("data/day09.txt".to_string());

    // Part 1
    let mut vm = VirtualMachine::new(&program);
    vm.add_input(1);
    loop {
        match vm.run() {
            VirtualMachineState::Output(v) => {
                println!("{}", v);
            }
            VirtualMachineState::Terminated => {
                break;
            }
            _ => {
                panic!("Unexpected request for input");
            }
        }
    }
    
    // Part 2
    vm = VirtualMachine::new(&program);
    vm.add_input(2);
    loop {
        match vm.run() {
            VirtualMachineState::Output(v) => {
                println!("{}", v);
            }
            VirtualMachineState::Terminated => {
                break;
            }
            _ => {
                panic!("Unexpected request for input");
            }
        }
    }
}
