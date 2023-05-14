use clap::Parser;
use intcode_computer::{ program::run_program, state::{ State, Memory, Input } };
use itertools::Itertools;

#[derive(Parser, Debug)]
struct Args {
    /// memory (instructions and data) as a comma-separated list of ints, e.g. "3,0,4,0,99"
    #[arg()]
    memory: String,

    /// input as a comma-separated list of ints, e.g. "1,2,3"
    #[arg()]
    input: String,
}

fn main() {
    let args = Args::parse();

    let memory = Memory::parse(&args.memory);
    let input = Input::parse(&args.input);

    let state = State::with_input(memory, input);
    let output = run_program(state).output.iter().join(",");

    println!("{output}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_clap_config() {
        use clap::CommandFactory;

        Args::command().debug_assert()
    }
}