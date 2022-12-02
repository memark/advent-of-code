use itertools::Itertools;
use std::fs;
use Outcome::*;
use Shape::*;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split(' ').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(elf, me)| {
            let elf = Shape::from_abc(elf);
            let me = Shape::from_xyz(me);
            let outcome = Outcome::from_shapes(&elf, &me);

            me.score() + outcome.score()
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split(' ').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(elf, outcome)| {
            let elf = Shape::from_abc(elf);
            let outcome = Outcome::from_xyz(outcome);
            let me = &outcome.with_elf(&elf);

            me.score() + outcome.score()
        })
        .sum()
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_abc(s: &str) -> Shape {
        match s {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("{}", s),
        }
    }

    fn from_xyz(s: &str) -> Shape {
        match s {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("{}", s),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_shapes(elf: &Shape, me: &Shape) -> Self {
        match (elf, &me) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Lose,

            (Paper, Rock) => Lose,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Win,

            (Scissors, Rock) => Win,
            (Scissors, Paper) => Lose,
            (Scissors, Scissors) => Draw,
        }
    }

    fn with_elf(&self, elf: &Shape) -> Shape {
        match self {
            Lose => match elf {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Draw => match elf {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            },
            Win => match elf {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
        }
    }

    fn from_xyz(s: &str) -> Outcome {
        match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("{}", s),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
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

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 12_989);
    }
}
