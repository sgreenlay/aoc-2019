
use std::cmp;
use std::fs;
use std::hash;

use std::collections::HashMap;
use std::collections::HashSet;

use elapsed::measure_time;

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

fn bredth_first_search(start: &(usize, usize), map: &Vec<Vec<char>>, stop: &mut dyn FnMut((usize, usize), u128) -> bool) {
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

fn shortest_path(start: &(usize, usize), end: &(usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let (height, width) = (map.len(), map[0].len());

    let mut paths: HashMap<(usize, usize), u128> = HashMap::new();
    bredth_first_search(start, map, &mut |p, d| -> bool {
        paths.insert(p, d);
        p == *end
    });

    if !paths.contains_key(&end) {
        panic!("Couldn't find a path from {},{} to {},{}", start.0, start.1, end.0, end.1);
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
    path
}

struct Cache<'a, K: cmp::Eq + hash::Hash + Copy, V: Clone> {
    cache: HashMap<K, V>,
    miss: &'a mut dyn FnMut(K) -> V,
}

impl<'a, K: cmp::Eq + hash::Hash + Copy, V: Clone> Cache<'a, K, V>
{
    fn new(miss: &'a mut dyn FnMut(K) -> V) -> Cache<K, V> {
        Cache {
            cache: HashMap::new(),
            miss: miss,
        }
    }

    fn get(&mut self, k: K) -> V {
        if !self.cache.contains_key(&k) {
            let v: V = (self.miss)(k);
            self.cache.insert(k, v);
        }
        self.cache[&k].clone()
    }
}

fn shortest_distance_to_all_keys(
    current: &char,
    visited: &Vec<char>,
    remaining: &Vec<char>,
    path_cache: &mut Cache<(char, char), (usize, Vec<char>)>,
    cache: &mut HashMap<String, usize>
) -> usize {

    let v_p: String = remaining.iter().collect();
    let r_p: String = visited.iter().collect();

    let hash = format!("{}-{}-{}", v_p, current, r_p).to_string();
    if cache.contains_key(&hash) {
        return cache[&hash];
    }
    
    let has_key = |d: &char| -> bool  {
        let k: char = d.to_lowercase().collect::<Vec<_>>()[0];

        if &k == current {
            return true;
        }

        for v in visited {
            if v == &k {
                return true;
            }
        }
        false
    };

    let reachable_keys: Vec<char> = remaining.iter().filter_map(|k| -> Option<char> {
        let current_k: (char, char) = (*current, *k);
        let path = path_cache.get(current_k);

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
    }).collect();

    if reachable_keys.len() == 0 {
        panic!("Can't reach any keys");
    }

    let mut visited_k = visited.clone();
    visited_k.push(*current);
    visited_k.sort();

    let mut min_distance: Option<usize> = None;
    for k in reachable_keys {
        let current_k = (*current, k);

        let (mut distance, _) = path_cache.get(current_k);

        if remaining.len() > 1 {
            let mut remaining_k = remaining.clone();
            remaining_k.retain(|&r| r != k);

            distance += shortest_distance_to_all_keys(&k, &visited_k, &remaining_k, path_cache, cache);
        }

        if min_distance.is_none() || (distance < min_distance.unwrap()) {
            min_distance = Some(distance);
        }
    }

    let d = min_distance.unwrap();
    cache.insert(hash, d);
    d
}

pub fn run() {
    let map = load_input("data/day18.txt".to_string());
    let (height, width) = (map.len(), map[0].len());

    let mut doors: HashMap<char, (usize, usize)> = HashMap::new();
    let mut keys: HashMap<char, (usize, usize)> = HashMap::new();
    let mut start = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let tile = map[y][x];
            if tile.is_ascii_uppercase() {
                doors.insert(tile, (x, y));
            } else if tile.is_ascii_lowercase() {
                keys.insert(tile, (x, y));
            } else if tile == '@' {
                start = (x, y);
            }
        }
    }

    let mut all_keys: Vec<char> = keys.keys().map(|k| *k).collect();
    all_keys.sort();

    let a_to_b = |a, b| -> (usize, Vec<char>) {
        let path_a_to_b = shortest_path(&a, &b, &map);
        let doors_between_a_and_b = path_a_to_b.iter().filter_map(|p| -> Option<char> {
                let tile = map[p.1][p.0];
                if tile.is_ascii_uppercase() {
                    Some(tile)
                } else {
                    None
                }
            }).collect();
        (path_a_to_b.len() - 1, doors_between_a_and_b)
    };

    // Part 1
    let mut path_cache_miss = |k: (char, char)| -> (usize, Vec<char>) {
        let (a, b) = k;

        let p_a = if a == '@' { start } else { keys[&a] };
        let p_b = if b == '@' { start } else { keys[&b] };

        let path = a_to_b(p_a, p_b);
        path.clone()
    };
    let mut path_cache = Cache::new(&mut path_cache_miss);

    let mut cache: HashMap<String, usize> = HashMap::new();
    let (elapsed, total_distance) = measure_time(|| {
        shortest_distance_to_all_keys(&'@', &vec![], &all_keys, &mut path_cache, &mut cache)
    });

    println!("Compute {} in {}", total_distance, elapsed);
}
