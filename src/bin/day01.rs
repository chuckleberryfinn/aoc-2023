use std::collections::HashMap;

use regex::Regex;

fn get_inputs(regex: &str) -> Vec<Vec<u32>> {
    let re = Regex::new(regex).unwrap();
    let numbers: HashMap<_, _> = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    include_str!("../../input/day01.txt")
        .lines()
        .map(|s| {
            s.char_indices()
                .filter_map(|(i, _)| re.find(&s[i..]))
                .map(|m| *numbers.get(m.as_str()).unwrap())
                .collect()
        })
        .collect()
}

fn calibration(regex: &str) -> u32 {
    get_inputs(regex)
        .iter()
        .map(|v| (v.first().unwrap() * 10) + v.last().unwrap())
        .sum()
}

fn part1() -> u32 {
    calibration(r"\d")
}

fn part2() -> u32 {
    calibration(r"one|two|three|four|five|six|seven|eight|nine|ten|\d")
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1() == 54_940);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 54_208);
    }
}
