extern crate aoc_common;

mod ops;

use aoc_common::{advent, usize_from_str};
use ops::*;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug)]
struct Program {
    ip: usize,
    instructions: Vec<(String, Param, Param, Param)>,
}

fn parse_input(input: &str) -> Result<Program, String> {
    let mut ip = 0;
    let mut instructions = Vec::new();

    for line in input.lines() {
        if line.starts_with("#ip") {
            ip = usize_from_str(&line[3..].trim())?;
        } else {
            let mut parts = line.split_whitespace();
            instructions.push(
                (
                    parts.next().ok_or("Instruction missing")?.to_owned(),
                    parts.next().map(usize_from_str).unwrap_or(Err("Missing arg 1".to_owned()))?,
                    parts.next().map(usize_from_str).unwrap_or(Err("Missing arg 2".to_owned()))?,
                    parts.next().map(usize_from_str).unwrap_or(Err("Missing arg 3".to_owned()))?,
                )
            )
        }
    }

    Ok(Program {
        ip,
        instructions,
    })
}

fn run_program(prog: &Program, registers: &mut Vec<Reg>) {
    let ops = load_operations();

    loop {
        let idx = registers[prog.ip];
        if idx < 0 || idx >= prog.instructions.len() as i64 {
            break;
        }

        let (op_code, a, b, c) = &prog.instructions[idx as usize];
        let op = ops[&op_code[..]];

//        print!("{} {} {} {} {:?} -> ", op_code, a, b, c, registers);

        op(registers, *a, *b, *c);

//        println!("{:?}", registers);

        registers[prog.ip] = { registers[prog.ip] + 1 };
    }
}

fn part_one(prog: &Program) -> Result<String, String> {
    let mut registers: Vec<Reg> = vec![0, 0, 0, 0, 0, 0];

    run_program(&prog, &mut registers);

    Ok(format!("{} ({:?})", registers[0], registers))
}

fn part_two(prog: &Program) -> Result<String, String> {
    let mut registers: Vec<Reg> = vec![1, 0, 0, 0, 0, 0];

    run_program(&prog, &mut registers);

    Ok(format!("{} ({:?})", registers[0], registers))
}