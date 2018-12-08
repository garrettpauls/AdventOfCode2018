extern crate aoc_common;

use aoc_common::advent;
use std::collections::{HashSet, HashMap};

fn main() {
    advent(&parse_input, &part_one, &part_two);
}


fn parse_input(input: &str) -> Result<Vec<(String, String)>, String> {
    // Step C must be finished before step A can begin.
    input.lines().map(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 10 {
            return Err(format!("Expected 10 words in the line: {}", line));
        }

        let subject = parts[1].to_owned();
        let dependent = parts[7].to_owned();

        Ok((subject, dependent))
    }).collect::<Result<Vec<_>, _>>()
}

fn part_one(input: &Vec<(String, String)>) -> Result<String, String> {
    let mut order: Vec<&String> = Vec::new();
    let mut available = determine_root_instructions(input);

    while available.len() > 0 {
        available.sort();
        available.reverse();

        let current = available.pop().unwrap();
        order.push(current);

        let next = order.iter()
            .map(|x| determine_next(input, x))
            .flatten()
            .filter(|x| !order.contains(x));
        for n in next {
            if are_prereqs_filled(input, &n, &order)
                && !available.contains(&n)
                && !order.contains(&n) {
                available.push(n)
            }
        }
    }

    let result = order.iter().fold("".to_owned(), |acc, x| acc + x);
    Ok(result)
}

fn determine_next<'a>(input: &'a Vec<(String, String)>, current: &str) -> Vec<&'a String> {
    input.iter()
        .filter(|(before, _)| before == current)
        .map(|(_, after)| after)
        .collect()
}

fn are_prereqs_filled(input: &Vec<(String, String)>, current: &str, filled: &Vec<&String>) -> bool {
    input.iter()
        .filter(|(_, after)| after == current)
        .all(|(before, _)| filled.contains(&before))
}

fn determine_root_instructions(input: &Vec<(String, String)>) -> Vec<&String> {
    let result: HashSet<_> = input.iter().map(|(before, _)| before)
        .filter(|x| input.iter().all(|(_, after)| after != *x))
        .collect();
    result.iter().map(|x| *x).collect()
}

fn part_two(input: &Vec<(String, String)>) -> Result<String, String> {
    let worker_count = 5;
    let base_seconds = 60;

    let mut completed: Vec<&String> = Vec::new();
    let mut available = determine_root_instructions(input);
    let mut in_progress = HashMap::new();
    let mut seconds = 0;

    while available.len() > 0 || in_progress.len() > 0 {
        available.sort();
        available.reverse();

        // populate steps in progress
        while available.len() > 0 && in_progress.len() < worker_count {
            let current = available.pop().unwrap();
            in_progress.entry(current)
                .or_insert(get_step_time(current) + base_seconds);
        }

        // process steps until one is completed
        let mut just_completed = Vec::new();
        loop {
            seconds += 1;

            for (key, sec) in in_progress.iter_mut() {
                *sec -= 1;
                if *sec == 0 {
                    just_completed.push(*key);
                }
            }

            if just_completed.len() > 0 {
                break;
            }
        }

        for key in just_completed {
            completed.push(key);
            in_progress.remove(key);
        }

        let next = completed.iter()
            .map(|x| determine_next(input, x))
            .flatten()
            .filter(|x| !completed.contains(x));
        for n in next {
            if are_prereqs_filled(input, &n, &completed)
                && !available.contains(&n)
                && !completed.contains(&n) {
                available.push(n)
            }
        }
    }

    Ok(format!("{}", seconds))
}

fn get_step_time(step: &str) -> i32 {
    let delay: usize = step.chars()
        .map(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".char_indices()
            .find(|(_, x)| *x == c)
            .map(|(i, _)| i + 1)
            .unwrap_or(0))
        .sum();
    delay as i32
}
