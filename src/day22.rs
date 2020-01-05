
use std::fs;

use regex::Regex;

use lazy_static;

enum Technique {
    DealIncrement(i128),
    Cut(i128),
    Deal
}

fn load_input(filename : String) -> Vec<Technique> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .split('\n')
        .filter_map(|line: &str| {
            if line.len() == 0 {
                None
            } else {
                lazy_static! {	
                    // DealIncrement with increment #
                    static ref DEAL_INC_RE: Regex = Regex::new(r"deal with increment (-?[\d]+)").unwrap();

                    // cut #
                    static ref CUT_RE: Regex = Regex::new(r"cut (-?[\d]+)").unwrap();

                    // DealIncrement into new stack
                    static ref NEW_RE: Regex = Regex::new(r"deal into new stack").unwrap();
                }
                if DEAL_INC_RE.is_match(&line) {	
                    for line_cap in DEAL_INC_RE.captures_iter(&line) {	
                        let inc: i128 = line_cap[1].parse().unwrap();
                        return Some(Technique::DealIncrement(inc));
                    }	
                }
                if CUT_RE.is_match(&line) {	
                    for line_cap in CUT_RE.captures_iter(&line) {	
                        let cut: i128 = line_cap[1].parse().unwrap();
                        return Some(Technique::Cut(cut));
                    }	
                }
                if NEW_RE.is_match(&line) {
                    return Some(Technique::Deal);
                }
                panic!("Invalid input");
            }
        })
        .collect()
}

fn modular(a: i128, m: i128) -> i128 {
    if a >= 0 {
        a % m
    } else {
        m + a % m
    }
}

fn get_coefficients(card_count: i128, shuffle_count: i128, steps: &Vec<Technique>) -> (i128, i128) {
    let (mut a, mut b) = (1, 0);
    for _ in 0..shuffle_count {
        let (a_p, b_p) = steps.iter().fold((a, b), |(a, b), s| -> (i128, i128) {
            match s {
                Technique::DealIncrement(x) => {
                    (modular(a * x, card_count), modular(b * x, card_count))
                },
                Technique::Cut(x) => {
                    (a, modular(b - x, card_count))
                },
                Technique::Deal => {
                    (modular(-a, card_count), modular(card_count - 1 - b, card_count))
                },
            }
        });
        a = a_p;
        b = b_p;
    }
    (a, b)
}

pub fn run() {
    let steps = load_input("data/day22.txt".to_string());
    
    // All three dealing operations can be written as a linear operation of the form:
    //    y = a * x + b
    // where x is the initial position of the card, and y is the final position of the card

    // Part 1
    let card_count = 10007;
    let shuffle_count = 1;

    let (a, b) = get_coefficients(card_count, shuffle_count, &steps);
    let y = modular(a * 2019 + b, card_count);
    println!("{}", y);
}