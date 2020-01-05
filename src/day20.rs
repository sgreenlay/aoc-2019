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


#[derive(PartialEq, Clone, Copy)]
enum Direction {
    In,
    Out
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::In => Direction::Out,
            Direction::Out => Direction::In,
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Start,
    End,
    Wall,
    Floor,
    Portal((String, Direction))
}

fn get_portals(
    label: &String,
    map: &HashMap<(usize, usize), Tile>,
    multi_level: bool
) -> Vec<((usize, usize), Direction)> {
    let mut portals: Vec<((usize, usize), Direction)> = Vec::new();

    for m in map {
        match m.1 {
            Tile::Portal(p) => {
                if p.0.cmp(label) == cmp::Ordering::Equal {
                    portals.push((*m.0, p.1.inverse()));
                }
            }
            _ => {}
        }
    }

    portals
}

fn get_adjacent(
    current: &(usize, usize, usize),
    map: &HashMap<(usize, usize), Tile>,
    multi_level: bool
) -> Vec<(usize, usize, usize)> {
    let mut adjacent: Vec<(usize, usize, usize)> = Vec::new();

    let mut possible_adjacent = vec![
        (current.0, current.1 - 1, current.2),
        (current.0, current.1 + 1, current.2),
        (current.0 - 1, current.1, current.2),
        (current.0 + 1, current.1, current.2)
    ];

    match &map[&(current.0, current.1)] {
        Tile::Portal(p) => {
            let portals: Vec<(usize, usize, usize)> = get_portals(&p.0, map, multi_level)
                .iter()
                .filter_map(|p| {
                    let pos = p.0;
                    if pos != (current.0, current.1) {
                        if multi_level == false {
                            Some((pos.0, pos.1, current.2))
                        } else {
                            match p.1 {
                                Direction::In => {
                                    Some((pos.0, pos.1, current.2 + 1))
                                },
                                Direction::Out => {
                                    if current.2 != 0 {
                                        Some((pos.0, pos.1, current.2 - 1))
                                    } else {
                                        None
                                    }
                                }
                            }
                        }
                    } else {
                        None
                    }
                }).collect();
            match portals.len() {
                0 => {},
                1 => {
                    possible_adjacent.push(portals[0]);
                },
                _ => {
                    panic!("Found too many portals!");
                }
            }
        }
        _ => {}
    }

    for p in possible_adjacent {
        let map_p = (p.0, p.1);
        if map.contains_key(&map_p) {
            match &map[&map_p] {
                Tile::Wall => {},
                _ => {
                    adjacent.push(p);
                }
            }
        }
    }

    adjacent
}

fn bredth_first_search(
    start: &(usize, usize, usize),
    map: &HashMap<(usize, usize), Tile>,
    multi_level: bool,
    stop: &mut dyn FnMut((usize, usize, usize), u128) -> bool
) {
    let mut frontier: Vec<((usize, usize, usize), u128)> = Vec::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

    visited.insert(*start);
    frontier.push((*start, 0));

    if stop(frontier[0].0, 0) {
        return;
    }

    while !frontier.is_empty() {
        let (current, distance) = frontier.remove(0);
        let v_distance = distance + 1;

        let visit: Vec<(usize, usize, usize)> = get_adjacent(&current, map, multi_level);
        for v in visit {
            if !visited.contains(&v) {
                if map[&(v.0, v.1)] == Tile::Wall {
                    continue;
                }

                frontier.push((v, v_distance));
                visited.insert(v);

                if stop(v, v_distance) {
                    return;
                }
            }
        }
    }
}

fn shortest_path(
    start: &(usize, usize, usize),
    end: &(usize, usize, usize),
    map: &HashMap<(usize, usize), Tile>,
    multi_level: bool
) -> Option<Vec<(usize, usize, usize)>> {
    let mut paths: HashMap<(usize, usize, usize), u128> = HashMap::new();
    bredth_first_search(start, map, multi_level, &mut |p, d| -> bool {
        paths.insert(p, d);
        (p.2 == 0) && (p.0 == end.0) && (p.1 == end.1)
    });

    if !paths.contains_key(&end) {
        return None;
    }

    let mut path: Vec<(usize, usize, usize)> = Vec::new();

    let mut current: (usize, usize, usize) = *end;
    path.push(current);

    while &current != start {
        let visit: Vec<(usize, usize, usize)> = get_adjacent(&current, map, multi_level);

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

fn get_tile(x: usize, y: usize, width: usize, height: usize, input: &Vec<Vec<char>>) -> Option<Tile> {
    match input[y][x] {
        '.' => {
            let n = input[y-1][x];
            if n.is_ascii_uppercase() {
                let label = format!("{}{}", input[y-2][x], input[y-1][x]);

                if label.eq(&"AA".to_string()) {
                    return Some(Tile::Start);
                } else if label.eq(&"ZZ".to_string()) {
                    return Some(Tile::End);
                } else if y < height / 2 {
                    return Some(Tile::Portal((label, Direction::Out)));
                } else {
                    return Some(Tile::Portal((label, Direction::In)));
                }
            }

            let s = input[y+1][x];
            if s.is_ascii_uppercase() {
                let label = format!("{}{}", input[y+1][x], input[y+2][x]);

                if label.eq(&"AA".to_string()) {
                    return Some(Tile::Start);
                } else if label.eq(&"ZZ".to_string()) {
                    return Some(Tile::End);
                } else if y > height / 2 {
                    return Some(Tile::Portal((label, Direction::Out)));
                } else {
                    return Some(Tile::Portal((label, Direction::In)));
                }
            }

            let e = input[y][x+1];
            if e.is_ascii_uppercase() {
                let label = format!("{}{}", input[y][x+1], input[y][x+2]);

                if label.eq(&"AA".to_string()) {
                    return Some(Tile::Start);
                } else if label.eq(&"ZZ".to_string()) {
                    return Some(Tile::End);
                } else if x > width / 2 {
                    return Some(Tile::Portal((label, Direction::Out)));
                } else {
                    return Some(Tile::Portal((label, Direction::In)));
                }
            }

            let w = input[y][x-1];
            if w.is_ascii_uppercase() {
                let label = format!("{}{}", input[y][x-2], input[y][x-1]);

                if label.eq(&"AA".to_string()) {
                    return Some(Tile::Start);
                } else if label.eq(&"ZZ".to_string()) {
                    return Some(Tile::End);
                } else if x < width / 2 {
                    return Some(Tile::Portal((label, Direction::Out)));
                } else {
                    return Some(Tile::Portal((label, Direction::In)));
                }
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
            let is_tile = get_tile(x, y, width, height, &input);
            if is_tile.is_some() {
                let tile = is_tile.unwrap();

                if tile == Tile::Start {
                    start = (x, y);
                } else if tile == Tile::End {
                    end = (x, y);
                }

                map.insert((x, y), tile);
            }
        }
    }

    // Part 1
    let path = shortest_path(&(start.0, start.1, 0), &(end.0, end.1, 0), &map, false);
    println!("Shortest path is {} tiles", path.unwrap().len() - 1);

    // Part 2
    let path = shortest_path(&(start.0, start.1, 0), &(end.0, end.1, 0), &map, true);
    println!("Shortest path is {} tiles", path.unwrap().len() - 1);
}