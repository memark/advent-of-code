use crate::state::State;

pub fn run_program(state: State) -> State {
    let mut state = state;
    loop {
        state = state.process_one_instruction();
        if state.halted {
            break;
        }
    }
    state
}

pub fn run_program_one_output(state: State) -> State {
    let output_len = state.output.len();
    let mut state = state;
    loop {
        state = state.process_one_instruction();
        // println!("{} - {}", state.output.len(), output_len);
        if state.output.len() > output_len {
            break;
        }
        if state.halted {
            break;
        }
    }
    state
}

pub fn run_program_n_output(state: State, n: i32) -> State {
    let mut state = state;
    // Is this a fold?
    for _ in 0..n {
        state = run_program_one_output(state);
    }
    state
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::input::Input;
    use crate::memory::Memory;

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50")]
    #[case("1,0,0,0,99", "2,0,0,0,99")]
    #[case("2,3,0,3,99", "2,3,0,6,99")]
    #[case("2,4,4,5,99,0", "2,4,4,5,99,9801")]
    #[case("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")]
    #[case("1002,4,3,4,33", "1002,4,3,4,99")]
    #[case("1101,100,-1,4,0", "1101,100,-1,4,99")]
    fn runs_program_with_mem(#[case] memory: &str, #[case] expected_memory: &str) {
        let actual = run_program(State::from_memory(Memory::parse(memory)));

        assert_eq!(actual.memory, Memory::parse(expected_memory))
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
    fn runs_program_with_io(
        #[case] memory: &str,
        #[case] input: &str,
        #[case] expected_output: &str,
    ) {
        let actual = run_program(State::with_input(
            Memory::parse(memory),
            Input::parse(input),
        ));

        assert_eq!(actual.output.iter().join(","), expected_output)
    }
}
