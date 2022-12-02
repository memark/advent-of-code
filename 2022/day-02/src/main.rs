use std::{error::Error, fs, str::FromStr};
use Outcome::*;
use Shape::*;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(elf, me)| (elf.parse().unwrap(), me.parse().unwrap()))
        .map(|(elf, me): (Shape, Shape)| me.score() + Outcome::for_me(&elf, &me).score())
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(elf, outcome)| (elf.parse().unwrap(), outcome.parse().unwrap()))
        .map(|(elf, outcome): (Shape, Outcome)| outcome.with_other(&elf).score() + outcome.score())
        .sum()
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => panic!("Invalid shape: {}", s),
        }
    }
}

impl Shape {
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

impl FromStr for Outcome {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => panic!("{}", s),
        }
    }
}

impl Outcome {
    fn for_me(other: &Shape, me: &Shape) -> Self {
        match (other, &me) {
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

    fn with_other(&self, other: &Shape) -> Shape {
        match self {
            Lose => match other {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Draw => match other {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            },
            Win => match other {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
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
