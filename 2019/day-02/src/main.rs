#![allow(dead_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::{ num::ParseIntError, fs };

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

fn part1() -> usize {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut mem = parse_memory(&file);

    mem[1] = 12;
    mem[2] = 2;
    mem = run_program(mem);

    mem[0]
}

fn part2() -> usize {
    let file = fs::read_to_string("input.txt").unwrap();
    let orig_mem = parse_memory(&file);
    let target = 19690720;

    (0..=99)
        .cartesian_product(0..=99)
        .find_map(|(noun, verb)| {
            let mut mem = orig_mem.clone();

            mem[1] = noun;
            mem[2] = verb;
            let output = run_program(mem)[0];

            if output == target {
                Some(100 * noun + verb)
            } else {
                None
            }
        })
        .unwrap()
}

fn run_program(mut mem: Vec<usize>) -> Vec<usize> {
    let mut ip = 0;
    loop {
        let (i, ip_delta) = Instruction::from_ints(&mem[ip..]);
        if i == Halt {
            break;
        }
        mem = i.process(mem);
        ip += ip_delta;
    }
    mem
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add {
        src1: usize,
        src2: usize,
        dst: usize,
    },
    Multiply {
        src1: usize,
        src2: usize,
        dst: usize,
    },
    Halt,
}
use Instruction::*;

impl Instruction {
    fn from_ints(ints: &[usize]) -> (Self, usize) {
        match ints[0] {
            1 => (Instruction::Add { src1: ints[1], src2: ints[2], dst: ints[3] }, 4),
            2 => (Instruction::Multiply { src1: ints[1], src2: ints[2], dst: ints[3] }, 4),
            99 => (Instruction::Halt, 1),
            _ => panic!("Unknown opcode"),
        }
    }

    fn process(self, mut mem: Vec<usize>) -> Vec<usize> {
        match self {
            Add { src1, src2, dst } => {
                mem[dst] = mem[src1] + mem[src2];
            }
            Multiply { src1, src2, dst } => {
                mem[dst] = mem[src1] * mem[src2];
            }
            Halt => {}
        }
        mem
    }
}

fn parse_memory(s: &str) -> Vec<usize> {
    s.split(',')
        .map(|ss| ss.trim().parse().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn parses_memory() {
        let input = "1,9,10,3,
        2,3,11,0,
        99,
        30,40,50";

        let actual = parse_memory(input);

        assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], actual);
    }

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", (Instruction::Add { src1: 9, src2: 10, dst: 3 }, 4))]
    #[case("2,3,11,0,99,30,40,50", (Instruction::Multiply { src1: 3, src2: 11, dst: 0 }, 4))]
    #[case("99,30,40,50", (Instruction::Halt, 1))]
    fn parses_opcode(#[case] input: &str, #[case] expected: (Instruction, usize)) {
        assert_eq!(expected, Instruction::from_ints(&parse_memory(input)));
    }

    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn panics_on_unknown_opcode() {
        let input = "123,2,3,11,0,99,30,40,50";

        let actual = Instruction::from_ints(&parse_memory(input));
    }

    #[rstest]
    #[case(
        Instruction::Add {
            src1: 9,
            src2: 10,
            dst: 3,
        },
        "1,9,10,3,2,3,11,0,99,30,40,50",
        "1,9,10,70,2,3,11,0,99,30,40,50"
    )]
    #[case(
        Instruction::Multiply { src1: 3, src2: 11, dst: 0 },
        "1,9,10,70,2,3,11,0,99,30,40,50",
        "3500,9,10,70,2,3,11,0,99,30,40,50"
    )]
    fn processes_opcode(#[case] opcode: Instruction, #[case] mem: &str, #[case] expected: &str) {
        assert_eq!(expected, opcode.process(parse_memory(mem)).iter().join(","));
    }

    // Skapa en memory struct som gör egen parsning?
    // Är det värt det, att kunna klistra in exempel på flera rader?

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50")]
    #[case("1,0,0,0,99", "2,0,0,0,99")]
    #[case("2,3,0,3,99", "2,3,0,6,99")]
    #[case("2,4,4,5,99,0", "2,4,4,5,99,9801")]
    #[case("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")]
    fn runs_program(#[case] input: &str, #[case] expected: &str) {
        let actual = run_program(parse_memory(input)).iter().join(",");

        assert_eq!(actual, expected)
    }

    #[test]
    fn runs_part1() {
        assert_eq!(3895705, part1());
    }

    #[test]
    fn runs_part2() {
        assert_eq!(6417, part2());
    }
}