
use std::io::BufRead;
use std::io;

use std::cmp;
use std::fmt;
use std::fs;

use regex::Regex;

use lazy_static;

use std::collections::HashMap;

#[derive(Clone)]
struct Ingredient {
    chemical: String,
    quantity: i128,
}

impl Ingredient {
    fn from_string(s: String) -> Ingredient {
        lazy_static! {	
            // # XXX
            static ref LINE_RE: Regex = Regex::new(r"([\d]+) (.*)").unwrap();	
        }
        if LINE_RE.is_match(&s) {	
            for line_cap in LINE_RE.captures_iter(&s) {	
                let quantity: i128 = line_cap[1].parse().unwrap();	
                let chemical = &line_cap[2];
                return Ingredient{ chemical: chemical.to_string(), quantity: quantity };
            }	
        }
        panic!("Invalid input");
    }
}

impl fmt::Display for Ingredient {	
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {	
        write!(f, "{} {}", self.quantity, self.chemical)	
    }	
}

struct Recipe {
    inputs: Vec<Ingredient>,
    output: Ingredient
}

impl fmt::Display for Recipe {	
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inputs.first().unwrap())?;
        for i in 1..self.inputs.len() {
            write!(f, ", {}", self.inputs[i])?;
        }
        write!(f, " => {}", self.output)
    }	
}

fn read_inputs(filename: String) -> HashMap<String, Recipe> {
    let file_in = fs::File::open(filename).expect("Can't open file");
    let file_reader = io::BufReader::new(file_in);
    
    let recipe_list: Vec<Recipe> = file_reader.lines().filter_map(io::Result::ok).map(|line| {
        let split_by_produce: Vec<&str> = line.split(" => ").collect();
        if split_by_produce.len() != 2 {
            panic!("Invalid line");
        }

        let inputs: Vec<&str> = split_by_produce[0].split(", ").collect();
        let output: &str = split_by_produce[1];

        Recipe {
            inputs: inputs.iter().map(|s| {
                Ingredient::from_string(s.to_string())
            }).collect(),
            output: Ingredient::from_string(output.to_string())
        }
    }).collect();

    let mut recipies: HashMap<String, Recipe> = HashMap::new();
    for r in recipe_list {
        let k = r.output.chemical.clone();
        recipies.insert(k, r);
    }
    recipies
}

fn get_number_of_steps(chemical: &String, recipies: &HashMap<String, Recipe>, visited: &mut HashMap<String, i128>) -> i128
{
    let recipe = &recipies[chemical];
    let ore = "ORE".to_string();

    let mut steps = 1;
    for i in &recipe.inputs {
        if i.chemical.cmp(&ore) == cmp::Ordering::Equal {
            continue;
        } else if visited.contains_key(&i.chemical) {
            steps += visited[&i.chemical];
        } else {
            steps += get_number_of_steps(&i.chemical, recipies, visited);
        }
    }

    visited.insert(chemical.clone(), steps);
    steps
}

fn get_fuel_cost(fuel_quantity: i128, recipies: &HashMap<String, Recipe>) -> i128
{
    let ore = "ORE".to_string();
    let fuel = "FUEL".to_string();
    
    let mut steps: HashMap<String, i128> = HashMap::new();
    let fuel_steps = get_number_of_steps(&fuel, &recipies, &mut steps);

    let mut have: HashMap<&String, i128> = HashMap::new();
    let mut extras: HashMap<&String, i128> = HashMap::new();

    have.insert(&fuel, fuel_quantity);

    let mut ore_quantity = 0;

    while !have.is_empty()
    {   
        // Find the chemical we have with the most steps to create ORE
        let need = have.keys().fold(&ore, |a, &b| -> &String {
            if (a == &ore) || (steps[b] > steps[a]) {
                b
            } else {
                a
            }
        });
        let quantity = have[need];
        have.remove(need);

        // Use exess chemicals before creating new ones
        let mut remaining = quantity;
        if extras.contains_key(need) {
            let mut extra_quantity = extras[&need];
            if remaining > extra_quantity {
                extra_quantity = 0;
                remaining -= extra_quantity;
            } else {
                extra_quantity -= remaining;
                remaining = 0;
            }
            if extra_quantity == 0 {
                extras.remove(need);
            } else {
                extras.insert(need, extra_quantity);
            }
            if remaining == 0 {
                continue;
            }
        }

        let recipe = &recipies[need];
        let iterations: i128 = ((remaining as f64) / (recipe.output.quantity as f64)).ceil() as i128;

        for i in &recipe.inputs {
            if i.chemical.cmp(&ore) == cmp::Ordering::Equal {
                ore_quantity += i.quantity * iterations;
            } else {
                let mut new_quantity = i.quantity * iterations;
                if have.contains_key(&i.chemical) {
                    new_quantity += have[&i.chemical];
                }
                have.insert(&i.chemical, new_quantity);
            }
        }
        
        let mut excess = iterations * recipe.output.quantity - remaining;
        if excess > 0 {
            if extras.contains_key(&recipe.output.chemical) {
                excess += extras[&recipe.output.chemical];
            }
            extras.insert(&recipe.output.chemical, excess);
        }
    }

    ore_quantity
}

pub fn run() {
    let recipies = read_inputs("data/day14.txt".to_string());

    // Part 1
    let cost_of_one_fuel = get_fuel_cost(1, &recipies);
    println!("{}", cost_of_one_fuel);

    // Part 2
    let ore: i128 = 1000000000000;

    let search = |start, increment: &dyn Fn(i128) -> i128| -> (i128, i128) {
        let mut f = start;
        let mut prev_f = start;
        let mut o = get_fuel_cost(f, &recipies);

        while o < ore {
            prev_f = f;
            f = increment(f);
            o = get_fuel_cost(f, &recipies);
        }

        (prev_f, f)
    };

    let mut start = 1;
    let increment = |a| a * 10;
    let (lower, upper) = search(start, &increment);

    let window_count = 10;

    start = lower;
    let mut window_size = (upper - lower) / window_count;

    while window_size > 1 {
        let increment = |a| a + window_size;
        let (lower, upper) = search(start, &increment);

        window_size = (upper - lower) / window_count;
        start = lower;
    }

    let increment = |a| a + 1;
    let (lower, _) = search(start, &increment);
    println!("{}", lower);
}