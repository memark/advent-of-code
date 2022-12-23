#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    clippy::print_literal,
    clippy::manual_map,
    clippy::let_and_return,
    clippy::ptr_arg
)]

use colored::Colorize;
use itertools::{iproduct, Itertools};
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Not,
};

fn main() {
    // println!("Part 1: {}", solve_part_1(&file("example_2")));
    // println!("Part 1: {}", solve_part_1(&file("example_1")));
    println!("Part 1: {}", solve_part_1(&file("input")));

    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> i32 {
    // The scan shows Elves # and empty ground .
    // outside your scan, more empty ground extends a long way in every direction.

    // The scan is oriented so that north is up; orthogonal directions are written N (north), S (south), W (west), and E (east), while diagonal directions are written NE, NW, SE, SW.

    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(li, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(ci, _)| (ci as isize, li as isize))
        })
        .collect::<Map>();
    // dbg!(&elves);

    println!("Elves:");
    print_map(&elves);

    let meta_dirs = [N_ISH, S_ISH, W_ISH, E_ISH];
    let meta_dirs_map = HashMap::from([(N_ISH, N), (S_ISH, S), (W_ISH, W), (E_ISH, E)]);

    // let mut curr_meta_dirs = meta_dirs.iter().cycle();

    for i in 1.. {
        println!("\n== Begin round {} == ", i.to_string().bold());

        // During the first half of each round, each Elf considers the eight positions adjacent to themself. If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
        let (alone_elves, not_alone_elves): (HashSet<_>, HashSet<_>) =
            elves.iter().partition::<HashSet<Coord>, _>(|e| {
                EIGHT
                    .iter()
                    // Hade varit fint med en plus-metod än gång för alla.
                    .all(|d| !elves.contains(&(e.0 + d.0, e.1 + d.1)))
            });
        // dbg!(&alone_elves);
        println!("Alone elves:");
        print_map(&alone_elves);

        // dbg!(&not_alone_elves);
        println!("Not alone elves:");
        print_map(&not_alone_elves);

        // let meta_dirs = match i {
        //     1 => [N_ISH, S_ISH, W_ISH, E_ISH],
        //     2 => [S_ISH, W_ISH, E_ISH, N_ISH],
        //     3 => [W_ISH, E_ISH, N_ISH, S_ISH],
        //     _ => panic!(),
        // };
        let meta_dirs = {
            let mut temp = [N_ISH, S_ISH, W_ISH, E_ISH];
            temp.rotate_left((i - 1) % 4);
            temp
        };
        // dbg!(i, meta_dirs);

        // let curr_curr_meta_dirs = curr_meta_dirs.clone().take(4).collect_vec();
        // dbg!(&curr_curr_meta_dirs);
        // curr_meta_dirs.next();
        // let curr_curr_meta_dirs = curr_meta_dirs.clone().take(4).collect_vec();
        // dbg!(&curr_curr_meta_dirs);

        // Otherwise, the Elf looks in each of four directions in the following order and proposes moving one step in the first valid direction:
        // - If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
        // - If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
        // - If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
        // - If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
        let proposed_directions = not_alone_elves
            .iter()
            .map(|&e| {
                let pd = if let Some(&found_md) = meta_dirs.iter().find(|md| {
                    if e == (2, 2) {
                        dbg!(md);
                    }

                    md.iter().all(|d| {
                        let to_check = (e.0 + d.0, e.1 + d.1);
                        let free = !elves.contains(&to_check);
                        if e == (2, 2) {
                            dbg!(e, d, to_check, free);
                        }
                        free
                    })
                }) {
                    Some(meta_dirs_map[&found_md])
                } else {
                    None
                };
                (e, pd)
            })
            .collect::<HashMap<_, _>>();

        println!("Proposed directions:");
        print_directions(&proposed_directions);

        let proposed_directions = proposed_directions
            .iter()
            .filter(|(e, pd)| pd.is_some())
            .map(|(e, pd)| (e, pd.unwrap()))
            .collect::<HashMap<_, _>>();

        let proposed_destinations = proposed_directions
            .iter()
            .map(|(&e, &pd)| (e, (e.0 + pd.0, e.1 + pd.1)))
            .collect::<HashMap<_, _>>();

        println!("Proposed destinations:");
        print_proposed_destinations(&proposed_destinations.iter().map(|(e, d)| *d).collect_vec());

        let duplicate_destinations = proposed_destinations
            .iter()
            .map(|(&e, &pd)| pd)
            .duplicates()
            .collect::<Map>();

        // println!("Duplicate destinations:");
        // print_map(&duplicate_destinations);

        let unique_destinations = proposed_destinations
            .iter()
            .filter(|(e, pd)| !duplicate_destinations.contains(pd))
            .collect::<HashMap<_, _>>();

        if unique_destinations.is_empty() {
            println!("No elves need to move in round {i}.");
            panic!();
        }

        // println!("Unique destinations:");
        // print_map(&unique_destinations.iter().map(|(e, pd)| **pd).collect());

        for (e, d) in unique_destinations {
            elves.remove(e);
            elves.insert(*d);
        }
        println!("Elves:");
        print_map(&elves);

        // Finally, at the end of the round, the first direction the Elves considered is moved to the end of the list of directions.
        // For example, during the second round, the Elves would try proposing a move to the south first, then west, then east, then north.
        // On the third round, the Elves would first consider west, then east, then north, then south.
        // curr_meta_dirs.next();
    }

    // After each Elf has had a chance to propose a move, the second half of the round can begin.
    // Simultaneously, each Elf moves to their proposed destination tile
    // if they were the only Elf to propose moving to that position.
    // If two or more Elves propose moving to the same position, none of those Elves move.

    // To make sure they're on the right track, the Elves like to check after round 10 that they're making good progress toward covering enough ground.
    // To do this, count the number of empty ground tiles contained by the smallest rectangle that contains every Elf.

    let min_x = elves.iter().map(|(x, _)| x).min().unwrap() - 0;
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap() + 0;
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap() - 0;
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap() + 0;

    let score = iproduct!(min_x..=max_x, min_y..=max_y)
        .filter(|&(x, y)| !elves.contains(&(x, y)))
        .count();

    score as i32
}

