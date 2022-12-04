#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(&['-', ','])
                .map(|x| x.parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .filter(|(a, b, c, d)| {
            let h1: HashSet<_> = HashSet::from_iter(*a..=*b);
            let h2: HashSet<_> = HashSet::from_iter(*c..=*d);
            h1.is_subset(&h2) || h2.is_subset(&h1)
        })
        .count()
}

fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(&['-', ','])
                .map(|x| x.parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .filter(|(a, b, c, d)| {
            let h1: HashSet<_> = HashSet::from_iter(*a..=*b);
            let h2: HashSet<_> = HashSet::from_iter(*c..=*d);
            !h1.is_disjoint(&h2)
        })
        .count()
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
        assert_eq!(solve_part_1(&file("example_1")), 2);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 483);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), 4);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 874);
    }
}
