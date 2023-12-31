use std::collections::HashMap;

use regex::Regex;

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calibration(regex: &str) -> u32 {
    let re = Regex::new(regex).unwrap();
    let numbers: HashMap<&str, u32> = DIGITS
        .iter()
        .zip(1..=9)
        .map(|(&word, number)| (word, number))
        .collect();

    include_str!("../../input/day01.txt")
        .lines()
        .map(|s| {
            (0..s.len())
                .filter_map(|i| re.find(&s[i..]))
                .map(|m| match numbers.get(m.as_str()) {
                    Some(x) => *x,
                    None => m.as_str().parse().unwrap(),
                })
                .collect::<Vec<u32>>()
        })
        .fold(0, |acc, v| acc + v[0] * 10 + v[v.len() - 1])
}

fn part1() -> u32 {
    calibration(r"\d")
}

fn part2() -> u32 {
    calibration(&(DIGITS.join("|") + r"|\d"))
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
