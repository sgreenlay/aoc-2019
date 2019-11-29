
use std::io::BufRead;
use std::io;

use std::fs;

fn read_inputs(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

pub fn run() {
    let inputs = read_inputs("data/day02.txt".to_string())
        .expect("Can't read file");
    
    // Part 1
    let mut twice_count = 0;
    let mut three_count = 0;
    for input in &inputs {
        let mut char_counts: [u32; 26] = [0; 26];
        for ch in input.chars() {
            let zero = 'a' as usize;
            let idx = ch as usize;
            char_counts[idx - zero] += 1;
        }
        
        let mut has_twice = false;
        let mut has_three = false;

        for count in char_counts.into_iter() {
            if count == &2 {
                has_twice = true;
            }
            if count == &3 {
                has_three = true;
            }
            if has_twice && has_three {
                break;
            }
        }
        if has_twice {
            twice_count += 1;
        }
        if has_three {
            three_count += 1;
        }
    }
    println!("{} * {} = {}", twice_count, three_count, twice_count * three_count);

    // Part 2
    let mut found = false;
    for input in &inputs {
        for compare in &inputs {
            if input.len() != compare.len() {
                break;
            }

            let count = input.chars().count();

            let mut input_iter = input.chars();
            let mut compare_iter = compare.chars();

            let mut diff_count = 0;
            for _ in 0..count {
                if input_iter.next() != compare_iter.next() {
                    diff_count += 1;
                }
                if diff_count > 1 {
                    break;
                }
            }
            if diff_count == 1 {
                let mut input_iter = input.chars();
                let mut compare_iter = compare.chars();

                let mut common_chars = String::from("");
                for _ in 0..count {
                    let ch = input_iter.next();
                    if ch == compare_iter.next() && ch != None {
                        common_chars.push(ch.unwrap());
                    }
                }
                println!("{}", common_chars);
                found = true;

                break;
            }
        }
        if found {
            break;
        }
    }
}
