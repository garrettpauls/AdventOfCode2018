extern crate aoc_common;

use aoc_common::advent;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

// Note: this could probably be rewritten to return &str instead of String
// need to learn more about lifetimes.
fn parse_input(input: &str) -> Result<Vec<String>, String> {
    Ok(input.lines().map(|s| s.to_owned()).collect())
}

fn part_one(input: &Vec<String>) -> Result<String, String> {
    use std::collections::HashMap;

    let mut twos = 0;
    let mut threes = 0;
    let mut counts = HashMap::new();

    for value in input {
        counts.clear();

        for ch in value.chars() {
            counts.entry(ch)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        if counts.values().any(|v| *v == 2) {
            twos += 1;
        }
        if counts.values().any(|v| *v == 3) {
            threes += 1;
        }
    }

    let checksum = twos * threes;
    Ok(format!("{}", checksum))
}

fn part_two(_input: &Vec<String>) -> Result<String, String> {
    Err("Not implemented".to_owned())
}