fn print_map(elves: &Map) {
    if elves.is_empty() {
        println!("Empty!");
        return;
    }

    let min_x = elves.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap() + 1;
    // dbg!(min_x, max_x, min_y, max_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(x, y)) {
                print!("{}", "#".blue());
            } else {
                print!("{}", ".");
            }
        }
        println!();
    }
}

fn print_directions(elves_with_directions: &HashMap<Coord, Option<Coord>>) {
    if elves_with_directions.is_empty() {
        println!("Empty!");
        return;
    }

    let elves = elves_with_directions.iter().map(|(e, d)| e).collect_vec();
    let min_x = elves.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap() + 1;
    // dbg!(min_x, max_x, min_y, max_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if let Some(d) = elves_with_directions.get(&(x, y)) {
                match *d {
                    Some(N) => "^".blue(),
                    Some(E) => ">".blue(),
                    Some(S) => "v".blue(),
                    Some(W) => "<".blue(),
                    None => "x".blue(),
                    _ => panic!(),
                }
            } else {
                ".".normal()
            };
            print!("{c}");
        }
        println!();
    }
}

fn print_proposed_destinations(destinations: &Vec<Coord>) {
    if destinations.is_empty() {
        println!("Empty!");
        return;
    }

    let min_x = destinations.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = destinations.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = destinations.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = destinations.iter().map(|(_, y)| y).max().unwrap() + 1;
    // dbg!(min_x, max_x, min_y, max_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if destinations.contains(&(x, y)) {
                if destinations.iter().filter(|&d| d == &(x, y)).count() > 1 {
                    print!("{}", "#".red());
                } else {
                    print!("{}", "#".blue());
                }
            } else {
                print!("{}", ".");
            }
        }
        println!();
    }
}

type Map = HashSet<Coord>;

type Coord = (isize, isize);

const N: Coord = (0, -1);
const NE: Coord = (1, -1);
const E: Coord = (1, 0);
const SE: Coord = (1, 1);
const S: Coord = (0, 1);
const SW: Coord = (-1, 1);
const W: Coord = (-1, 0);
const NW: Coord = (-1, -1);

const EIGHT: [Coord; 8] = [N, NE, E, SE, S, SW, W, NW];
const MAIN: [Coord; 4] = [N, E, S, W];

const N_ISH: [Coord; 3] = [NW, N, NE];
const E_ISH: [Coord; 3] = [NE, E, SE];
const S_ISH: [Coord; 3] = [SE, S, SW];
const W_ISH: [Coord; 3] = [SW, W, NW];

fn solve_part_2(input: &str) -> i32 {
    0
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
        assert_eq!(solve_part_1(&file("example_1")), 110);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 3812);
    }

    #[ignore]
    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), todo!());
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
