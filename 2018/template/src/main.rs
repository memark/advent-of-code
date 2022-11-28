#![allow(dead_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::fs;

fn main() {
    let input = read_and_trim_input();

    let part_1a = "?";
    println!("{}: {}", stringify!(part_1a), part_1a);

    let part_2a = "?";
    println!("{}: {}", stringify!(part_2a), part_2a);
}

fn read_and_trim_input() -> String {
    let file = fs::read_to_string("input").unwrap();
    file.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(1, 1);
    }

    #[test]
    fn part_1_input() {
        let input = read_and_trim_input();

        assert_eq!(1, 1);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(1, 1);
    }

    #[test]
    fn part_2_input() {
        let input = read_and_trim_input();

        assert_eq!(1, 1);
    }
}
