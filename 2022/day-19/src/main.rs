// https://adventofcode.com/2022/day/19

#![allow(
    dead_code,
    unreachable_code,
    unused_imports,
    unused_variables,
    unused_mut,
    clippy::type_complexity
)]

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    fs,
    hash::{Hash, Hasher},
    str::FromStr,
    string::ParseError,
};

fn main() {
    // println!("Part 1: {}", solve_part_1(&file("example_1")));
    // println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_2(input: &str) -> i32 {
    let blueprints = input
        .lines()
        .map(|l| l.parse::<Blueprint>().unwrap())
        .collect_vec();
    // dbg!(&blueprints);

    let minute_limit = 32;

    let triangle: HashMap<_, _> = (1..=minute_limit)
        .map(|n| (n, (1..=n).sum::<i32>()))
        .collect();
    // dbg!(&triangle);

    // let mut results = Vec::<(i32, i32)>::new();
    // for blueprint in blueprints.par_iter().for {
    let results = blueprints[0..=2]
        .par_iter()
        .map(|blueprint| {
            // let blueprint = blueprints[1].clone();
            // {
            println!("\n\n==== Using blueprint {} ====", blueprint.id);

            let most_expensive_robot_ore_cost = *blueprint
                .robot_costs
                .values()
                .filter_map(|v| v.get(&Ore))
                .max()
                .unwrap_or(&u8::MAX);
            // dbg!(&most_expensive_robot_ore_cost);

            let most_expensive_robot_clay_cost = *blueprint
                .robot_costs
                .values()
                .filter_map(|v| v.get(&Clay))
                .max()
                .unwrap_or(&u8::MAX);
            // dbg!(&most_expensive_robot_clay_cost);

            let most_expensive_robot_obsidian_cost = *blueprint
                .robot_costs
                .values()
                .filter_map(|v| v.get(&Obsidian))
                .max()
                .unwrap_or(&u8::MAX);
            // dbg!(&most_expensive_robot_obsidian_cost);

            let mut max_geodes: Option<u8> = None;
            let mut cache: HashMap<u8, Vec<((u8, u8, u8, u8), (u8, u8, u8, u8))>> = HashMap::new();

            let mut queue: VecDeque<(State, Operation)> = VecDeque::new();

            queue.push_front((
                State {
                    minute: 1,
                    robots: [(Ore, 1), (Clay, 0), (Obsidian, 0), (Geode, 0)].into(),
                    resources: [(Ore, 0), (Clay, 0), (Obsidian, 0), (Geode, 0)].into(),
                },
                Idle,
            ));

            let mut iteration = 0_u64;

            let mut max_obsidian = 0;

            while let Some(item) = queue.pop_front() {
                iteration += 1;
                let (
                    State {
                        minute,
                        robots,
                        resources,
                    },
                    operation,
                ) = item;

                // println!("\n== Minute {minute} ==");

                let mut robots = robots.clone();
                let mut resources = resources.clone();

                match operation {
                    Idle => {
                        // Collect resources.
                        for (r, n) in robots.iter().sorted_by_key(|r| r.0) {
                            if n > &0 {
                                *resources.get_mut(r).unwrap() += n;
                                let sum = resources[r];
                                // println!(
                                //     "+ {n} {r:?}-collecting robot collects {n} {r:?}; you now have {sum} {r:?}.",
                                // );
                            }
                        }
                    }
                    BuildRobot(rr) => {
                        // Start building new robot.
                        let rc = &blueprint.robot_costs[&rr];

                        // if rr == Geode {
                        // println!("Building robot {rr:?}.");
                        // }

                        // let cost_str = rc
                        //     .iter()
                        //     .map(|(r, n)| format!("{n} {r:?}"))
                        //     .collect_vec()
                        //     .join(" and ");
                        // println!("- Spend {cost_str} to start building a {rr:?}-collecting robot.",);

                        for (r, n) in rc.iter() {
                            *resources.get_mut(r).unwrap() -= n;
                        }

                        // Collect resources.
                        for (r, n) in robots.iter().sorted_by_key(|r| r.0) {
                            if n > &0 {
                                *resources.get_mut(r).unwrap() += n;
                                // let sum = resources[r];
                                // println!(
                                //     "+ {n} {r:?}-collecting robot collects {n} {r:?}; you now have {sum} {r:?}.",
                                // );
                            }
                        }

                        // Finish building new robot.
                        *robots.get_mut(&rr).unwrap() += 1;
                        // println!(
                        //     "* The new {rr:?}-collecting robot is ready. You now have {} of them.",
                        //     robots[&rr]
                        // );
                    }
                }

                // if resources[&Obsidian] > max_obsidian {
                //     max_obsidian = resources[&Obsidian];
                // println!(
                //     "New obsidian max {max_obsidian} in minute {minute} in iteration {iteration} with robots {robots:?}."
                // );
                // }

                // När minuten är slut.

                // let mut hej = &mut cache
                //     .entry(minute)
                //     .or_insert_with(|| vec![((0, 0, 0, 0), (0, 0, 0, 0))]);
                // for (h0, h1) in hej.iter() {
                //     if resources[&Ore] <= h0.0
                //         && resources[&Clay] <= h0.1
                //         && resources[&Obsidian] <= h0.2
                //         && resources[&Geode] <= h0.3
                //         && robots[&Ore] <= h1.0
                //         && robots[&Clay] <= h1.1
                //         && robots[&Obsidian] <= h1.2
                //         && robots[&Geode] <= h1.3
                //     {
                //         // dbg!("Continue!");
                //         continue;
                //     }
                // }
                // hej.push((
                //     (
                //         resources[&Ore],
                //         resources[&Clay],
                //         resources[&Obsidian],
                //         resources[&Geode],
                //     ),
                //     (
                //         robots[&Ore],
                //         robots[&Clay],
                //         robots[&Obsidian],
                //         robots[&Geode],
                //     ),
                // ));
                // dbg!(&hej);

                // let hash = {
                //     let mut s = DefaultHasher::new();
                //     minute.hash(&mut s);
                //     robots.iter().collect_vec().hash(&mut s);
                //     resources.iter().collect_vec().hash(&mut s);
                //     s.finish()
                // };
                // dbg!(&hash);

                if minute as i32 >= minute_limit {
                    let geodes = resources[&Geode];
                    if max_geodes.is_none() || geodes > max_geodes.unwrap() {
                        max_geodes = Some(geodes);
                        println!("{iteration} => {geodes}");
                        // dbg!(&results);
                    }
                } else {
                    let remaining_minutes = minute_limit - minute as i32;
                    let possible_outcome = (resources[&Geode] as i32)
                        + (robots[&Geode] as i32 * remaining_minutes)
                        + triangle[&remaining_minutes];

                    if max_geodes.is_some() && possible_outcome < max_geodes.unwrap() as i32 {
                        // println!("Continue! {possible_outcome}");
                        continue;
                    }

                    let can_afford = blueprint
                        .robot_costs
                        .iter()
                        .filter(|&(r, c)| c.iter().all(|(rr, cc)| &resources[rr] >= cc))
                        .map(|(r, c)| r);

                    let can_build_ore_robot = can_afford.clone().any(|r| *r == Ore);
                    let can_build_clay_robot = can_afford.clone().any(|r| *r == Clay);
                    let can_build_obsidian_robot = can_afford.clone().any(|r| *r == Obsidian);
                    let can_build_geode_robot = can_afford.clone().any(|r| *r == Geode);

                    let queue_build_robot = |robot, queue: &mut VecDeque<(State, Operation)>| {
                        queue.push_front((
                            State {
                                minute: minute + 1,
                                robots: robots.clone(),
                                resources: resources.clone(),
                            },
                            BuildRobot(robot),
                        ));
                    };

                    let queue_idle = |queue: &mut VecDeque<(State, Operation)>| {
                        queue.push_front((
                            State {
                                minute: minute + 1,
                                robots: robots.clone(),
                                resources: resources.clone(),
                            },
                            Idle,
                        ));
                    };

                    // Lägg till byggbara robotar.

                    if can_build_geode_robot {
                        queue_build_robot(Geode, &mut queue);
                        // println!("Building only Geode robot.");

                        // 144 s med raden under inkommenterad. 153 s utkommenterad.
                        // continue;
                    }

                    if can_build_obsidian_robot
                        && robots[&Obsidian] < most_expensive_robot_obsidian_cost
                    {
                        queue_build_robot(Obsidian, &mut queue);
                    }

                    if can_build_clay_robot && robots[&Clay] < most_expensive_robot_clay_cost {
                        queue_build_robot(Clay, &mut queue);
                    }

                    if can_build_ore_robot && robots[&Ore] < most_expensive_robot_ore_cost {
                        queue_build_robot(Ore, &mut queue);
                    }

                    queue_idle(&mut queue);
                }
            }
            println!(
                "==== Finished blueprint {} with {} geodes after {iteration} iterations. ====",
                blueprint.id,
                max_geodes.unwrap()
            );
            // results.push((blueprint.id as i32, max_geodes.unwrap() as i32));
            max_geodes.unwrap() as i32
            // }
        })
        .collect::<Vec<_>>();
    dbg!(&results);

    // let results2 = &results.iter().map(|(a, b)| (*a, *b, a * b)).collect_vec();
    // dbg!(&results2);

    // let results3 = &results2.iter().map(|(_, _, c)| c).sum::<i32>();
    // dbg!(&results3);

    // *results3

    results.iter().product()
}

