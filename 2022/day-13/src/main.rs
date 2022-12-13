#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::Ordering::{self, *},
    fs,
    str::FromStr,
    string::ParseError,
};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> usize {
    let packets = input
        .split("\n\n")
        .map(|pair| pair.split_once("\n").unwrap())
        .map(|(a, b)| (a.parse::<Value>().unwrap(), b.parse::<Value>().unwrap()))
        .collect_vec();

    let left_smaller_indices = packets
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .collect_vec();

    left_smaller_indices.iter().sum()
}

fn solve_part_2(input: &str) -> usize {
    let divider_packet_1 = "[[2]]".parse::<Value>().unwrap();
    let divider_packet_2 = "[[6]]".parse::<Value>().unwrap();

    let sorted_packets = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Value>().unwrap())
        .chain([divider_packet_1.clone(), divider_packet_2.clone()])
        .sorted()
        .collect_vec();

    let divider_packet_indices = sorted_packets
        .iter()
        .positions(|p| *p == divider_packet_1 || *p == divider_packet_2)
        .map(|i| i + 1)
        .collect_vec();

    divider_packet_indices.iter().product()
}

use Value::*;
#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>() {
            Ok(i) => Ok(Integer(i)),
            Err(_) => {
                lazy_static! {
                    // static ref RE: Regex = Regex::new(r"\[((?:\d+,?)*)\]").unwrap();
                    static ref RE: Regex = Regex::new(r"^\[(.*)\]$").unwrap();
                }

                if s == "" {
                    return Ok(List(vec![]));
                }

                dbg!(&s);
                let inner = &RE.captures(s).unwrap()[1];

                match inner {
                    "" => Ok(List(vec![])),
                    s => Ok(List(
                        split_at_this_level(s)
                            .iter()
                            .map(|ss| ss.parse().unwrap())
                            .collect_vec(),
                    )),
                }
            }
        }
    }
}

