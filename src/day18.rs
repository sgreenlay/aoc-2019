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

fn bredth_first_search(
    start: &(usize, usize),
    map: &Vec<Vec<char>>,
    stop: &mut dyn FnMut((usize, usize), u128) -> bool,
) {
    let (height, width) = (map.len(), map[0].len());
    let mut frontier: Vec<((usize, usize), u128)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    visited.insert(*start);
    frontier.push((*start, 0));

    if stop(*start, 0) {
        return;
    }

    while !frontier.is_empty() {
        let (current, distance) = frontier.remove(0);

        let mut visit: Vec<(usize, usize)> = Vec::new();
        if current.1 > 0 {
            let up = (current.0, current.1 - 1);
            visit.push(up);
        }
        if current.1 < (height - 1) {
            let down = (current.0, current.1 + 1);
            visit.push(down);
        }
        if current.0 > 0 {
            let left = (current.0 - 1, current.1);
            visit.push(left);
        }
        if current.0 < (width - 1) {
            let rigth = (current.0 + 1, current.1);
            visit.push(rigth);
        }

        let v_distance = distance + 1;
        for v in visit {
            if !visited.contains(&v) {
                let next = map[v.1][v.0];
                if next == '#' {
                    continue;
                }

                frontier.push((v, v_distance));
                visited.insert(v);

                if stop(v, v_distance) {
                    break;
                }
            }
        }
    }
}

fn shortest_path(
    start: &(usize, usize),
    end: &(usize, usize),
    map: &Vec<Vec<char>>,
) -> Option<Vec<(usize, usize)>> {
    let (height, width) = (map.len(), map[0].len());

    let mut paths: HashMap<(usize, usize), u128> = HashMap::new();
    bredth_first_search(start, map, &mut |p, d| -> bool {
        paths.insert(p, d);
        p == *end
    });

    if !paths.contains_key(&end) {
        return None;
    }

    let mut path: Vec<(usize, usize)> = Vec::new();

    let mut current: (usize, usize) = *end;
    path.push(current);

    while current != *start {
        let mut visit: Vec<(usize, usize)> = Vec::new();
        if current.1 > 0 {
            let up = (current.0, current.1 - 1);
            visit.push(up);
        }
        if current.1 < (height - 1) {
            let down = (current.0, current.1 + 1);
            visit.push(down);
        }
        if current.0 > 0 {
            let left = (current.0 - 1, current.1);
            visit.push(left);
        }
        if current.0 < (width - 1) {
            let rigth = (current.0 + 1, current.1);
            visit.push(rigth);
        }

        let closest = visit
            .iter()
            .filter(|p| paths.contains_key(&p))
            .min_by_key(|p| paths[p])
            .unwrap();

        if *closest == current {
            panic!("Stuck in local minima");
        }

        current = *closest;
        path.push(current);
    }

    path.reverse();
    Some(path)
}

