extern crate aoc_common;

use aoc_common::advent;
use std::iter::{FromIterator, Iterator};

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

fn parse_input(input: &str) -> Result<Vec<char>, String> {
    Ok(input.trim().chars().collect())
}

fn create_reduction<'a, I>(input: I) -> Vec<char>
    where I: Iterator<Item=&'a char> {
    let mut polymer = input.map(|&c| c.to_owned()).collect();

    while reduce(&mut polymer) {}

    polymer
}

fn reduce(polymer: &mut Vec<char>) -> bool {
    let mut modified = false;

    let mut i = 0;
    while i < polymer.len() - 1 {
        let x = polymer[i];
        let y = polymer[i + 1];

        let same_letter = x.eq_ignore_ascii_case(&y);
        let diff_case = !x.eq(&y);

        if same_letter && diff_case {
            polymer.remove(i);
            polymer.remove(i);
            modified = true;
        }

        i += 1;
    }

    modified
}

fn part_one(input: &Vec<char>) -> Result<String, String> {
    let polymer = create_reduction(input.iter());

    Ok(format!("{} = {}", polymer.len(), String::from_iter(&polymer)))
}

fn part_two(input: &Vec<char>) -> Result<String, String> {
    use std::collections::HashSet;

    let units: HashSet<_> = input.iter()
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let mut shortest_count = usize::max_value();
    let mut shortest_str = "".to_owned();

    for unit in units {
        let filtered = input.iter()
            .filter(|c| !c.eq_ignore_ascii_case(&unit));

        let polymer = create_reduction(filtered);
        let len = polymer.len();
        if len < shortest_count {
            shortest_count = len;
            shortest_str = String::from_iter(polymer.iter());
        }
    }

    Ok(format!("{} = {}", shortest_count, shortest_str))
}