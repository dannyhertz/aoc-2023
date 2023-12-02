use lazy_static::lazy_static;

lazy_static! {
    static ref DIGITS: Vec<(&'static str, i32)> = {
        vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]
    };
}

pub fn solve(input: &str) -> i32 {
    return input
        .lines()
        .filter_map(|line| {
            let first_number = first_number(line);
            let last_number = last_number(line);

            match (first_number, last_number) {
            (Some(first), Some(last)) => return Some(first * 10 + last),
            _ => return None
            }
        })
        .sum();
}

fn first_number(arr: &str) -> Option<i32> {
    return DIGITS
        .iter()
        .filter_map(|digit| {
            return arr.find(digit.0).map(|idx| { (idx, digit) })
        })
        .min_by(|a, b| {
            a.0.cmp(&b.0)
        })
        .map(|tuple| {
            return tuple.1.1
        });
}

fn last_number(arr: &str) -> Option<i32> {
    return DIGITS
        .iter()
        .filter_map(|digit| {
            return arr.rfind(digit.0).map(|idx| { (idx, digit) })
        })
        .max_by(|a, b| {
            a.0.cmp(&b.0)
        })
        .map(|tuple| {
            return tuple.1.1
        });
}