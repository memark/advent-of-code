use intcode_computer::{ Int, state::{ State, Memory, Input }, program::run_program };

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

fn part1() -> Int {
    let file = include_str!("../input.txt");
    let memory = Memory::parse(file);
    let input = Input::parse("1");

    *run_program(State::with_input(memory, input)).output.last().expect("No output found")
}

fn part2() -> Int {
    let file = include_str!("../input.txt");
    let memory = Memory::parse(file);
    let input = Input::parse("2");

    *run_program(State::with_input(memory, input)).output.last().expect("No output found")
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn runs_part1() {
        assert_eq!(2204990589, part1());
    }

    #[test]
    fn runs_part2() {
        assert_eq!(50008, part2());
    }
}