
use std::io::BufRead;
use std::io;

use std::hash;
use std::fmt;
use std::fs;
use std::ops;

use regex::Regex;

use lazy_static;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point3D {	
    x: i128,
    y: i128,
    z: i128,
}

impl fmt::Display for Point3D {	
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {	
        write!(f, "{},{},{}", self.x, self.y, self.z)	
    }	
}

impl hash::Hash for Point3D {	
    fn hash<H: hash::Hasher>(&self, state: &mut H) {	
        self.x.hash(state);	
        self.y.hash(state);	
        self.z.hash(state);	
    }	
}

impl ops::AddAssign for Point3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Moon {
    position: Point3D,
    velocity: Point3D,
}

impl hash::Hash for Moon {	
    fn hash<H: hash::Hasher>(&self, state: &mut H) {	
        self.position.hash(state);	
        self.velocity.hash(state);
    }	
}

impl Moon {
    fn new(p: Point3D) -> Moon {
        Moon {
            position: p,
            velocity: Point3D{x: 0, y: 0, z: 0}
        }
    }
    fn potential_energy(&self) -> i128 {
        // A moon's potential energy is the sum of the absolute 
        // values of its position coordinates.
        return self.position.x.abs() +
               self.position.y.abs() +
               self.position.z.abs();
    }
    fn kinetic_energy(&self) -> i128 {
        // A moon's kinetic energy is the sum of the absolute 
        // values of its velocity coordinates.
        return self.velocity.x.abs() +
               self.velocity.y.abs() +
               self.velocity.z.abs();
    }
}

impl fmt::Display for Moon {	
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {	
        write!(f, "position:{}, velocity:{}", self.position, self.velocity)	
    }	
}

fn read_inputs(filename: String) -> Vec<Moon> {
    let file_in = fs::File::open(filename).expect("Can't open file");
    let file_reader = io::BufReader::new(file_in);
    file_reader.lines().filter_map(io::Result::ok).map(|line| {
        lazy_static! {	
            // <x=#, y=#, z=#>
            static ref LINE_RE: Regex = Regex::new(r"<x=(-?[\d]+), y=(-?[\d]+), z=(-?[\d]+)>").unwrap();	
        }
        if LINE_RE.is_match(&line) {	
            for line_cap in LINE_RE.captures_iter(&line) {	
                let x: i128 = line_cap[1].parse().unwrap();	
                let y: i128 = line_cap[2].parse().unwrap();	
                let z: i128 = line_cap[3].parse().unwrap();
                return Moon::new(Point3D { x: x, y: y, z: z });
            }	
        }
        panic!("Invalid input");
    }).collect()
}

fn simulate_step(moons: &mut Vec<Moon>, axis: Vec<usize>) {
    // Simulate the motion of the moons in time steps. 
    // Within each time step, first update the velocity of every moon 
    // by applying gravity.

    // To apply gravity, consider every pair of moons. On each axis 
    // (x, y, and z), the velocity of each moon changes by exactly 
    // +1 or -1 to pull the moons together. If the positions on a 
    // given axis are the same, the velocity on that axis does not 
    // change for that pair of moons.

    let count = moons.len();
    for i in 0..count {
        for j in 0..count {
            if i == j {
                continue;
            }

            for a in &axis {
                match a {
                    0 => {
                        if moons[j].position.x < moons[i].position.x {
                            moons[i].velocity.x -= 1;
                        } else if moons[j].position.x > moons[i].position.x {
                            moons[i].velocity.x += 1;
                        }
                    }
                    1 => {
                        if moons[j].position.y < moons[i].position.y {
                            moons[i].velocity.y -= 1;
                        } else if moons[j].position.y > moons[i].position.y {
                            moons[i].velocity.y += 1;
                        }
                    }
                    2 => {
                        if moons[j].position.z < moons[i].position.z {
                            moons[i].velocity.z -= 1;
                        } else if moons[j].position.z > moons[i].position.z {
                            moons[i].velocity.z += 1;
                        }
                    }
                    _ => {
                        panic!("Unknown axis");
                    }
                }
            }
        }
    }

    // Then, once all moons' velocities have been updated, update the 
    // position of every moon by applying velocity.

    for i in 0..count {
        let v = moons[i].velocity;
        moons[i].position += v;
    }
}

fn prime_factors(num: i128) -> HashMap<i128, i128> {
    let mut n = num;
    let mut factors: HashMap<i128, i128> = HashMap::new();

    for i in 2..=n {
        let mut count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        if count > 0 {
            factors.insert(i, count);
        }
    }

    factors
}

fn lowest_common_multiple(nums: Vec<i128>) -> i128 {
    let mut factors: HashMap<i128, i128> = HashMap::new();
    for n in nums {
        let n_factors = prime_factors(n);
        for f in n_factors {
            if factors.contains_key(&f.0) {
                if f.1 > factors[&f.0] {
                    factors.insert(f.0, f.1);
                }
            } else {
                factors.insert(f.0, f.1);
            }
        }
    }

    let mut min_multiple = 1;
    for f in &factors {
        for _ in 0..*f.1 {
            min_multiple *= f.0;
        }
    }

    min_multiple
}

pub fn run() {
    let moons: Vec<Moon> = read_inputs("data/day12.txt".to_string());
    
    // Part 1
    let mut part1 = moons.clone();
    for _ in 0..1000 {
        simulate_step(&mut part1, vec![0, 1, 2]);
    }

    // The total energy for a single moon is its potential energy 
    // multiplied by its kinetic energy.
    let mut total_energy = 0;
    for i in 0..part1.len() {
        let pe = part1[i].potential_energy();
        let ke = part1[i].kinetic_energy();
        total_energy += pe * ke;
    }
    println!("{}", total_energy);

    // Part 2
    let mut periods: Vec<i128> = Vec::new();
    for i in 0..=2 {
        let mut time = 0;
        let mut part2 = moons.clone();
        loop {
            time += 1;
            simulate_step(&mut part2, vec![i]);
            if part2 == moons {
                periods.push(time);
                break;
            }
        }
    }

    let min = lowest_common_multiple(periods);
    println!("{}", min);
}