fn queue_build_robot(
    robot: Resource,
    queue: &mut VecDeque<(State, Operation)>,
    minute: u8,
    robots: &HashMap<Resource, u8>,
    resources: &HashMap<Resource, u8>,
) {
    queue.push_front((
        State {
            minute: minute + 1,
            robots: robots.clone(),
            resources: resources.clone(),
        },
        BuildRobot(robot),
    ));
}

fn queue_idle(
    robot: Resource,
    queue: &mut VecDeque<(State, Operation)>,
    minute: u8,
    robots: &HashMap<Resource, u8>,
    resources: &HashMap<Resource, u8>,
) {
    queue.push_front((
        State {
            minute: minute + 1,
            robots: robots.clone(),
            resources: resources.clone(),
        },
        BuildRobot(robot),
    ));
}

#[derive(Debug, Clone)]
struct State {
    // blueprint: Blueprint,
    minute: u8,
    robots: HashMap<Resource, u8>,
    resources: HashMap<Resource, u8>,
}

use Operation::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operation {
    Idle,
    BuildRobot(Resource),
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    robot_costs: HashMap<Resource, HashMap<Resource, u8>>,
}

impl FromStr for Blueprint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

        let captures = re.captures(s).unwrap();

        Ok(Blueprint {
            id: captures[1].parse().unwrap(),
            robot_costs: [
                (Ore, [(Ore, captures[2].parse().unwrap())].into()),
                (Clay, [(Ore, captures[3].parse().unwrap())].into()),
                (
                    Obsidian,
                    [
                        (Ore, captures[4].parse().unwrap()),
                        (Clay, captures[5].parse().unwrap()),
                    ]
                    .into(),
                ),
                (
                    Geode,
                    [
                        (Ore, captures[6].parse().unwrap()),
                        (Obsidian, captures[7].parse().unwrap()),
                    ]
                    .into(),
                ),
            ]
            .into_iter()
            .collect(),
        })
    }
}

use Resource::*;
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Resource {
    Ore = 1,
    Clay = 2,
    Obsidian = 3,
    Geode = 4,
}

// fn solve_part_2(input: &str) -> i32 {
//     0
// }

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn part_1_examples() {
    //     assert_eq!(solve_part_1(&file("example_1")), 33);
    // }

    // #[ignore]
    // #[test]
    // fn part_1_input() {
    //     assert_eq!(solve_part_1(&file("input")), todo!());
    // }

    // #[ignore]
    // #[test]
    // fn part_2_examples() {
    //     assert_eq!(solve_part_2(&file("example_1")), todo!());
    // }

    // #[ignore]
    // #[test]
    // fn part_2_input() {
    //     assert_eq!(solve_part_2(&file("input")), todo!());
    // }
}
