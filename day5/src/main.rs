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
    Err("Not implemented".to_owned())
}