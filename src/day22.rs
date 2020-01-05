
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

// https://algorithmist.com/wiki/Modular_inverse
fn iterative_egcd(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut a, mut b) = (a, b);
    let (mut x, mut y, mut u, mut v) = (0, 1, 1, 0);
    while a != 0 {
        let (q, r) = ((b as f64 / a as f64).floor() as i128, b % a);
        let (m, n) = (x - u * q, y - v * q);

        let (b_p, a_p, x_p, y_p, u_p, v_p) = (a, r, u, v, m, n);

        b = b_p;
        a = a_p;
        x = x_p;
        y = y_p;
        u = u_p;
        v = v_p;
    }
    (b, x, y)
}

fn modular_inverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = iterative_egcd(a, m);
    if g != 1 {
        None
    } else {
        Some(x % m)
    }
}

fn modular(a: i128, m: i128) -> i128 {
    if a >= 0 {
        a % m
    } else {
        m + a % m
    }
}

fn poly_pow(a: i128, b: i128, m: i128, n: i128) -> (i128, i128) {
    if m == 0 {
        (1, 0)
    } else if m % 2 == 0 {
        poly_pow(a * a % n, (a * b + b) % n, m / 2, n)
    } else {
        let (c, d) = poly_pow(a, b, m-1, n);
        (a * c % n, (a * d + b) % n)
    }
}

fn get_coefficients(card_count: i128, steps: &Vec<Technique>) -> (i128, i128) {
    steps.iter().fold((1, 0), |(a, b), s| -> (i128, i128) {
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
    })
}

pub fn run() {
    let steps = load_input("data/day22.txt".to_string());
    
    // All three dealing operations can be written as a linear operation of the form:
    //    y = a * x + b
    // where x is the initial position of the card, and y is the final position of the card

    // Part 1
    let card_count = 10007;
    let (a, b) = get_coefficients(card_count, &steps);

    let x = 2019;
    let y = modular(a * x + b, card_count);
    println!("{}", y);

    // Part 2
    let card_count = 119315717514047;
    let (a, b) = get_coefficients(card_count, &steps);

    let shuffle_count = 101741582076661;
    let (a, b) = poly_pow(a, b, shuffle_count, card_count);

    let y = 2020;
    let x = modular((y - b) * modular_inverse(a, card_count).unwrap(), card_count);
    println!("{}", x);
}