#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    unused_mut
)]

use itertools::Itertools;
use std::fs;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> String {
    // Parse
    //   split på \n\n
    //   läs ut stacks mha position (regex?)
    //   moves också position (flera siffror)

    dbg!(&input);

    let (a, b) = input.split_once("\n\n").unwrap();
    let a = a.lines().collect_vec();
    let b = b.lines().collect_vec();
    // dbg!(&a);
    // dbg!(&b);

    let num_stacks = &a
        .iter()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    dbg!(&num_stacks);

    let mut stack_rows = a.clone();
    stack_rows.pop();
    dbg!(&stack_rows);

    let mut stacks = Vec::new();

    for stack_index in 0..*num_stacks {
        dbg!(&stack_index);

        let mut stack = Vec::new();

        for row_index in (0..stack_rows.len()).rev() {
            let stack_row = stack_rows[row_index];
            let pos = 1 + stack_index * 4;
            let c = &stack_row.chars().nth(pos).unwrap();
            if *c != ' ' {
                // dbg!(&pos);
                // dbg!(&c);
                stack.push(*c);
            }
        }
        // dbg!(&stack);

        stacks.push(stack);
        // for stack_row in &stack_rows {
        //     //
        //     let pos = 1 + stack_index * 4;
        //     dbg!(&pos);
        //     let c = &stack_row.chars().nth(pos).unwrap();
        //     dbg!(&c);
        // }
    }
    dbg!(&stacks);

    // let mut s1 = vec!['Z', 'N'];
    // let mut s2 = vec!['M', 'C', 'D'];
    // let mut s3 = vec!['P'];

    // let mut ss = vec![s1, s2, s3];

    let steps = &b
        .iter()
        .map(|l| l.split(' ').collect_vec())
        .map(|l| {
            (
                l[1].parse().unwrap(),
                l[3].parse().unwrap(),
                l[5].parse().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();
    // dbg!(&steps);

    // flytta crates
    //   >1 innebär separata steg!

    for (count, from, to) in steps {
        for i in 0..*count {
            //
            // dbg!((count, from, to));
            let t = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(t);
        }
    }
    dbg!(&stacks);

    let top = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    dbg!(&top);

    top
}

fn solve_part_2(input: &str) -> String {
    // Parse
    //   split på \n\n
    //   läs ut stacks mha position (regex?)
    //   moves också position (flera siffror)

    dbg!(&input);

    let (a, b) = input.split_once("\n\n").unwrap();
    let a = a.lines().collect_vec();
    let b = b.lines().collect_vec();
    // dbg!(&a);
    // dbg!(&b);

    let num_stacks = &a
        .iter()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    dbg!(&num_stacks);

    let mut stack_rows = a.clone();
    stack_rows.pop();
    dbg!(&stack_rows);

    let mut stacks = Vec::new();

    for stack_index in 0..*num_stacks {
        dbg!(&stack_index);

        let mut stack = Vec::new();

        for row_index in (0..stack_rows.len()).rev() {
            let stack_row = stack_rows[row_index];
            let pos = 1 + stack_index * 4;
            let c = &stack_row.chars().nth(pos).unwrap();
            if *c != ' ' {
                // dbg!(&pos);
                // dbg!(&c);
                stack.push(*c);
            }
        }
        // dbg!(&stack);

        stacks.push(stack);
        // for stack_row in &stack_rows {
        //     //
        //     let pos = 1 + stack_index * 4;
        //     dbg!(&pos);
        //     let c = &stack_row.chars().nth(pos).unwrap();
        //     dbg!(&c);
        // }
    }
    dbg!(&stacks);

    // let mut s1 = vec!['Z', 'N'];
    // let mut s2 = vec!['M', 'C', 'D'];
    // let mut s3 = vec!['P'];

    // let mut ss = vec![s1, s2, s3];

    let steps = &b
        .iter()
        .map(|l| l.split(' ').collect_vec())
        .map(|l| {
            (
                l[1].parse().unwrap(),
                l[3].parse().unwrap(),
                l[5].parse().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();
    // dbg!(&steps);

    // flytta crates
    //   >1 innebär separata steg!

    for (count, from, to) in steps {
        let mut temp = Vec::new();
        for i in 0..*count {
            // dbg!((count, from, to));
            let t = stacks[from - 1].pop().unwrap();
            temp.push(t);
        }
        // dbg!(&temp);
        // temp.reverse();
        // dbg!(&temp);
        for i in 0..*count {
            // dbg!((count, from, to));
            let t = temp.pop().unwrap();

            stacks[to - 1].push(t);
        }
    }
    dbg!(&stacks);

    let top = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    dbg!(&top);

    top
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), "CMZ".to_owned());
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), "LJSVLTWQM".to_owned());
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), "MCD".to_owned());
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), "BRQWDBBJM".to_owned());
    }
}
