use std::collections::HashMap;

use regex::Regex;

fn get_inputs(regex: &str) -> Vec<Vec<u32>> {
    let re = Regex::new(regex).unwrap();
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let numbers: HashMap<&str, u32> = digits
        .iter()
        .zip(1..=9)
        .map(|(&word, number)| (word, number))
        .chain(
            words
                .iter()
                .zip(1..=9)
                .map(|(&word, number)| (word, number)),
        )
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
