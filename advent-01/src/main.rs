use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/input.txt")
        .expect("Something went wrong reading the file");
    let numbers = read_numbers(&contents);

    // problem #1
    println!("Frequency: {}", get_frequency(&numbers));
    // problem #2
    println!("First repeat: {}", get_repeat(&numbers));
}

fn read_numbers(contents: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let value: i32 = line.parse()
            .expect("Couldn't parse line to number");

        numbers.push(value);
    }

    numbers
}

fn get_frequency(numbers: &Vec<i32>) -> i32 {
    let mut frequency: i32 = 0;

    for value in numbers {
        frequency += value;
    }

    frequency
}

fn get_repeat(numbers: &Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    let mut frequency: i32 = 0;

    seen.insert(0);

    loop {
        for value in numbers {
            frequency += value;

            if seen.contains(&frequency) {
                return frequency;
            }

            seen.insert(frequency);
        }
    }
}