fn split_at_this_level(s: &str) -> Vec<String> {
    let mut indent = 0;
    let mut acc = String::new();
    let mut result = vec![];

    for c in s.chars() {
        if c == '[' {
            indent += 1;
            acc.push(c);
        } else if c == ']' {
            indent -= 1;
            acc.push(c);
        } else if c == ',' && indent == 0 {
            dbg!(&acc);
            result.push(acc.clone());
            dbg!(&result);
            acc.clear();
        } else {
            acc.push(c);
        }
    }

    if acc.len() > 0 {
        result.push(acc.clone());
    }

    result
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Value::cmp(self, other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let v1 = self;
        let v2 = other;

        match (v1, v2) {
            // "If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input."
            (Integer(i1), Integer(i2)) => u8::cmp(i1, i2),

            // "If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input."
            (List(l1), List(l2)) => {
                use itertools::EitherOrBoth::*;

                l1.iter()
                    .zip_longest(l2)
                    .map(|z| match z {
                        Both(b1, b2) => Value::cmp(b1, b2),
                        Left(_) => Greater,
                        Right(_) => Less,
                    })
                    .find(|&ord| ord != Equal)
                    .unwrap_or(Equal)
            }

            // "If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2]."
            (Integer(_), List(_)) => Value::cmp(&List(vec![v1.clone()]), v2),
            (List(_), Integer(_)) => Value::cmp(v1, &List(vec![v2.clone()])),
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
    fn value_cmp_tests() {
        assert_eq!(Value::cmp(&Integer(1), &Integer(2)), Less);
        assert_eq!(Value::cmp(&Integer(2), &Integer(1)), Greater);
        assert_eq!(Value::cmp(&Integer(3), &Integer(3)), Equal);

        assert_eq!(Value::cmp(&List(vec![]), &List(vec![])), Equal);
        assert_eq!(
            Value::cmp(&List(vec![Integer(0)]), &List(vec![Integer(0)])),
            Equal
        );
        assert_eq!(
            Value::cmp(&List(vec![Integer(0)]), &List(vec![Integer(1)])),
            Less
        );
        assert_eq!(
            Value::cmp(&List(vec![Integer(1)]), &List(vec![Integer(0)])),
            Greater
        );

        assert_eq!(Value::cmp(&Integer(0), &List(vec![Integer(0)])), Equal);
        assert_eq!(Value::cmp(&List(vec![Integer(0)]), &Integer(0)), Equal);

        assert_eq!(Value::cmp(&Integer(0), &List(vec![Integer(1)])), Less);
        assert_eq!(Value::cmp(&List(vec![Integer(0)]), &Integer(1)), Less);
        assert_eq!(Value::cmp(&Integer(1), &List(vec![Integer(0)])), Greater);
        assert_eq!(Value::cmp(&List(vec![Integer(1)]), &Integer(0)), Greater);

        assert_eq!(
            Value::cmp(&List(vec![Integer(0), Integer(0), Integer(0)]), &Integer(2)),
            Less
        );
        assert_eq!(
            Value::cmp(&Integer(2), &List(vec![Integer(0), Integer(0), Integer(0)])),
            Greater
        );

        assert_eq!(Value::cmp(&List(vec![]), &List(vec![])), Equal);
    }

    #[test]
    fn parse_value_tests() {
        assert_eq!("[]".parse::<Value>(), Ok(List(vec![])));
        assert_eq!("[1]".parse::<Value>(), Ok(List(vec![Integer(1)])));
        assert_eq!(
            "[1,2]".parse::<Value>(),
            Ok(List(vec![Integer(1), Integer(2)]))
        );
        assert_eq!("[[]]".parse::<Value>(), Ok(List(vec![List(vec![])])));
        assert_eq!(
            "[[[]]]".parse::<Value>(),
            Ok(List(vec![List(vec![List(vec![])])]))
        );
        assert_eq!(
            "[[],2]".parse::<Value>(),
            Ok(List(vec![List(vec![]), Integer(2)]))
        );
        assert_eq!(
            "[[1],2]".parse::<Value>(),
            Ok(List(vec![List(vec![Integer(1)]), Integer(2)]))
        );
        assert_eq!(
            "[[1],[2]]".parse::<Value>(),
            Ok(List(vec![List(vec![Integer(1)]), List(vec![Integer(2)])]))
        );
        // assert_eq!(
        //     "[[1],[2,3]]".parse::<Value>(),
        //     Ok(List(vec![
        //         List(vec![Integer(1)]),
        //         List(vec![Integer(2), Integer(3)])
        //     ]))
        // );

        // [[1],[2,3,4]]
    }

    #[test]
    fn split_at_this_level_tests() {
        assert_eq!(split_at_this_level("1"), vec!["1"]);
        assert_eq!(split_at_this_level("1,2"), vec!["1", "2"]);
        assert_eq!(split_at_this_level("1,[],2"), vec!["1", "[]", "2"]);
        assert_eq!(split_at_this_level("1,[3],2"), vec!["1", "[3]", "2"]);
        assert_eq!(split_at_this_level("[[1],[2,3,4]]"), vec!["[[1],[2,3,4]]"]);
        assert_eq!(split_at_this_level("[1],[2,3,4]"), vec!["[1]", "[2,3,4]"]);
    }

    #[test]
    fn part_1_examples() {
        assert!("[1]".parse::<Value>().unwrap() == "[1]".parse().unwrap());
        assert!("[1]".parse::<Value>().unwrap() < "[2]".parse().unwrap());
        assert_eq!(
            Value::cmp(&"[1,1]".parse().unwrap(), &"[1,2]".parse().unwrap()),
            Less
        );
        assert_eq!(
            Value::cmp(&"[2,1]".parse().unwrap(), &"[1,1]".parse().unwrap()),
            Greater
        );
        assert_eq!(
            Value::cmp(&"[1,1]".parse().unwrap(), &"[1,1]".parse().unwrap()),
            Equal
        );
        assert_eq!(
            Value::cmp(&"[1]".parse().unwrap(), &"[1,1]".parse().unwrap()),
            Less
        );
        assert_eq!(
            Value::cmp(&"[1,1]".parse().unwrap(), &"[1]".parse().unwrap()),
            Greater
        );

        // Actual examples
        assert_eq!(
            Value::cmp(
                &"[1,1,3,1,1]".parse().unwrap(),
                &"[1,1,5,1,1]".parse().unwrap()
            ),
            Less
        );
        assert_eq!(
            Value::cmp(
                &"[[1],[2,3,4]]".parse().unwrap(),
                &"[[1],4]".parse().unwrap()
            ),
            Less
        );

        assert_eq!(solve_part_1(&file("example_1")), 13);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 5938);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 140);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 29025);
    }
}
