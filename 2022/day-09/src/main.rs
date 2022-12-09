#![allow(
    // dead_code,
    unreachable_code,
    // unused_imports,
    // unused_variables,
    // unused_mut,
    // clippy::redundant_clone
)]

use itertools::Itertools;
use std::{fs, str::FromStr, string::ParseError};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));

    // 7055, too high
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> usize {
    let motions = input
        .lines()
        .map(|l| l.parse::<Motion>().unwrap())
        .collect_vec();

    // Any start position is fine, arbitrarily choosing the one from the example.
    let start = Coord { x: 0, y: 4 };

    let (_final_head, _final_tail, final_visited) = motions.iter().fold(
        // "Assume the head and the tail both start at the same position, overlapping."
        (start.clone(), start.clone(), vec![start]),
        |(head, tail, visited), motion| {
            let (new_head, new_tail, new_visited) = move_head_and_tail(&head, &tail, motion);
            (new_head, new_tail, [visited, new_visited].concat())
        },
    );

    final_visited.into_iter().unique().count()
}

fn solve_part_2(input: &str) -> usize {
    let motions = input
        .lines()
        .map(|l| l.parse::<Motion>().unwrap())
        .collect_vec();

    // Any start position is fine, arbitrarily choosing the one from the example.
    let start = Coord { x: 0, y: 4 };

    let (_final_knots, final_visited) = motions.iter().fold(
        // "Assume the head and the tail both start at the same position, overlapping."
        (vec![start.clone(), start.clone()], vec![start]),
        |(knots, visited), motion| {
            let (new_knots, new_visited) = move_knots(&knots, motion);
            (new_knots, [visited, new_visited].concat())
        },
    );

    final_visited.into_iter().unique().count()
}

fn move_head_and_tail(head: &Coord, tail: &Coord, motion: &Motion) -> (Coord, Coord, Vec<Coord>) {
    (1..=motion.steps).fold((head.clone(), tail.clone(), vec![]), |(h, t, mut v), _| {
        let new_head = h.adjust(&motion.dir);
        let new_tail = adjust_knot(&new_head, &t);
        v.push(new_tail.clone());

        (new_head, new_tail, v)
    })
}

fn move_knots(knots: &[Coord], motion: &Motion) -> (Vec<Coord>, Vec<Coord>) {
    (1..=motion.steps).fold((knots.to_vec(), vec![]), |(ks, mut visited), _| {
        let new_knots = move_rope(&ks, &motion.dir);
        let new_tail = new_knots.last().unwrap().clone();
        visited.push(new_tail);

        (new_knots, visited)
    })
}

fn move_rope(knots: &[Coord], dir: &Direction) -> Vec<Coord> {
    let new_head = knots[0].adjust(dir);

    let mut new_knots = vec![];
    // Adding new head twice, since one will be consumed by tuple_windows().
    new_knots.push(new_head.clone());
    new_knots.push(new_head);
    for kn in knots {
        new_knots.push(kn.clone());
    }
    // println!("new_knots: {new_knots:?}");

    new_knots
        .iter()
        .tuple_windows()
        // .inspect(|x| println!("window: {x:?}"))
        .map(|(prev_k, curr_k)| adjust_knot(prev_k, curr_k))
        // .inspect(|x| println!("result: {x:?}"))
        .take(10)
        .collect_vec()
}

