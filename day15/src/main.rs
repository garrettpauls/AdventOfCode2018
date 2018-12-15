extern crate aoc_common;

use aoc_common::advent;

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

#[derive(Copy, Clone)]
enum Tile {
    Open,
    Wall,
    Goblin(i32),
    Elf(i32),
}

struct State {
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
}

impl State {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(self.index(x, y))
    }
}

fn parse_input(input: &str) -> Result<State, String> {
    let mut tiles = Vec::new();
    let mut height = 0;

    // making assumption that all lines will always be the same length
    for line in input.lines() {
        height += 1;
        for char in line.chars() {
            tiles.push(parse_tile(char));
        }
    }

    return Ok(State {
        height,
        width: tiles.len() / height,
        tiles,
    });

    fn parse_tile(ch: char) -> Tile {
        match ch {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            'G' => Tile::Goblin(0),
            'E' => Tile::Elf(0),
            _ => panic!("Unsupported character: {}", ch),
        }
    }
}

fn show_state(state: &State) -> String {
    let mut result = String::new();

    for y in 0..state.height {
        for x in 0..state.width {
            if let Some(tile) = state.get_tile(x, y) {
                result += match tile {
                    Tile::Wall => "#",
                    Tile::Open => ".",
                    Tile::Elf(_) => "E",
                    Tile::Goblin(_) => "G",
                }
            }
        }

        result += "\n";
    }

    result
}

fn part_one(input: &State) -> Result<String, String> {
    println!("{}", show_state(&input));

    Err("Not implemented".to_owned())
}

fn part_two(input: &State) -> Result<String, String> {
    Err("Not implemented".to_owned())
}