extern crate aoc_common;

use aoc_common::{advent, i32_from_str};

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

impl Point {
    fn distance_to(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

fn parse_input(input: &str) -> Result<Vec<Point>, String> {
    return input.lines().enumerate()
        .map(&parse_point)
        .collect();

    fn parse_point((i, line): (usize, &str)) -> Result<Point, String> {
        let c = line.find(",").ok_or(format!("Input '{}' did not contain comma.", &line))?;
        let (x, y) = line.split_at(c);
        Ok(Point {
            id: i,
            x: i32_from_str(x)?,
            y: i32_from_str(y.trim_start_matches(", "))?,
        })
    }
}

fn part_one(input: &Vec<Point>) -> Result<String, String> {
    use std::collections::HashMap;

    let mut closest = HashMap::new();
    let mx = input.iter()
        .map(|p| p.x)
        .max().expect("At least one point must exist");
    let my = input.iter()
        .map(|p| p.y)
        .max().expect("At least one point must exist");

    for x in 0..=mx {
        for y in 0..=my {
            let mut dist = i32::max_value();
            let mut id = None;

            for pt in input {
                let d = pt.distance_to(x, y);
                if d < dist {
                    dist = d;
                    id = Some(pt.id)
                } else if d == dist && id.is_some() {
                    id = None
                }
            }

            closest.insert((x, y), id);
        }
    }

    let mut area = 0;
    'point: for pt in input {
        let mut pa = 0;
        let owned = closest.iter()
            .filter(|(_, v)| v
                .unwrap_or(usize::max_value()) == pt.id)
            .map(|(k, _)| k);
        for (x, y) in owned {
            if *x == 0 || *y == 0 || *x == mx || *y == my {
                // infinite, move to next point
                continue 'point;
            }

            pa += 1;
        }

        if pa > area {
            println!("Greatest area so far from {}, {}: {}", pt.x, pt.y, pa);
            area = pa;
        }
    }

    Ok(format!("{}", area))
}

fn part_two(input: &Vec<Point>) -> Result<String, String> {
    let threshold = 10000;

    let mx = input.iter()
        .map(|p| p.x)
        .max().expect("At least one point must exist");
    let my = input.iter()
        .map(|p| p.y)
        .max().expect("At least one point must exist");

    let mut area = 0;

    for x in 0..=mx {
        'y: for y in 0..=my {
            let mut distance = 0;

            for pt in input {
                distance += pt.distance_to(x, y);
                if distance >= threshold {
                    continue 'y;
                }
            }

            area += 1;
        }
    }

    Ok(format!("{}", area))
}