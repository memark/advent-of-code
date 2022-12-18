use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::{collections::HashSet, fs};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> usize {
    solve(input, false)
}

fn solve_part_2(input: &str) -> usize {
    solve(input, true)
}

type Coord = (i8, i8, i8);

fn solve(input: &str, needs_path_out: bool) -> usize {
    let cubes = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple::<Coord>()
                .unwrap()
        })
        .collect::<HashSet<_>>();

    cubes
        .iter()
        .map(|cube| {
            let other_cubes = cubes.iter().filter(|c| *c != cube).collect::<HashSet<_>>();
            let (x, y, z) = cube;

            NEIGHBOURS
                .iter()
                .map(|(xd, yd, zd)| (x + xd, y + yd, z + zd))
                .filter(|n| !other_cubes.contains(n))
                .filter(|n| !needs_path_out || exists_path_out(n, &cubes))
                .count()
        })
        .sum()
}

fn exists_path_out(start: &Coord, cubes: &HashSet<Coord>) -> bool {
    let (x_max, y_max, z_max) = {
        (
            cubes.iter().map(|(x, _, _)| x).max().unwrap(),
            cubes.iter().map(|(_, y, _)| y).max().unwrap(),
            cubes.iter().map(|(_, _, z)| z).max().unwrap(),
        )
    };

    let successors = |(x, y, z)| {
        NEIGHBOURS
            .iter()
            .map(move |(xd, yd, zd)| (x + xd, y + yd, z + zd))
            .filter(|c| !cubes.contains(c))
    };

    bfs(
        start,
        |&c| successors(c),
        |c| *c == (x_max + 1, y_max + 1, z_max + 1),
    )
    .is_some()
}

const NEIGHBOURS: [(i8, i8, i8); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), 10);
        assert_eq!(solve_part_1(&file("example_2")), 64);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 3610);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), 58);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 2082);
    }
}
