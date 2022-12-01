#![allow(dead_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::{fmt::Display, fs};

fn main() {
    let input = read_and_trim_file("input");
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> String {
    input.to_owned()
}

fn solve_part_2(input: &str) -> String {
    input.to_owned()
}

fn read_and_trim_file(path: &str) -> String {
    let file = fs::read_to_string(path).unwrap();
    file.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1("?"), "?");
    }

    #[ignore]
    #[test]
    fn part_1_input() {
        let input = read_and_trim_file("input");

        assert_eq!(solve_part_1(&input), input);
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2("?"), "?");
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        let input = read_and_trim_file("input");

        assert_eq!(solve_part_2(&input), input);
    }
}
