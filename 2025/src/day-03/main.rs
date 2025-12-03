use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {:?}", solve(Part::Part1, INPUT));
    println!("Part 2: {:?}", solve(Part::Part2, INPUT));
}

enum Part {
    Part1,
    Part2,
}

fn solve(part: Part, data: &str) -> i64 {
    data.split_terminator("\n")
        .map(|row| {
            let n = match part {
                Part::Part1 => 2,
                Part::Part2 => 12,
            };
            (0..n)
                .rev()
                .scan(0, |last_pos, digits_left| {
                    let (p, c) = *row
                        .char_indices()
                        .dropping_back(digits_left)
                        .dropping(*last_pos)
                        .max_set_by_key(|&(_, c)| c)
                        .first()
                        .unwrap();
                    *last_pos = p + 1;
                    Some(c)
                })
                .join("")
                .parse::<i64>()
                .unwrap()
        })
        .sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[rstest]
    #[case::part1_example(Part::Part1, EXAMPLE, 357)]
    #[case::part1_input(Part::Part1, INPUT, 17244)]
    #[case::part2_example(Part::Part2, EXAMPLE, 3121910778619)]
    #[case::part2_input(Part::Part2, INPUT, 171435596092638)]
    fn solves_correctly(#[case] part: Part, #[case] data: &str, #[case] expected: i64) {
        assert_eq!(solve(part, data), expected);
    }
}
