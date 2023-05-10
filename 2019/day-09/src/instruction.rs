use super::Int;

use crate::state::{ State, Mem };
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
    JumpIfTrue {
        src: Parameter,
        dst: Parameter,
    },
    JumpIfFalse {
        src: Parameter,
        dst: Parameter,
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
    SetRelativeBase {
        src: Parameter,
    },
    Halt,
}
use Instruction::*;

impl Instruction {
    pub fn from_mem_and_ip(mem: &Mem, ip: Int) -> (Self, Int) {
        let (opcode, mode1, mode2, mode3) = get_modes(mem[&ip]);

        let get_p1 = || { Parameter::create(mode1, mem[&(ip + 1)]) };
        let get_p2 = || { Parameter::create(mode2, mem[&(ip + 2)]) };
        let get_p3 = || { Parameter::create(mode3, mem[&(ip + 3)]) };

        match opcode {
            1 => { (Add { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            2 => { (Multiply { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            3 => { (Input { dst: get_p1() }, 2) }
            4 => { (Instruction::Output { src: get_p1() }, 2) }
            5 => { (JumpIfTrue { src: get_p1(), dst: get_p2() }, 3) }
            6 => { (JumpIfFalse { src: get_p1(), dst: get_p2() }, 3) }
            7 => { (LessThan { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            8 => { (Equals { src1: get_p1(), src2: get_p2(), dst: get_p3() }, 4) }
            9 => { (SetRelativeBase { src: get_p1() }, 2) }
            99 => { (Halt {}, 1) }
            _ => panic!("Unknown opcode"),
        }
    }

    pub fn process(self, mut state: State) -> ProcessResult {
        match self {
            Self::Add { src1, src2, dst } => {
                state.mem.insert(dst.eval_pos(), src1.eval(&state) + src2.eval(&state));
                ProcessResult::new(state, None)
            }
            Self::Multiply { src1, src2, dst } => {
                state.mem.insert(dst.eval_pos(), src1.eval(&state) * src2.eval(&state));
                ProcessResult::new(state, None)
            }
            Self::Input { dst } => {
                state.mem.insert(dst.eval_pos(), state.input.remove(0));
                ProcessResult::new(state, None)
            }
            Self::Output { src } => {
                state.output.push(src.eval(&state));
                ProcessResult::new(state, None)
            }
            Self::JumpIfTrue { src, dst } => {
                let new_ip = if src.eval(&state) != 0 { Some(dst.eval(&state)) } else { None };
                ProcessResult::new(state, new_ip)
            }
            Self::JumpIfFalse { src, dst } => {
                let new_ip = if src.eval(&state) == 0 { Some(dst.eval(&state)) } else { None };
                ProcessResult::new(state, new_ip)
            }
            Self::LessThan { src1, src2, dst } => {
                state.mem.insert(dst.eval_pos(), if src1.eval(&state) < src2.eval(&state) {
                    1
                } else {
                    0
                });
                ProcessResult::new(state, None)
            }
            Self::Equals { src1, src2, dst } => {
                state.mem.insert(dst.eval_pos(), if src1.eval(&state) == src2.eval(&state) {
                    1
                } else {
                    0
                });
                ProcessResult::new(state, None)
            }
            Self::SetRelativeBase { src } => {
                state.rb += src.eval(&state);
                ProcessResult::new(state, None)
            }
            Self::Halt => { ProcessResult::new(state, None) }

            #[allow(unreachable_patterns)]
            _ => unimplemented!("{self:?}"),
        }
    }
}

pub struct ProcessResult {
    pub state: State,
    pub new_ip: Option<Int>,
}

impl ProcessResult {
    fn new(state: State, new_ip: Option<Int>) -> Self {
        ProcessResult { state, new_ip }
    }
}

pub fn get_modes(int: Int) -> (Int, Int, Int, Int) {
    (int % 100, (int / 100) % 10, (int / 1000) % 10, (int / 10000) % 10)
}

#[cfg(test)]
mod test {
    use crate::ints_to_hashmap;

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
    #[case("1005,4,3", (
        JumpIfTrue {
            src: Position(4),
            dst: Immediate(3),
        },
        3,
    ))]
    #[case("1006,4,3", (
        JumpIfFalse {
            src: Position(4),
            dst: Immediate(3),
        },
        3,
    ))]
    #[case("1007,4,3,4", (LessThan { src1: Position(4), src2: Immediate(3), dst: Position(4) }, 4))]
    #[case("1008,4,3,4", (Equals { src1: Position(4), src2: Immediate(3), dst: Position(4) }, 4))]
    fn parses_instruction(#[case] input: &str, #[case] expected: (Instruction, Int)) {
        assert_eq!(expected, Instruction::from_mem_and_ip(&ints_to_hashmap(parse_ints(input)), 0));
    }

    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn panics_on_unknown_opcode() {
        let input = "123,2,3,11,0,99,30,40,50";

        let actual = Instruction::from_mem_and_ip(&ints_to_hashmap(parse_ints(input)), 0);
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
            ints_to_hashmap(parse_ints(expected)),
            instruction.process(State::from_mem(ints_to_hashmap(parse_ints(mem)))).state.mem
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
        let actual = instruction.process(
            State::with_input(ints_to_hashmap(parse_ints(mem)), parse_ints(input))
        );

        assert_eq!(actual.state.mem, ints_to_hashmap(parse_ints(expected_mem)));
        assert_eq!(actual.state.input.iter().join(","), expected_input);
        assert_eq!(actual.state.output.iter().join(","), expected_output);
    }

    #[rstest]
    #[case(1002, (2, 0, 1, 0))]
    #[case(11122, (22, 1, 1, 1))] // My own
    #[case(22, (22, 0, 0, 0))] // My own
    fn gets_modes(#[case] input: Int, #[case] expected: (Int, Int, Int, Int)) {
        assert_eq!(get_modes(input), expected)
    }
}