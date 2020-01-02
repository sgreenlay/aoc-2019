
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

struct Cache<'a, K: cmp::Eq + hash::Hash, V: Clone> {
    cache: HashMap<K, V>,
    miss: &'a mut dyn FnMut(&K) -> V,
}

impl<'a, K: cmp::Eq + hash::Hash, V: Clone> Cache<'a, K, V>
{
    fn new(miss: &'a mut dyn FnMut(&K) -> V) -> Cache<K, V> {
        Cache {
            cache: HashMap::new(),
            miss: miss,
        }
    }

    fn get(&mut self, k: K) -> V {
        if !self.cache.contains_key(&k) {
            let v: V = (self.miss)(&k);
            self.cache.insert(k, v.clone());
            v.clone()
        } else {
            self.cache[&k].clone()
        }
    }
}

fn shortest_distance_to_all_keys(
    current: &char,
    visited: &String,
    remaining: &String,
    path_cache: &mut Cache<(char, char), (usize, Vec<char>)>,
    distance_cache: &mut Cache<String, usize>
) -> usize {
    let has_key = |d: &char| -> bool  {
        let k: char = d.to_lowercase().collect::<Vec<_>>()[0];

        if &k == current {
            return true;
        }

        for v in visited.chars() {
            if v == k {
                return true;
            }
        }
        false
    };

    let reachable_keys: Vec<char> = remaining.chars().filter_map(|k| -> Option<char> {
        let current_k: (char, char) = (*current, k);
        let path = path_cache.get(current_k);

        let mut door_in_way = false;

        for d in &path.1 {
            if !has_key(d) {
                door_in_way = true;
                break;
            }
        }

        if !door_in_way {
            Some(k)
        } else {
            None
        }
    }).collect();

    if reachable_keys.len() == 0 {
        panic!("Can't reach any keys");
    }

    let mut visited_k_chs: Vec<char> = visited.chars().collect();
    visited_k_chs.push(*current);
    visited_k_chs.sort();
    let visited_k: String = visited_k_chs.iter().collect();

    let mut min_distance: Option<usize> = None;
    for k in reachable_keys {
        let current_k = (*current, k);

        let (mut distance, _) = path_cache.get(current_k);

        if remaining.len() > 1 {
            let mut remaining_k_chs: Vec<char> = remaining.chars().collect();
            remaining_k_chs.retain(|&r| r != k);
            let remaining_k: String = remaining_k_chs.iter().collect();

            let hash = format!("{}-{}-{}", visited_k, k, remaining_k).to_string();
            let distance_k = distance_cache.get(hash);  

            distance += distance_k;
        }

        if min_distance.is_none() || (distance < min_distance.unwrap()) {
            min_distance = Some(distance);
        }
    }

    min_distance.unwrap()
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
            } else if (tile != '@') && (tile != '#') {
                start = (x, y);
            }
        }
    }

    let mut all_keys_chs: Vec<char> = keys.keys().map(|k| *k).collect();
    all_keys_chs.sort();
    let all_keys: String = all_keys_chs.iter().collect();

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
    let mut path_cache_miss = |k: &(char, char)| -> (usize, Vec<char>) {
        let (a, b) = k;

        let p_a = if a == &'@' { start } else { keys[&a] };
        let p_b = if b == &'@' { start } else { keys[&b] };

        let path = a_to_b(p_a, p_b);
        path.clone()
    };
    let mut path_cache = Cache::new(&mut path_cache_miss);

    let mut temp_distance_cache_miss = |_: &String| -> usize { 0 };
    let mut distance_cache = Cache::new(&mut temp_distance_cache_miss);
    let mut distance_cache_miss = |k: &String| -> usize {
        let inputs: Vec<&str> = k.split("-").collect();

        let visited: String = inputs[0].to_string();
        let current: char = *(inputs[1].chars().collect::<Vec<char>>().first().unwrap());
        let remaining: String = inputs[2].to_string();
        
        shortest_distance_to_all_keys(&current, &visited, &remaining, &mut path_cache, &mut distance_cache)
    };
    distance_cache.miss = &mut distance_cache_miss;

    let total_distance = distance_cache.get(format!("-@-{}", all_keys));
    println!("{}", total_distance);
}
