use intcode_computer::{ ints_to_hashmap, parse_ints, run_program, Int, state::State };

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}

fn part1() -> Int {
    let file = include_str!("../input.txt");
    let mem = ints_to_hashmap(parse_ints(file));
    let input = vec![1];

    *run_program(State::with_input(mem, input)).output.last().unwrap()
}

fn part2() -> Int {
    let file = include_str!("../input.txt");
    let mem = ints_to_hashmap(parse_ints(file));
    let input = vec![2];

    *run_program(State::with_input(mem, input)).output.last().unwrap()
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