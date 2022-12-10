use itertools::Itertools;
use std::fs;

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!(
        "Part 2:\n{}",
        solve_part_2(&file("input")).replace('.', " ")
    );
}

fn solve_part_1(input: &str) -> i32 {
    let x_values = get_x_values(&input.lines().collect_vec());
    let signal_strengths = x_values
        .iter()
        .enumerate()
        .map(|(c, x)| (c + 1) as i32 * *x)
        .collect_vec();
    let interesting = [20, 60, 100, 140, 180, 220].map(|c| signal_strengths[c - 1]);
    let sum = interesting.iter().sum();

    sum
}

fn solve_part_2(input: &str) -> String {
    let x_values = get_x_values(&input.lines().collect_vec());
    let pixels = get_pixels(x_values)
        .iter()
        .map(|&p| if p { '#' } else { '.' })
        .collect_vec();
    let chunks = pixels.chunks(40);
    let crt_display = chunks.into_iter().map(|a| a.iter().join("")).join("\n");

    crt_display
}

fn get_x_values(lines: &[&str]) -> Vec<i32> {
    let mut x = 1;
    let mut cycle_vec = vec![];

    for l in lines {
        match *l {
            "noop" => {
                cycle_vec.push(x);
            }
            _ => {
                let (i, o) = l.split_once(' ').unwrap();
                let o = o.parse::<i32>().unwrap();
                if i != "addx" {
                    panic!();
                }

                cycle_vec.push(x);
                cycle_vec.push(x);

                x += o;
            }
        }
    }

    cycle_vec
}

fn get_pixels(x_values: Vec<i32>) -> Vec<bool> {
    let sprite_vec = &x_values
        .iter()
        .copied()
        .map(|x| (x - 1, x, x + 1))
        .collect_vec();

    let cycles = (1..=240).collect_vec();

    let zip = &cycles.iter().zip(sprite_vec).collect_vec();

    zip.iter()
        .copied()
        .map(|(&cycle, &(a, b, c))| {
            let cycle = cycle - 1;
            let cycle = cycle % 40;
            cycle == a || cycle == b || cycle == c
        })
        .collect_vec()
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
        assert_eq!(solve_part_1(&file("example_2")), 13140);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 15220);
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), "?");
    }

    #[test]
    fn part_2_input() {
        assert_eq!(
            solve_part_2(&file("input")),
            "###..####.####.####.#..#.###..####..##..
#..#.#.......#.#....#.#..#..#.#....#..#.
#..#.###....#..###..##...###..###..#..#.
###..#.....#...#....#.#..#..#.#....####.
#.#..#....#....#....#.#..#..#.#....#..#.
#..#.#....####.####.#..#.###..#....#..#."
        );
    }
}
