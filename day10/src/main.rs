extern crate aoc_common;

use aoc_common::{advent, i32_from_str};

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Input {
    position: Point,
    velocity: Point,
}

fn parse_input(input: &str) -> Result<Vec<Input>, String> {
    return input.lines().map(&parse_line).collect();

    fn parse_line(line: &str) -> Result<Input, String> {
        // position=<-3,  6> velocity=< 2, -1>
        let mut parts = line
            .split(|c: char| c == '<' || c == '>' || c == ',')
            .map(|s| s.trim());

        parts.next(); // position=
        let position = Point {
            x: i32_from_str(parts.next().ok_or("Position x not supplied")?)?,
            y: i32_from_str(parts.next().ok_or("Position y not supplied")?)?,
        };

        parts.next(); // velocity=
        let velocity = Point {
            x: i32_from_str(parts.next().ok_or("Velocity x not supplied")?)?,
            y: i32_from_str(parts.next().ok_or("Velocity y not supplied")?)?,
        };

        Ok(Input {
            position,
            velocity,
        })
    }
}

fn part_one(input: &Vec<Input>) -> Result<String, String> {
    use std::io::{stdin, Write};
    use std::fs::File;

    let mut points = simulate(input, 1);
    let mut area = get_area(input);
    let mut exit = "".to_owned();
    let mut iterations = 1;

    loop {
        iterations += 1;
        points = simulate(&points, 1);
        let a = get_area(&points);

        if a > area {
            points = simulate(&points, -1);
            iterations -= 1;
            break;
        }

        area = a;
    }

    Err(format!("seconds: {}\n{}", iterations, show(&points)))
}

fn is_candidate(input: &Vec<Input>) -> bool {
    return true;
}

fn get_bounds(input: &Vec<Input>) -> (i32, i32, i32, i32) {
    let left = input.iter().map(|p| p.position.x).min().unwrap();
    let right = input.iter().map(|p| p.position.x).max().unwrap();
    let top = input.iter().map(|p| p.position.y).min().unwrap();
    let bottom = input.iter().map(|p| p.position.y).max().unwrap();

    (left, top, right, bottom)
}

fn get_area(input: &Vec<Input>) -> i64 {
    let (left, top, right, bottom) = get_bounds(input);
    let width = right - left;
    let height = bottom - top;
    width as i64 * height as i64
}

fn show(input: &Vec<Input>) -> String {
    let (left, top, right, bottom) = get_bounds(input);
    let mut result = "\n".to_owned();

    for y in top..=bottom {
        for x in left..=right {
            if let Some(i) = input.iter().find(|i| i.position.x == x && i.position.y == y) {
                result += "#";
            } else {
                result += ".";
            }
        }
        result += "\n";
    }

    result
}

fn simulate(input: &Vec<Input>, dir: i32) -> Vec<Input> {
    input.iter().map(|i| Input {
        position: Point {
            x: i.position.x + i.velocity.x * dir,
            y: i.position.y + i.velocity.y * dir,
        },
        velocity: Point {
            x: i.velocity.x,
            y: i.velocity.y,
        },
    }).collect()
}

fn part_two(input: &Vec<Input>) -> Result<String, String> {
    Err("See seconds from part 1".to_owned())
}
