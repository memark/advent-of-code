#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input"), 2000000));
    // 1 => 4389717, not right
    // 2 => 4787893, too low
    // 4 => 5369857, too low

    // println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str, y: i32) -> i32 {
    // integer coordinates
    // no tie between beacons
    // manhattan distance

    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let lines = input
        .lines()
        .map(|l| {
            // dbg!(l);
            let captures = re.captures(l).unwrap();
            Line {
                sensor: (Coord {
                    x: captures[1].parse().unwrap(),
                    y: captures[2].parse().unwrap(),
                }),
                beacon: (Coord {
                    x: captures[3].parse().unwrap(),
                    y: captures[4].parse().unwrap(),
                }),
            }
        })
        .collect_vec();
    // dbg!(&lines);

    // print_grid(&lines);

    // OK, så nära varje sensor, på ett avstånd kortare än det till beacon, kan det inte finnas några andra beacons. Därför är alla koord kortare ogiltiga. Hitta alla koord som inte räknas bort pga detta. Troligen bäst att helt enkelt loopa över alla koord.
    // Glöm inte varianten att använda ett HashSet idag, eller möjligen HashMap om man vill ha med typen.

    let sensors = lines.iter().map(|l| l.sensor).collect_vec();
    let beacons = lines.iter().map(|l| l.beacon).collect_vec();
    let all_coords = lines
        .iter()
        .flat_map(|l| [l.sensor, l.beacon])
        .collect_vec();

    let xs = all_coords.iter().map(|Coord { x, y }| x).collect_vec();
    let ys = all_coords.iter().map(|Coord { x, y }| y).collect_vec();

    let x_min = **xs.iter().min().unwrap() * 5;
    let x_max = **xs.iter().max().unwrap() * 5;
    let y_min = **ys.iter().min().unwrap() * 5;
    let y_max = **ys.iter().max().unwrap() * 5;
    dbg!(x_min, x_max, y_min, y_max);

    let sensor_safe_zones = lines
        .iter()
        .map(|l| {
            let d = mh_dist(&l.sensor, &l.beacon);
            (l.sensor, d)
        })
        .collect::<HashMap<_, _>>();

    let is_sensor = |c: &Coord| sensors.iter().contains(c);
    let is_beacon = |c: &Coord| beacons.iter().contains(c);

    let safes = (x_min..=x_max)
        .filter(|&x| {
            let c = Coord { x, y };

            let safe = sensor_safe_zones
                .iter()
                .any(|sz| mh_dist(sz.0, &c) <= *sz.1);

            if is_sensor(&c) {
                false
            } else if is_beacon(&c) {
                false
            } else if safe {
                true
            } else {
                false
            }
        })
        .collect_vec();

    safes.len() as i32

    // 0
}

#[derive(Debug, Clone, Copy)]
struct Line {
    sensor: Coord,
    beacon: Coord,
}

// #[derive(Debug, Clone, Copy)]
// struct Sensor(Coord);

// #[derive(Debug, Clone, Copy)]
// struct Beacon(Coord);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

fn print_grid(lines: &Vec<Line>) {
    let sensors = lines.iter().map(|l| l.sensor).collect_vec();
    let beacons = lines.iter().map(|l| l.beacon).collect_vec();
    let all_coords = lines
        .iter()
        .flat_map(|l| [l.sensor, l.beacon])
        .collect_vec();

    let xs = all_coords.iter().map(|Coord { x, y }| x).collect_vec();
    let ys = all_coords.iter().map(|Coord { x, y }| y).collect_vec();

    let x_min = **xs.iter().min().unwrap();
    let x_max = **xs.iter().max().unwrap();
    let y_min = **ys.iter().min().unwrap();
    let y_max = **ys.iter().max().unwrap();

    // let line = lines
    //     .iter()
    //     .find(|l| l.sensor == Coord { x: 8, y: 7 })
    //     .unwrap();
    // let s = line.sensor;
    // let b = line.beacon;
    // let d = mh_dist(&s, &b);
    // dbg!(&d);

    let sensor_safe_zones = lines
        .iter()
        .map(|l| {
            let d = mh_dist(&l.sensor, &l.beacon);
            (l.sensor, d)
        })
        .collect::<HashMap<_, _>>();
    // dbg!(&sensor_safe_zones);

    let is_sensor = |c: &Coord| sensors.iter().contains(c);
    let is_beacon = |c: &Coord| beacons.iter().contains(c);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = Coord { x, y };

            let safe = sensor_safe_zones
                .iter()
                .any(|sz| mh_dist(sz.0, &c) <= *sz.1);
            // dbg!(&safe);

            if is_sensor(&c) {
                print!("S");
            } else if is_beacon(&c) {
                print!("B");
            // } else if mh_dist(&s, &Coord { x, y }) <= d {
            } else if safe {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn mh_dist(a: &Coord, b: &Coord) -> u32 {
    (a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()
}

fn solve_part_2(input: &str) -> i32 {
    todo!()
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
        assert_eq!(solve_part_1(&file("example_1"), 10), 26);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input"), 2000000), 5607466);
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
