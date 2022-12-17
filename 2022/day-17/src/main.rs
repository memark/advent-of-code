#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    non_upper_case_globals,
    unused_parens,
    unused_mut,
    unused_assignments
)]

mod coord;
mod jet;
mod rock;

use coord::*;
use itertools::{iproduct, Itertools};
use jet::*;
use rock::*;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
    string::ParseError,
};

fn main() {
    println!();

    // println!("Part 1: {}", solve_part_1(&file("example_1")));
    println!("Part 1: {}", solve_part_1(&file("input")));

    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> i32 {
    calc_height(input, 2022 * 10)
}

fn solve_part_2(input: &str) -> i32 {
    // calc_height(input, 1000000000000)
    0
}

// The tall, vertical chamber is exactly seven units wide.

// Each rock appears so that
//      its left edge is two units away from the left wall and
//      its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).

// If any movement would cause any part of the rock to move into the walls, floor, or a stopped rock,
//      the movement instead does not occur.

// If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock,
//      the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.

fn calc_height(input: &str, number_of_rocks: i32) -> i32 {
    let jet_pattern = input.parse::<JetPattern>().unwrap();

    let mut jet_stream = jet_pattern.stream();
    let mut rock_stream = Rock::stream();

    let mut stationary_rocks = Vec::<RockPos>::new();
    let mut stationary_rocks_coords = HashSet::<Coord>::new();

    for r_i in 1..=number_of_rocks {
        // println!("\nA new rock {r_i} begins falling.");

        let rock = rock_stream.next().unwrap();

        // Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
        let mut rock_pos = {
            let max_rock_y = stationary_rocks_coords.iter().map(|c| c.y).max();
            Coord::new(2, max_rock_y.unwrap_or(-1) + 4)
        };
        // println!("Starting position: {rock_pos}");
        // print_current_state(&stationary_rocks, Some(&(rock.clone(), rock_pos)));

        loop {
            // ## Gas

            let jet = jet_stream.next().unwrap();
            let rock_new_pos = match jet {
                Jet::PushLeft => rock_pos.left(),
                Jet::PushRight => rock_pos.right(),
            };
            let rock_real_coords = rock.real_coords(rock_new_pos);

            let collides_with_wall = rock_real_coords.iter().any(|c| c.x <= -1 || c.x >= 7);

            let collides_with_rock = {
                let collides_with_stationary_rock = rock_real_coords
                    .iter()
                    .any(|c| stationary_rocks_coords.contains(c));

                collides_with_stationary_rock
            };

            if collides_with_wall || collides_with_rock {
                // println!("Jet of gas pushes rock, but nothing happens.");
            } else {
                // println!("Jet of gas pushes rock ({jet}) from {rock_pos} to {rock_new_pos}.");
                rock_pos = rock_new_pos;
            }
            // print_current_state(&stationary_rocks, Some(&(rock.clone(), rock_pos)));

            // ## Falling

            let rock_new_pos = rock_pos.down();
            let rock_real_coords = rock.real_coords(rock_new_pos);

            let collides_with_rock_or_bottom = {
                let collides_with_stationary_rock = rock_real_coords
                    .iter()
                    .any(|c| stationary_rocks_coords.contains(c));

                let collides_with_bottom = rock_real_coords.iter().any(|c| c.y <= -1);

                collides_with_stationary_rock || collides_with_bottom
            };

            if collides_with_rock_or_bottom {
                // println!("Rock falls 1 unit, causing it to come to rest.");
                stationary_rocks.push((rock.clone(), rock_pos));
                stationary_rocks_coords.extend(rock.real_coords(rock_pos));
                // print_current_state(&stationary_rocks, Some(&(rock.clone(), rock_pos)));
                break;
            } else {
                // println!("Rock falls 1 unit from {rock_pos} to {rock_new_pos}.");
                rock_pos = rock_new_pos;
                // print_current_state(&stationary_rocks, Some(&(rock.clone(), rock_pos)));
            }
        }
        // print_current_state(&stationary_rocks, None);
    }

    let max_rock_y = stationary_rocks
        .iter()
        .flat_map(|sr| sr.0.real_coords(sr.1).iter().map(|c| c.y).collect_vec())
        .max()
        .unwrap_or(0);

    max_rock_y + 1
}

fn print_current_state(stationary_rocks: &Vec<RockPos>, falling_rock: Option<&RockPos>) {
    let y_max = {
        let mut all_rocks = stationary_rocks.clone();
        if let Some(fr) = falling_rock {
            all_rocks.push(fr.clone())
        };
        all_rocks
            .iter()
            .flat_map(|sr| sr.0.real_coords(sr.1).iter().map(|c| c.y).collect_vec())
            .max()
            .unwrap_or(0)
    };

    for y in (-1..=y_max).rev() {
        for x in -1..=7 {
            let c = Coord::new(x, y);
            let symbol = match (x, y) {
                (-1, -1) => "+",
                (7, -1) => "+",
                (-1, _) | (7, _) => "|",
                (_, -1) => "-",
                _ => {
                    let is_stationary_rock = stationary_rocks
                        .iter()
                        .any(|sr| sr.0.real_coords(sr.1).contains(&c));

                    let is_falling_rock = {
                        if let Some(rock) = falling_rock {
                            let relative_falling_rock_coord = c - rock.1;
                            rock.0.covers_relative_coord(relative_falling_rock_coord)
                        } else {
                            false
                        }
                    };

                    if is_stationary_rock {
                        "#"
                    } else if is_falling_rock {
                        "@"
                    } else {
                        "."
                    }
                }
            };
            print!("{symbol}");
        }
        println!();
    }
    println!();
}

type RockPos = (Rock, Coord);

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), 3068);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 3069);
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_2")), todo!());
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
