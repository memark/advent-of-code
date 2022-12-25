#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::fs;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("example_1")));
    // println!("Part 1: {}", solve_part_1(&file("input")));

    // println!("Part 2: {}", solve_part_2(&file("example_1")));
    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> i32 {
    0
}

fn solve_part_2(input: &str) -> i32 {
    0
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), todo!());
    }

    #[ignore]
    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), todo!());
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), todo!());
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
