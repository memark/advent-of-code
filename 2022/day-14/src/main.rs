#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unused_assignments
)]

use itertools::{iproduct, Itertools};
use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
    iter::once,
    thread::sleep,
    time::Duration,
};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

// TODO: Prova newtypes för x- resp y-koordinater, så att de inte blandas ihop.

type Coord = (u32, u32);

fn solve_part_1(input: &str) -> u32 {
    let sos = (500, 0);

    let paths = get_paths(input);

    let full_path_coords = get_full_paths(paths);

    print_grid(&full_path_coords, sos, &HashSet::from_iter(vec![]));

    let ys = full_path_coords
        .iter()
        .map(|(x, y)| *y)
        .chain(once(sos.1))
        .collect_vec();

    let y_max = *ys.iter().max().unwrap();

    let mut occupied = HashSet::<(u32, u32)>::from_iter(full_path_coords.clone());
    let mut num_units = 0;

    let mut last_stopped = true;
    while last_stopped {
        let mut curr_pos = sos;
        num_units += 1;

        let mut moving = true;
        while moving {
            let (cpx, cpy) = curr_pos;
            let down = (cpx, cpy + 1);
            let down_left = (cpx - 1, cpy + 1);
            let down_right = (cpx + 1, cpy + 1);

            if !occupied.contains(&down) {
                curr_pos = down;
            } else if !occupied.contains(&down_left) {
                curr_pos = down_left;
            } else if !occupied.contains(&down_right) {
                curr_pos = down_right;
            } else {
                moving = false;
            }

            // if false && moving {
            //     let temp_occupied = occupied
            //         .iter()
            //         .chain(once(&curr_pos))
            //         .copied()
            //         .collect_vec();

            //     println!("{num_units}");
            //     print_grid(&full_path_coords, sos, &temp_occupied);
            // }
            // sleep(Duration::from_millis(50));

            if curr_pos.1 > y_max {
                last_stopped = false;
                break;
            }
        }
        occupied.insert(curr_pos);
        // sleep(Duration::from_millis(100));
    }

    // print_grid(&full_path_coords, sos, &occupied);

    num_units - 1
}

fn solve_part_2(input: &str) -> u32 {
    let sos = (500, 0);

    let paths = get_paths(input);

    let full_path_coords = get_full_paths(paths);

    // print_grid(&full_path_coords, sos, &vec![]);

    let xs = full_path_coords
        .iter()
        .map(|(x, y)| *x)
        .chain(once(sos.0))
        .collect_vec();

    let ys = full_path_coords
        .iter()
        .map(|(x, y)| *y)
        .chain(once(sos.1))
        .collect_vec();

    let x_min = *xs.iter().min().unwrap();
    let x_max = *xs.iter().max().unwrap();
    let y_min = *ys.iter().min().unwrap();
    let y_max = *ys.iter().max().unwrap();

    let x_span = x_max - x_min;
    let y_span = y_max - y_min;

    let floor_coords = ((500_u32 - 2 * y_span)..=(500 + 2 * y_span))
        .map(|x| (x, y_max + 2))
        .collect_vec();

    let full_path_coords = full_path_coords
        .iter()
        .chain(floor_coords.iter())
        .copied()
        .collect_vec();

    let mut occupied = HashSet::<(u32, u32)>::from_iter(full_path_coords.clone());
    let mut num_units = 0;

    let mut last_stopped = true;
    while last_stopped {
        let mut curr_pos = sos;
        num_units += 1;

        let mut moving = true;
        while moving {
            let (cpx, cpy) = curr_pos;
            let down = (cpx, cpy + 1);
            let down_left = (cpx - 1, cpy + 1);
            let down_right = (cpx + 1, cpy + 1);

            if !occupied.contains(&down) {
                curr_pos = down;
            } else if !occupied.contains(&down_left) {
                curr_pos = down_left;
            } else if !occupied.contains(&down_right) {
                curr_pos = down_right;
            } else {
                moving = false;
            }

            // if false && moving {
            //     let temp_occupied = occupied
            //         .iter()
            //         .chain(once(&curr_pos))
            //         .copied()
            //         .collect_vec();

            //     println!("{num_units}");
            //     print_grid(&full_path_coords, sos, &temp_occupied);
            // }
            // sleep(Duration::from_millis(50));
        }
        occupied.insert(curr_pos);
        // sleep(Duration::from_millis(100));

        if curr_pos == sos {
            break;
        }
    }

    // print_grid(&full_path_coords, sos, &occupied);

    num_units - 1 + 1
}

fn get_paths(input: &str) -> Vec<Vec<(u32, u32)>> {
    let paths: Vec<Vec<Coord>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| c.split_once(',').unwrap())
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .collect_vec()
        })
        .collect_vec();
    paths
}

fn get_full_paths(paths: Vec<Vec<(u32, u32)>>) -> HashSet<(u32, u32)> {
    HashSet::from_iter(
        paths
            .iter()
            .flat_map(|p| {
                p.iter().tuple_windows().map(|((x1, y1), (x2, y2))| {
                    if x1 == x2 {
                        (*min(y1, y2)..=*max(y1, y2))
                            .map(|yy| (*x1, yy))
                            .collect_vec()
                    } else if y1 == y2 {
                        (*min(x1, x2)..=*max(x1, x2))
                            .map(|xx| (xx, *y1))
                            .collect_vec()
                    } else {
                        panic!()
                    }
                })
            })
            .flatten(),
    )
}

fn print_grid(full_path_coords: &HashSet<Coord>, sos: Coord, occupied: &HashSet<Coord>) {
    let xs = full_path_coords
        .iter()
        .map(|(x, y)| *x)
        .chain(once(sos.0))
        .collect_vec();

    let ys = full_path_coords
        .iter()
        .map(|(x, y)| *y)
        .chain(once(sos.1))
        .collect_vec();

    let x_min = *xs.iter().min().unwrap();
    let x_max = *xs.iter().max().unwrap();
    let y_min = *ys.iter().min().unwrap();
    let y_max = *ys.iter().max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            // dbg!((x, y));
            if (x, y) == sos {
                print!("+");
            } else if full_path_coords.contains(&&(x, y)) {
                print!("#");
            } else if occupied.contains(&&(x, y)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
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
        assert_eq!(solve_part_1(&file("example_1")), 24);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 655);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 93);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), 26484);
    }
}
