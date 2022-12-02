#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use itertools::Itertools;
use std::{fmt::Display, fs};

fn main() {
    // println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> i32 {
    input
        .split('\n')
        .map(|l| l.split(' ').collect_vec())
        .map(|h| {
            let elf = h[0];
            let me = h[1];
            // score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
            let shape = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,

                _ => panic!(),
            };

            // A rock, B paper, C scissor
            // X rock, Y paper, Z scissor

            // score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
            let outcome = match (elf, me) {
                ("A", "X") => 3,
                ("A", "Y") => 6,
                ("A", "Z") => 0,

                ("B", "X") => 0,
                ("B", "Y") => 3,
                ("B", "Z") => 6,

                ("C", "X") => 6,
                ("C", "Y") => 0,
                ("C", "Z") => 3,

                _ => {
                    println!("{:?}", (elf, me));
                    panic!()
                }
            };

            let score = shape + outcome;

            println!("{:?} => {} + {} = {}", (me, elf), shape, outcome, score);

            score
        })
        .sum()
}

fn solve_part_2(input: &str) -> i32 {
    input
        .split('\n')
        .map(|l| l.split(' ').collect_vec())
        .map(|h| {
            let elf = h[0];
            let outcome = h[1];

            // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
            // A rock, B paper, C scissor
            // X rock, Y paper, Z scissor
            let me = match outcome {
                // lose
                "X" => match elf {
                    "A" => "Z",
                    "B" => "X",
                    "C" => "Y",
                    _ => panic!(),
                },

                // draw
                "Y" => match elf {
                    "A" => "X",
                    "B" => "Y",
                    "C" => "Z",
                    _ => panic!(),
                },

                // win
                "Z" => match elf {
                    "A" => "Y",
                    "B" => "Z",
                    "C" => "X",
                    _ => panic!(),
                },
                _ => panic!(),
            };
            let outcome_score = match outcome {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!(),
            };

            // score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
            let shape = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,

                _ => panic!(),
            };

            let score = shape + outcome_score;

            println!("{:?} => {} + {} = {}", (me, elf), shape, outcome, score);

            score
        })
        .sum()
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
        assert_eq!(solve_part_1(&file("example_1")), 15);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 15_691);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), 12);
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
