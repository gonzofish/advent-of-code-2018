use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/input.txt")
        .expect("Could not read input file");
    let mut twos = 0;
    let mut threes = 0;

    for line in contents.lines() {
        let (has_two, has_three) = get_has_exacts(get_counts(&line));

        if has_two {
            twos += 1;
        }

        if has_three {
            threes += 1;
        }
    }

    println!("Checksum: {}", twos * threes);
}

fn get_counts(line: &str) -> Vec<i32> {
    let mut counts = HashMap::new();
    let mut values: Vec<i32> = Vec::new();

    for letter in line.split("") {
        if letter != "" {
            let count = counts.entry(letter).or_insert(0);

            *count += 1;
        }
    }

    for value in counts.values() {
        values.push(*value);
    }

    values
}

fn get_has_exacts(counts: Vec<i32>) -> (bool, bool) {
    let mut has_two = false;
    let mut has_three = false;

    for count in counts {
        if count == 2 {
            has_two = true;
        }

        if count == 3 {
            has_three = true;
        }

        if has_two && has_three {
            break;
        }
    }

    (has_two, has_three)
}
