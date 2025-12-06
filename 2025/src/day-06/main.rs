#![allow(unused)]

use core::num;
use itertools::Itertools;
use ranges::{GenericRange, Ranges};
use rayon::prelude::*;
use std::{
    fmt::format,
    iter::once,
    ops::{Bound::*, RangeBounds},
};

const INPUT: &str = include_str!("input.txt");

const EXAMPLE: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

type Int = u64;

fn main() {
    println!("Part 1: {:?}", solve(Part::Part1, INPUT));
    println!("Part 2: {:?}", solve(Part::Part2, INPUT));
}

enum Part {
    Part1,
    Part2,
}

fn solve(part: Part, str: &str) -> Int {
    // Idéer:
    // - börja med transpose
    // - använd typ Polars

    match part {
        Part::Part1 => {
            let vv = str
                .trim()
                .lines()
                .map(|l| l.split_whitespace().collect_vec())
                .collect_vec();
            let problems = vv.len();
            let numbers = vv[0].len();

            let data = (0..numbers)
                .map(|col| (0..problems).map(|row| vv[row][col]).collect_vec())
                .collect_vec();

            let parsed = data
                .iter()
                .map(|p| {
                    p.split_last()
                        .map(|(&a, b)| {
                            (
                                a,
                                b.iter().map(|bb| bb.parse::<Int>().unwrap()).collect_vec(),
                            )
                        })
                        .unwrap()
                })
                .collect_vec();

            parsed
                .iter()
                .map(|(op, nums)| match *op {
                    "+" => nums.iter().sum(),
                    "*" => nums.iter().product(),
                    _ => panic!() as u64,
                })
                .sum()
        }
        Part::Part2 => {
            let lines = str.trim_matches('\n').lines().collect_vec();

            let seps = (0..lines[0].len())
                .filter(|&i| lines.iter().all(|l| l.chars().nth(i).unwrap() == ' '))
                .collect_vec();

            let splitted_groups = lines
                .iter()
                .map(|&l| {
                    once(&0_usize)
                        .chain(&seps)
                        .rev()
                        .scan(l, |state, i| {
                            let (rest, str) = state.split_at(*i);
                            *state = rest;
                            Some(str)
                        })
                        .collect_vec()
                })
                .collect_vec();

            let mut transposed = (0..splitted_groups[0].len())
                .map(|col| {
                    (0..splitted_groups.len())
                        .map(|row| splitted_groups[row][col].to_owned())
                        .collect_vec()
                })
                .collect_vec();

            let len = transposed.len();
            transposed[len - 1] = transposed[len - 1]
                .iter()
                .map(|s| format!(" {s}"))
                .collect_vec();

            // ta bort " " från alla gruppers rader
            let reduced = transposed
                .iter()
                .map(|x| x.iter().map(|b| &b[1..]).collect_vec())
                .collect_vec();

            // dela upp i tuple med op och str
            let tupled = reduced
                .iter()
                .map(|x| {
                    let (rest, last) = x.split_at(x.len() - 1);
                    (rest, last[0].trim().chars().next().unwrap())
                })
                .collect_vec();

            // gå från höger, parsa ut tal
            let numbers = tupled
                .iter()
                .map(|(nums, op)| {
                    let len = nums[0].len();
                    let ns = (0..len)
                        .map(|i| {
                            nums.iter()
                                .map(move |n| n.chars().nth(i).unwrap())
                                .collect::<String>()
                                .trim()
                                .parse::<Int>()
                                .unwrap()
                        })
                        .collect_vec();
                    (ns, op)
                })
                .collect_vec();

            numbers
                .iter()
                .map(|(nums, op)| match op {
                    '+' => nums.iter().sum(),
                    '*' => nums.iter().product(),
                    _ => panic!() as u64,
                })
                .sum()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case::part1_example(Part::Part1, EXAMPLE, 4277556)]
    #[case::part1_input(Part::Part1, INPUT, 6757749566978)]
    #[case::part2_example(Part::Part2, EXAMPLE, 3263827)]
    #[case::part2_input(Part::Part2, INPUT, 10603075273949)]
    fn solves_correctly(#[case] part: Part, #[case] data: &str, #[case] expected: Int) {
        assert_eq!(solve(part, data), expected);
    }
}
