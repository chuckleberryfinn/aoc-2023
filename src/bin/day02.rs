use std::collections::HashMap;

fn get_input() -> Vec<Vec<Vec<(&'static str, i32)>>> {
    include_str!("../../input/day02.txt")
        .lines()
        .map(|s| {
            s.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .map(|game| {
                    game.split(", ")
                        .map(|draw| draw.split_once(' ').unwrap())
                        .map(|(count, colour)| (colour, count.parse().unwrap()))
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn part1() -> u32 {
    (1..)
        .zip(get_input())
        .filter(|(_, game)| {
            game.iter().all(|draw| {
                draw.iter()
                    .fold(
                        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]),
                        |mut colours, &(colour, count)| {
                            let current: i32 = colours.get(colour).unwrap() - count;
                            colours.insert(colour, current);
                            colours
                        },
                    )
                    .values()
                    .all(|v| *v >= 0)
            })
        })
        .fold(0, |acc, (i, _)| acc + i)
}

fn part2() -> i32 {
    get_input()
        .iter()
        .map(|game| {
            game.iter().fold(
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]),
                |mut colours, draw| {
                    draw.iter().for_each(|(colour, count)| {
                        colours
                            .insert(colour, std::cmp::max(*colours.get(colour).unwrap(), *count));
                    });
                    colours
                },
            )
        })
        .fold(0, |acc, colours| acc + colours.values().product::<i32>())
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
        assert!(part1() == 3_059);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 65_371);
    }
}
