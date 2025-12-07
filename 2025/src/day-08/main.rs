#![allow(unused)]

use itertools::Itertools;
use pathfinding::prelude::count_paths;
use std::{collections::HashSet, mem};
use std::{fmt::Debug, hash::Hash};

const INPUT: &str = include_str!("input.txt");

type Int = i64;

const EXAMPLE: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

// This list describes the position of 20 junction boxes, one per line. Each position is given as X,Y,Z coordinates. So, the first junction box in the list is at X=162, Y=817, Z=812.

fn main() {
    println!("Part 1: {:?}", solve(Part::Part1(1000), INPUT));
    println!("Part 2: {:?}", solve(Part::Part2, INPUT));
}

enum Part {
    Part1(usize),
    Part2,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Box(Int, Int, Int);

impl Box {
    fn new((x, y, z): (Int, Int, Int)) -> Self {
        Self(x, y, z)
    }

    fn dist_to(&self, other: &Box) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}

impl Debug for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Fält 0 är unikt i exemplet, så det räcker.
        write!(f, "{:03}", self.0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pair(Box, Box);

impl Pair {
    fn new((a, b): (Box, Box)) -> Self {
        if a > b { Self(b, a) } else { Self(a, b) }
    }

    fn dist(&self) -> f64 {
        self.0.dist_to(&self.1)
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.0, self.1)
    }
}

type Circuit = HashSet<Box>;

fn solve(part: Part, str: &str) -> i64 {
    let boxes = parse_boxes(str);
    let pairs = create_ordered_pairs(&boxes);
    let mut circuits = create_circuits(&boxes);

    match part {
        Part::Part1(num_pairs) => {
            for p in pairs.into_iter().take(num_pairs) {
                process_pair(&mut circuits, &p);
            }
            circuits
                .into_iter()
                .map(|c| c.len() as i64)
                .sorted()
                .rev()
                .take(3)
                .product()
        }
        Part::Part2 => {
            for p in pairs {
                process_pair(&mut circuits, &p);
                if circuits.len() == 1 {
                    return p.0.0 * p.1.0;
                }
            }
            panic!()
        }
    }
}

fn parse_boxes(str: &str) -> HashSet<Box> {
    str.trim()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|v| v.parse::<Int>().unwrap())
                .collect_tuple()
                .map(Box::new)
                .unwrap()
        })
        .collect::<HashSet<_>>()
}

fn create_ordered_pairs(boxes: &HashSet<Box>) -> Vec<Pair> {
    boxes
        .iter()
        .cloned()
        .tuple_combinations()
        .map(Pair::new)
        .sorted_by(|a, b| a.dist().total_cmp(&(b).dist()))
        .collect_vec()
}

fn create_circuits(boxes: &HashSet<Box>) -> Vec<HashSet<Box>> {
    boxes.iter().map(|&b| HashSet::from([b])).collect()
}

fn process_pair(circuits: &mut Vec<Circuit>, p: &Pair) {
    match (
        circuits.iter().find_position(|c| c.contains(&p.0)),
        circuits.iter().find_position(|c| c.contains(&p.1)),
    ) {
        (Some((ia, _)), Some((ib, _))) if ia == ib => (),

        (Some((ia, _)), Some((ib, _))) => {
            let a = mem::take(&mut circuits[ia]);
            circuits[ib].extend(a);
            circuits.remove(ia);
        }

        (Some((ia, _)), None) => {
            circuits[ia].insert(p.1);
        }

        (None, Some((ib, _))) => {
            circuits[ib].insert(p.0);
        }

        (None, None) => circuits.push(HashSet::from([p.0, p.1])),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case::part1_example(Part::Part1(10), EXAMPLE, 40)]
    #[case::part1_input(Part::Part1(1000), INPUT, 181584)]
    #[case::part2_example(Part::Part2, EXAMPLE, 25272)]
    #[case::part2_input(Part::Part2, INPUT, 8465902405)]
    fn solves_correctly(#[case] part: Part, #[case] data: &str, #[case] expected: Int) {
        assert_eq!(solve(part, data), expected);
    }
}
