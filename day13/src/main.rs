extern crate aoc_common;

use aoc_common::advent;
use std::collections::{HashMap, HashSet};

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

type Point = (usize, usize);

#[derive(Debug)]
enum TrackPart {
    Horizontal,
    Vertical,
    Intersection,
    CurveFS,
    CurveBS,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn update(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    // \
    fn curve_backslash(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
        }
    }

    // /
    fn curve_forwardslash(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    fn turn(&self, turn: &Turn) -> Direction {
        match turn {
            Turn::Straight => self.clone(),
            Turn::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            Turn::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn default() -> Turn {
        Turn::Left
    }

    fn next(&self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    location: Point,
    direction: Direction,
    next_turn: Turn,
}

#[derive(Debug)]
struct Track {
    width: usize,
    height: usize,
    tracks: HashMap<Point, TrackPart>,
}

#[derive(Debug)]
struct Input {
    track: Track,
    carts: Vec<Cart>,
}

fn parse_input(input: &str) -> Result<Input, String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut width = 0;
    let height = lines.len();
    let mut tracks = HashMap::new();
    let mut carts = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if x > width {
                width = x;
            }

            let point = (x, y);

            let (maybe_track, maybe_cart) = match ch {
                '|' => (Some(TrackPart::Vertical), None),
                '^' => (Some(TrackPart::Vertical), Some(Direction::Up)),
                'v' => (Some(TrackPart::Vertical), Some(Direction::Down)),
                '-' => (Some(TrackPart::Horizontal), None),
                '<' => (Some(TrackPart::Horizontal), Some(Direction::Left)),
                '>' => (Some(TrackPart::Horizontal), Some(Direction::Right)),
                '+' => (Some(TrackPart::Intersection), None),
                '/' => (Some(TrackPart::CurveFS), None),
                '\\' => (Some(TrackPart::CurveBS), None),
                _ => (None, None),
            };

            if let Some(track) = maybe_track {
                tracks.entry(point).or_insert(track);
            }

            if let Some(direction) = maybe_cart {
                carts.push(Cart {
                    direction,
                    location: point,
                    next_turn: Turn::default(),
                });
            }
        }
    }

    Ok(Input {
        track: Track {
            width: width + 1,
            height,
            tracks,
        },
        carts,
    })
}

fn simulate(track: &Track, carts: &Vec<Cart>) -> Vec<Cart> {
    let mut new_carts = Vec::new();

    for cart in carts {
        let (x, y) = cart.location;
        let new_cart = match (&track.tracks[&cart.location], cart.direction) {
            (TrackPart::Horizontal, Direction::Right) => Cart {
                location: (x + 1, y),
                direction: Direction::Right,
                next_turn: cart.next_turn,
            },
            (TrackPart::Horizontal, Direction::Left) => Cart {
                location: (x - 1, y),
                direction: Direction::Left,
                next_turn: cart.next_turn,
            },
            (TrackPart::Vertical, Direction::Up) => Cart {
                location: (x, y - 1),
                direction: Direction::Up,
                next_turn: cart.next_turn,
            },
            (TrackPart::Vertical, Direction::Down) => Cart {
                location: (x, y + 1),
                direction: Direction::Down,
                next_turn: cart.next_turn,
            },
            (TrackPart::CurveBS, dir) => {
                let direction = dir.curve_backslash();
                Cart {
                    direction,
                    location: direction.update(x, y),
                    next_turn: cart.next_turn,
                }
            }
            (TrackPart::CurveFS, dir) => {
                let direction = dir.curve_forwardslash();
                Cart {
                    direction,
                    location: direction.update(x, y),
                    next_turn: cart.next_turn,
                }
            }
            (TrackPart::Intersection, dir) => {
                let direction = dir.turn(&cart.next_turn);
                let next_turn = cart.next_turn.next();
                Cart {
                    direction,
                    location: direction.update(x, y),
                    next_turn,
                }
            }
            (part, dir) => panic!("Unsupported combination of part {:?} and direction {:?}", part, dir),
        };
        new_carts.push(new_cart);
    }

    new_carts
}

fn get_crashes(carts: &Vec<Cart>) -> HashSet<Point> {
    let mut occupied = HashSet::new();
    let mut crashes = HashSet::new();

    for cart in carts {
        if !occupied.insert(cart.location) {
            crashes.insert(cart.location);
        }
    }

    crashes
}

fn part_one(input: &Input) -> Result<String, String> {
    let mut crashes = HashSet::new();
    let mut carts: Vec<_> = input.carts.to_owned();
    let mut tick: u32 = 0;

    while crashes.is_empty() && carts.len() > 1 {
        tick += 1;
        carts = simulate(&input.track, &carts);
        crashes = get_crashes(&carts);
    }

    if let Some((x, y)) = crashes.iter().next() {
        Ok(format!("{} = {},{}", tick, x, y))
    } else {
        Err("No crashes detected".to_owned())
    }
}

fn part_two(_input: &Input) -> Result<String, String> {
    Err("Not implemented".to_owned())
}