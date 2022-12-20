#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    clippy::if_same_then_else,
    unused_mut
)]

use itertools::Itertools;
use std::{collections::VecDeque, fs};

fn main() {
    // println!("Part 1: {}", solve_part_1(&file("example_1")));
    println!("Part 1: {}", solve_part_1(&file("input")));

    // println!("Part 2: {}", solve_part_2(&file("example_1")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> isize {
    let original = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    dbg!(&original);
    let len = original.len();

    let moved = move_stuff(original.clone(), original.len());

    let p0 = moved.iter().position(|n| *n == 0).unwrap();

    let p1 = (p0 + 1000 + len) % len;
    let n1 = moved[p1];
    dbg!(&n1);

    let p2 = (p0 + 2000 + len) % len;
    let n2 = moved[p2];
    dbg!(&n2);

    let p3 = (p0 + 3000 + len) % len;
    let n3 = moved[p3];
    dbg!(&n3);

    n1 + n2 + n3
}

fn solve_part_2(input: &str) -> isize {
    let original = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    dbg!(&original);
    let len = original.len();

    let encrypted = encrypt(original);

    let moved = move_stuff_2(encrypted, 10);

    let p0 = moved.iter().position(|n| *n == 0).unwrap();

    let p1 = (p0 + 1000 + len) % len;
    let n1 = moved[p1];
    dbg!(&n1);

    let p2 = (p0 + 2000 + len) % len;
    let n2 = moved[p2];
    dbg!(&n2);

    let p3 = (p0 + 3000 + len) % len;
    let n3 = moved[p3];
    dbg!(&n3);

    n1 + n2 + n3
}

static DECRYPTION_KEY: isize = 811589153;

fn encrypt(original: Vec<isize>) -> Vec<isize> {
    original.iter().map(|n| n * DECRYPTION_KEY).collect_vec()
}

