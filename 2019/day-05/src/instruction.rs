use super::Int;

use crate::state::State;
use crate::parameter::Parameter::{ self, * };
use crate::parse_ints;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add {
        src1: Parameter,
        src2: Parameter,
        dst: Parameter,
    },
    Multiply {
        src1: Parameter,
        src2: Parameter,
        dst: Parameter,
    },
    Input {
        dst: Parameter,
    },
    Output {
        src: Parameter,
    },
    LessThan {
        src1: Parameter,
        src2: Parameter,
        dst: Parameter,
    },
    Equals {
        src1: Parameter,
        src2: Parameter,
        dst: Parameter,
    },
    Halt,
}
use Instruction::*;

impl Instruction {
    pub fn from_ints(ints: &[Int]) -> (Self, Int) {
        let (opcode, mode1, mode2, mode3) = get_modes(ints[0]);

        let get_p1 = || { Parameter::create(mode1, ints[1]) };
        let get_p2 = || { Parameter::create(mode2, ints[2]) };
        let get_p3 = || { Parameter::create(mode3, ints[3]) };

        match opcode {
            1 => { (Add { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            2 => { (Multiply { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            3 => { (Input { dst: get_p1() }, 2) }
            4 => { (Instruction::Output { src: get_p1() }, 2) }
            7 => { (LessThan { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            8 => { (Equals { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            99 => { (Halt {}, 1) }
            _ => panic!("Unknown opcode"),
        }
    }

    pub fn process(self, mut state: State) -> State {
        println!("Executing {:?}", self);

        match self {
            Self::Add { src1, src2, dst } => {
                let src1_value = match src1 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let src2_value = match src2 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let dst = match dst {
                    Position(p) => p,
                    Immediate(_) => panic!(),
                };
                state.mem[dst as usize] = src1_value + src2_value;
            }
            Self::Multiply { src1, src2, dst } => {
                let src1_value = match src1 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let src2_value = match src2 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let dst = match dst {
                    Position(p) => p,
                    Immediate(_) => panic!(),
                };
                state.mem[dst as usize] = src1_value * src2_value;
            }
            Self::Input { dst } => {
                let dst = match dst {
                    Position(p) => p,
                    Immediate(_) => panic!(),
                };
                state.mem[dst as usize] = state.input.remove(0);
            }
            Self::Output { src } => {
                let src_value = match src {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                println!("Outputting {}", src_value);
                state.output.push(src_value);
            }
            Self::LessThan { src1, src2, dst } => {
                let src1_value = match src1 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let src2_value = match src2 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let dst = match dst {
                    Position(p) => p,
                    Immediate(_) => panic!(),
                };
                state.mem[dst as usize] = if src1_value < src2_value { 1 } else { 0 };
            }
            Self::Equals { src1, src2, dst } => {
                let src1_value = match src1 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let src2_value = match src2 {
                    Position(p) => state.mem[p as usize],
                    Immediate(i) => i,
                };
                let dst = match dst {
                    Position(p) => p,
                    Immediate(_) => panic!(),
                };
                state.mem[dst as usize] = if src1_value == src2_value { 1 } else { 0 };
            }
            Self::Halt => {}
            #[allow(unreachable_patterns)]
            _ => unimplemented!("{self:?}"),
        }
        state
    }
}

pub fn get_modes(int: Int) -> (Int, Int, Int, Int) {
    (int % 100, (int / 100) % 10, (int / 1000) % 10, (int / 10000) % 10)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", (
        Add {
            src1: Position(9),
            src2: Position(10),
            dst: Position(3),
        },
        4,
    ))]
    #[case("2,3,11,0,99,30,40,50", (
        Multiply {
            src1: Position(3),
            src2: Position(11),
            dst: Position(0),
        },
        4,
    ))]
    #[case("3,50", (Input { dst: Position(50) }, 2))]
    #[case("4,50", (Instruction::Output { src: Position(50) }, 2))]
    #[case("99,30,40,50", (Halt, 1))]
    #[case("1002,4,3,4,33", (
        Multiply {
            src1: Position(4),
            src2: Immediate(3),
            dst: Position(4),
        },
        4,
    ))]
    fn parses_instruction(#[case] input: &str, #[case] expected: (Instruction, Int)) {
        assert_eq!(expected, Instruction::from_ints(&parse_ints(input)));
    }

    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn panics_on_unknown_opcode() {
        let input = "123,2,3,11,0,99,30,40,50";

        let actual = Instruction::from_ints(&parse_ints(input));
    }

    #[rstest]
    #[case(
        Add {
            src1: Position(9),
            src2: Position(10),
            dst: Position(3),
        },
        "1,9,10,3,2,3,11,0,99,30,40,50",
        "1,9,10,70,2,3,11,0,99,30,40,50"
    )]
    #[case(
        Multiply {
            src1: Position(3),
            src2: Position(11),
            dst: Position(0),
        },
        "1,9,10,70,2,3,11,0,99,30,40,50",
        "3500,9,10,70,2,3,11,0,99,30,40,50"
    )]
    #[case(
        Multiply {
            src1: Position(4),
            src2: Immediate(3),
            dst: Position(4),
        },
        "1002,4,3,4,33",
        "1002,4,3,4,99"
    )]
    fn processes_instruction_with_mem(
        #[case] instruction: Instruction,
        #[case] mem: &str,
        #[case] expected: &str
    ) {
        assert_eq!(
            expected,
            instruction
                .process(State::from_mem(parse_ints(mem)))
                .mem.iter()
                .join(",")
        );
    }

    #[rstest]
    #[case(Input { dst: Position(0) }, "0", "123", "123", "", "")]
    #[case(Instruction::Output { src: Position(0) }, "123", "", "123", "", "123")]
    fn processes_instruction_with_io(
        #[case] instruction: Instruction,
        #[case] mem: &str,
        #[case] input: &str,
        #[case] expected_mem: &str,
        #[case] expected_input: &str,
        #[case] expected_output: &str
    ) {
        let state = instruction.process(State::with_input(parse_ints(mem), parse_ints(input)));

        assert_eq!(state.mem.iter().join(","), expected_mem);
        assert_eq!(state.input.iter().join(","), expected_input);
        assert_eq!(state.output.iter().join(","), expected_output);
    }

    #[rstest]
    #[case(1002, (2, 0, 1, 0))]
    #[case(11122, (22, 1, 1, 1))] // My own
    #[case(22, (22, 0, 0, 0))] // My own
    fn gets_modes(#[case] input: Int, #[case] expected: (Int, Int, Int, Int)) {
        let actual = get_modes(input);

        assert_eq!(actual, expected)
    }
}