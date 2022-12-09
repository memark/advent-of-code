// https://adventofcode.com/2022/day/1

use itertools::Itertools;
use std::fs;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|sect| sect.lines().map(|cal| cal.parse::<u32>().unwrap()))
        .map(|elf| elf.sum::<u32>())
        .max()
        .unwrap()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|sect| sect.lines().map(|cal| cal.parse::<u32>().unwrap()))
        .map(|elf| elf.sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), 24_000);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 68_802);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), 45_000);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 205_370);
    }
}
