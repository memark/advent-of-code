#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::{iproduct, Itertools};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::RangeInclusive,
};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input"), 2000000));
    println!("Part 2: {}", solve_part_2(&file("input"), &(0..=4000000)));

    let x = 0..=4000000;
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
    // dbg!(x_min, x_max, y_min, y_max);
    if y == 2000000 {
        dbg!("hej");
    }

    let sensor_safe_zones = lines
        .iter()
        .map(|l| {
            let d = mh_dist(&l.sensor, &l.beacon);
            (l.sensor, d)
        })
        .collect::<HashMap<_, _>>();
    if y == 2000000 {
        dbg!("hej");
    }

    let sensors = lines.iter().map(|l| l.sensor).collect::<HashSet<_>>();
    let beacons = lines.iter().map(|l| l.beacon).collect::<HashSet<_>>();
    if y == 2000000 {
        dbg!("hej");
    }

    let sensor_and_safe_dist = lines.iter().map(|l| {
        let d = mh_dist(&l.sensor, &l.beacon) as i32;
        (l.sensor, d)
    });
    if y == 2000000 {
        dbg!("hej");
    }

    let safe_per_sensor = sensor_and_safe_dist.map(|(sensor, dist)| {
        (
            sensor,
            iproduct!(
                (sensor.x - dist..=sensor.x + dist),
                (sensor.y - dist..=sensor.y + dist)
            )
            .map(|(x, y)| Coord { x, y })
            .filter(move |&c| mh_dist(&sensor, &c) <= dist.clone()),
        )
    });
    // dbg!(&safe_per_sensor);
    if y == 2000000 {
        dbg!("hej");
    }

    let all_safes = safe_per_sensor
        .flat_map(|(sensor, coords)| coords)
        .collect::<HashSet<_>>();
    if y == 2000000 {
        dbg!("hej");
    }

    let safes = (x_min..=x_max)
        .filter(|&x| {
            let c = Coord { x, y };

            let safe = all_safes.clone().contains(&c);
            // .iter()
            // .any(|sz| mh_dist(sz.0, &c) <= *sz.1);

            if sensors.iter().contains(&c) {
                false
            } else if beacons.iter().contains(&c) {
                false
            } else if safe {
                true
            } else {
                false
            }
        })
        .collect_vec();

    safes.len() as i32
}

fn solve_part_2(input: &str, r: &RangeInclusive<i32>) -> i32 {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let lines = input
        .lines()
        .map(|l| {
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

    let sensors = lines.iter().map(|l| l.sensor).collect::<HashSet<_>>();
    let beacons = lines.iter().map(|l| l.beacon).collect::<HashSet<_>>();

    let sensor_and_safe_dist = lines
        .iter()
        .map(|l| {
            let d = mh_dist(&l.sensor, &l.beacon) as i32;
            (l.sensor, d)
        })
        .collect::<HashMap<_, _>>();

    let safe_per_sensor = sensor_and_safe_dist
        .iter()
        .map(|(&sensor, &dist)| {
            (
                sensor,
                iproduct!(
                    (sensor.x - dist..=sensor.x + dist),
                    (sensor.y - dist..=sensor.y + dist)
                )
                .map(|(x, y)| Coord { x, y })
                .filter(|&c| mh_dist(&sensor, &c) <= dist)
                .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    // dbg!(&safe_per_sensor);

    let all_safes = safe_per_sensor
        .iter()
        .flat_map(|(sensor, coords)| coords)
        .collect::<HashSet<_>>();

    let unsafes = iproduct!(r.clone(), r.clone())
        .map(|(x, y)| Coord { x, y })
        .filter(|&c| {
            if sensors.contains(&c) {
                false
            } else if beacons.contains(&c) {
                false
            } else if all_safes.contains(&c) {
                false
            } else {
                true
            }
        })
        .collect_vec();

    // dbg!(&unsafes.len());

    let safe = *unsafes.iter().exactly_one().unwrap();

    safe.x * 4000000 + safe.y
}

#[derive(Debug, Clone, Copy)]
struct Line {
    sensor: Coord,
    beacon: Coord,
}

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

    let sensor_safe_zones = lines
        .iter()
        .map(|l| {
            let d = mh_dist(&l.sensor, &l.beacon);
            (l.sensor, d)
        })
        .collect::<HashMap<_, _>>();

    let is_sensor = |c: &Coord| sensors.iter().contains(c);
    let is_beacon = |c: &Coord| beacons.iter().contains(c);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = Coord { x, y };

            let safe = sensor_safe_zones
                .iter()
                .any(|sz| mh_dist(sz.0, &c) <= *sz.1);
            if is_sensor(&c) {
                print!("S");
            } else if is_beacon(&c) {
                print!("B");
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

fn mh_dist(a: &Coord, b: &Coord) -> i32 {
    ((a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()) as i32
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

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1"), &(0..=20)), 56000011);
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input"), &(0..=4000000)), todo!());
    }
}
