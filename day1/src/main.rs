extern crate aoc_common;

use aoc_common::advent;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

fn parse_input(input: &str) -> Result<Vec<i32>, String> {
    use std::str::FromStr;

    input.lines()
        .map(|line| i32::from_str(line)
            .map_err(|e| format!("Failed to parse {} as i32: {}", line, e)))
        .collect()
}

fn part_one(input: &Vec<i32>) -> Result<String, String> {
    Ok(input.iter().sum::<i32>().to_string())
}

fn part_two(input: &Vec<i32>) -> Result<String, String> {
    use std::collections::HashSet;

    let mut frequencies = HashSet::new();
    let mut last = 0;

    let mut iter = input.iter();

    loop {
        if let Some(x) = iter.next() {
            last += x;

            let seen_before = !frequencies.insert(last);
            if seen_before {
                return Ok(last.to_string());
            }
        } else {
            iter = input.iter();
        }
    }
}
