use itertools::FoldWhile::{Continue, Done};
use itertools::{iproduct, Itertools};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> usize {
    let trees = parse_trees(input);

    assert_eq!(trees.len(), trees[0].len());
    let len = trees.len();

    let visible_trees = iproduct!(0..len, 0..len)
        .filter(|(y, x)| {
            let (y, x) = (*y, *x);
            // No special handling is required for the trees around the edge.

            let tree_height = trees[y][x];

            let visible_from_left = (0..x).all(|xx| trees[y][xx] < tree_height);
            let visible_from_right = (x + 1..len).all(|xx| trees[y][xx] < tree_height);
            let visible_from_top = (0..y).all(|yy| trees[yy][x] < tree_height);
            let visible_from_bot = (y + 1..len).all(|yy| trees[yy][x] < tree_height);

            visible_from_left || visible_from_right || visible_from_top || visible_from_bot
        })
        .collect_vec();

    visible_trees.len()
}

fn solve_part_2(input: &str) -> u32 {
    let trees = parse_trees(input);

    assert_eq!(trees.len(), trees[0].len());
    let len = trees.len();

    let visible_trees = iproduct!(0..len, 0..len)
        .map(|(y, x)| {
            let check_view = |range: Vec<usize>, f: &dyn Fn(usize) -> bool| {
                range
                    .iter()
                    .fold_while(0, |acc, yy| {
                        if f(*yy) {
                            Done(acc + 1)
                        } else {
                            Continue(acc + 1)
                        }
                    })
                    .into_inner()
            };

            let view_up = check_view((0..y).rev().collect_vec(), &|yy| {
                trees[yy][x] >= trees[y][x]
            });

            let view_left = check_view((0..x).rev().collect_vec(), &|xx| {
                trees[y][xx] >= trees[y][x]
            });

            let view_right = check_view((x + 1..len).collect_vec(), &|xx| {
                trees[y][xx] >= trees[y][x]
            });

            let view_down = check_view((y + 1..len).collect_vec(), &|yy| {
                trees[yy][x] >= trees[y][x]
            });

            (view_up, view_left, view_right, view_down)
        })
        .map(|(a, b, c, d)| a * b * c * d)
        .collect_vec();

    visible_trees.iter().max().unwrap().to_owned()
}

type Trees = Vec<Vec<u32>>;

fn parse_trees(input: &str) -> Trees {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

// ---

fn file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), 21);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 1825);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 8);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 235200);
    }
}
