
use std::cmp;
use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

fn load_input(filename : String) -> Vec<Vec<char>> {
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

#[derive(PartialEq)]
enum Tile {
    Wall,
    Floor,
    Portal(String)
}

fn get_portals(
    label: &String,
    map: &HashMap<(usize, usize), Tile>
) -> Vec<(usize, usize)> {
    let mut portals: Vec<(usize, usize)> = Vec::new();

    for m in map {
        match m.1 {
            Tile::Portal(p) => {
                if p.cmp(label) == cmp::Ordering::Equal {
                    portals.push(*m.0);
                }
            }
            _ => {}
        }
    }

    portals
}

fn get_adjacent(
    current: &(usize, usize),
    map: &HashMap<(usize, usize), Tile>,
) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = Vec::new();

    let mut possible_adjacent = vec![
        (current.0, current.1 - 1),
        (current.0, current.1 + 1),
        (current.0 - 1, current.1),
        (current.0 + 1, current.1)
    ];

    match &map[current] {
        Tile::Portal(p) => {
            let portals: Vec<(usize, usize)> = get_portals(p, map)
                .iter()
                .filter_map(|p| {
                    if p != current {
                        Some(*p)
                    } else {
                        None
                    }
                }).collect();
            if portals.len() == 1 {
                possible_adjacent.push(portals[0]);
            }
        }
        _ => {}
    }

    for p in possible_adjacent {
        if map.contains_key(&p) {
            match &map[&p] {
                Tile::Floor | Tile::Portal(_) => {
                    adjacent.push(p);
                }
                Tile::Wall => {}
            }
        }
    }

    adjacent
}

fn bredth_first_search(
    start: &(usize, usize),
    map: &HashMap<(usize, usize), Tile>,
    stop: &mut dyn FnMut((usize, usize), u128) -> bool
) {
    let mut frontier: Vec<((usize, usize), u128)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    visited.insert(*start);
    frontier.push((*start, 0));

    if stop(*start, 0) {
        return;
    }

    while !frontier.is_empty() {
        let (current, distance) = frontier.remove(0);
        let v_distance = distance + 1;

        let visit: Vec<(usize, usize)> = get_adjacent(&current, map);
        for v in visit {
            if !visited.contains(&v) {
                if map[&v] == Tile::Wall {
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
    map: &HashMap<(usize, usize), Tile>
) -> Option<Vec<(usize, usize)>> {
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
        let visit: Vec<(usize, usize)> = get_adjacent(&current, map);

        let closest = visit.iter()
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

fn get_tile(x: usize, y: usize, input: &Vec<Vec<char>>) -> Option<Tile> {
    match input[y][x] {
        '.' => {
            let n = input[y-1][x];
            if n.is_ascii_uppercase() {
                return Some(Tile::Portal(format!("{}{}", input[y-2][x], input[y-1][x])));
            }

            let s = input[y+1][x];
            if s.is_ascii_uppercase() {
                return Some(Tile::Portal(format!("{}{}", input[y+1][x], input[y+2][x])));
            }

            let e = input[y][x+1];
            if e.is_ascii_uppercase() {
                return Some(Tile::Portal(format!("{}{}", input[y][x+1], input[y][x+2])));
            }

            let w = input[y][x-1];
            if w.is_ascii_uppercase() {
                return Some(Tile::Portal(format!("{}{}", input[y][x-2], input[y][x-1])));
            }

            Some(Tile::Floor)
        },
        '#' => Some(Tile::Wall),
        _ => None
    }
}

pub fn run() {
    let input = load_input("data/day20.txt".to_string());

    let (height, width) = (input.len(), input[0].len());
    let mut map: HashMap<(usize, usize), Tile> = HashMap::new();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let is_tile = get_tile(x, y, &input);
            if is_tile.is_some() {
                let tile = is_tile.unwrap();

                if tile == Tile::Portal("AA".to_string()) {
                    start = (x, y);
                } else if tile == Tile::Portal("ZZ".to_string()) {
                    end = (x, y);
                }

                map.insert((x, y), tile);
            }
        }
    }

    let path = shortest_path(&start, &end, &map);
    println!("Shortest path is {} tiles", path.unwrap().len() - 1);
}