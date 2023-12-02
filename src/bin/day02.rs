use std::collections::HashMap;

fn get_input() -> Vec<Vec<Vec<(i32, &'static str)>>> {
    include_str!("../../input/day02.txt")
        .lines()
        .map(|s| {
            s.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .map(|game| {
                    game.split(", ")
                        .map(|balls| balls.split_once(' ').unwrap())
                        .map(|ball| (ball.0.parse().unwrap(), ball.1))
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
                        |mut colours, &(count, colour)| {
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
    get_input().iter().fold(0, |acc, game| {
        acc + game
            .iter()
            .fold(
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]),
                |mut colours, draw| {
                    draw.iter().all(|(count, colour)| {
                        colours
                            .insert(colour, std::cmp::max(*colours.get(colour).unwrap(), *count));
                        true
                    });
                    colours
                },
            )
            .values()
            .copied()
            .product::<i32>()
    })
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