fn adjust_knot(previous_knot: &Coord, knot: &Coord) -> Coord {
    let xhd = previous_knot.x - knot.x;
    let yhd = previous_knot.y - knot.y;

    let mhd = mh_dist(previous_knot, knot);
    assert!(mhd <= 3);
    match mhd {
        // "Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching)"
        0 => knot.clone(),
        1 => knot.clone(),
        2 => {
            match (xhd, yhd) {
                // "If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough"
                (0, 2) => knot.adjust(&Down),
                (0, -2) => knot.adjust(&Up),
                (2, 0) => knot.adjust(&Right),
                (-2, 0) => knot.adjust(&Left),

                _ => knot.to_owned(),
            }
        }
        3 => {
            match (xhd, yhd) {
                // "Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up"
                (1, 2) => knot.adjust(&Right).adjust(&Down),
                (1, -2) => knot.adjust(&Right).adjust(&Up),
                (-1, 2) => knot.adjust(&Left).adjust(&Down),
                (-1, -2) => knot.adjust(&Left).adjust(&Up),

                (2, 1) => knot.adjust(&Right).adjust(&Down),
                (2, -1) => knot.adjust(&Right).adjust(&Up),
                (-2, 1) => knot.adjust(&Left).adjust(&Down),
                (-2, -1) => knot.adjust(&Left).adjust(&Up),

                _ => panic!(),
            }
        }
        _ => unreachable!(),
    }
}

