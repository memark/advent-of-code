#![allow(dead_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::{ num::ParseIntError, fs, process::Output };

type Int = i32;

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

fn part1() -> Int {
    0
}

fn part2() -> Int {
    0
}

fn parse_ints(s: &str) -> Vec<Int> {
    if s.is_empty() {
        vec![]
    } else {
        s.split(',')
            .map(|ss| ss.trim().parse().unwrap())
            .collect_vec()
    }
}

fn run_program(mut mem: Vec<Int>) -> Vec<Int> {
    let mut ip = 0;
    loop {
        let (i, ip_delta) = Instruction::from_ints(&mem[ip..]);
        if i == Halt {
            break;
        }
        mem = i.process(mem);
        ip += ip_delta as usize;
    }
    mem
}

fn run_program_v2(mut mem: Vec<Int>) -> Vec<Int> {
    let mut ip = 0;
    loop {
        let (i, ip_delta) = Instruction::from_ints_v2(&mem[ip..]);
        if i == Halt || i == HaltV2 {
            break;
        }
        mem = i.process(mem);
        ip += ip_delta as usize;
    }
    mem
}

fn run_program_with_io(mut mem: Vec<Int>, mut input: Vec<Int>) -> Vec<Int> {
    let mut output = vec![];
    let mut ip = 0;
    loop {
        let (i, ip_delta) = Instruction::from_ints(&mem[ip..]);
        if i == Halt {
            break;
        }
        (mem, input, output) = i.process_with_io(mem, input, output);
        ip += ip_delta as usize;
    }
    output
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add {
        src1: Int,
        src2: Int,
        dst: Int,
    },
    AddV2 {
        src1: Parameter,
        src2: Parameter,
        dst: Parameter,
    },
    Multiply {
        src1: Int,
        src2: Int,
        dst: Int,
    },
    MultiplyV2 {
        src1: Parameter,
        src2: Parameter,
        dst: Parameter,
    },
    Input {
        dst: Int,
    },
    InputV2 {
        dst: Parameter,
    },
    Output {
        src: Int,
    },
    OutputV2 {
        src: Parameter,
    },
    Halt,
    HaltV2,
}
use Instruction::*;

impl Instruction {
    fn from_ints(ints: &[Int]) -> (Self, Int) {
        let (opcode, mode1, mode2, mode3) = get_modes(ints[0]);
        match opcode {
            1 => (Self::Add { src1: ints[1], src2: ints[2], dst: ints[3] }, 4),
            2 => (Self::Multiply { src1: ints[1], src2: ints[2], dst: ints[3] }, 4),
            3 => (Self::Input { dst: ints[1] }, 2),
            4 => (Self::Output { src: ints[1] }, 2),
            99 => (Self::Halt, 1),
            _ => panic!("Unknown opcode"),
        }
    }

