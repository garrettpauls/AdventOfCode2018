extern crate aoc_common;

mod ops;

use aoc_common::read_file_contents_as_string_from_path;
use ops::*;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

fn main() {
    match part_one() {
        Ok(msg) => println!("One Success: {}", msg),
        Err(err) => println!("One Failed: {}", err),
    }
    match part_two() {
        Ok(msg) => println!("Two Success: {}", msg),
        Err(err) => println!("Two Failed: {}", err),
    }
}

fn part_one() -> Result<String, String> {
    let ops = load_all_operations();
    let input = load_samples()?;
    let mut samples_matching_three = 0;

    for item in input {
        let (_, a, b, c) = item.args;
        let mut behave_like_count = 0;

        for op in ops.iter() {
            let mut registers = item.reg_before.clone();
            op(&mut registers, a, b, c);
            if registers == item.reg_after {
                behave_like_count += 1;
            }
        }

        if behave_like_count >= 3 {
            samples_matching_three += 1;
        }
    }

    Ok(format!("{}", samples_matching_three))
}

fn part_two() -> Result<String, String> {
    let program = load_program()?;
    let op_codes = resolve_op_codes()?;

    let mut registers = vec![0, 0, 0, 0];

    for (op, a, b, c) in program {
        op_codes[&op](&mut registers, a, b, c);
    }

    Ok(format!("{}", registers[0]))
}

fn resolve_op_codes() -> Result<HashMap<Param, &'static Fn(&mut Vec<Reg>, Param, Param, Param)>, String> {
    let operations = load_all_operations();
    let sample = load_samples()?;
    let mut candidates: HashMap<Param, (HashSet<usize>, HashSet<usize>)> = HashMap::new();

    for item in sample {
        let (op_code, a, b, c) = item.args;
        let (ref mut matches, ref mut rejected) = candidates
            .entry(op_code)
            .or_insert_with(|| (HashSet::new(), HashSet::new()));

        for (i, op) in operations.iter().enumerate() {
            let mut registers = item.reg_before.clone();
            op(&mut registers, a, b, c);
            if registers == item.reg_after {
                if !rejected.contains(&i) {
                    matches.insert(i);
                }
            } else {
                matches.remove(&i);
                rejected.insert(i);
            }
        }
    }

    for (i, (set, _)) in &candidates {
        if set.len() != 1 {
            return Err(format!("Didn't resolve to single op {}: {:?}", i, set));
        }
    }

    let results: HashMap<_, _> = candidates.iter()
        .map(|(k, (v, _))| (*k, operations[*v.iter().next().unwrap()]))
        .collect();

    Ok(results)
}

#[derive(Debug)]
struct PartOneInput {
    reg_before: Vec<Reg>,
    reg_after: Vec<Reg>,
    args: (Param, Param, Param, Param),
}

fn load_samples() -> Result<Vec<PartOneInput>, String> {
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

fn load_program() -> Result<Vec<(Param, Param, Param, Param)>, String> {
    let input = read_file_contents_as_string_from_path("test-program.txt")?;
    return input.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let op = parse_param(parts.next())?;
            let a = parse_param(parts.next())?;
            let b = parse_param(parts.next())?;
            let c = parse_param(parts.next())?;
            Ok((op, a, b, c))
        })
        .collect::<Result<Vec<_>, _>>();

    fn parse_param(value: Option<&str>) -> Result<Param, String> {
        if let Some(v) = value {
            Param::from_str(v)
                .map_err(|e| format!("{}", e))
        } else {
            Err("Can not parse None as param".to_owned())
        }
    }
}