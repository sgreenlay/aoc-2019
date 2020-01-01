
use std::fs;

fn read_input(filename : String) -> Vec<i128> {
    let digits: Vec<i128> = fs::read_to_string(filename)
        .expect("Can't read file")
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i128)
        .collect();

    digits
}

pub fn run() {
    let mut input = read_input("data/day16.txt".to_string());
    for _ in 0..100 {
        // FFT operates in repeated phases. In each phase, a new list is constructed 
        // with the same length as the input list. This new list is also used as the 
        // input for the next phase.

        let mut next: Vec<i128> = Vec::new();
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

            let mut acc = 0;
            for k in 0..input.len() {
                if (k + 1) % (i + 1) == 0 {
                    j = (j + 1) % base_pattern.len();
                }

                let p = base_pattern[j];
                acc += p * input[k];
            }

            let d = (acc % 10).abs();
            next.push(d);
        }
        input = next;
    }

    for i in 0..8 {
        print!("{}", input[i]);
    }
    println!("");
}
