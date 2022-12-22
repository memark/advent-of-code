// https://adventofcode.com/2022/day/22

#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    unused_mut,
    clippy::never_loop,
    clippy::single_char_pattern
)]

use chrono::ParseError;
use itertools::Itertools;
use std::{fs, str::FromStr};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> Number {
    let (a, b) = input.split_once("\n\n").unwrap();
    let a = a.lines().map(|l| l.chars().collect_vec()).collect_vec();
    dbg!((&a, &b));

    let max_x = a.iter().map(|aa| aa.len()).max().unwrap();
    let max_y = a.len() - 1;
    // dbg!(&max_x, &max_y);

    let map = a
        .iter()
        .map(|l| {
            l.iter()
                .chain([' '].iter().cycle())
                .take(max_x)
                .collect_vec()
        })
        .collect_vec();
    // dbg!(&map);

    let b_csv = b.replace("R", ",R,").replace("L", ",L,");
    let instructions = b_csv
        .split(",")
        .map(|bb| bb.parse::<Instruction>().unwrap())
        .collect_vec();
    // dbg!(&instructions);

    // "You begin the path in the leftmost open tile of the top row of tiles.""
    let start_x = map[0].iter().position(|&&c| c != ' ').unwrap() as Number;
    let start_y = 0;
    let start_pos = (start_x, start_y);
    dbg!(start_pos);

    let mut pos = start_pos;

    // "Initially, you are facing to the right."
    let mut dir = (1, 0);

    for instr in instructions {
        dbg!(&instr);
        match instr {
            TurnClockwise => {
                dir = match dir {
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    _ => panic!(),
                };
                println!("Turning  cw to {dir:?}.");
            }
            TurnCounterclockwise => {
                dir = match dir {
                    (0, -1) => (-1, 0),
                    (1, 0) => (0, -1),
                    (0, 1) => (1, 0),
                    (-1, 0) => (0, 1),
                    _ => panic!(),
                };
                println!("Turning ccw to {dir:?}.");
            }
            Move(steps) => {
                println!("Moving {steps} steps from {pos:?}.");

                for step in 1..=steps {
                    dbg!(step);

                    dbg!(pos, dir, max_x, max_y);

                    let new_pos = (
                        (pos.0 + dir.0 + max_x as i32) % max_x as Number,
                        (pos.1 + dir.1 + max_y as i32) % max_y as Number,
                    );
                    dbg!(&new_pos);

                    let x = new_pos.0 as usize;
                    let y = new_pos.1 as usize;
                    let new_pos_char = map[y][x];
                    dbg!(new_pos_char);
                    match new_pos_char {
                        '.' => {
                            // Normal, just move.
                            pos = new_pos;
                        }

                        '#' => {
                            // Wall, do nothing.
                            dbg!("Wall, do nothing");
                        }

                        ' ' => {
                            dbg!("Off grid");
                            let new_new_pos = match dir {
                                (0, -1) => {
                                    // up
                                    let col = map.iter().map(|row| row[x]).collect_vec();
                                    let new_y = col
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .find(|(i, &&c)| c != ' ')
                                        .unwrap()
                                        .0;
                                    (x, new_y)
                                }
                                (1, 0) => {
                                    // right
                                    let row = &map[y];
                                    let new_x =
                                        row.iter().enumerate().find(|(i, &&c)| c != ' ').unwrap().0;
                                    (new_x, y)
                                }
                                (0, 1) => {
                                    // down
                                    let col = map.iter().map(|row| row[x]).collect_vec();
                                    let new_y =
                                        col.iter().enumerate().find(|(i, &&c)| c != ' ').unwrap().0;
                                    (x, new_y)
                                }
                                (-1, 0) => {
                                    // left
                                    let row = &map[y];
                                    let new_x = row
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .find(|(i, &&c)| c != ' ')
                                        .unwrap()
                                        .0;
                                    (new_x, y)
                                }
                                _ => panic!(),
                            };

                            let new_new_char = map[new_new_pos.1][new_new_pos.0];
                            dbg!(new_new_char);
                            match new_new_char {
                                '#' => {
                                    // Wall, do nothing.
                                    dbg!("Wall, do nothing");
                                }

                                '.' => {
                                    // Normal, just move.
                                    let new_new_pos =
                                        (new_new_pos.0 as Number, new_new_pos.1 as Number);
                                    pos = new_new_pos;
                                }

                                _ => panic!(),
                            }
                        }

                        _ => panic!(),
                    }
                    dbg!(&pos);
                }
            }
        }
    }

    let final_row = pos.1 + 1;
    let final_column = pos.0 + 1;
    let final_facing = match dir {
        (0, -1) => 3,
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        _ => panic!(),
    };
    dbg!(final_row, final_column, final_facing);

    1000 * final_row + 4 * final_column + final_facing
}

// Scoring

// Facing is 0 for right (>),
// 1 for down (v),
// 2 for left (<), and
// 3 for up (^).

// The final password is the sum of 1000 times the row, 4 times the column, and the facing.

type Number = i32;

type Coord = (Number, Number);

use Instruction::*;
#[derive(Debug)]
enum Instruction {
    Move(u8),
    TurnClockwise,
    TurnCounterclockwise,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(TurnClockwise),
            "L" => Ok(TurnCounterclockwise),
            n => Ok(Move(n.parse().unwrap())),
        }
    }
}

fn solve_part_2(input: &str) -> Number {
    0
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), 6032);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 117054);
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), todo!());
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
