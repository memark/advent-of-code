#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::fs;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(s1, s2)| s1.chars().find(|c1| s2.chars().contains(c1)).unwrap())
        .map(|c| {
            if c.is_ascii_lowercase() {
                (c as i32) - ('a' as i32) + 1
            } else {
                (c as i32) - ('A' as i32) + 27
            }
        })
        .sum::<i32>()
}

fn solve_part_2(input: &str) -> i32 {
    input
        .lines()
        .tuples::<(&str, &str, &str)>()
        .map(|(s1, s2, s3)| {
            s1.chars()
                .find(|c1| s2.chars().contains(c1) && s3.chars().contains(c1))
                .unwrap()
        })
        .map(|c| {
            if c.is_ascii_lowercase() {
                (c as i32) - ('a' as i32) + 1
            } else {
                (c as i32) - ('A' as i32) + 27
            }
        })
        .sum::<i32>()
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
        assert_eq!(solve_part_1(&file("example_1")), 157);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 7917);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), 70);
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
