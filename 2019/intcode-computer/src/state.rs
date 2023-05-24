use std::collections::HashMap;

use itertools::Itertools;

use crate::instruction::Instruction;
use crate::Int;

#[derive(Debug, PartialEq)]
pub struct State {
    pub memory: Memory,
    pub input: Input,
    pub output: Vec<Int>,
    pub ip: Int,
    pub rb: Int,
}

impl State {
    pub fn from_memory(memory: Memory) -> Self {
        State { memory, input: Input::default(), output: vec![], ip: 0, rb: 0 }
    }

    pub fn with_input(memory: Memory, input: Input) -> Self {
        State { memory, input, output: vec![], ip: 0, rb: 0 }
    }

    pub fn process(self, instruction: Instruction) -> ProcessResult {
        let mut state = self;
        match instruction {
            Instruction::Add { src1, src2, dst } => {
                state.memory.0.insert(dst.eval_pos(&state), src1.eval(&state) + src2.eval(&state));
                ProcessResult::proceed(state)
            }
            Instruction::Multiply { src1, src2, dst } => {
                state.memory.0.insert(dst.eval_pos(&state), src1.eval(&state) * src2.eval(&state));
                ProcessResult::proceed(state)
            }
            Instruction::Input { dst } => {
                state.memory.0.insert(dst.eval_pos(&state), state.input.0.remove(0));
                ProcessResult::proceed(state)
            }
            Instruction::Output { src } => {
                state.output.push(src.eval(&state));
                ProcessResult::proceed(state)
            }
            Instruction::JumpIfTrue { src, dst } => {
                let new_ip = if src.eval(&state) != 0 { Some(dst.eval(&state)) } else { None };
                ProcessResult::jump(state, new_ip)
            }
            Instruction::JumpIfFalse { src, dst } => {
                let new_ip = if src.eval(&state) == 0 { Some(dst.eval(&state)) } else { None };
                ProcessResult::jump(state, new_ip)
            }
            Instruction::LessThan { src1, src2, dst } => {
                state.memory.0.insert(dst.eval_pos(&state), if
                    src1.eval(&state) < src2.eval(&state)
                {
                    1
                } else {
                    0
                });
                ProcessResult::proceed(state)
            }
            Instruction::Equals { src1, src2, dst } => {
                state.memory.0.insert(dst.eval_pos(&state), if
                    src1.eval(&state) == src2.eval(&state)
                {
                    1
                } else {
                    0
                });
                ProcessResult::proceed(state)
            }
            Instruction::SetRelativeBase { src } => {
                state.rb += src.eval(&state);
                ProcessResult::proceed(state)
            }
            Instruction::Halt => ProcessResult::halt(state),

            #[allow(unreachable_patterns)]
            _ => unimplemented!("{instruction:?}"),
        }
    }
}

pub struct ProcessResult {
    pub state: State,
    pub new_ip: Option<Int>,
    pub halted: bool,
}

impl ProcessResult {
    fn proceed(state: State) -> Self {
        ProcessResult { state, new_ip: None, halted: false }
    }

    fn jump(state: State, new_ip: Option<Int>) -> Self {
        ProcessResult { state, new_ip, halted: false }
    }

    fn halt(state: State) -> Self {
        ProcessResult { state, new_ip: None, halted: true }
    }
}

pub fn get_modes(int: Int) -> (Int, Int, Int, Int) {
    (int % 100, (int / 100) % 10, (int / 1000) % 10, (int / 10000) % 10)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Memory(pub HashMap<Int, Int>);

impl Memory {
    pub fn parse(s: &str) -> Self {
        Self(ints_to_hashmap(parse_ints(s)))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Input(pub Vec<Int>);

impl Input {
    pub fn parse(s: &str) -> Self {
        Self(parse_ints(s))
    }
}

fn ints_to_hashmap(ints: Vec<Int>) -> HashMap<Int, Int> {
    ints.into_iter()
        .enumerate()
        .map(|(i, x)| (i as Int, x))
        .collect()
}

fn parse_ints(s: &str) -> Vec<Int> {
    if s.is_empty() {
        vec![]
    } else {
        s.split(',')
            .map(|ss| ss.trim().parse().expect("Unable to parse int"))
            .collect_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use itertools::Itertools;

    use crate::instruction::Instruction::*;
    use crate::parameter::Parameter::*;
    use crate::state::Input;

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
        #[case] memory: &str,
        #[case] expected: &str
    ) {
        assert_eq!(
            Memory::parse(expected),
            State::from_memory(Memory::parse(memory)).process(instruction).state.memory
        );
    }

    #[rstest]
    #[case(Instruction::Input { dst: Position(0) }, "0", "123", "123", "", "")]
    #[case(Instruction::Output { src: Position(0) }, "123", "", "123", "", "123")]
    fn processes_instruction_with_io(
        #[case] instruction: Instruction,
        #[case] memory: &str,
        #[case] input: &str,
        #[case] expected_memory: &str,
        #[case] expected_input: &str,
        #[case] expected_output: &str
    ) {
        let actual = State::with_input(Memory::parse(memory), Input::parse(input)).process(
            instruction
        );

        assert_eq!(actual.state.memory, Memory::parse(expected_memory));
        assert_eq!(actual.state.input.0.iter().join(","), expected_input);
        assert_eq!(actual.state.output.iter().join(","), expected_output);
    }

    #[rstest]
    #[case(1002, (2, 0, 1, 0))]
    #[case(11122, (22, 1, 1, 1))]
    #[case(22, (22, 0, 0, 0))]
    fn gets_modes(#[case] input: Int, #[case] expected: (Int, Int, Int, Int)) {
        assert_eq!(get_modes(input), expected)
    }

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
}