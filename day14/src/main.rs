fn main() {
//    part_one(9);
//    part_one(5);
//    part_one(18);
//    part_one(2018);
//    part_one(330121);
    part_two(vec![5, 1, 5, 8, 9]);
    part_two(vec![0, 1, 2, 4, 5]);
    part_two(vec![9, 2, 5, 1, 0]);
    part_two(vec![5, 9, 4, 1, 4]);
    part_two(vec![3, 3, 0, 1, 2, 1]);

    fn part_one(after: usize) {
        let digits = calc_part_one(after);
        println!("{}: {:?}", after, show_digits(&digits));
    }

    fn part_two(target: Vec<usize>) {
        println!("{}: {}", show_digits(&target), calc_part_two(&target));
    }

    fn show_digits(digits: &Vec<usize>) -> String {
        let mut result = String::new();
        for digit in digits {
            result.push_str(&format!("{}", digit));
        }

        result
    }
}

fn get_digits(value: usize) -> Vec<usize> {
    if value == 0 {
        return vec![0];
    }

    let mut remaining = value;
    let mut digits = Vec::new();

    while remaining > 0 {
        let ones = remaining % 10;
        digits.push(ones);
        remaining -= ones;
        remaining /= 10;
    }

    digits.reverse();

    digits
}

fn add_scores(scores: &mut Vec<usize>, positions: &mut Vec<usize>) {
    let next_sum = scores.iter().enumerate()
        .filter(|(i, _)| positions.contains(i))
        .map(|(_, v)| v)
        .sum();
    scores.append(&mut get_digits(next_sum));

    for position in positions.iter_mut() {
        *position = (*position + 1 + scores[*position]) % scores.len();
    }
}

fn calc_part_one(after: usize) -> Vec<usize> {
    let mut scores: Vec<usize> = vec![3, 7];
    let mut positions = vec![0, 1];

    while scores.len() < after + 10 {
        add_scores(&mut scores, &mut positions);
    }

    scores.iter().skip(after).take(10)
        .map(|v| *v)
        .collect()
}

fn calc_part_two(target: &Vec<usize>) -> usize {
    let mut scores: Vec<usize> = vec![3, 7];
    let mut positions = vec![0, 1];

    loop {
        add_scores(&mut scores, &mut positions);
        if let Some(x) = index_vec(&scores, &target) {
            return x;
        }
    }
}

fn index_vec<T>(haystack: &Vec<T>, needle: &Vec<T>) -> Option<usize>
    where T: Eq {
    if needle.len() > haystack.len() {
        return None;
    }

    'haystack: for i in 0..=(haystack.len() - needle.len()) {
        for j in 0..needle.len() {
            if haystack[i + j] != needle[j] {
                continue 'haystack;
            }
        }

        return Some(i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_contains_vec() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(
            index_vec(&v, &vec![1, 2, 3]),
            Some(0)
        );
        assert_eq!(
            index_vec(&v, &vec![2, 3]),
            Some(1)
        );
        assert_eq!(
            index_vec(&v, &vec![3]),
            Some(2)
        );
        assert_eq!(
            index_vec(&v, &vec![4, 5]),
            Some(3)
        );
        assert_eq!(
            index_vec(&v, &vec![5]),
            Some(4)
        );
        assert_eq!(
            index_vec(&v, &vec![5, 4]),
            None
        );
        assert_eq!(
            index_vec(&v, &vec![6]),
            None
        );
        assert_eq!(
            index_vec(&v, &vec![]),
            Some(0)
        );
        assert_eq!(
            index_vec(&v, &vec![1, 2, 4]),
            None
        );
        assert_eq!(
            index_vec(&v, &vec![1, 2, 3, 4, 5]),
            Some(0)
        );
        assert_eq!(
            index_vec(&v, &vec![1, 2, 3, 4, 5, 6]),
            None
        );
    }
}