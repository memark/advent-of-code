#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    clippy::print_literal,
    clippy::if_same_then_else,
    clippy::vec_init_then_push,
    unused_mut
)]

use colored::Colorize;
use itertools::{iproduct, Itertools};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // println!("Part 1: {}", solve_part_1(&file("example_2")));

    println!("Part 1: {}", solve_part_1(&file("input")));
    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> u32 {
    // The walls of the valley are drawn as #; everything else is ground.
    // Clear ground - where there is currently no blizzard - is drawn as .
    // Otherwise, blizzards are drawn with an arrow indicating their direction of motion: up (^), down (v), left (<), or right (>).

    let chars = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    // dbg!(&chars);

    let x_len = chars[0].len();
    let y_len = chars.len();
    // dbg!(&x_len, &y_len);

    // panic!();

    let mut blizzards = iproduct!(0..x_len, 0..y_len)
        .filter(|&(x, y)| ['>', 'v', '<', '^'].contains(&chars[y][x]))
        .map(|(x, y)| ((x, y), chars[y][x]))
        .collect::<BlizzardMap>();
    // dbg!(&blizzards);

    let start = (1, 0);
    let end = (x_len - 2, y_len - 1);

    let me = start;

    println!("\nInitial state:");
    print_map(&start, &start, &end, &blizzards, x_len, y_len);

    // panic!();

    let mut queue: Vec<(State, char)> = Vec::new();

    queue.push((
        State {
            minute: 1,
            me: start,
            blizzards: blizzards.clone(),
        },
        'v',
    ));
    queue.push((
        State {
            minute: 1,
            me: start,
            blizzards: blizzards.clone(),
        },
        'w',
    ));

    let minute_limit = 200;

    let mut results = vec![];

    while let Some(item) = queue.pop() {
        // dbg!(&queue);

        let (
            State {
                minute,
                me,
                blizzards,
            },
            operation,
        ) = item;

        let mut new_blizzards = HashSet::<(Coord, char)>::new();

        let (mx, my) = me;
        let new_me = match operation {
            '>' if mx == x_len - 2 => continue,
            '>' => (mx + 1, my),

            'v' if my == y_len - 2 && (mx, my + 1) != end => continue,
            'v' => (mx, my + 1),

            '<' if mx == 1 => continue,
            '<' => (mx - 1, my),

            '^' if my == 1 => continue,
            '^' => (mx, my - 1),

            'w' => (mx, my),

            _ => panic!(),
        };

        for ((bx, by), bc) in blizzards {
            let (nx, ny) = match bc {
                '>' if bx == x_len - 2 => (1, by),
                '>' => (bx + 1, by),

                'v' if by == y_len - 2 => (bx, 1),
                'v' => (bx, by + 1),

                '<' if bx == 1 => (x_len - 2, by),
                '<' => (bx - 1, by),

                '^' if by == 1 => (bx, y_len - 2),
                '^' => (bx, by - 1),

                _ => panic!(),
            };
            new_blizzards.insert(((nx, ny), bc));
        }

        // # När minuten är slut.

        if new_blizzards.iter().any(|((x, y), c)| (*x, *y) == new_me) {
            // We hit a blizzard. Don't store this result.
            continue;
        }

        if minute >= minute_limit {
            // Didn't reach the goal in time.
            // continue;
        }

        if new_me == end {
            // Reached the goal.
            if minute < *results.iter().min().unwrap_or(&5000) {
                dbg!(&minute);
            }
            results.push(minute);

            println!("\nMinute {minute}:");
            print_map(&new_me, &start, &end, &new_blizzards, x_len, y_len);

            continue;
        }

        // Hitta möjliga vägar att gå.
        // Skapa operationer och pusha till kön.
        // Kanske kan jag bara pusha alla fyra varianterna och hantera dem ovan såsom misslyckade?

        // dbg!(&queue);
        for c in ['>', 'v', '<', '^', 'w'] {
            queue.push((
                State {
                    minute: minute + 1,
                    me: new_me,
                    blizzards: new_blizzards.clone(),
                },
                c,
            ));
        }
        // dbg!(&queue);

        // println!("\nMinute {minute}:");
        // print_map(&new_me, &start, &end, &new_blizzards, x_len, y_len);
    }
    // Snöstormarna åker med samma hastighet hela tiden, och kommer därför alltid vara på samma ställe vid t % len.
    // Len är olika i x- och y-led.

    dbg!(&results);

    *results.iter().min().unwrap_or(&0)
}

#[derive(Debug, Clone)]
struct State {
    minute: u32,
    me: Coord,
    blizzards: BlizzardMap,
}

type Coord = (usize, usize);

type BlizzardMap = HashSet<(Coord, char)>;

fn print_map(
    me: &Coord,
    start: &Coord,
    end: &Coord,
    blizzards: &BlizzardMap,
    x_len: usize,
    y_len: usize,
) {
    for y in 0..y_len {
        for x in 0..x_len {
            let bs = blizzards
                .iter()
                .filter(|((bx, by), bc)| *bx == x && *by == y)
                .collect_vec();

            if (x, y) == *me {
                print!("{}", "M".red());
            } else if (x, y) == *start {
                print!("{}", "S");
            } else if (x, y) == *end {
                print!("{}", "E");
            } else if bs.len() == 1 {
                print!("{}", bs[0].1.to_string().blue());
            } else if bs.len() >= 2 {
                print!("{}", bs.len().to_string().blue());
            } else if x == 0 || x == x_len - 1 || y == 0 || y == y_len - 1 {
                print!("{}", "#");
            } else {
                print!("{}", ".");
            }
        }
        println!();
    }
}

fn solve_part_2(input: &str) -> i32 {
    0
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
        assert_eq!(solve_part_1(&file("example_1")), todo!());
    }

    #[ignore]
    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), todo!());
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