fn mh_dist(a: &Coord, b: &Coord) -> u32 {
    (a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn with_delta(&self, xd: i32, yd: i32) -> Self {
        Coord {
            x: self.x + xd,
            y: self.y + yd,
        }
    }

    fn adjust(&self, step_dir: &Direction) -> Coord {
        match step_dir {
            Direction::Right => self.with_delta(1, 0),
            Direction::Up => self.with_delta(0, -1),
            Direction::Down => self.with_delta(0, 1),
            Direction::Left => self.with_delta(-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Motion {
    dir: Direction,
    steps: i32,
}
impl FromStr for Motion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(' ').unwrap();
        let steps = b.parse().unwrap();
        let dir = a.parse().unwrap();
        Ok(Motion { dir, steps })
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Up,
    Down,
    Left,
}

use Direction::*;
impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!(),
        };
        Ok(dir)
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
        assert_eq!(solve_part_1(&file("example_1")), 13);
    }

    #[test]
    fn apply_step_tests() {
        assert_eq!(Coord { x: 1, y: 1 }.adjust(&Right), Coord { x: 2, y: 1 });
        assert_eq!(Coord { x: 1, y: 1 }.adjust(&Up), Coord { x: 1, y: 0 });
        assert_eq!(Coord { x: 1, y: 1 }.adjust(&Left), Coord { x: 0, y: 1 });
        assert_eq!(Coord { x: 1, y: 1 }.adjust(&Down), Coord { x: 1, y: 2 });
    }

    #[test]
    fn move_rope_tests() {
        assert_eq!(
            move_rope(&[Coord { x: 0, y: 4 }], &Right),
            &[Coord { x: 1, y: 4 }, Coord { x: 0, y: 4 }]
        );
        assert_eq!(
            move_rope(&[Coord { x: 1, y: 4 }, Coord { x: 0, y: 4 }], &Right),
            &[
                Coord { x: 2, y: 4 },
                Coord { x: 1, y: 4 },
                Coord { x: 0, y: 4 }
            ]
        );
        assert_eq!(
            move_rope(
                &[
                    Coord { x: 8, y: 4 },
                    Coord { x: 7, y: 4 },
                    Coord { x: 6, y: 4 },
                    Coord { x: 5, y: 4 },
                    Coord { x: 4, y: 4 },
                    Coord { x: 3, y: 4 },
                    Coord { x: 2, y: 4 },
                    Coord { x: 1, y: 4 },
                    Coord { x: 0, y: 4 }
                ],
                &Right
            ),
            &[
                Coord { x: 9, y: 4 },
                Coord { x: 8, y: 4 },
                Coord { x: 7, y: 4 },
                Coord { x: 6, y: 4 },
                Coord { x: 5, y: 4 },
                Coord { x: 4, y: 4 },
                Coord { x: 3, y: 4 },
                Coord { x: 2, y: 4 },
                Coord { x: 1, y: 4 },
                Coord { x: 0, y: 4 }
            ]
        );
        assert_eq!(
            move_rope(
                &[
                    Coord { x: 9, y: 4 },
                    Coord { x: 8, y: 4 },
                    Coord { x: 7, y: 4 },
                    Coord { x: 6, y: 4 },
                    Coord { x: 5, y: 4 },
                    Coord { x: 4, y: 4 },
                    Coord { x: 3, y: 4 },
                    Coord { x: 2, y: 4 },
                    Coord { x: 1, y: 4 },
                    Coord { x: 0, y: 4 }
                ],
                &Right
            ),
            &[
                Coord { x: 10, y: 4 },
                Coord { x: 9, y: 4 },
                Coord { x: 8, y: 4 },
                Coord { x: 7, y: 4 },
                Coord { x: 6, y: 4 },
                Coord { x: 5, y: 4 },
                Coord { x: 4, y: 4 },
                Coord { x: 3, y: 4 },
                Coord { x: 2, y: 4 },
                Coord { x: 1, y: 4 },
            ]
        );
    }

    #[test]
    fn adjust_knot_tests() {
        // Initial state
        assert_eq!(
            adjust_knot(&Coord { x: 0, y: 4 }, &Coord { x: 0, y: 4 }),
            Coord { x: 0, y: 4 }
        );

        // R 4 : 1
        assert_eq!(
            adjust_knot(&Coord { x: 1, y: 4 }, &Coord { x: 0, y: 4 }),
            Coord { x: 0, y: 4 }
        );
        // R 4 : 2
        assert_eq!(
            adjust_knot(&Coord { x: 2, y: 4 }, &Coord { x: 0, y: 4 }),
            Coord { x: 1, y: 4 }
        );
        // R 4 : 3
        assert_eq!(
            adjust_knot(&Coord { x: 3, y: 4 }, &Coord { x: 1, y: 4 }),
            Coord { x: 2, y: 4 }
        );
        // R 4 : 4
        assert_eq!(
            adjust_knot(&Coord { x: 4, y: 4 }, &Coord { x: 2, y: 4 }),
            Coord { x: 3, y: 4 }
        );

        // U 4 : 1
        assert_eq!(
            adjust_knot(&Coord { x: 4, y: 3 }, &Coord { x: 3, y: 4 }),
            Coord { x: 3, y: 4 }
        );
        // U 4 : 2
        assert_eq!(
            adjust_knot(&Coord { x: 4, y: 2 }, &Coord { x: 3, y: 4 }),
            Coord { x: 4, y: 3 }
        );
        // U 4 : 3
        assert_eq!(
            adjust_knot(&Coord { x: 4, y: 1 }, &Coord { x: 4, y: 3 }),
            Coord { x: 4, y: 2 }
        );
        // U 4 : 3
        assert_eq!(
            adjust_knot(&Coord { x: 4, y: 0 }, &Coord { x: 4, y: 2 }),
            Coord { x: 4, y: 1 }
        );
    }

    #[test]
    fn move_head_and_tail_tests() {
        // R 4
        assert_eq!(
            move_head_and_tail(
                &Coord { x: 0, y: 4 },
                &Coord { x: 0, y: 4 },
                &"R 4".parse().unwrap()
            ),
            (
                Coord { x: 4, y: 4 },
                Coord { x: 3, y: 4 },
                vec![
                    Coord { x: 0, y: 4 },
                    Coord { x: 1, y: 4 },
                    Coord { x: 2, y: 4 },
                    Coord { x: 3, y: 4 }
                ]
            )
        );
        // U 4
        assert_eq!(
            move_head_and_tail(
                &Coord { x: 4, y: 4 },
                &Coord { x: 3, y: 4 },
                &"U 4".parse().unwrap()
            ),
            (
                Coord { x: 4, y: 0 },
                Coord { x: 4, y: 1 },
                vec![
                    Coord { x: 3, y: 4 },
                    Coord { x: 4, y: 3 },
                    Coord { x: 4, y: 2 },
                    Coord { x: 4, y: 1 },
                ]
            )
        );
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 6190);
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 36);
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
