use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/input.txt")
        .expect("Something went wrong reading the file");

    // problem #1
    println!("Frequency: {}", get_frequency(&contents));
    // problem #2
    println!("First repeat: {}", get_repeat(&contents));
}

fn get_frequency(contents: &str) -> i32 {
    let mut frequency: i32 = 0;

    for line in contents.lines() {
        let value: i32 = line.parse()
            .expect("Couldn't parse line to number");

        frequency += value;
    }

    frequency
}

fn get_repeat(contents: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut frequency: i32 = 0;

    seen.insert(0);

    loop {
        for line in contents.lines() {
            let value: i32 = line.parse()
                .expect("Couldn't parse line to number");

            frequency += value;

            if seen.contains(&frequency) {
                return frequency;
            }

            seen.insert(frequency);
        }
    }
}
