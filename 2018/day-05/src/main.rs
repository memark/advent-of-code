#![allow(unused_imports, dead_code)]

use itertools::Itertools;
use std::fs;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let file = file.trim();

    let part_1a = react(file).len();
    println!("{}: {}", stringify!(part_1a), part_1a);

    let part_2a = improve_polymer_and_react(file);
    println!("{}: {}", stringify!(part_2a), part_2a);
}

fn improve_polymer_and_react(file: &str) -> usize {
    ('a'..='z')
        .map(|c| react(&remove_units(file, c)).len())
        .min()
        .unwrap()
}

fn react(polymer: &str) -> String {
    let mut result = polymer.to_owned();
    let mut changed = true;
    while changed {
        changed = false;
        for c in ('a'..='z').chain('A'..='Z') {
            let from = [c, switch_case(c)].iter().collect::<String>();
            if result.contains(&from) {
                result = result.replace(&from, "");
                changed = true;
            }
        }
    }
    result
}

fn remove_units(polymer: &str, c: char) -> String {
    polymer.replace([c, switch_case(c)], "")
}

fn switch_case(c: char) -> char {
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn react_examples() {
        assert_eq!(react("aA"), "");
        assert_eq!(react("abBA"), "");
        assert_eq!(react("abAB"), "abAB");
        assert_eq!(react("aabAAB"), "aabAAB");
    }

    #[test]
    fn react_larger_example() {
        assert_eq!(react("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(remove_units("dabAcCaCBAcCcaDA", 'a'), "dbcCCBcCcD");
        assert_eq!(react(&remove_units("dabAcCaCBAcCcaDA", 'a')), "dbCBcD");
    }
}
