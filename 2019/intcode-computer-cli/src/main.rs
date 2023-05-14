use clap::Parser;
use intcode_computer::{ run_program, state::State, ints_to_hashmap, parse_ints };

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

    let memory = ints_to_hashmap(parse_ints(&args.memory));
    let input = parse_ints(&args.input);

    let state = State::with_input(memory, input);
    let output = run_program(state).output;

    println!("{output:?}");
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