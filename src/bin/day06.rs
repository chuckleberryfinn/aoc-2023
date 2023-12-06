use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

fn get_input() -> Vec<Vec<&'static str>> {
    include_str!("../../input/day06.txt")
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.split_whitespace().collect())
        .collect()
}

fn calculate_race(races: &[(i64, i64)]) -> u32 {
    races
        .iter()
        .map(|(time, distance)| {
            (1..(*distance))
                .fold_while(0, |acc, hold| {
                    if ((time - hold) * hold) > *distance {
                        Continue(acc + 1)
                    } else if acc == 0 {
                        Continue(acc)
                    } else {
                        Done(acc)
                    }
                })
                .into_inner()
        })
        .product()
}

fn part1() -> u32 {
    let inputs = get_input();
    let races = (0..inputs[0].len())
        .map(|i| (inputs[0][i].parse().unwrap(), inputs[1][i].parse().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    calculate_race(&races)
}

fn part2() -> u32 {
    let races = get_input()
        .iter()
        .map(|num| num.iter().map(|n| n.to_string()).collect::<String>())
        .collect::<Vec<String>>();

    calculate_race(&[(races[0].parse().unwrap(), races[1].parse().unwrap())])
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
        assert!(part1() == 2_065_338);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 34_934_171);
    }
}
