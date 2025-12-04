#![allow(unused, clippy::let_and_return)]

use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashMap, iter::successors};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {:?}", solve(Part::Part1, INPUT));
    println!("Part 2: {:?}", solve(Part::Part2, INPUT));
}

enum Part {
    Part1,
    Part2,
}

fn solve(part: Part, data: &str) -> usize {
    let grid = data
        .split_terminator("\n")
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    match part {
        Part::Part1 => find_accessible_roll_coords(&grid).len(),
        Part::Part2 => {
            let start = grid.clone();
            let end = successors(Some(grid), |prev| {
                let rolls = find_accessible_roll_coords(prev);
                let next = prev
                    .par_iter()
                    .map(|(&k, &c)| if rolls.contains(&k) { (k, '.') } else { (k, c) })
                    .collect();
                (prev != &next).then_some(next)
            })
            .last()
            .unwrap();

            let count_rolls = |g: &Grid| g.iter().filter(|&(_, &c)| c == '@').count();
            count_rolls(&start) - count_rolls(&end)
        }
    }
}

type Grid = HashMap<Coord, char>;
type Coord = (i32, i32);

fn find_accessible_roll_coords(grid: &Grid) -> Vec<Coord> {
    grid.par_iter()
        .filter(|&(_, &c)| c == '@')
        .filter(|&((x, y), _)| {
            get_possible_neighbours(&(*x, *y))
                .iter()
                .filter_map(|k| grid.get(k))
                .filter(|&&c| c == '@')
                .count()
                < 4
        })
        .map(|(&k, _)| k)
        .collect()
}

fn get_possible_neighbours((x, y): &Coord) -> Vec<Coord> {
    NEIGHBOURS
        .iter()
        .map(|&(dx, dy)| ((x + dx), (y + dy)))
        .collect()
}

const NEIGHBOURS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 1),
];

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[rstest]
    #[case::part1_example(Part::Part1, EXAMPLE, 13)]
    #[case::part1_input(Part::Part1, INPUT, 1587)]
    #[case::part2_example(Part::Part2, EXAMPLE, 43)]
    #[case::part2_input(Part::Part2, INPUT, 8946)]
    fn solves_correctly(#[case] part: Part, #[case] data: &str, #[case] expected: usize) {
        assert_eq!(solve(part, data), expected);
    }
}
