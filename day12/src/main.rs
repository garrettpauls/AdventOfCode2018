extern crate aoc_common;

use aoc_common::advent;
use std::collections::HashMap;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug)]
struct Input {
    initial_state: Vec<bool>,
    rules: HashMap<String, bool>,
}

fn parse_input(input: &str) -> Result<Input, String> {
    let lines: Vec<&str> = input.lines().collect();

    let state = lines.iter()
        .find(|l| l.starts_with("initial state: "))
        .map(|l| l.trim_start_matches("initial state: "))
        .ok_or("Could not find initial state")?;

    let rules: HashMap<_, _> = lines.iter()
        .filter(|l| l.contains(" => "))
        .map(|l| {
            let mut parts = l.split(" => ");
            let key = parts.next().unwrap();
            let result = parts.next().unwrap();

            (key.to_owned(),
             *parse_plant_list(result).first().unwrap_or(&false))
        }).collect();

    return Ok(Input {
        rules,
        initial_state: parse_plant_list(state),
    });
}

fn parse_plant(ch: char) -> bool {
    ch == '#'
}

fn parse_plant_list(input: &str) -> Vec<bool> {
    input.chars().map(&parse_plant).collect()
}

fn show_plant_list(plants: &[bool]) -> String {
    plants.iter().map(|c| if *c { '#' } else { '.' }).collect()
}

fn simulate(zero: usize, current: &Vec<bool>, rules: &HashMap<String, bool>) -> (usize, Vec<bool>) {
    let mut padded = vec![false, false, false, false];
    padded.append(&mut current.clone());
    padded.append(&mut vec![false, false, false, false]);
    let mut offset = 2;
    let mut next = Vec::new();

    for i in 2..padded.len() - 2 {
        let s = show_plant_list(&padded[(i - 2)..(i + 3)]);
        let n = if let Some(n) = rules.get(&s) {
            *n
        } else {
            false
        };
        next.push(n);
    }

    if let Some(start) = next.iter().position(|x| *x) {
        let start = start.min(offset);
        offset -= start;
        for _ in 0..start {
            next.remove(0);
        }
    }

    if let Some(end) = next.iter().rposition(|x| *x) {
        let end = next.len() - end - 1;
        for _ in 0..end {
            let l = next.len() - 1;
            next.remove(l);
        }
    }

    (zero + offset, next)
}

fn simulate_for(gen: u64, input: &Input, show_prog_every: u64) -> (usize, Vec<bool>) {
    let mut state = input.initial_state.clone();
    let mut zero = 0;

//    println!("{}: {}", zero, show_plant_list(&state));
    for p in 0..gen {
        if p % show_prog_every == 0 {
            println!("{}", p);
        }
        let (nz, ns) = simulate(zero, &state, &input.rules);
        state = ns;
        zero = nz;
//        println!("{}: {}", zero, show_plant_list(&state));
    }

    (zero, state)
}

fn sum_plant_locations(zero: usize, state: &Vec<bool>) -> i64 {
    let mut sum: i64 = 0;
    for (i, v) in state.iter().enumerate() {
        if *v {
            sum += i as i64 - zero as i64;
        }
    }
    sum
}

fn part_one(input: &Input) -> Result<String, String> {
    let (zero, state) = simulate_for(20, input, 1);
    let sum = sum_plant_locations(zero, &state);
    Ok(format!("{}", sum))
}

fn part_two(input: &Input) -> Result<String, String> {
//    let target: i64 = 50000000000;
    let target: i64 = 2000;
    let mut state = input.initial_state.clone();
    let mut zero = 0;
    let mut last_value = 0;

    for i in 0..=target {
        let sum = sum_plant_locations(zero, &state);
        println!("Iteration {}: {} ({})", i, sum, sum - last_value);
        last_value = sum;

        let (nz, ns) = simulate(zero, &state, &input.rules);

        state = ns;
        zero = nz;
    }
//    let (zero, state) = simulate_for(50000000000, input, 10000);
    let sum = sum_plant_locations(zero, &state);
    Ok(format!("{}", sum))
}