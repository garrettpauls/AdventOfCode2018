extern crate aoc_common;

mod state;

use aoc_common::advent;
use state::*;

fn main() {
    advent(&State::from_str, &part_one, &part_two);
}

fn part_one(input: &State) -> Result<String, String> {
    let mut current = input.simulate();
    for _ in 1..10 {
        current = current.simulate();
    }

    println!("Minute 10:\n{}", current);

    let trees = current.count_of(&Tile::Trees);
    let lumber = current.count_of(&Tile::Lumberyard);

    Err(format!("Trees {} x Lumber {} = {}", trees, lumber, trees * lumber))
}

fn part_two(_input: &State) -> Result<String, String> {
    Err("Not implemented".to_owned())
}