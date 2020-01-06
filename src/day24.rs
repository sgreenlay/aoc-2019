use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

fn load_input(filename: String) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .split('\n')
        .filter_map(|s: &str| {
            if s.len() == 0 {
                None
            } else {
                Some(s.chars().collect())
            }
        })
        .collect()
}

pub fn run_iteration(
    bugs: &Option<&HashSet<(i64, i64)>>,
    width: usize,
    height: usize,
    adjacent_bugs: &mut dyn FnMut((i64, i64)) -> usize,
) -> HashSet<(i64, i64)> {
    let mut new_bugs: HashSet<(i64, i64)> = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            let p = (x as i64, y as i64);
            let adjacent = adjacent_bugs(p);
            if bugs.is_some() && bugs.unwrap().contains(&p) {
                match adjacent {
                    1 => {
                        new_bugs.insert(p);
                    }
                    _ => {}
                }
            } else {
                match adjacent {
                    1 | 2 => {
                        new_bugs.insert(p);
                    }
                    _ => {}
                }
            }
        }
    }

    new_bugs
}

pub fn run() {
    let input = load_input("data/day24.txt".to_string());
    let (height, width) = (input.len(), input[0].len());

    let bug_hash = |bugs: &HashSet<(i64, i64)>| -> String {
        let mut s = String::new();
        for y in 0..height {
            for x in 0..width {
                let p = (x as i64, y as i64);
                if bugs.contains(&p) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    };

    let mut bugs: HashSet<(i64, i64)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            match input[y][x] {
                '#' => {
                    let p = (x as i64, y as i64);
                    bugs.insert(p);
                }
                _ => {}
            }
        }
    }

    // Part 1
    let mut part1 = bugs.clone();

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(bug_hash(&part1));
    loop {
        let mut count_bugs = |(x, y): (i64, i64)| -> usize {
            vec![
                    (x as i64 - 1, y as i64),
                    (x as i64 + 1, y as i64),
                    (x as i64, y as i64 - 1),
                    (x as i64, y as i64 + 1),
                ]
                .into_iter()
                .map(|p| {
                    if part1.contains(&p) {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        };
        part1 = run_iteration(&Some(&part1), width, height, &mut count_bugs);

        let hash = bug_hash(&part1);
        if visited.contains(&hash) {
            break;
        }
        visited.insert(hash);
    }

    let mut biodiversity: u128 = 0;
    let mut pow2: u128 = 1;
    for y in 0..height {
        for x in 0..width {
            let p = (x as i64, y as i64);
            if part1.contains(&p) {
                biodiversity += pow2;
            }
            pow2 *= 2;
        }
    }
    println!("{}", biodiversity);

    // Part 2
    let mut levels: HashMap<i64, HashSet<(i64, i64)>> = HashMap::new();
    levels.insert(0, bugs);

    let (mid_x, mid_y) = (
        (width as f64 / 2.0).floor() as i64,
        (height as f64 / 2.0).floor() as i64,
    );

    for _ in 0..200 {
        let min = levels.keys().min().unwrap();
        let max = levels.keys().max().unwrap();

        let mut next_levels: HashMap<i64, HashSet<(i64, i64)>> = HashMap::new();

        for i in (min-1)..=(max+1) {
            let previous_bugs = levels.get(&(i-1));
            let current_bugs = levels.get(&i);
            let next_bugs = levels.get(&(i+1));

            let mut count_bugs = |(x, y): (i64, i64)| -> usize {
                if (x == mid_x) && (y == mid_y) {
                    0
                } else {
                    let adjacent = vec![
                        (x as i64 - 1, y as i64),
                        (x as i64 + 1, y as i64),
                        (x as i64, y as i64 - 1),
                        (x as i64, y as i64 + 1),
                    ];

                    let mut bug_count = 0;
                    for a in 0..adjacent.len() {
                        let p = adjacent[a];
                        if (p.0 == mid_x) && (p.1 == mid_y) {
                            match next_bugs {    
                                None => {},
                                Some(next_bugs) => {
                                    match a {
                                        0 | 1 => {
                                            let x = match a {
                                                0 => width - 1,  // last column
                                                1 => 0, // first column
                                                _ => panic!("Error"),
                                            };
                                            for y in 0..height {
                                                let p_next = (x as i64, y as i64);
                                                if next_bugs.contains(&p_next) {
                                                    bug_count += 1;
                                                }
                                            }
                                        },
                                        2 | 3 => {
                                            let y = match a {
                                                2 => height - 1,  // last row
                                                3 => 0, // first row
                                                _ => panic!("Error"),
                                            };
                                            for x in 0..width {
                                                let p_next = (x as i64, y as i64);
                                                if next_bugs.contains(&p_next) {
                                                    bug_count += 1;
                                                }
                                            }
                                        },
                                        _ => panic!("Unknown direction")
                                    }
                                },
                            }
                        } else if (p.0 >= 0) && 
                                  (p.0 < width as i64) && 
                                  (p.1 >= 0) && 
                                  (p.1 < height as i64)
                        {
                            match current_bugs {
                                None => {},
                                Some(current_bugs) => {
                                    if  current_bugs.contains(&p) {
                                        bug_count += 1;
                                    }
                                },
                            }
                        } else {
                            match previous_bugs {    
                                None => {},
                                Some(previous_bugs) => {
                                    if p.0 == -1 {
                                        let p_prev = (mid_x - 1, mid_y);
                                        if previous_bugs.contains(&p_prev) {
                                            bug_count += 1;
                                        }
                                    } else if p.0 == width as i64 {
                                        let p_prev = (mid_x + 1, mid_y);
                                        if previous_bugs.contains(&p_prev) {
                                            bug_count += 1;
                                        }
                                    } else if p.1 == -1 {
                                        let p_prev = (mid_x, mid_y - 1);
                                        if previous_bugs.contains(&p_prev) {
                                            bug_count += 1;
                                        }
                                    } else if p.1 == height as i64 {
                                        let p_prev = (mid_x, mid_y + 1);
                                        if previous_bugs.contains(&p_prev) {
                                            bug_count += 1;
                                        }
                                    } else {
                                        panic!("Error");
                                    }
                                },
                            }
                        }
                    }
                    bug_count
                }
            };
            let bugs = run_iteration(&current_bugs, width, height, &mut count_bugs);
            if !bugs.is_empty() {
                next_levels.insert(i, bugs);
            }
        }

        levels = next_levels;
    }

    let mut bug_count = 0;
    for (_, bugs) in levels {
        bug_count += bugs.len();
    }
    println!("{}", bug_count);
}
