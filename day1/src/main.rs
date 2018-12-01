fn main() {
    let input = read_input("input.txt");
    let result = input.map(part_one);
    match result {
        Err(e) => println!("{}", e),
        Ok(answer) => println!("Answer: {}", answer),
    }
}

fn part_one(input: Vec<i32>) -> i32 {
    input.iter().sum()
}

fn read_input(name: &str) -> Result<Vec<i32>, String> {
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;

    let mut file = File::open(name)
        .map_err(|e| format!("Failed to open input file {}: {}", name, e))?;
    let mut input = String::new();
    file.read_to_string(&mut input)
        .map_err(|e| format!("Failed to parse input: {}", e))?;

    input.lines()
        .map(|line| i32::from_str(line)
            .map_err(|e| format!("Failed to parse {} as i32: {}", line, e)))
        .collect()
}