use intcode_computer::{memory::Memory, program::run_program, state::State, Int};
use itertools::iproduct;

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

fn part1() -> Int {
    let file = include_str!("../input.txt");
    let mut memory = Memory::parse(file);

    memory.set(1, 12);
    memory.set(2, 2);

    run_program(State::from_memory(memory)).memory.get(0)
}

fn part2() -> Int {
    let file = include_str!("../input.txt");
    let orig_memory = Memory::parse(file);
    let target = 19690720;

    iproduct!(0..=99, 0..=99)
        .find_map(|(noun, verb)| {
            let mut memory = orig_memory.clone();

            memory.set(1, noun);
            memory.set(2, verb);

            let output = run_program(State::from_memory(memory)).memory.get(0);

            if output == target {
                Some(100 * noun + verb)
            } else {
                None
            }
        })
        .expect("No solution found")
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn runs_part1() {
        assert_eq!(3895705, part1());
    }

    #[test]
    fn runs_part2() {
        assert_eq!(6417, part2());
    }
}
