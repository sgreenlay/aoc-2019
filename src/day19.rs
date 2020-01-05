use crate::intcode::{VirtualMachine, VirtualMachineState, load_program};

use std::collections::HashMap;

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

fn run_program(program: &Vec<i128>, input: (i128, i128)) -> i128 {
    let mut vm = VirtualMachine::new(&program);

    vm.add_input(input.0);
    vm.add_input(input.1);

    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                break;
            }
            VirtualMachineState::Output(v) => {
                return v;
            }
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }

    panic!("No output");
}

pub fn run() {
    let program = load_program("data/day19.txt".to_string());

    // Part 1
    let mut affected_points = 0;
    for y in 0..50 {
        for x in 0..50 {
            let output = run_program(&program, (x, y));
            match output {
                0 => {}
                1 => {
                    affected_points += 1;
                }
                _ => {
                    panic!("Invalid output");
                }
            }
        }
    }
    println!("{} affected points", affected_points);

    // Part 2
    let mut scanned: HashMap<(usize, usize), i128> = HashMap::new();
    let mut scan = |x: usize, y: usize| -> i128 {
        let p = (x, y);
        if !scanned.contains_key(&p) {
            let output = run_program(&program, (x as i128, y as i128));
            scanned.insert(p, output);
        }
        scanned[&p]
    };
    let mut scan_axis = |start: usize, other: usize, axis: char| -> (usize, usize) {
        let (mut x, mut y);
        match axis {
            'x' => {
                x = start;
                y = other;
            }
            'y' => {
                y = start;
                x = other;
            }
            _ => {
                panic!("Invalid axis");
            }
        }

        let (mut first, mut last) = (0, 0);
        let mut found_signal = false;

        enum ScanMode {
            Unknown,
            Backwards,
            Forwards,
        };
        let mut mode = ScanMode::Unknown;

        loop {
            let output = scan(x, y);
            match mode {
                ScanMode::Unknown => match output {
                    0 => {
                        mode = ScanMode::Forwards;
                    }
                    1 => {
                        mode = ScanMode::Backwards;
                    }
                    _ => {
                        panic!("Invalid output");
                    }
                },
                ScanMode::Backwards => {
                    match output {
                        0 => {
                            match axis {
                                'x' => {
                                    first = x + 1;
                                    x = start;
                                }
                                'y' => {
                                    first = y + 1;
                                    y = start;
                                }
                                _ => {
                                    panic!("Invalid axis");
                                }
                            }

                            mode = ScanMode::Forwards;
                            found_signal = true;
                            continue;
                        }
                        1 => {}
                        _ => {
                            panic!("Invalid output");
                        }
                    }
                    match axis {
                        'x' => {
                            x -= 1;
                        }
                        'y' => {
                            y -= 1;
                        }
                        _ => {
                            panic!("Invalid axis");
                        }
                    }
                }
                ScanMode::Forwards => {
                    match output {
                        0 => {
                            if found_signal {
                                match axis {
                                    'x' => {
                                        last = x - 1;
                                    }
                                    'y' => {
                                        last = y - 1;
                                    }
                                    _ => {
                                        panic!("Invalid axis");
                                    }
                                }
                                break;
                            }
                        }
                        1 => {
                            if !found_signal {
                                match axis {
                                    'x' => {
                                        first = x;
                                    }
                                    'y' => {
                                        first = y;
                                    }
                                    _ => {
                                        panic!("Invalid axis");
                                    }
                                }
                                found_signal = true;
                            }
                        }
                        _ => {
                            panic!("Invalid output");
                        }
                    }
                    match axis {
                        'x' => {
                            x += 1;
                        }
                        'y' => {
                            y += 1;
                        }
                        _ => {
                            panic!("Invalid axis");
                        }
                    }
                }
            }
        }

        (first, last)
    };

    let (mut start_x, mut end_x) = (0, 0);
    let mut y = 51;

    let mut increment = 100;
    loop {
        while {
            let scan_row = scan_axis(start_x, y, 'x');

            start_x = scan_row.0;
            end_x = scan_row.1;

            if (end_x - start_x + 1) < 100 {
                true
            } else {
                let scan_bottom_left =
                    run_program(&program, ((end_x - 99) as i128, (y + 99) as i128));
                (scan_bottom_left != 1)
            }
        } {
            y += increment;
        }
        if increment > 1 {
            y -= increment;
            increment /= 10;
            start_x = 0;
        } else {
            break;
        }
    }

    println!("{},{} == {}", end_x - 99, y, 10000 * (end_x - 99) + y);
}
