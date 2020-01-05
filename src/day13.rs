use std::cmp;
use std::fmt;

use crate::intcode::{VirtualMachine, VirtualMachineState, load_program};

use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i128,
    y: i128,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

enum State {
    OutputX,
    OutputY,
    OutputTile,
}

fn run_game(program: &Vec<i128>, play: bool) -> (HashMap<Point, i128>, i128) {
    let mut screen: HashMap<Point, i128> = HashMap::new();

    let mut score = 0;
    let mut ball = Point { x: 0, y: 0 };
    let mut paddle = Point { x: 0, y: 0 };

    let mut state = State::OutputX;
    let mut p = Point { x: 0, y: 0 };

    let mut vm = VirtualMachine::new(program);

    if play {
        // Memory address 0 represents the number of quarters that have been
        // inserted; set it to 2 to play for free.
        vm.set_memory(0, 2);
    }

    loop {
        match vm.run() {
            VirtualMachineState::WaitForInput => {
                // If the joystick is in the neutral position, provide 0.
                // If the joystick is tilted to the left, provide -1.
                // If the joystick is tilted to the right, provide 1.

                if paddle.x < ball.x {
                    vm.add_input(1);
                } else if paddle.x > ball.x {
                    vm.add_input(-1);
                } else {
                    vm.add_input(0);
                }
            }
            VirtualMachineState::Output(v) => {
                match state {
                    State::OutputX => {
                        p.x = v;
                        state = State::OutputY;
                    }
                    State::OutputY => {
                        p.y = v;
                        state = State::OutputTile;
                    }
                    State::OutputTile => {
                        if (p.x == -1) && (p.y == 0) {
                            // When three output instructions specify X=-1, Y=0,
                            // the third output instruction is not a tile; the
                            // value instead specifies the new score to show in
                            // the segment display.
                            score = v;
                        } else {
                            match v {
                                4 => {
                                    ball = p;
                                }
                                3 => {
                                    paddle = p;
                                }
                                _ => {}
                            }
                            screen.insert(p, v);
                        }
                        state = State::OutputX;
                    }
                }
            }
            VirtualMachineState::Terminated => {
                break;
            }
        }
    }

    (screen, score)
}

pub fn run() {
    let program = load_program("data/day13.txt".to_string());

    // Part 1
    let part1 = run_game(&program, false).0;
    let mut block_count = 0;
    for tile in part1.values() {
        if tile == &2 {
            block_count += 1;
        }
    }
    println!("{} blocks", block_count);

    // Part 2
    let part2 = run_game(&program, true).1;
    println!("{}", part2);
}
