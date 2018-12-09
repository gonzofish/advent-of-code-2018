use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/input.txt")
        .expect("Could not read input file");

    // part 1
    println!("Checksum: {}", get_checksum(&contents));

    // part 2
    println!("Common: {}", get_common(&contents));
}

fn get_checksum(contents: &str) -> i32 {
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

    return twos * threes;
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

fn get_common(contents: &str) -> String {
    let ids: Vec<&str> = contents.lines().collect();

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if let Some(common) = check_common(&ids[i], &ids[j]) {
                return common;
            }
        }
    }

    "common string not found".to_string()
}

fn check_common(id1: &str, id2: &str) -> Option<String> {
    if id1.len() == id2.len() {
        let without_common: String = id1.chars().zip(id2.chars())
            .filter(|&(letter1, letter2)| letter1 == letter2)
            .map(|(letter, _)| letter)
            .collect();

        if without_common.len() == id1.len() - 1 {
            return Some(without_common);
        }
    }

    None
}

// hnzdozexltwgsfamqprbnuc
// wczgokexltwgsfamvprbnuy

