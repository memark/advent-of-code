#![allow(dead_code, unused_imports, unused_variables)]

mod state;
use state::State;

mod instruction;
use instruction::Instruction::{ self, * };

mod parameter;
use parameter::Parameter::{ self, * };

use itertools::Itertools;
use std::{ num::ParseIntError, fs, process::Output };

type Int = i32;

fn main() {
    println!("Part 1: {:?}", part1());
    // println!("Part 2: {:?}", part2());
}

fn part1() -> Int {
    let file = fs::read_to_string("input.txt").unwrap();
    let mem = parse_ints(&file);
    let input = vec![1];

    *run_program(State::with_input(mem, input)).output.last().unwrap()
}

fn part2() -> Int {
    0
}

pub fn parse_ints(s: &str) -> Vec<Int> {
    if s.is_empty() {
        vec![]
    } else {
        s.split(',')
            .map(|ss| ss.trim().parse().unwrap())
            .collect_vec()
    }
}

fn run_program(mut state: State) -> State {
    let mut ip = 0;

    loop {
        let (i, ip_delta) = instruction::Instruction::from_ints(&state.mem[ip..]);
        if i == Halt {
            break;
        }
        state = i.process(state);
        ip += ip_delta as usize;
    }
    state
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn parses_ints_with_newlines() {
        let input = "1,9,10,3,
        2,3,11,0,
        99,
        30,40,50";

        let actual = parse_ints(input);

        assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], actual);
    }

    #[rstest]
    fn parses_ints_empty() {
        let input = "";

        let actual = parse_ints(input);

        assert_eq!(vec![0; 0], actual);
    }

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50")]
    #[case("1,0,0,0,99", "2,0,0,0,99")]
    #[case("2,3,0,3,99", "2,3,0,6,99")]
    #[case("2,4,4,5,99,0", "2,4,4,5,99,9801")]
    #[case("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")]
    #[case("1002,4,3,4,33", "1002,4,3,4,99")]
    #[case("1101,100,-1,4,0", "1101,100,-1,4,99")]
    fn runs_program_with_mem(#[case] mem: &str, #[case] expected_mem: &str) {
        let actual = run_program(State::from_mem(parse_ints(mem)))
            .mem.iter()
            .join(",");

        assert_eq!(actual, expected_mem)
    }

    #[rstest]
    #[case("3,0,4,0,99", "123", "123")]
    fn runs_program_with_io(#[case] mem: &str, #[case] input: &str, #[case] expected_output: &str) {
        let actual = run_program(State::with_input(parse_ints(mem), parse_ints(input)))
            .output.iter()
            .join(",");

        assert_eq!(actual, expected_output)
    }

    #[test]
    fn runs_part1() {
        assert_eq!(5346030, part1());
    }

    // #[test]
    // fn runs_part2() {
    //     assert_eq!(513116, part2());
    // }
}