#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use regex::Regex;
use std::{fs, num::ParseIntError, str::FromStr};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> String {
    solve_with_strategy(input, CrateMover9000)
}

fn solve_part_2(input: &str) -> String {
    solve_with_strategy(input, CrateMover9001)
}

fn solve_with_strategy(input: &str, crate_mover: impl CrateMover) -> String {
    let (a, b) = input.split_once("\n\n").unwrap();

    let stack_rows = a.lines().dropping_back(1).collect_vec();
    let stack_header = a.lines().last().unwrap();

    let step_rows = b.lines().collect_vec();

    let num_stacks = get_num_stacks(stack_header);

    let stacks = parse_stacks(num_stacks, &stack_rows);
    let steps = parse_steps(&step_rows);

    let stacks = crate_mover.operate(&steps, &stacks);

    let top = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    top
}

fn get_num_stacks(stack_header: &str) -> usize {
    stack_header
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_stacks(num_stacks: usize, stack_rows: &[&str]) -> Vec<Vec<char>> {
    (0..num_stacks)
        .map(|stack_index| {
            stack_rows
                .iter()
                .rev()
                .filter_map(|stack_row| {
                    let pos = 1 + stack_index * 4;
                    let c = stack_row.chars().nth(pos).unwrap();
                    if c != ' ' {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

fn parse_steps(b: &[&str]) -> Vec<Step> {
    b.iter().map(|l| l.parse().unwrap()).collect_vec()
}

struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Step {
            count: caps[1].parse().unwrap(),
            from: caps[2].parse().unwrap(),
            to: caps[3].parse().unwrap(),
        })
    }
}

type Stack = Vec<char>;

trait CrateMover {
    fn operate(&self, steps: &[Step], stacks: &[Stack]) -> Vec<Stack>;
}

struct CrateMover9000;
impl CrateMover for CrateMover9000 {
    fn operate(&self, steps: &[Step], stacks: &[Stack]) -> Vec<Stack> {
        let mut stacks = stacks.to_vec();
        for Step { count, from, to } in steps {
            for _ in 0..*count {
                let t = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(t);
            }
        }
        stacks.to_vec()
    }
}

struct CrateMover9001;
impl CrateMover for CrateMover9001 {
    fn operate(&self, steps: &[Step], stacks: &[Stack]) -> Vec<Stack> {
        let mut stacks = stacks.to_vec();
        for Step { count, from, to } in steps {
            let mut temp = Vec::new();
            for _ in 0..*count {
                let t = stacks[from - 1].pop().unwrap();
                temp.push(t);
            }
            for _ in 0..*count {
                let t = temp.pop().unwrap();
                stacks[to - 1].push(t);
            }
        }
        stacks.to_vec()
    }
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), "CMZ".to_owned());
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), "LJSVLTWQM".to_owned());
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), "MCD".to_owned());
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), "BRQWDBBJM".to_owned());
    }
}
