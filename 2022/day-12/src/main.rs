#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::{iproduct, Itertools};
use pathfinding::prelude::*;
use std::fs;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> usize {
    let map: Map = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    // dbg!(&map);

    let start = {
        let y = map.iter().position(|l| l.contains(&b'S')).unwrap();
        let x = map[y].iter().position(|i| i == &b'S').unwrap();
        (x, y)
    };
    // dbg!(&start);

    let end = {
        let y = map.iter().position(|l| l.contains(&b'E')).unwrap();
        let x = map[y].iter().position(|i| i == &b'E').unwrap();
        (x, y)
    };
    // dbg!(&end);

    let successors = |(x, y)| {
        get_neighbors(&map, (x, y))
            .iter()
            // .inspect(|(nx, ny)| println!("{:?}", (nx, ny)))
            .filter(|(nx, ny)| {
                let hn = get_height(map[*ny][*nx]);
                let h = get_height(map[y][x]);
                let diff = hn as i32 - h as i32;
                diff <= 1
            })
            .copied()
            .collect_vec()
    };

    // let result = dfs(&start, |&c| successors(*c), |c| **c == end);
    // dbg!(&result);

    let path = bfs(&start, |&c| successors(c), |c| *c == end).unwrap();
    let num_steps = path.len() - 1;

    num_steps
}

fn solve_part_2(input: &str) -> usize {
    let map: Map = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();

    let start = {
        let y = map.iter().position(|l| l.contains(&b'S')).unwrap();
        let x = map[y].iter().position(|i| i == &b'S').unwrap();
        (x, y)
    };

    let end = {
        let y = map.iter().position(|l| l.contains(&b'E')).unwrap();
        let x = map[y].iter().position(|i| i == &b'E').unwrap();
        (x, y)
    };

    let successors = |(x, y)| {
        get_neighbors(&map, (x, y))
            .iter()
            .filter(|(nx, ny)| {
                let hn = get_height(map[*ny][*nx]);
                let h = get_height(map[y][x]);
                let diff = hn as i32 - h as i32;
                diff <= 1
            })
            .copied()
            .collect_vec()
    };

    let x_len = map[0].len();
    let y_len = map.len();

    let paths = iproduct!(0..x_len, 0..y_len)
        .filter(|(x, y)| map[*y][*x] == b'a')
        .filter_map(|(x, y)| bfs(&(x, y), |&c| successors(c), |c| *c == end))
        .collect_vec();

    let shortest_path = paths.iter().min_by_key(|p| p.len()).unwrap();

    shortest_path.len() - 1
}

type Coord = (usize, usize);
type Map = Vec<Vec<u8>>;

fn get_height(b: u8) -> u8 {
    match b {
        b'S' => get_height(b'a'),
        b'E' => get_height(b'z'),
        c => c - b'a',
    }
}

fn get_neighbors(map: &Map, (x, y): Coord) -> Vec<Coord> {
    [
        (x as i32 - 1, y as i32),
        (x as i32 + 1, y as i32),
        (x as i32, y as i32 - 1),
        (x as i32, y as i32 + 1),
    ]
    .into_iter()
    .filter(|(x, y)| is_valid(&map, (*x as usize, *y as usize)))
    .map(|(x, y)| (x as usize, y as usize))
    .collect_vec()
}

fn is_valid(map: &Map, (x, y): Coord) -> bool {
    let x_len = map[0].len();
    let y_len = map.len();

    x.clamp(0, x_len - 1) == x && y.clamp(0, y_len - 1) == y
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
        assert_eq!(solve_part_1(&file("example_1")), 31);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 481);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 29);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 480);
    }
}
