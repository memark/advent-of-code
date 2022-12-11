use colored::Colorize;
use itertools::Itertools;
use regex::Regex;
use std::{fs, str::FromStr, string::ParseError};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> u64 {
    solve(input, 20, DivBy3, false)
}

fn solve_part_2(input: &str) -> u64 {
    solve(input, 10_000, ModuloBySumOfAllTestDivisors, false)
}

fn solve(
    input: &str,
    num_rounds: u64,
    worry_reduction_method: WorryReductionMethod,
    should_log: bool,
) -> u64 {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect_vec();

    for round in 1..=num_rounds {
        for mi in 0..monkeys.len() {
            if should_log {
                println!("Monkey {}:", mi.to_string().bold());
            }

            monkeys[mi].items.reverse();

            while let Some(i) = monkeys[mi].items.pop() {
                if should_log {
                    println!(
                        "  Monkey inspect an time with a worry level of {}.",
                        i.to_string().bold()
                    );
                }
                monkeys[mi].inspect_count += 1;

                let opa = match monkeys[mi].opa {
                    Old => i,
                    Number(n) => n,
                };
                let higher_worry = match monkeys[mi].op {
                    Add => {
                        let new_i = i + opa;
                        if should_log {
                            println!("    Worry level is increased by {opa} to {new_i}.");
                        }
                        new_i
                    }
                    Times => {
                        let new_i = i * opa;
                        if should_log {
                            println!("    Worry level is multiplied by {opa} to {new_i}.");
                        }
                        new_i
                    }
                };

                let maybe_lower_worry = match worry_reduction_method {
                    DivBy3 => {
                        let div = 3;
                        let temp = higher_worry / div;
                        if should_log {
                            println!(
                        "    Monkey gets bored with item. Worry level is divided by {div} to {temp}.");
                        }
                        temp
                    }
                    ModuloBySumOfAllTestDivisors => {
                        let modulo = monkeys.iter().map(|m| m.test_div).product::<u64>();
                        let temp = higher_worry % modulo;
                        if should_log {
                            println!(
                        "    Monkey gets bored with item. Worry level is moduloed by {modulo} to {temp}.");
                        }
                        temp
                    }
                };
                assert_ne!(maybe_lower_worry, 0);

                let is_div = maybe_lower_worry % monkeys[mi].test_div == 0;
                let dest = if is_div {
                    if should_log {
                        println!(
                            "    Current worry level is divisible by {}.",
                            monkeys[mi].test_div
                        );
                        println!(
                            "    Item with worry level {maybe_lower_worry} is thrown to monkey {}.",
                            monkeys[mi].true_dest.to_string().bold()
                        );
                    }
                    monkeys[mi].true_dest
                } else {
                    if should_log {
                        println!(
                            "    Current worry level is not divisible by {}.",
                            monkeys[mi].test_div
                        );
                        println!(
                            "    Item with worry level {maybe_lower_worry} is thrown to monkey {}.",
                            monkeys[mi].false_dest.to_string().bold()
                        );
                    }
                    monkeys[mi].false_dest
                };
                monkeys[dest as usize].items.push(maybe_lower_worry);
            }
            monkeys[mi].items.clear();
        }

        if should_log {
            println!(
                "\nAfter round {round}, the monkeys are holding items with these worry levels:"
            );
            for (index, m) in monkeys.iter().enumerate() {
                let items = m.items.iter().map(|i| i.to_string()).join(", ");
                println!("Monkey {index}: {items}",);
            }
        }
    }

    if should_log {
        println!();
        for (index, m) in monkeys.iter().enumerate() {
            println!("Monkey {index} inspected items {} times.", m.inspect_count);
        }
        println!();
    }

    monkeys
        .iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

use WorryReductionMethod::*;
enum WorryReductionMethod {
    DivBy3,
    ModuloBySumOfAllTestDivisors,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operator,
    opa: Operand,
    test_div: u64,
    true_dest: u64,
    false_dest: u64,
    inspect_count: u64,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Monkey \d+:\n.+Starting items: ([\d, ]+)\n.+Operation: new = old ([*+]) (\d+|old)\n.+Test: divisible by (\d+)\n.+If true: throw to monkey (\d+)\n.+If false: throw to monkey (\d+)$$").unwrap();

        let cs = re.captures(s).unwrap();

        Ok(Monkey {
            items: cs[1].split(", ").map(|x| x.parse().unwrap()).collect_vec(),
            op: match &cs[2] {
                "+" => Add,
                "*" => Times,
                _ => panic!(),
            },
            opa: match &cs[3] {
                "old" => Old,
                n => Number(n.parse().unwrap()),
            },
            test_div: cs[4].parse().unwrap(),
            true_dest: cs[5].parse().unwrap(),
            false_dest: cs[6].parse().unwrap(),
            inspect_count: 0,
        })
    }
}

use Operator::*;
#[derive(Debug)]
enum Operator {
    Add,
    Times,
}

use Operand::*;
#[derive(Debug)]
enum Operand {
    Old,
    Number(u64),
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
        assert_eq!(solve_part_1(&file("example_1")), 10605);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 57348);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 2713310158);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 14106266886);
    }
}
