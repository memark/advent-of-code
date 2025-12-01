#![allow(unused)]

use itertools::Itertools;

fn main() {
    let example = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    let input = include_str!("input.txt");

    let data = input;

    println!("Part 1: {:?}", solve(input, Part::Part1)); // 1123
    println!("Part 2: {:?}", solve(input, Part::Part2)); // 6695
}

fn solve(data: &str, part: Part) -> usize {
    data.replace("L", "-")
        .replace("R", "")
        .split_terminator("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .scan(50, |state, x| {
            let start = *state;
            *state += x;
            let end = *state;
            Some(match part {
                Part::Part1 => (vec![end]),
                Part::Part2 => {
                    ((if end < start {
                        end..start
                    } else {
                        start + 1..end + 1
                    })
                    .collect_vec())
                }
            })
        })
        .flatten()
        .map(|x| x.rem_euclid(100))
        .filter(|&x| x == 0)
        .count()
}

enum Part {
    Part1,
    Part2,
}
