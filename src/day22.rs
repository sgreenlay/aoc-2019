
use std::fmt;
use std::fs;

use regex::Regex;

use lazy_static;

enum Technique {
    DealIncrement(usize),
    Cut(i128),
    Deal
}

impl fmt::Display for Technique {	
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {	
        match self {
            Technique::DealIncrement(inc) => write!(f, "DealIncrement with increment {}", inc),
            Technique::Cut(cut) => write!(f, "cut {}", cut),
            Technique::Deal => write!(f, "DealIncrement into new stack"),
        }
    }	
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
                        let inc: usize = line_cap[1].parse().unwrap();
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

pub fn run() {
    let input = load_input("data/day22.txt".to_string());

    let card_count = 10007;
    let mut cards: Vec<i128> = (0..card_count).into_iter().collect::<Vec<_>>();

    for i in input {
        match i {
            Technique::DealIncrement(inc) => {
                let mut scratch = vec![0; cards.len()];
                let mut i = 0;
                while !cards.is_empty() {
                    let c = cards.remove(0);
                    scratch[i] = c;
                    i = (i + inc) % scratch.len();
                }
                cards = scratch;
            },
            Technique::Cut(cut) => {
                if cut > 0 {
                    for _ in 0..cut {
                        let c = cards.remove(0);
                        cards.push(c);
                    }
                } else {
                    for _ in 0..cut.abs() {
                        let c = cards.pop().unwrap();
                        cards.insert(0, c);
                    }
                }
            },
            Technique::Deal => {
                cards.reverse()
            },
        }
    }

    for i in 0..cards.len() {
        if cards[i] == 2019 {
            println!("{}", i);
            break;
        }
    }
}