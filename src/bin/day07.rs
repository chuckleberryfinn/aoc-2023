use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum CamelHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_ordering(card: char, jokers_wild: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => match jokers_wild {
            true => 1,
            false => 11,
        },
        'T' => 10,
        '2'..='9' => card.to_digit(10).unwrap() as usize,
        _ => panic!("Unexpected card"),
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> u32 {
    get_winnings(false)
}

fn part2() -> u32 {
    get_winnings(true)
}

fn get_winnings(jokers_wild: bool) -> u32 {
    get_input()
        .iter()
        .sorted_by(|a, b| compare_hand(a.0, b.0, jokers_wild))
        .zip(1..)
        .fold(0, |acc, (v, i)| acc + (v.1 * i))
}

fn get_input() -> Vec<(&'static str, u32)> {
    include_str!("../../input/day07.txt")
        .lines()
        .map(|l| {
            let splits = l.split_once(' ').unwrap();
            (splits.0, splits.1.parse().unwrap())
        })
        .collect::<Vec<(&'static str, u32)>>()
}

fn compare_hand(hand1: &'static str, hand2: &'static str, jokers_wild: bool) -> Ordering {
    let rank1 = evaluate_hand(hand1, jokers_wild);
    let rank2 = evaluate_hand(hand2, jokers_wild);

    match rank1.cmp(&rank2) {
        Ordering::Equal => compare_cards(hand1, hand2, jokers_wild),
        order => order,
    }
}

fn evaluate_hand(hand: &str, jokers_wild: bool) -> CamelHand {
    let mut cards = hand
        .chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, u8>, l| {
            *acc.entry(l).or_default() += 1;
            acc
        });

    if jokers_wild {
        let sorted = cards
            .iter()
            .sorted_by(|a, b| b.1.cmp(a.1))
            .map(|(k, _)| *k)
            .collect::<Vec<char>>();
        if sorted[0] != 'J' {
            *cards.get_mut(&sorted[0]).unwrap() += cards.remove(&'J').unwrap_or(0);
        } else if sorted.len() > 1 {
            *cards.get_mut(&'J').unwrap() += cards.remove(&sorted[1]).unwrap();
        }
    }

    let items: Vec<(&char, &u8)> = cards.iter().collect();

    match items.len() {
        1 => CamelHand::FiveOfAKind,
        2 => match items[0].1 {
            1 | 4 => CamelHand::FourOfAKind,
            _ => CamelHand::FullHouse,
        },
        3 => match items[0].1 {
            1 => match items[1].1 {
                2 => CamelHand::TwoPair,
                _ => CamelHand::ThreeOfAKind,
            },
            2 => CamelHand::TwoPair,
            _ => CamelHand::ThreeOfAKind,
        },
        4 => CamelHand::OnePair,
        _ => CamelHand::HighCard,
    }
}

fn compare_cards(hand1: &'static str, hand2: &'static str, jokers_wild: bool) -> Ordering {
    hand1
        .chars()
        .zip(hand2.chars())
        .find_map(|(h1, h2)| {
            let order_h1 = card_ordering(h1, jokers_wild);
            let order_h2 = card_ordering(h2, jokers_wild);
            match order_h1.cmp(&order_h2) {
                Ordering::Equal => None,
                order => Some(order),
            }
        })
        .unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1() == 253_638_586);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 253_253_225);
    }
}
