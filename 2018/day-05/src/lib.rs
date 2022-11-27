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

pub fn improve_polymer_and_react(file: &str) -> usize {
    ('a'..='z')
        .map(|c| react(&remove_units(file, c)).len())
        .min()
        .unwrap()
}

pub fn react(polymer: &str) -> String {
    let mut result = polymer.to_owned();
    let mut changed = true;
    while changed {
        changed = false;
        for c in ('a'..='z').chain('A'..='Z') {
            let from = [c, switch_case(c)].iter().collect::<String>();
            let new_result = result.replace(&from, "");
            if new_result.len() != result.len() {
                result = new_result;
                changed = true;
            }
        }
    }
    result
}

pub fn react_2(polymer: &str) -> String {
    let mut result = polymer.to_owned();
    let mut changed = true;

    while changed {
        changed = false;

        let chars_count = result.chars().count();

        let mut new_result = "".to_owned();
        let mut i = 0;

        while i < chars_count {
            if i < chars_count - 1
                && should_react(
                    result[i..=i].chars().next().unwrap(),
                    result[i + 1..=i + 1].chars().next().unwrap(),
                )
            {
                changed = true;
                i += 2;
            } else {
                new_result.push_str(&result[i..=i]);
                i += 1;
            }
        }
        result = new_result;
    }
    result
}

// react_3: use vector of chars all the way?

fn remove_units(polymer: &str, c: char) -> String {
    polymer.replace([c, switch_case(c)], "")
}

fn should_react(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase()
        && ((a.is_ascii_lowercase() && b.is_ascii_uppercase())
            || (a.is_ascii_uppercase() && b.is_ascii_lowercase()))
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
    fn react_2_examples() {
        assert_eq!(react_2("aA"), "");
        assert_eq!(react_2("abBA"), "");
        assert_eq!(react_2("abAB"), "abAB");
        assert_eq!(react_2("aabAAB"), "aabAAB");
    }

    #[test]
    fn react_2_larger_example() {
        assert_eq!(react_2("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(remove_units("dabAcCaCBAcCcaDA", 'a'), "dbcCCBcCcD");
        assert_eq!(react(&remove_units("dabAcCaCBAcCcaDA", 'a')), "dbCBcD");
    }

    #[test]
    fn part_2_input() {
        let file = fs::read_to_string("input").unwrap();
        let file = file.trim();

        assert_eq!(improve_polymer_and_react(file), 6918);
    }
}
