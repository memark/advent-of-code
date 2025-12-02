#![allow(unused)]

use itertools::Itertools;

fn main() {
    let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    let input = include_str!("input.txt");

    println!("Part 1: {:?}", solve(input, Part::Part1)); // 23701357374
    println!("Part 2: {:?}", solve(input, Part::Part2)); // 34284458938
}

fn solve(data: &str, part: Part) -> i64 {
    data.replace("\n", "")
        .split_terminator(",")
        .map(|x| {
            x.split("-")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .flat_map(|(a, b)| (a..=b))
        .filter(|x| {
            let chars = x.to_string().chars().collect_vec();
            let max = match part {
                Part::Part1 => 2,
                Part::Part2 => chars.len() / 2 + 1, // why +1 ?
            };
            (2..=max).any(|x| {
                chars.len() / x > 0
                    && (matches!(part, Part::Part2) || chars.len() % x == 0)
                    && chars
                        .chunks(chars.len() / x)
                        .collect_vec()
                        .windows(2)
                        .all(|w| w[0] == w[1])
            })
        })
        .sum()
}

enum Part {
    Part1,
    Part2,
}
