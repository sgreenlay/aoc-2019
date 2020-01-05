use std::fs;

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

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(bug_hash(&bugs));

    loop {
        let mut new_bugs: HashSet<(i64, i64)> = HashSet::new();

        for y in 0..height {
            for x in 0..width {
                let p = (x as i64, y as i64);
                let adjacent: Vec<(i64, i64)> = vec![
                        (x as i64 - 1, y as i64),
                        (x as i64 + 1, y as i64),
                        (x as i64, y as i64 - 1),
                        (x as i64, y as i64 + 1)
                    ]
                    .into_iter()
                    .filter(|p| bugs.contains(&p))
                    .collect();
                
                if bugs.contains(&p) {
                    match adjacent.len() {
                        1 => {
                            new_bugs.insert(p);
                        },
                        _ => {},
                    }
                } else {
                    match adjacent.len() {
                        1 | 2 => {
                            new_bugs.insert(p);
                        },
                        _ => {},
                    }
                }
            }
        }

        bugs = new_bugs;

        let hash = bug_hash(&bugs);
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
            if bugs.contains(&p) {
                biodiversity += pow2;
            }
            pow2 *= 2;
        }
    }
    println!("{}", biodiversity);
}
