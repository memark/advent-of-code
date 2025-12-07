#![allow(unused)]

use itertools::Itertools;
use pathfinding::prelude::count_paths;
use std::collections::HashSet;
use std::hash::Hash;

const INPUT: &str = include_str!("input.txt");

type Int = u64;

fn main() {
    println!("Part 1: {:?}", solve(Part::Part1, INPUT));
    println!("Part 2: {:?}", solve(Part::Part2, INPUT));
}

enum Part {
    Part1,
    Part2,
}

pub trait MyItertools: Iterator {
    fn collect_hashset(self) -> HashSet<Self::Item>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        self.collect()
    }
}

impl<I: Iterator> MyItertools for I {}

fn solve(part: Part, str: &str) -> Int {
    let str = str.trim();
    let rows = str.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let coords = (0..num_rows).cartesian_product(0..num_cols);
    let start_coord = coords.clone().find(|x| rows[x.0][x.1] == 'S').unwrap();
    let splitter_coords = coords
        .clone()
        .filter(|x| rows[x.0][x.1] == '^')
        .collect_hashset();
    let is_splitter = |coord: &_| splitter_coords.contains(coord);

    match part {
        Part::Part1 => {
            let (_, num_splits) = (0..num_rows).fold(
                (HashSet::from_iter([start_coord.1]), 0),
                |(beams, num), ri| {
                    let new_beams = beams.iter().flat_map(|&bi| {
                        if is_splitter(&(ri, bi)) {
                            vec![bi - 1, bi + 1]
                        } else {
                            vec![bi]
                        }
                    });
                    let new_splits = new_beams.clone().count() - beams.len();
                    (new_beams.collect_hashset(), num + new_splits)
                },
            );
            num_splits as u64
        }
        Part::Part2 => count_paths(
            start_coord,
            |&(row, col)| {
                if is_splitter(&(row, col)) {
                    vec![(row + 1, col - 1), (row + 1, col + 1)]
                } else {
                    vec![(row + 1, col)]
                }
            },
            |&(row, _)| row >= num_rows,
        ) as u64,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const EXAMPLE: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[rstest]
    #[case::part1_example(Part::Part1, EXAMPLE, 21)]
    #[case::part1_input(Part::Part1, INPUT, 1533)]
    #[case::part2_example(Part::Part2, EXAMPLE, 40)]
    #[case::part2_input(Part::Part2, INPUT, 10733529153890)]
    fn solves_correctly(#[case] part: Part, #[case] data: &str, #[case] expected: Int) {
        assert_eq!(solve(part, data), expected);
    }
}
