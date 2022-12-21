use core::panic;
use itertools::Itertools;
use std::{collections::HashMap, fs, str::FromStr, string::ParseError};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

type Number = f64;

fn solve_part_1(input: &str) -> Number {
    let monkeys = input
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once(": ").unwrap();
            (name.to_owned(), rest.parse::<Job>().unwrap())
        })
        .collect::<HashMap<_, _>>();
    dbg!(&monkeys);

    let result = recurse("root", &monkeys);
    dbg!(&result);

    result
}

fn solve_part_2(input: &str) -> Number {
    let monkeys = input
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once(": ").unwrap();
            (name.to_owned(), rest.parse::<Job>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let initial: Number = 3360559990000_i64 as Number * 2 as Number;

    let mut b = -initial;
    let mut e = initial;

    loop {
        let m = (b + e) / 2 as Number;
        print!("{b} -> {m} -> {e}");

        let (b_res_0, _) = get_values_for_root(&monkeys, b);
        let (m_res_0, m_res_1) = get_values_for_root(&monkeys, m);
        let (e_res_0, _) = get_values_for_root(&monkeys, e);

        let b_res = b_res_0;
        let m_res = m_res_0;
        let e_res = e_res_0;
        println!(" => {b_res} - {m_res} - {e_res}");

        let root_1_value = m_res_1;

        if m_res_0 == m_res_1 {
            println!("{m} => ({m_res_0}, {m_res_1})");
            return m;
        }

        assert!(
            (b_res < root_1_value && e_res > root_1_value)
                || (b_res > root_1_value && e_res < root_1_value),
            "Out of range."
        );

        if b_res < root_1_value && m_res < root_1_value {
            b = m;
        } else if e_res < root_1_value && m_res < root_1_value {
            e = m;
        } else if b_res > root_1_value && m_res > root_1_value {
            b = m;
        } else if e_res > root_1_value && m_res > root_1_value {
            e = m;
        } else {
            panic!()
        }
    }
}

fn get_values_for_root(monkeys: &HashMap<String, Job>, n: Number) -> (Number, Number) {
    let (root_1_key, root_2_key) = if let Calculation(a, _, c) = &monkeys["root"] {
        (a, c)
    } else {
        panic!()
    };

    let mut monkeys = monkeys.clone();
    if let Some(humn) = monkeys.get_mut("humn") {
        *humn = Yelling(n);
    }

    (recurse(root_1_key, &monkeys), recurse(root_2_key, &monkeys))
}

fn recurse(key: &str, monkeys: &HashMap<String, Job>) -> Number {
    match &monkeys[key] {
        Yelling(n) => *n,
        Calculation(a, b, c) => {
            let a = recurse(a, monkeys);
            let c = recurse(c, monkeys);
            match b {
                '+' => a + c,
                '-' => a - c,
                '*' => a * c,
                '/' => a / c,
                _ => panic!(),
            }
        }
    }
}

use Job::*;
#[derive(Debug, Clone)]
enum Job {
    Yelling(Number),
    Calculation(String, char, String),
}

impl FromStr for Job {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<Number>() {
            Ok(Yelling(n))
        } else {
            let (a, b, c) = s.split(' ').collect_tuple().unwrap();
            Ok(Calculation(
                a.to_owned(),
                b.chars().exactly_one().unwrap(),
                c.to_owned(),
            ))
        }
    }
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
        assert_eq!(solve_part_1(&file("example_1")), 152 as Number);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 152479825094094_i64 as Number);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 301 as Number);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 3360561285172_i64 as Number);
    }
}
