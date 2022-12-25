use itertools::{unfold, Itertools};
use std::fs;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
}

fn solve_part_1(input: &str) -> String {
    decimal_to_snafu(input.lines().map(snafu_to_decimal).sum())
}

type Number = i64;

fn snafu_to_decimal(s: &str) -> Number {
    s.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        let n = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        };
        let j = (5 as Number).pow(i as u32);
        acc + (n * j)
    })
}

fn decimal_to_snafu(n: Number) -> String {
    unfold(n, |m| {
        if *m == 0 {
            None
        } else {
            let (c, quot) = match *m % 5 {
                2 => ('2', (*m / 5)),
                1 => ('1', (*m / 5)),
                0 => ('0', (*m / 5)),
                4 => ('-', (*m / 5 + 1)),
                3 => ('=', (*m / 5 + 1)),
                _ => panic!(),
            };
            *m = quot;
            Some(c)
        }
    })
    .collect_vec()
    .iter()
    .rev()
    .collect()
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[trace]
    #[case("1=-0-2", 1747)]
    #[case("12111", 906)]
    #[case("2=0=", 198)]
    #[case("21", 11)]
    #[case("2=01", 201)]
    #[case("111", 31)]
    #[case("20012", 1257)]
    #[case("112", 32)]
    #[case("1=-1=", 353)]
    #[case("1-12", 107)]
    #[case("12", 7)]
    #[case("1=", 3)]
    #[case("122", 37)]
    fn snafu_to_decimal_tests(#[case] input: &str, #[case] expected: Number) {
        assert_eq!(snafu_to_decimal(input), expected);
    }

    #[rstest]
    #[trace]
    #[case(1, "1")]
    #[case(2, "2")]
    #[case(3, "1=")]
    #[case(4, "1-")]
    #[case(5, "10")]
    #[case(6, "11")]
    #[case(7, "12")]
    #[case(8, "2=")]
    #[case(9, "2-")]
    #[case(10, "20")]
    #[case(15, "1=0")]
    #[case(20, "1-0")]
    #[case(2022, "1=11-2")]
    #[case(12345, "1-0---0")]
    #[case(314159265, "1121-1110-1=0")]
    fn decimal_to_snafu_tests(#[case] input: Number, #[case] expected: &str) {
        assert_eq!(decimal_to_snafu(input), expected);
    }

    #[test]
    fn part_1_example() {
        assert_eq!(solve_part_1(&file("example_1")), "2=-1=0");
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), "2==0=0===02--210---1");
    }
}
