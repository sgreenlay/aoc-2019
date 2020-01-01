
use std::fs;

fn read_input(filename : String) -> Vec<u8> {
    let digits: Vec<u8> = fs::read_to_string(filename)
        .expect("Can't read file")
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

    digits
}

fn run_fft(start: &Vec<u8>) -> Vec<u8> {
    let mut input = start.clone();
    for _ in 0..100 {
        // FFT operates in repeated phases. In each phase, a new list is constructed 
        // with the same length as the input list. This new list is also used as the 
        // input for the next phase.

        let mut next: Vec<u8> = Vec::new();
        for i in 0..input.len() {
            // Each element in the new list is built by multiplying every value in 
            // the input list by a value in a repeating pattern and then adding up the 
            // results.

            // While each element in the output array uses all of the same input array 
            // elements, the actual repeating pattern to use depends on which output 
            // element is being calculated. 

            // The base pattern is 0, 1, 0, -1.

            let base_pattern = vec![0, 1, 0, -1];

            // Then, repeat each value in the pattern a number of times equal to the 
            // position in the output list being considered.

            let mut j = 0;

            let mut acc: i64 = 0;
            for k in 0..input.len() {
                if (k + 1) % (i + 1) == 0 {
                    j = (j + 1) % base_pattern.len();
                }

                let v = input[k] as i64;
                let p = base_pattern[j];
                acc += p * v;
            }

            let d = (acc.abs() % 10) as u8;
            next.push(d);
        }
        input = next;
    }

    input
}

pub fn run() {
    let input = read_input("data/day16.txt".to_string());
    
    // Part 1
    let part1 = run_fft(&input);
    for i in 0..8 {
        print!("{}", part1[i]);
    }
    println!("");

    // Part 2

    // The first seven digits of your initial input signal also represent the message 
    // offset. The message offset is the location of the eight-digit message in the 
    // final output list. Specifically, the message offset indicates the number of 
    // digits to skip before reading the eight-digit message.
    let mut offset: usize = 0;
    let mut mult: usize = 1;
    for i in 0..7 {
        offset += (input[6 - i] as usize) * mult;
        mult *= 10;
    }
    println!("Offset: {}", offset);

    // The real signal is your puzzle input repeated 10000 times.
    let mut real_signal: Vec<u8> = Vec::new();
    for _ in 0..10000 {
        for i in &input {
            real_signal.push(*i);
        }
    }

    //
    // https://www.reddit.com/r/adventofcode/comments/ebf5cy/2019_day_16_part_2_understanding_how_to_come_up/
    //
    // The second half of the digits only require the remaining digits summed together to find the next digit.
    //

    if offset < real_signal.len() / 2 {
        panic!("The trick doesn't work!");
    }

    for _ in 0..100 {
        for i in (offset..real_signal.len()-1).rev() {
            real_signal[i] = (real_signal[i] + real_signal[i + 1]) % 10;
        }
    }

    for i in 0..8 {
        print!("{}", real_signal[offset + i]);
    }
}
