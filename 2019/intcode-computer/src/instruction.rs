use crate::Int;
use crate::parameter::Parameter;
use crate::state::{ Memory, get_modes };

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
    pub fn from_memory_and_ip(memory: &Memory, ip: Int) -> Self {
        let (opcode, mode1, mode2, mode3) = get_modes(memory.0[&ip]);

        let get_p1 = || Parameter::create(mode1, memory.0[&(ip + 1)]);
        let get_p2 = || Parameter::create(mode2, memory.0[&(ip + 2)]);
        let get_p3 = || Parameter::create(mode3, memory.0[&(ip + 3)]);

        match opcode {
            1 =>
                Instruction::Add {
                    src1: get_p1(),
                    src2: get_p2(),
                    dst: get_p3(),
                },
            2 =>
                Instruction::Multiply {
                    src1: get_p1(),
                    src2: get_p2(),
                    dst: get_p3(),
                },
            3 => Instruction::Input { dst: get_p1() },
            4 => Instruction::Output { src: get_p1() },
            5 =>
                Instruction::JumpIfTrue {
                    src: get_p1(),
                    dst: get_p2(),
                },
            6 =>
                Instruction::JumpIfFalse {
                    src: get_p1(),
                    dst: get_p2(),
                },
            7 =>
                Instruction::LessThan {
                    src1: get_p1(),
                    src2: get_p2(),
                    dst: get_p3(),
                },
            8 =>
                Instruction::Equals {
                    src1: get_p1(),
                    src2: get_p2(),
                    dst: get_p3(),
                },
            9 => SetRelativeBase { src: get_p1() },
            99 => Halt {},
            _ => panic!("Unknown opcode"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::parameter::Parameter::*;

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", Add {
        src1: Position(9),
        src2: Position(10),
        dst: Position(3),
    })]
    #[case("2,3,11,0,99,30,40,50", Multiply {
        src1: Position(3),
        src2: Position(11),
        dst: Position(0),
    })]
    #[case("3,50", Instruction::Input { dst: Position(50) })]
    #[case("4,50", Instruction::Output { src: Position(50) })]
    #[case("99,30,40,50", Halt)]
    #[case("1002,4,3,4,33", Multiply {
        src1: Position(4),
        src2: Immediate(3),
        dst: Position(4),
    })]
    #[case("1005,4,3", JumpIfTrue {
        src: Position(4),
        dst: Immediate(3),
    })]
    #[case("1006,4,3", JumpIfFalse {
        src: Position(4),
        dst: Immediate(3),
    })]
    #[case("1007,4,3,4", LessThan { src1: Position(4), src2: Immediate(3), dst: Position(4) })]
    #[case("1008,4,3,4", Equals { src1: Position(4), src2: Immediate(3), dst: Position(4) })]
    fn parses_instruction(#[case] memory: &str, #[case] expected: Instruction) {
        assert_eq!(expected, Instruction::from_memory_and_ip(&Memory::parse(memory), 0));
    }

    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn panics_on_unknown_opcode() {
        let mem = "123,2,3,11,0,99,30,40,50";

        Instruction::from_memory_and_ip(&Memory::parse(mem), 0);
    }
}