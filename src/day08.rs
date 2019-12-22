
use std::fs;

fn load_image(filename : String) -> Vec<u32> {
    fs::read_to_string(filename)
        .expect("Can't read file")
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

fn count_digits(image: &[u32], digit: u32) -> usize {
    let digits: Vec<u32> = image.iter().cloned().filter(|&n| n == digit).collect();
    digits.len()
}

pub fn run() {
    let image = load_image("data/day08.txt".to_string());
    let (width, height) = (25, 6);

    // Part 1
    let layer_size = width * height;
    let layer_count = image.len() / layer_size;

    let mut counts = (layer_size, 0, 0);
    for i in 0..layer_count {
        let layer: &[u32] = &image[i*layer_size..(i+1)*layer_size];

        let zero_count = count_digits(layer, 0);
        if zero_count < counts.0 { 
            counts = (zero_count, count_digits(layer, 1), count_digits(layer, 2));
        }
    }
    println!("0:{} 1:{} 2:{} {}", counts.0, counts.1, counts.2, counts.1 * counts.2);
}