use OldNumber::*;
#[derive(Debug, Clone, Copy, PartialEq)]
enum OldNumber {
    Unmoved(isize),
    Moved(isize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Number {
    number: isize,
    order: isize,
}

impl Number {
    fn new(n: isize, o: isize) -> Self {
        Number {
            number: n,
            order: o,
        }
    }
}

fn move_stuff_2(original: Vec<isize>, rounds: u8) -> Vec<isize> {
    println!("==== MOVE STUFF ====\n");

    let len = original.len() as isize;

    let mut working = original
        .iter()
        .enumerate()
        .map(|(i, n)| Number::new(*n, i as isize))
        .collect_vec();

    println!(
        "Initial arrangement:\n{}",
        &working.iter().map(|n| format!("{n:?}")).join(", ")
    );

    for round in 1..=rounds {
        println!("\n== Starting round {round} ==");

        for i in 0..len {
            let Some(pos) = working
                .iter()
                .position(|n|  n.order == i) else {
                    println!("No more numbers found to move. Round finished."); 
                    break;
                };

            let o = working[pos].number;

            let new_pos = {
                let pos = pos as isize;
                // dbg!(&pos);

                if o == 0 {
                    pos
                } else {
                    let mut temp = ((pos + o) % (len - 1) + len - 1) % (len - 1);
                    // dbg!(&temp);

                    if temp == 0 {
                        len - 1
                    } else {
                        temp
                    }
                }
            };
            // dbg!(&new_pos);

            let elem = working.remove(pos);
            working.insert(
                new_pos.try_into().expect("working.insert()"),
                Number::new(elem.number, elem.order),
            );

            println!("\n{o} moves from pos {pos} to pos {new_pos}:");
            // println!("{}", &working.iter().map(|n| n.number).join(", "));

            println!();
        }
    }

    working.iter().map(|n| n.number).collect_vec()
}

fn move_stuff(original: Vec<isize>, iterations: usize) -> Vec<isize> {
    println!("== MOVE STUFF ==\n");

    let len = original.len() as isize;

    let mut working = original.iter().map(|n| Unmoved(*n)).collect_vec();

    println!(
        "Initial arrangement:\n{}",
        &working.iter().map(|n| format!("{n:?}")).join(", ")
    );

    let mut iteration = 0;

    loop {
        if iteration >= iterations {
            dbg!("Enough iterations.");
            break;
        }

        let pos = working.iter().position(|n| matches!(n, Unmoved(_)));
        if pos.is_none() {
            dbg!("No more unmoved values.");
            break;
        }
        let pos = pos.unwrap();

        let Unmoved(o) = working[pos] else {panic!()};

        let new_pos = {
            let pos = pos as isize;
            // dbg!(&pos);

            let mut temp = ((pos + o) % (len - 1) + len - 1) % (len - 1);
            // dbg!(&temp);

            if temp == 0 {
                len - 1
            } else {
                temp
            }
        };
        // dbg!(&new_pos);

        let Unmoved(elem) = working.remove(pos) else {panic!()};
        working.insert(new_pos.try_into().expect("working.insert()"), Moved(elem));

        println!("\n{o} moves from pos {pos} to pos {new_pos}:");
        // println!("{}", &working.iter().join(", "));

        println!();

        iteration += 1;
    }

    working
        .iter()
        .map(|n| {
            match n {
                Unmoved(nn) => *nn,
                Moved(nn) => *nn,
            }
            // let Moved(nn) = n else {panic!()};
            // *nn
        })
        .collect_vec()
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn move_stuff_tests() {
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 0),
            vec![1, 2, -3, 3, -2, 0, 4]
        );
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 1),
            vec![2, 1, -3, 3, -2, 0, 4]
        );
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 2),
            vec![1, -3, 2, 3, -2, 0, 4]
        );
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 3),
            vec![1, 2, 3, -2, -3, 0, 4]
        );
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 4),
            vec![1, 2, -2, -3, 0, 3, 4]
        );
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 5),
            vec![1, 2, -3, 0, 3, 4, -2]
        );
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 6),
            vec![1, 2, -3, 0, 3, 4, -2]
        );
        assert_eq!(
            move_stuff(vec![1, 2, -3, 3, -2, 0, 4], 7),
            vec![1, 2, -3, 4, 0, 3, -2]
        );
    }

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), 3);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 2215);
    }

    #[test]
    fn encrypt_tests() {
        assert_eq!(
            encrypt(vec![1, 2, -3, 3, -2, 0, 4]),
            vec![
                811589153,
                1623178306,
                -2434767459,
                2434767459,
                -1623178306,
                0,
                3246356612
            ]
        );
    }

    #[test]
    fn move_stuff_2_tests() {
        assert_eq!(
            move_stuff_2(
                vec![
                    811589153,
                    1623178306,
                    -2434767459,
                    2434767459,
                    -1623178306,
                    0,
                    3246356612
                ],
                0
            ),
            vec![
                811589153,
                1623178306,
                -2434767459,
                2434767459,
                -1623178306,
                0,
                3246356612
            ]
        );
        assert_eq!(
            move_stuff_2(
                vec![
                    811589153,
                    1623178306,
                    -2434767459,
                    2434767459,
                    -1623178306,
                    0,
                    3246356612
                ],
                1
            ),
            vec![
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153
            ]
        );
        assert_eq!(
            move_stuff_2(
                vec![
                    811589153,
                    1623178306,
                    -2434767459,
                    2434767459,
                    -1623178306,
                    0,
                    3246356612
                ],
                2
            ),
            vec![
                0,
                2434767459,
                1623178306,
                3246356612,
                -2434767459,
                -1623178306,
                811589153
            ]
        );
        // ...
        assert_eq!(
            move_stuff_2(
                vec![
                    811589153,
                    1623178306,
                    -2434767459,
                    2434767459,
                    -1623178306,
                    0,
                    3246356612
                ],
                10
            ),
            vec![
                0,
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153
            ]
        );
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 1623178306);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 8927480683);
    }
}
