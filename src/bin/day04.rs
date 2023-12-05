use std::collections::{HashMap, HashSet};

fn get_input() -> Vec<u32> {
    include_str!("../../input/day04.txt")
        .lines()
        .map(|l| {
            l.split_once(": ")
                .unwrap()
                .1
                .split(" | ")
                .map(|s| {
                    s.split_whitespace().fold(HashSet::new(), |mut digits, d| {
                        digits.insert(d.trim().parse::<u8>().unwrap());
                        digits
                    })
                })
                .collect()
        })
        .map(|numbers: Vec<HashSet<u8>>| numbers[0].intersection(&numbers[1]).count() as u32)
        .collect()
}

fn part1() -> u32 {
    get_input()
        .iter()
        .filter(|c| **c > 0)
        .map(|c| c.saturating_sub(1))
        .fold(0, |acc, c| acc + 2_u32.pow(c))
}

fn part2() -> usize {
    (1..)
        .zip(get_input())
        .fold(HashMap::<usize, usize>::new(), |acc, (start, winners)| {
            let previous = *acc.get(&start).unwrap_or(&1);
            let limit = start + (winners as usize);

            (start..=limit).enumerate().fold(acc, |mut acc, (i, card)| {
                let current = previous
                    + match i {
                        0 => 0,
                        _ => *acc.get(&card).unwrap_or(&1),
                    };
                acc.insert(card, current);
                acc
            })
        })
        .values()
        .sum()
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
        assert!(part1() == 20_407);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 23_806_951);
    }
}
