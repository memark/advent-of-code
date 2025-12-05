#![allow(unused)]

use itertools::Itertools;
use ranges::{GenericRange, Ranges};
use rayon::prelude::*;
use std::ops::{Bound::*, RangeBounds};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {:?}", solve(Part::Part1, INPUT));
    println!("Part 2: {:?}", solve(Part::Part2, INPUT));
}

enum Part {
    Part1,
    Part2,
}

fn solve(part: Part, str: &str) -> u64 {
    let (s1, s2) = str.split_once("\n\n").unwrap();

    let ranges = s1
        .lines()
        .map(|l| {
            l.split("-")
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .map(|(a, b)| GenericRange::new_closed(a, b))
                .unwrap()
        })
        .collect::<Vec<_>>();

    match part {
        Part::Part1 => s2
            .lines()
            .map(|l| l.parse::<u64>().unwrap())
            .filter(|id| ranges.iter().any(|r| r.contains(id)))
            .count() as u64,
        Part::Part2 => ranges
            .into_iter()
            .fold(Ranges::new(), |acc, r| acc.union(r))
            .as_slice()
            .iter()
            .map(|r| match (r.start_bound(), r.end_bound()) {
                (Included(rs), Included(re)) => re - rs + 1,
                _ => panic!(),
            })
            .sum::<u64>(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[rstest]
    #[case::part1_example(Part::Part1, EXAMPLE, 3)]
    #[case::part1_input(Part::Part1, INPUT, 739)]
    #[case::part2_example(Part::Part2, EXAMPLE, 14)]
    #[case::part2_input(Part::Part2, INPUT, 344486348901788)]
    fn solves_correctly(#[case] part: Part, #[case] data: &str, #[case] expected: u64) {
        assert_eq!(solve(part, data), expected);
    }
}
