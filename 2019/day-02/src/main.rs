use intcode_computer::{ ints_to_hashmap, parse_ints, program::run_program, Int, state::State };
use itertools::Itertools;

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

fn part1() -> Int {
    let file = include_str!("../input.txt");
    let mut mem = ints_to_hashmap(parse_ints(file));

    mem.insert(1, 12);
    mem.insert(2, 2);

    run_program(State::from_mem(mem)).mem[&0]
}

fn part2() -> Int {
    let file = include_str!("../input.txt");
    let orig_mem = ints_to_hashmap(parse_ints(file));
    let target = 19690720;

    (0..=99)
        .cartesian_product(0..=99)
        .find_map(|(noun, verb)| {
            let mut mem = orig_mem.clone();

            mem.insert(1, noun);
            mem.insert(2, verb);

            let output = run_program(State::from_mem(mem)).mem[&0];

            if output == target {
                Some(100 * noun + verb)
            } else {
                None
            }
        })
        .unwrap()
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