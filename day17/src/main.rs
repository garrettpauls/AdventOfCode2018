extern crate aoc_common;
extern crate regex;

use aoc_common::{advent, i32_from_str};
use regex::Regex;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

type Point = (i32, i32);

#[derive(Debug)]
struct State {
    width: Point,
    height: Point,
    clay: Vec<Point>,
}

fn parse_input(input: &str) -> Result<State, String> {
    let mut clay = Vec::new();
    let mut left = 500;
    let mut right = 500;
    let mut top = i32::max_value();
    let mut bottom = i32::min_value();

    // y=13, x=498..504
    let rx = Regex::new(r"(?x)
(?P<fixed>[xy])=(?P<fixed_value>\d+),\s*
[xy]=(?P<low>\d+)..(?P<high>\d+)
").unwrap();

    for line in input.lines() {
        if let Some(caps) = rx.captures(line) {
            let is_fixed_x = &caps["fixed"] == "x";
            let fixed = i32_from_str(&caps["fixed_value"])?;
            let low = i32_from_str(&caps["low"])?;
            let high = i32_from_str(&caps["high"])?;

            for v in low..=high {
                let (x, y) = if is_fixed_x { (fixed, v) } else { (v, fixed) };
                clay.push((x, y));

                if x < left {
                    left = x;
                }
                if x > right {
                    right = x;
                }
                if y < top {
                    top = y;
                }
                if y > bottom {
                    bottom = y;
                }
            }
        } else {
            return Err(format!("Invalid input: {}", line));
        }
    }

    Ok(State {
        clay,
        width: (left - 1, right + 1),
        height: (top, bottom),
    })
}

fn print_state(state: &State) {
    for y in state.height.0..=state.height.1 {
        for x in state.width.0..=state.width.1 {
            if x == 500 && y == 1 {
                print!("+");
            } else if state.clay.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn part_one(input: &State) -> Result<String, String> {
//    println!("{:?}", input);
    print_state(&input);
    Err("Not implemented".to_owned())
}

fn part_two(_input: &State) -> Result<String, String> {
    Err("Not implemented".to_owned())
}