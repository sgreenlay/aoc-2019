use std::fs;

use std::collections::HashSet;

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

pub fn run() {
    let program = load_program("data/day23.txt".to_string());

    let mut template = VirtualMachine::new(&program);
    match template.run() {
        VirtualMachineState::WaitForInput => {}
        _ => {
            panic!("Should request NIC id");
        }
    }

    let mut vms: Vec<VirtualMachine> = (0..50).into_iter().map(|_| template.clone()).collect();
    let mut packets: Vec<(usize, i128)> = (0..50).map(|addr| (addr, addr as i128)).collect();
    let mut nat: Option<(i128, i128)> = None;
    let mut seen_nat_y: HashSet<i128> = HashSet::new();

    loop {
        while !packets.is_empty() {
            let packet = packets.remove(0);

            let vm = &mut vms[packet.0];
            vm.add_input(packet.1);

            loop {
                match vm.run() {
                    VirtualMachineState::WaitForInput => {
                        break;
                    }
                    VirtualMachineState::Output(v) => {
                        let addr = v as usize;
                        let x = match vm.run() {
                            VirtualMachineState::Output(v) => v,
                            _ => panic!("Expecting output"),
                        };
                        let y = match vm.run() {
                            VirtualMachineState::Output(v) => v,
                            _ => panic!("Expecting output"),
                        };

                        if addr == 255 {
                            if nat.is_none() {
                                println!("{}", y);
                            }
                            nat = Some((x, y));
                        } else {
                            packets.push((addr, x));
                            packets.push((addr, y));
                        }
                    }
                    VirtualMachineState::Terminated => panic!("Machine Terminated!"),
                }
            }
        }
        if nat.is_some() {
            let (x, y) = nat.unwrap();

            if seen_nat_y.contains(&y) {
                println!("{}", y);
                break;
            } else {
                seen_nat_y.insert(y);
            }

            packets.push((0, x));
            packets.push((0, y));
        } else {
            packets = (0..50).map(|addr| (addr, -1)).collect();
        }
    }
}
