#![allow(dead_code, unused_imports, unused_variables)]

mod state;
use state::State;

mod instruction;
use instruction::Instruction::{ self, * };

mod parameter;
use parameter::Parameter::{ self, * };

use itertools::Itertools;
use std::collections::HashMap;
use std::{ num::ParseIntError, fs, process::Output };

type Int = i64;

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

fn part1() -> Int {
    let file = fs::read_to_string("input.txt").unwrap();
    let mem = ints_to_hashmap(parse_ints(&file));
    let input = vec![1];

    *run_program(State::with_input(mem, input)).output.last().unwrap()
}

fn part2() -> Int {
    let file = fs::read_to_string("input.txt").unwrap();
    let mem = ints_to_hashmap(parse_ints(&file));
    let input = vec![2];

    *run_program(State::with_input(mem, input)).output.last().unwrap()
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

pub fn ints_to_hashmap(mem: Vec<Int>) -> HashMap<Int, Int> {
    mem.into_iter()
        .enumerate()
        .map(|(i, x)| (i as Int, x))
        .collect()
}

fn run_program(mut state: State) -> State {
    let mut ip = 0;

    loop {
        let (i, ip_delta) = instruction::Instruction::from_mem_and_ip(&state.mem, ip);
        if i == Halt {
            break;
        }
        let result = i.process(state);
        state = result.state;
        if let Some(new_ip) = result.new_ip {
            ip = new_ip;
        } else {
            ip += ip_delta;
        }
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
        let actual = run_program(State::from_mem(ints_to_hashmap(parse_ints(mem)))).mem;

        assert_eq!(actual, ints_to_hashmap(parse_ints(expected_mem)))
    }

    #[rstest]
    #[case("3,0,4,0,99", "123", "123")]
    // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    #[case("3,9,8,9,10,9,4,9,99,-1,8", "8", "1")]
    #[case("3,9,8,9,10,9,4,9,99,-1,8", "9", "0")]
    // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    #[case("3,9,7,9,10,9,4,9,99,-1,8", "7", "1")]
    #[case("3,9,7,9,10,9,4,9,99,-1,8", "8", "0")]
    //Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    #[case("3,3,1108,-1,8,3,4,3,99", "8", "1")]
    #[case("3,3,1108,-1,8,3,4,3,99", "9", "0")]
    // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    #[case("3,3,1107,-1,8,3,4,3,99", "7", "1")]
    #[case("3,3,1107,-1,8,3,4,3,99", "8", "0")]
    // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero.
    //   (using position mode)
    #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", "0", "0")]
    #[case("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", "42", "1")]
    //   (using immediate mode)
    #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", "0", "0")]
    #[case("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", "42", "1")]
    // The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.
    #[case(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        "7",
        "999"
    )]
    #[case(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        "8",
        "1000"
    )]
    #[case(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        "9",
        "1001"
    )]
    #[case(
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
        "",
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
    )]
    #[case("1102,34915192,34915192,7,4,7,99,0", "", "1219070632396864")]
    #[case("104,1125899906842624,99", "", "1125899906842624")]
    fn runs_program_with_io(#[case] mem: &str, #[case] input: &str, #[case] expected_output: &str) {
        let actual = run_program(
            State::with_input(ints_to_hashmap(parse_ints(mem)), parse_ints(input))
        )
            .output.iter()
            .join(",");

        assert_eq!(actual, expected_output)
    }

    #[test]
    fn runs_part1() {
        assert_eq!(2204990589, part1());
    }

    #[test]
    fn runs_part2() {
        assert_eq!(50008, part2());
    }
}