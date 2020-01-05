use std::fs;

fn read_inputs(filename: String) -> (u32, u32) {
    let values: Vec<u32> = fs::read_to_string(filename)
        .expect("Can't read file")
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    if values.len() != 2 {
        panic!("Should find to numbers");
    }

    (values[0], values[1])
}

pub fn run() {
    let (start, end) = read_inputs("data/day04.txt".to_string());

    let mut found_passwords_part1 = 0;
    let mut found_passwords_part2 = 0;

    for n in start..end {
        // A few key facts about the password:
        //   - Two adjacent digits are the same (like 22 in 122345).
        //   - Going from left to right, the digits never decrease;
        //     they only ever increase or stay the same (like 111123 or 135679).

        // one more important detail:
        //   - The two adjacent matching digits are not part of a larger group of matching digits.

        let digits: Vec<u32> = n
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        let mut increasing = true;
        let mut adjacent = false;
        let mut contains_double = false;
        for i in 1..digits.len() {
            if digits[i] < digits[i - 1] {
                increasing = false;
                break;
            }
            if digits[i] == digits[i - 1] {
                adjacent = true;

                if ((i == 1) || (digits[i] != digits[i - 2]))
                    && ((i == digits.len() - 1) || (digits[i] != digits[i + 1]))
                {
                    contains_double = true;
                }
            }
        }
        if increasing && adjacent {
            found_passwords_part1 += 1;

            if contains_double {
                found_passwords_part2 += 1;
            }
        }
    }
    println!("{}", found_passwords_part1);
    println!("{}", found_passwords_part2);
}