    fn from_ints_v2(ints: &[Int]) -> (Self, Int) {
        let (opcode, mode1, mode2, mode3) = get_modes(ints[0]);

        let get_p1 = || { get_p(mode1, ints[1]) };
        let get_p2 = || { get_p(mode2, ints[2]) };
        let get_p3 = || { get_p(mode3, ints[3]) };

        match opcode {
            1 => { (Self::AddV2 { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            2 => { (Self::MultiplyV2 { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            3 => { (Self::InputV2 { dst: get_p1() }, 2) }
            4 => { (Self::OutputV2 { src: get_p1() }, 2) }
            99 => { (Self::HaltV2 {}, 1) }
            _ => panic!("Unknown opcode"),
        }
    }

    fn process(self, mut mem: Vec<Int>) -> Vec<Int> {
        match self {
            Self::Add { src1, src2, dst } => {
                mem[dst as usize] = mem[src1 as usize] + mem[src2 as usize];
            }
            Self::AddV2 { src1, src2, dst } => {
                let src1_value = match src1 {
                    Position(p) => mem[p as usize],
                    Immediate(i) => i,
                };
                let src2_value = match src2 {
                    Position(p) => mem[p as usize],
                    Immediate(i) => i,
                };
                let dst = match dst {
                    Position(p) => p,
                    Immediate(_) => panic!(),
                };
                mem[dst as usize] = src1_value + src2_value;
            }
            Self::Multiply { src1, src2, dst } => {
                mem[dst as usize] = mem[src1 as usize] * mem[src2 as usize];
            }
            Self::MultiplyV2 { src1, src2, dst } => {
                let src1_value = match src1 {
                    Position(p) => mem[p as usize],
                    Immediate(i) => i,
                };
                let src2_value = match src2 {
                    Position(p) => mem[p as usize],
                    Immediate(i) => i,
                };
                let dst = match dst {
                    Position(p) => p,
                    Immediate(_) => panic!(),
                };
                mem[dst as usize] = src1_value * src2_value;
            }
            Self::Halt => {}
            _ => unimplemented!(),
        }
        mem
    }

    fn process_with_io(
        self,
        mut mem: Vec<Int>,
        mut input: Vec<Int>,
        mut output: Vec<Int>
    ) -> (Vec<Int>, Vec<Int>, Vec<Int>) {
        match self {
            Self::Add { src1, src2, dst } => {
                mem[dst as usize] = mem[src1 as usize] + mem[src2 as usize];
            }
            Self::Multiply { src1, src2, dst } => {
                mem[dst as usize] = mem[src1 as usize] * mem[src2 as usize];
            }
            Self::Input { dst } => {
                mem[dst as usize] = input.remove(0);
            }
            Self::Output { src } => output.push(mem[src as usize]),
            Self::Halt => {}
            _ => unimplemented!(),
        }
        (mem, input, output)
    }
}

#[derive(Debug, PartialEq)]
enum Parameter {
    Position(Int),
    Immediate(Int),
}
use Parameter::*;

fn get_p(mode: Int, value: Int) -> Parameter {
    match mode {
        0 => Parameter::Position(value),
        1 => Parameter::Immediate(value),
        _ => unimplemented!(),
    }
}

fn get_modes(int: Int) -> (Int, Int, Int, Int) {
    (int % 100, (int / 100) % 10, (int / 1000) % 10, (int / 10000) % 10)
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
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", (Instruction::Add { src1: 9, src2: 10, dst: 3 }, 4))]
    #[case("2,3,11,0,99,30,40,50", (Instruction::Multiply { src1: 3, src2: 11, dst: 0 }, 4))]
    #[case("3,50", (Instruction::Input { dst: 50 }, 2))]
    #[case("4,50", (Instruction::Output { src: 50 }, 2))]
    #[case("99,30,40,50", (Instruction::Halt, 1))]
    fn parses_instruction(#[case] input: &str, #[case] expected: (Instruction, Int)) {
        assert_eq!(expected, Instruction::from_ints(&parse_ints(input)));
    }

    #[rstest]
    #[case("1002,4,3,4,33", (
        Instruction::MultiplyV2 {
            src1: Parameter::Position(4),
            src2: Parameter::Immediate(3),
            dst: Parameter::Position(4),
        },
        4,
    ))]
    fn parses_instruction_v2(#[case] input: &str, #[case] expected: (Instruction, Int)) {
        assert_eq!(expected, Instruction::from_ints_v2(&parse_ints(input)));
    }

    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn panics_on_unknown_opcode() {
        let input = "123,2,3,11,0,99,30,40,50";

        let actual = Instruction::from_ints(&parse_ints(input));
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
    #[case(
        Instruction::MultiplyV2 {
            src1: Parameter::Position(4),
            src2: Parameter::Immediate(3),
            dst: Parameter::Position(4),
        },
        "1002,4,3,4,33",
        "1002,4,3,4,99"
    )]
    fn processes_instruction(
        #[case] instruction: Instruction,
        #[case] mem: &str,
        #[case] expected: &str
    ) {
        assert_eq!(expected, instruction.process(parse_ints(mem)).iter().join(","));
    }

    #[rstest]
    #[case(Instruction::Input { dst: 0 }, "0", "123", "123", "", "")]
    #[case(Instruction::Output { src: 0 }, "123", "", "123", "", "123")]
    fn processes_instruction_with_io(
        #[case] instruction: Instruction,
        #[case] mem: &str,
        #[case] input: &str,
        #[case] expected_mem: &str,
        #[case] expected_input: &str,
        #[case] expected_output: &str
    ) {
        let (actual_mem, actual_input, actual_output) = instruction.process_with_io(
            parse_ints(mem),
            parse_ints(input),
            vec![]
        );

        assert_eq!(actual_mem.iter().join(","), expected_mem);
        assert_eq!(actual_input.iter().join(","), expected_input);
        assert_eq!(actual_output.iter().join(","), expected_output);
    }

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50")]
    #[case("1,0,0,0,99", "2,0,0,0,99")]
    #[case("2,3,0,3,99", "2,3,0,6,99")]
    #[case("2,4,4,5,99,0", "2,4,4,5,99,9801")]
    #[case("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")]
    fn runs_program(#[case] input: &str, #[case] expected: &str) {
        let actual = run_program(parse_ints(input)).iter().join(",");

        assert_eq!(actual, expected)
    }

    #[rstest]
    #[case("1002,4,3,4,33", "1002,4,3,4,99")]
    #[case("1101,100,-1,4,0", "1101,100,-1,4,99")]
    fn runs_program_v2(#[case] input: &str, #[case] expected: &str) {
        let actual = run_program_v2(parse_ints(input)).iter().join(",");

        assert_eq!(actual, expected)
    }

    //

    #[rstest]
    #[case("3,0,4,0,99", "123", "123")]
    fn runs_program_with_io(#[case] mem: &str, #[case] input: &str, #[case] expected: &str) {
        let actual = run_program_with_io(parse_ints(mem), parse_ints(input)).iter().join(",");

        assert_eq!(actual, expected)
    }

    #[rstest]
    #[case(1002, (2, 0, 1, 0))]
    #[case(11122, (22, 1, 1, 1))] // My own
    #[case(22, (22, 0, 0, 0))] // My own
    fn gets_modes(#[case] input: Int, #[case] expected: (Int, Int, Int, Int)) {
        let actual = get_modes(input);

        assert_eq!(actual, expected)
    }

    // #[test]
    // fn runs_part1() {
    //     assert_eq!(5346030, part1());
    // }

    // #[test]
    // fn runs_part2() {
    //     assert_eq!(513116, part2());
    // }
}