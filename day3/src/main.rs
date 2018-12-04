extern crate aoc_common;
extern crate regex;

use aoc_common::advent;
use std::collections::HashMap;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug)]
struct Rect {
    id: String,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn coords(&self) -> Vec<(u32, u32)> {
        let mut coords = Vec::new();

        for r in self.left..(self.left + self.width) {
            for c in self.top..(self.top + self.height) {
                coords.push((r, c));
            }
        }

        coords
    }
}

fn parse_input(input: &str) -> Result<Vec<Rect>, String> {
    use regex::{Regex, Captures};

    let re = Regex::new(r"(?x)
# #3 @ 5,5: 2x2
^
(?P<id>[^\s@]+) # id (#3)
\s*@\s* # ( @ )
(?P<left>\d+)
,
(?P<top>\d+)
:\s*
(?P<width>\d+)
x
(?P<height>\d+)
$
").expect("Built in regex was incorrect");

    let results: Result<Vec<_>, _> = input.lines()
        .map(|line| re.captures(&line)
            .ok_or(format!("Regex failed to match line: {}", &line))
            .and_then(&parse_rect))
        .collect();

    return results;

    fn parse_rect(caps: Captures) -> Result<Rect, String> {
        let id = caps["id"].to_owned();
        let left = u32_from_str(&caps["left"])?;
        let top = u32_from_str(&caps["top"])?;
        let width = u32_from_str(&caps["width"])?;
        let height = u32_from_str(&caps["height"])?;
        Ok(Rect {
            id,
            left,
            top,
            width,
            height,
        })
    }

    fn u32_from_str(value: &str) -> Result<u32, String> {
        use std::str::FromStr;
        u32::from_str(&value).map_err(|e| e.to_string())
    }
}

fn calculate_shares(input: &Vec<Rect>) -> HashMap<(u32, u32), u32> {
    let mut shares = HashMap::new();

    for rect in input {
        for xy in rect.coords() {
            shares.entry(xy)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    shares
}

fn part_one(input: &Vec<Rect>) -> Result<String, String> {
    let shares = calculate_shares(input);

    let share_count = shares
        .values()
        .filter(|v| **v > 1)
        .count();

    Ok(format!("{}", share_count))
}

fn part_two(input: &Vec<Rect>) -> Result<String, String> {
    let shares = calculate_shares(input);

    for rect in input {
        let is_shared = rect.coords().iter()
            .filter_map(|xy| shares.get(xy))
            .any(|share_count| *share_count > 1);
        if !is_shared {
            return Ok(rect.id.clone());
        }
    }

    Err("All overlap".to_owned())
}