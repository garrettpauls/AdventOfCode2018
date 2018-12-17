extern crate aoc_common;

mod ops;

use aoc_common::read_file_contents_as_string_from_path;
use ops::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    match part_one() {
        Ok(msg) => println!("Success: {}", msg),
        Err(err) => println!("Failed: {}", err),
    }
}

fn part_one() -> Result<String, String> {
    let ops = load_all_operations();
    let mut op_count = HashMap::new();
    let input = load_part_one_input()?;

    for item in input {
        let (opid, a, b, c) = item.args;
        let mut max_behave_like_count = op_count.entry(opid).or_insert(0);
        let mut behave_like_count = 0;

        for (fnid, op) in ops.iter().enumerate() {
            let mut registers = item.reg_before.clone();
            op(&mut registers, a, b, c);
            if registers == item.reg_after {
                behave_like_count += 1;
            }
        }

        if behave_like_count > *max_behave_like_count {
            *max_behave_like_count = behave_like_count;
        }
    }

    for (k, v) in op_count.iter() {
        println!("Op {}: {:?}", k, v);
    }

    let three_or_more: Vec<_> = op_count.iter()
        .filter(|(_, v)| **v >= 3)
        .map(|(k, _)| *k)
        .collect();

    Ok(format!("{}: {:?}", three_or_more.len(), three_or_more))
}

#[derive(Debug)]
struct PartOneInput {
    reg_before: Vec<Reg>,
    reg_after: Vec<Reg>,
    args: (Param, Param, Param, Param),
}

fn load_part_one_input() -> Result<Vec<PartOneInput>, String> {
    let input = read_file_contents_as_string_from_path("before-after-opcodes.txt")?;
    let mut lines = input.lines();
    let mut results = Vec::new();

    while let Some(before) = lines.next() {
        let ops = lines.next().ok_or("Missing op code line")?;
        let after = lines.next().ok_or("Missing after line")?;
        lines.next(); // blank line optional for last

        let args = ops.split_whitespace()
            .map(|x| Param::from_str(x)
                .map_err(|e| format!("Could not parse {} as number: {}", x, e)))
            .collect::<Result<Vec<_>, _>>()?;

        if args.len() < 4 {
            return Err(format!("Expected 4 args but found: {}", ops));
        }

        results.push(PartOneInput {
            reg_before: parse_registers(before)?,
            reg_after: parse_registers(after)?,
            args: (args[0], args[1], args[2], args[3]),
        })
    }

    return Ok(results);

    fn parse_registers(input: &str) -> Result<Vec<Reg>, String> {
        let open = input.find("[").ok_or("Could not find [")?;
        let close = input.find("]").ok_or("Could not find ]")?;
        input[open + 1..close].split(",")
            .map(|s| Reg::from_str(s.trim()).map_err(|e| format!("Could not parse {}: {}", s, e)))
            .collect()
    }
}