extern crate aoc_common;

use aoc_common::advent;
use std::collections::VecDeque;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug)]
struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        let local: usize = self.metadata.iter().sum();
        let child: usize = self.children.iter().map(|c| c.sum_metadata()).sum();

        local + child
    }

    fn get_value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter()
                .map(|idx| self.children.get(*idx - 1)
                    .map_or(0, |c| c.get_value()))
                .sum()
        }
    }
}

fn parse_input(input: &str) -> Result<Node, String> {
    use std::str::FromStr;

    let mut numbers = input.split_whitespace()
        .map(|x| usize::from_str(x).map_err(|e| format!("Could not parse {}: {}", x, e)))
        .collect::<Result<VecDeque<_>, _>>()?;

    parse_node(&mut numbers)
}

fn parse_node(values: &mut VecDeque<usize>) -> Result<Node, String> {
    // [child count, metadata count, [children], metadata entries]
    let child_count = values.pop_front().ok_or("No child count supplied")?;
    let meta_count = values.pop_front().ok_or("No meta count supplied")?;
    let mut children = Vec::new();
    let mut metadata = Vec::new();

    for _ in 0..child_count {
        children.push(parse_node(values)?);
    }

    for _ in 0..meta_count {
        metadata.push(values.pop_front().ok_or("Not enough metadata")?);
    }

    Ok(Node {
        metadata,
        children,
    })
}

fn part_one(input: &Node) -> Result<String, String> {
    Ok(format!("{}", input.sum_metadata()))
}

fn part_two(input: &Node) -> Result<String, String> {
    Ok(format!("{}", input.get_value()))
}