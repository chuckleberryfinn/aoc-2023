use std::collections::HashMap;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let (instructions, maps) = get_input();
    count("AAA", instructions, &maps, "ZZZ")
}

fn part2() -> usize {
    let (instructions, maps) = get_input();
    maps.keys()
        .filter(|k| k.ends_with('A'))
        .map(|c| count(c, instructions, &maps, "Z"))
        .collect::<Vec<usize>>()
        .iter()
        .fold(1, |acc, x| lcm(acc, *x))
}

fn get_input() -> (
    &'static str,
    HashMap<&'static str, (&'static str, &'static str)>,
) {
    let (instructions, maps) = include_str!("../../input/day08.txt")
        .split_once("\n\n")
        .unwrap();

    (
        instructions,
        maps.lines()
            .map(|m| {
                let (key, value) = m.split_once(" = (").unwrap();
                let (v1, v2) = value.split_once(", ").unwrap();
                (key, (v1, &v2[0..3]))
            })
            .collect(),
    )
}

fn count(
    start: &'static str,
    instructions: &'static str,
    maps: &HashMap<&'static str, (&'static str, &'static str)>,
    end: &'static str,
) -> usize {
    let mut current = start;
    let mut total = 0;
    instructions.chars().cycle().find(|i| {
        let m = maps.get(&current).unwrap();
        current = match i {
            'R' => m.1,
            _ => m.0,
        };
        total += 1;
        current.ends_with(end)
    });
    total
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1() == 17_141);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 10_818_234_074_807);
    }
}
