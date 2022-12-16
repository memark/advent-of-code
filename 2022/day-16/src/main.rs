#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    unused_mut
)]

use itertools::Itertools;
use pathfinding::prelude::{astar_bag, bfs, bfs_reach, dfs, dfs_reach};
use regex::Regex;
use std::{collections::HashMap, fmt::Display, fs, str::FromStr, string::ParseError};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("example_1")));
    // println!("Part 1: {}", solve_part_1(&file("input")));
    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> i32 {
    let valves = input
        .lines()
        .map(|l| l.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();
    // dbg!(&valves);

    // är i rum AA
    let start = "AA";

    let mut operations = Vec::<Operation>::new();
    let mut results = Vec::<Vec<Operation>>::new();

    let mut current = start;

    loop {
        // if visited.len() + opened.len() >= 30 {
        if operations.len() >= 30 {
            results.push(operations);
            break;
        }

        let all_operations = {
            let mut res = vec![];
            res.push(Open(current.to_owned()));
            res.extend(valves[current].tunnels.iter().map(|t| Move(t.clone())));
            res
        };

        let possible_operations = all_operations
            .iter()
            .filter(|op| !operations.contains(op))
            .collect_vec();
        dbg!(possible_operations);

        // * öppna ett valv
        if valves[current].flow_rate > 0 && !operations.contains(&Open(current.to_owned())) {
            operations.push(Open(current.to_owned()));
            println!("Open {current}");

            continue;
        }
        // * gå genom en tunnel
        else if let Some(next_t) = valves[current]
            .tunnels
            .iter()
            .find(|t| !operations.contains(&Move((*t).to_owned())))
        {
            current = next_t;
            operations.push(Move(current.to_owned()));
            println!("Move {current}");

            continue;
        }
        // * gör ingenting.
        else {
            operations.push(Idle());
            println!("Idle");

            continue;
        }
    }

    // // 30 min på mej
    // for min in 1..=30 {
    //     println!("== Minute {min} ==");

    //     // Man kan varje minut göra
    //     // * öppna ett valv
    //     // * gå genom en tunnel

    //     if open.len() == 0 {
    //         println!("No valves are open.");
    //     } else {
    //         let vs = valves.iter().map(|v| &v.name).join(", ");
    //         let p = 0;
    //         println!("Valves {vs:#?} are open, releasing {p} pressure.");
    //     }

    //     println!();
    // }

    0
}

use Operation::*;
#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Move(String),
    Open(String),
    Idle(),
}

type Name = String;
type FlowRate = u8;

#[derive(Debug)]
struct Valve {
    name: Name,
    flow_rate: FlowRate,
    tunnels: Vec<Name>,
}

impl FromStr for Valve {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
        let cs = re.captures(s).unwrap();

        Ok(Valve {
            name: cs[1].to_owned(),
            flow_rate: cs[2].parse().unwrap(),
            tunnels: cs[3].split(", ").map(|t| t.to_owned()).collect_vec(),
        })
    }
}

fn solve_part_2(input: &str) -> i32 {
    todo!()
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
        assert_eq!(solve_part_1(&file("example_1")), 1651);
    }

    #[ignore]
    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), todo!());
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), todo!());
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
