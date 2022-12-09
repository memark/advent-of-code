#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::{fs, str::FromStr, string::ParseError};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> usize {
    // Assume the head and the tail both start at the same position, overlapping.

    // You just need to work out where the tail goes as the head follows a series of motions.

    let motions = input
        .lines()
        .map(|l| l.parse::<Motion>().unwrap())
        .collect_vec();
    dbg!(&motions);

    let start = Coord { x: 0, y: 4 };

    let mut head = start.clone();
    let mut tail = start.clone();
    let mut visited = vec![start.clone()];

    for m in motions {
        let (a, b, c) = move_head_and_tail(&head, &tail, &m);
        head = a;
        tail = b;
        for cc in &c {
            visited.push(cc.clone());
        }
    }

    visited.iter().unique().count()
}

fn move_head_and_tail(head: &Coord, tail: &Coord, motion: &Motion) -> (Coord, Coord, Vec<Coord>) {
    // Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching):

    let (a, b, c) =
        (1..=motion.steps).fold((head.clone(), tail.clone(), vec![]), |(h, t, v), m| {
            let new_head = move_head_one_step(&h, &motion.dir);
            let new_tail = adjust_tail(&new_head, &t);
            let mut new_visited = v.clone();
            new_visited.push(new_tail.clone());

            (new_head.clone(), new_tail.clone(), new_visited)
        });

    (a.to_owned(), b.to_owned(), c)
}

fn move_head_one_step(head: &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Right => Coord {
            x: head.x + 1,
            y: head.y,
        },
        Direction::Up => Coord {
            x: head.x,
            y: head.y - 1,
        },
        Direction::Down => Coord {
            x: head.x,
            y: head.y + 1,
        },
        Direction::Left => Coord {
            x: head.x - 1,
            y: head.y,
        },
    }
}

fn adjust_tail(head: &Coord, tail: &Coord) -> Coord {
    // uttrycker försprånget head har
    // head 4 och tail 2 => diff 2
    let xhd = head.x - tail.x;
    let yhd = head.y - tail.y;

    if false {
        panic!()
    }
    //
    // If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:
    else if xhd == 0 && yhd == 2 {
        tail.with_y(tail.y + 1)
    } else if xhd == 0 && yhd == -2 {
        tail.with_y(tail.y - 1)
    } else if xhd == 2 && yhd == 0 {
        tail.with_x(tail.x + 1)
    } else if xhd == -2 && yhd == 0 {
        tail.with_x(tail.x - 1)
    }
    //
    // Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

    // x diffar 1
    else if xhd == 1 && yhd == 2 {
        Coord {
            x: tail.x + 1,
            y: tail.y + 1,
        }
    } else if xhd == 1 && yhd == -2 {
        Coord {
            x: tail.x + 1,
            y: tail.y - 1,
        }
    } else if xhd == -1 && yhd == 2 {
        Coord {
            x: tail.x - 1,
            y: tail.y + 1,
        }
    } else if xhd == -1 && yhd == -2 {
        Coord {
            x: tail.x - 1,
            y: tail.y - 1,
        }
    }
    //

    // y diffar 1
    else if xhd == 2 && yhd == 1 {
        Coord {
            x: tail.x + 1,
            y: tail.y + 1,
        }
    } else if xhd == 2 && yhd == -1 {
        Coord {
            x: tail.x + 1,
            y: tail.y - 1,
        }
    } else if xhd == -2 && yhd == 1 {
        Coord {
            x: tail.x - 1,
            y: tail.y + 1,
        }
    } else if xhd == -2 && yhd == -1 {
        Coord {
            x: tail.x - 1,
            y: tail.y - 1,
        }
    }
    //
    else {
        tail.to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn with_x(&self, x: i32) -> Coord {
        Coord { x, y: self.y }
    }
    fn with_y(&self, y: i32) -> Coord {
        Coord { x: self.x, y }
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
        // Use '?' ?
        let (a, b) = s.split_once(' ').unwrap();
        let steps = b.parse().unwrap();
        let dir = match a {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!(),
        };
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
        assert_eq!(solve_part_1(&file("example_1")), 13);
    }

    #[test]
    fn move_head_one_step_tests() {
        assert_eq!(
            move_head_one_step(&Coord { x: 1, y: 1 }, &Direction::Right),
            Coord { x: 2, y: 1 }
        );
        assert_eq!(
            move_head_one_step(&Coord { x: 1, y: 1 }, &Direction::Up),
            Coord { x: 1, y: 0 }
        );
        assert_eq!(
            move_head_one_step(&Coord { x: 1, y: 1 }, &Direction::Left),
            Coord { x: 0, y: 1 }
        );
        assert_eq!(
            move_head_one_step(&Coord { x: 1, y: 1 }, &Direction::Down),
            Coord { x: 1, y: 2 }
        );
    }

    #[test]
    fn adjust_tail_tests() {
        // Initial state
        assert_eq!(
            adjust_tail(&Coord { x: 0, y: 4 }, &Coord { x: 0, y: 4 }),
            Coord { x: 0, y: 4 }
        );

        // R 4 : 1
        assert_eq!(
            adjust_tail(&Coord { x: 1, y: 4 }, &Coord { x: 0, y: 4 }),
            Coord { x: 0, y: 4 }
        );
        // R 4 : 2
        assert_eq!(
            adjust_tail(&Coord { x: 2, y: 4 }, &Coord { x: 0, y: 4 }),
            Coord { x: 1, y: 4 }
        );
        // R 4 : 3
        assert_eq!(
            adjust_tail(&Coord { x: 3, y: 4 }, &Coord { x: 1, y: 4 }),
            Coord { x: 2, y: 4 }
        );
        // R 4 : 4
        assert_eq!(
            adjust_tail(&Coord { x: 4, y: 4 }, &Coord { x: 2, y: 4 }),
            Coord { x: 3, y: 4 }
        );

        // U 4 : 1
        assert_eq!(
            adjust_tail(&Coord { x: 4, y: 3 }, &Coord { x: 3, y: 4 }),
            Coord { x: 3, y: 4 }
        );
        // U 4 : 2
        assert_eq!(
            adjust_tail(&Coord { x: 4, y: 2 }, &Coord { x: 3, y: 4 }),
            Coord { x: 4, y: 3 }
        );
        // U 4 : 3
        assert_eq!(
            adjust_tail(&Coord { x: 4, y: 1 }, &Coord { x: 4, y: 3 }),
            Coord { x: 4, y: 2 }
        );
        // U 4 : 3
        assert_eq!(
            adjust_tail(&Coord { x: 4, y: 0 }, &Coord { x: 4, y: 2 }),
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
