use itertools::Itertools;
use std::fs;

fn main() {
    let input = file("input");
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> usize {
    solve_with_window_size(input, 4)
}

fn solve_part_2(input: &str) -> usize {
    solve_with_window_size(input, 14)
}

fn solve_with_window_size(input: &str, window_size: usize) -> usize {
    let chars = input.chars().collect_vec();
    let (starting_index, _) = chars
        .windows(window_size)
        .find_position(|w| w.iter().all_unique())
        .unwrap();

    starting_index + window_size
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 1_034);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 2_472);
    }
}
