extern crate aoc_common;

use aoc_common::{advent, i32_from_str};

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Debug)]
struct Input {
    players: i32,
    marbles: i32,
}

fn parse_input(input: &str) -> Result<Vec<Input>, String> {
    input.lines()
        .map(|line| {
            let mut parts = line.split(';');
            let players = i32_from_str(parts.next().ok_or("Player count not supplied")?)?;
            let marbles = i32_from_str(parts.next().ok_or("Marble count not supplied")?)?;

            Ok(Input {
                players,
                marbles,
            })
        })
        .collect()
}

fn part_one(input: &Vec<Input>) -> Result<String, String> {
    let mut scores = Vec::new();

    for config in input {
        let (player, score) = run_game(config)?;
        scores.push(score);
        println!("{}, {}: player {} won with score {}", config.players, config.marbles, player, score);
    }

    Ok(format!("{:?}", scores))
}

fn run_game(input: &Input) -> Result<(usize, u64), String> {
    let mut marbles = vec![0];
    let mut current = 0;
    let mut scores: Vec<u64> = (0..input.players).map(|_| 0).collect();
    let mut current_player = scores.len() - 1;
    let progress = if input.marbles >= 100 { input.marbles / 100 } else { input.marbles } as u64;

    for m in 1..=input.marbles as u64 {
        current_player = (current_player + 1) % scores.len();

        if m % progress == 0 {
            println!("{}% {}", (m * 100) / input.marbles as u64, m);
        }

        if m % 23 == 0 {
            if let Some(v) = scores.get_mut(current_player) {
                *v += m;

                // temp so we can go negative if needbe
                let mut temp = current as i32 - 7;
                while temp < 0 {
                    temp += marbles.len() as i32;
                }
                current = temp as usize;
                let r = marbles.remove(current);

                *v += r;
            }
        } else {
            current = (current + 1) % marbles.len() + 1;
            marbles.insert(current, m);
        }
    }

    scores.iter().enumerate().max_by_key(|(_, s)| *s)
        .map(|(p, s)| (p + 1, *s))
        .ok_or("No highest score".to_owned())
}

fn part_two(input: &Vec<Input>) -> Result<String, String> {
    let mut scores = Vec::new();

    for config in input {
        let (player, score) = run_game(&Input {
            players: config.players,
            marbles: config.marbles * 100,
        })?;
        scores.push(score);
        println!("{}, {}: player {} won with score {}", config.players, config.marbles, player, score);
    }

    Ok(format!("{:?}", scores))
}
