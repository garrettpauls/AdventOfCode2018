extern crate rayon;

use std::collections::HashMap;

type Point = (i32, i32);

struct Grid {
    width: i32,
    height: i32,
    data: HashMap<Point, i32>,
}

fn main() {
    let grid = build_grid(9110, 300, 300);
    part_one(&grid);
    part_two(&grid);
}

fn part_one(grid: &Grid) {
    let ((x, y), amount) = find_largest_power_block(&grid, 3, 3);
    println!("Part 1: {},{},{}", x, y, amount);
}

fn part_two(grid: &Grid) {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    let result = (1..301)
        .into_par_iter()
        .map(|side_len| {
            let ((x, y), amount) = find_largest_power_block(&grid, side_len, side_len);
            println!("Side len of {} max: {},{},{}", side_len, x, y, amount);
            ((x, y), amount)
        }).max_by_key(|(_, amt)| *amt);

    if let Some(((x, y), max)) = result {
        println!("Part 2: {},{},{}", x, y, max);
    } else {
        println!("Part 2: no max value found");
    }
}

fn get_power_level(serial_number: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial_number;
    power *= rack_id;

    if power < 100 {
        power = 0;
    } else {
        power = power % 1000; // 12345 -> 345
        power -= power % 100; // 345 -> 300
        power /= 100; // 300 -> 3
    }

    power -= 5;

    power
}

fn build_grid(serial_number: i32, width: i32, height: i32) -> Grid {
    let mut data = HashMap::new();

    for x in 1..=width {
        for y in 1..=height {
            let power = get_power_level(serial_number, x, y);
            data.insert((x, y), power);
        }
    }

    Grid {
        width,
        height,
        data,
    }
}

#[cfg(allow_dead_code)]
fn show_grid(grid: Grid) -> String {
    let mut result = String::new();

    for y in 1..=grid.height {
        for x in 1..=grid.width {
            result += &format!("{}\t", grid.data[&(x, y)]);
        }
        result += "\n";
    }

    result
}

fn find_largest_power_block(grid: &Grid, width: i32, height: i32) -> (Point, i32) {
    let mut max = 0;
    let mut pos = (0, 0);
    for x in 1..=grid.width - width {
        for y in 1..=grid.height - height {
            let s = sum_block(grid, x, y, width, height);
            if s > max {
                max = s;
                pos = (x, y);
            }
        }
    }
    return (pos, max);

    fn sum_block(grid: &Grid, x: i32, y: i32, width: i32, height: i32) -> i32 {
        let mut sum = 0;

        for ox in 0..width {
            for oy in 0..height {
                if let Some(v) = grid.data.get(&(x + ox, y + oy)) {
                    sum += v;
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_power_level() {
        assert_eq!(get_power_level(8, 3, 5), 4);
        assert_eq!(get_power_level(57, 122, 79), -5);
        assert_eq!(get_power_level(39, 217, 196), 0);
        assert_eq!(get_power_level(71, 101, 153), 4);
    }

    #[test]
    fn test_find_largest_power_block() {
        assert_eq!(
            find_largest_power_block(
                &build_grid(18, 300, 300),
                3, 3,
            ), ((33, 45), 29)
        );

        assert_eq!(
            find_largest_power_block(
                &build_grid(42, 300, 300),
                3, 3,
            ), ((21, 61), 30)
        );
    }
}