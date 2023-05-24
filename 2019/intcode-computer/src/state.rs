use crate::input::Input;
use crate::instruction::Instruction;
use crate::Int;
use crate::memory::Memory;

#[derive(Debug, PartialEq)]
pub struct State {
    pub memory: Memory,
    pub input: Input,
    pub output: Vec<Int>,
    pub ip: Int,
    pub rb: Int,
    pub halted: bool,
}

impl State {
    pub fn from_memory(memory: Memory) -> Self {
        State { memory, input: Input::default(), output: vec![], ip: 0, rb: 0, halted: false }
    }

    pub fn with_input(memory: Memory, input: Input) -> Self {
        State { memory, input, output: vec![], ip: 0, rb: 0, halted: false }
    }

    pub(crate) fn process_one_instruction(self) -> State {
        let instr = Instruction::from_memory_and_ip(&self.memory, self.ip);
        self.process(instr)
    }

    pub(crate) fn process(self, instruction: Instruction) -> State {
        let mut state = self;
        match instruction {
            Instruction::Add { src1, src2, dst } => {
                state.memory.set(
                    dst.eval_pos(&state),
                    src1.eval_val(&state) + src2.eval_val(&state)
                );
                state.inc_ip(4)
            }
            Instruction::Multiply { src1, src2, dst } => {
                state.memory.set(
                    dst.eval_pos(&state),
                    src1.eval_val(&state) * src2.eval_val(&state)
                );
                state.inc_ip(4)
            }
            Instruction::Input { dst } => {
                state.memory.set(dst.eval_pos(&state), state.input.0.remove(0));
                state.inc_ip(2)
            }
            Instruction::Output { src } => {
                state.output.push(src.eval_val(&state));
                state.inc_ip(2)
            }
            Instruction::JumpIfTrue { src, dst } => {
                let new_ip = if src.eval_val(&state) != 0 {
                    Some(dst.eval_val(&state))
                } else {
                    None
                };
                if let Some(new_ip) = new_ip {
                    state.set_ip(new_ip)
                } else {
                    state.inc_ip(3)
                }
            }
            Instruction::JumpIfFalse { src, dst } => {
                let new_ip = if src.eval_val(&state) == 0 {
                    Some(dst.eval_val(&state))
                } else {
                    None
                };
                if let Some(new_ip) = new_ip {
                    state.set_ip(new_ip)
                } else {
                    state.inc_ip(3)
                }
            }
            Instruction::LessThan { src1, src2, dst } => {
                state.memory.set(dst.eval_pos(&state), if
                    src1.eval_val(&state) < src2.eval_val(&state)
                {
                    1
                } else {
                    0
                });
                state.inc_ip(4)
            }
            Instruction::Equals { src1, src2, dst } => {
                state.memory.set(dst.eval_pos(&state), if
                    src1.eval_val(&state) == src2.eval_val(&state)
                {
                    1
                } else {
                    0
                });
                state.inc_ip(4)
            }
            Instruction::SetRelativeBase { src } => {
                state.rb += src.eval_val(&state);
                state.inc_ip(2)
            }
            Instruction::Halt => state.halted(),

            #[allow(unreachable_patterns)]
            _ => unimplemented!("{instruction:?}"),
        }
    }

    pub fn inc_ip(mut self, ip_inc: Int) -> Self {
        self.ip += ip_inc;
        self
    }

    pub fn set_ip(mut self, new_ip: Int) -> Self {
        self.ip = new_ip;
        self
    }

    pub fn halted(mut self) -> Self {
        self.halted = true;
        self
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
    use itertools::Itertools;

    use crate::instruction::Instruction;
    use crate::parameter::Parameter::*;

    #[rstest]
    #[case(
        Instruction::Add {
            src1: Position(9),
            src2: Position(10),
            dst: Position(3),
        },
        "1,9,10,3,2,3,11,0,99,30,40,50",
        "1,9,10,70,2,3,11,0,99,30,40,50"
    )]
    #[case(
        Instruction::Multiply {
            src1: Position(3),
            src2: Position(11),
            dst: Position(0),
        },
        "1,9,10,70,2,3,11,0,99,30,40,50",
        "3500,9,10,70,2,3,11,0,99,30,40,50"
    )]
    #[case(
        Instruction::Multiply {
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
            State::from_memory(Memory::parse(memory)).process(instruction).memory
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

        assert_eq!(actual.memory, Memory::parse(expected_memory));
        assert_eq!(actual.input.0.iter().join(","), expected_input);
        assert_eq!(actual.output.iter().join(","), expected_output);
    }

    #[rstest]
    #[case(1002, (2, 0, 1, 0))]
    #[case(11122, (22, 1, 1, 1))]
    #[case(22, (22, 0, 0, 0))]
    fn gets_modes(#[case] input: Int, #[case] expected: (Int, Int, Int, Int)) {
        assert_eq!(get_modes(input), expected)
    }
}
