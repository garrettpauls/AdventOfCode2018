extern crate aoc_common;

mod state;

use aoc_common::advent;
use state::*;

fn main() {
    advent(&State::from_str, &part_one, &part_two);
}

fn get_total_after_minutes(minutes: usize, input: &State) -> String {
    let mut current = input.simulate();
    for m in 1..minutes {
        println!("{} = {}", m, get_score(&current));
        current = current.simulate();
    }

    println!("Minute {}:\n{}", minutes, current);


    format!("{}", get_score(&current))
}

fn get_score(state: &State) -> usize {
    let trees = state.count_of(&Tile::Trees);
    let lumber = state.count_of(&Tile::Lumberyard);

    trees * lumber
}

fn part_one(input: &State) -> Result<String, String> {
    Ok(get_total_after_minutes(10, input))
}

fn part_two(input: &State) -> Result<String, String> {
    // 1000000000
    Ok(get_total_after_minutes(2000, input))
}