fn shortest_distance_to_all_keys(
    current: &Vec<char>,
    visited: &Vec<char>,
    remaining: &Vec<char>,
    paths: &HashMap<(char, char), (usize, Vec<char>)>,
    cache: &mut HashMap<String, usize>,
) -> Option<usize> {
    let v_p: String = visited.iter().collect();
    let c_p: String = current.iter().collect();
    let r_p: String = remaining.iter().collect();

    let hash = format!("{}-{}-{}", v_p, c_p, r_p).to_string();
    if cache.contains_key(&hash) {
        return Some(cache[&hash]);
    }

    let has_key = |d: &char| -> bool {
        let k: char = d.to_lowercase().collect::<Vec<_>>()[0];
        for c in current {
            if c == &k {
                return true;
            }
        }
        for v in visited {
            if v == &k {
                return true;
            }
        }
        false
    };

    let mut min_distance: Option<usize> = None;
    for &c in current {
        let reachable_keys: Vec<char> = remaining
            .iter()
            .filter_map(|k| -> Option<char> {
                let current_k: (char, char) = (c, *k);

                if !paths.contains_key(&current_k) {
                    return None;
                }

                let path = &paths[&current_k];
                let mut door_in_way = false;

                for d in &path.1 {
                    if !has_key(d) {
                        door_in_way = true;
                        break;
                    }
                }

                if !door_in_way {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect();

        if reachable_keys.len() == 0 {
            continue;
        }

        let mut visited_k = visited.clone();
        visited_k.push(c);
        visited_k.sort();

        for k in reachable_keys {
            let mut distance = paths[&(c, k)].0;

            if remaining.len() > 1 {
                let mut remaining_k = remaining.clone();
                remaining_k.retain(|&r| r != k);

                let current_k: Vec<char> = current
                    .iter()
                    .map(|r| if r == &c { k } else { *r })
                    .collect();

                let remaining_distance = shortest_distance_to_all_keys(
                    &current_k,
                    &visited_k,
                    &remaining_k,
                    paths,
                    cache,
                );
                if remaining_distance.is_none() {
                    continue;
                }

                distance += remaining_distance.unwrap();
            }

            if min_distance.is_none() || (distance < min_distance.unwrap()) {
                min_distance = Some(distance);
            }
        }
    }

    if min_distance.is_some() {
        let d = min_distance.unwrap();
        cache.insert(hash, d);
        Some(d)
    } else {
        None
    }
}

fn solve(map: &Vec<Vec<char>>) -> usize {
    let (height, width) = (map.len(), map[0].len());

    let mut doors: HashMap<char, (usize, usize)> = HashMap::new();
    let mut keys: HashMap<char, (usize, usize)> = HashMap::new();
    let mut robots: HashMap<char, (usize, usize)> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let tile = map[y][x];
            if tile.is_ascii_uppercase() {
                doors.insert(tile, (x, y));
            } else if tile.is_ascii_lowercase() {
                keys.insert(tile, (x, y));
            } else if (tile != '#') && (tile != '.') {
                robots.insert(tile, (x, y));
            }
        }
    }

    let a_to_b = |a, b| -> Option<(usize, Vec<char>)> {
        let is_path_a_to_b = shortest_path(a, b, &map);
        if is_path_a_to_b.is_none() {
            return None;
        }

        let path_a_to_b = is_path_a_to_b.unwrap();
        let doors_between_a_and_b = path_a_to_b
            .iter()
            .filter_map(|p| -> Option<char> {
                let tile = map[p.1][p.0];
                if tile.is_ascii_uppercase() {
                    Some(tile)
                } else {
                    None
                }
            })
            .collect();
        Some((path_a_to_b.len() - 1, doors_between_a_and_b))
    };

    let mut paths: HashMap<(char, char), (usize, Vec<char>)> = HashMap::new();
    for a in &keys {
        for r in &robots {
            let p = a_to_b(&r.1, &a.1);
            if p.is_some() {
                paths.insert((*r.0, *a.0), p.unwrap());
            }
        }
        for b in &keys {
            if a.0 == b.0 {
                continue;
            }
            let p = a_to_b(&a.1, &b.1);
            if p.is_some() {
                paths.insert((*a.0, *b.0), p.unwrap());
            }
        }
    }

    let mut all_keys: Vec<char> = keys.keys().map(|k| *k).collect();
    all_keys.sort();

    let mut all_robots: Vec<char> = robots.keys().map(|k| *k).collect();
    all_robots.sort();

    let mut cache: HashMap<String, usize> = HashMap::new();

    shortest_distance_to_all_keys(&all_robots, &vec![], &all_keys, &paths, &mut cache).unwrap()
}

pub fn run() {
    let mut map = load_input("data/day18.txt".to_string());

    // Part 1
    let part1 = solve(&map);
    println!("{}", part1);

    // Part 2
    let (height, width) = (map.len(), map[0].len());
    let mut found_robot = false;
    for y in 0..height {
        for x in 0..width {
            let tile = map[y][x];
            if tile == '@' {
                map[y - 1][x - 1] = '^';
                map[y - 1][x] = '#';
                map[y - 1][x + 1] = '>';

                map[y][x - 1] = '#';
                map[y][x] = '#';
                map[y][x + 1] = '#';

                map[y + 1][x - 1] = '@';
                map[y + 1][x] = '#';
                map[y + 1][x + 1] = '<';

                found_robot = true;
                break;
            }
        }
        if found_robot {
            break;
        }
    }

    let part2 = solve(&map);
    println!("{}", part2);
}
