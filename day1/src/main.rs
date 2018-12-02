use std::fs::File;
use std::io::Error;

fn advent<Input>(
    parse_input: &Fn(&str) -> Result<Input, String>,
    part_one: &Fn(&Input) -> Result<String, String>,
    part_two: &Fn(&Input) -> Result<String, String>,
) {
    use std::env::args;

    // first arg is executable name
    for filename in args().skip(1) {
        println!("Input: {}", filename);

        let input = File::open(filename)
            .map_err(|err| format!("failed to open file: {}", err))
            .and_then(|mut file| read_file_contents_as_string(&mut file)
                .map_err(|err| format!("failed to read contents of file: {}", err)))
            .and_then(|input| parse_input(&input));

        match input {
            Ok(i) => {
                run_part("part 1", part_one, &i);
                run_part("part 2", part_two, &i);
            }
            Err(e) => println!("  Could not load input because {}", e),
        }

        println!()
    }

    fn run_part<Input>(name: &str, part: &Fn(&Input) -> Result<String, String>, input: &Input) {
        match part(&input) {
            Ok(answer) => println!("  Answer for {}: {}", name, answer),
            Err(err) => println!("  Calculation failed for {}: {}", name, err),
        }
    }
}

fn read_file_contents_as_string(file: &mut File) -> Result<String, Error> {
    use std::io::Read;

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e)
    }
}

fn main() {
    advent(&parse_input, &part_one, &part_two);
}

fn parse_input(input: &str) -> Result<Vec<i32>, String> {
    use std::str::FromStr;

    input.lines()
        .map(|line| i32::from_str(line)
            .map_err(|e| format!("Failed to parse {} as i32: {}", line, e)))
        .collect()
}

fn part_one(input: &Vec<i32>) -> Result<String, String> {
    Ok(input.iter().sum::<i32>().to_string())
}

fn part_two(input: &Vec<i32>) -> Result<String, String> {
    use std::collections::HashSet;

    let mut frequencies = HashSet::new();
    let mut last = 0;

    let mut iter = input.iter();

    loop {
        if let Some(x) = iter.next() {
            last += x;

            let seen_before = !frequencies.insert(last);
            if seen_before {
                return Ok(last.to_string());
            }
        } else {
            iter = input.iter();
        }
    }
}
