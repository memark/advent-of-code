use anyhow::Result;
use colored::Colorize;
use intcode_computer::{input::Input, memory::Memory, program::run_program_n_output, state::State};
use itertools::Itertools;

fn main() -> Result<()> {
    // 1152, too low
    // 2096, too low
    println!("Part 1: {}", part1()?);
    // println!("Part 2: {:?}", part2());

    Ok(())
}

fn part1() -> Result<usize> {
    let file = include_str!("../input.txt");
    let memory = Memory::parse(file);

    // The Intcode program will serve as the brain of the robot. The program uses input instructions to access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is over a white panel. Then, the program will output two values:

    // First, it will output a value indicating the color to paint the panel the robot is over: 0 means to paint the panel black, and 1 means to paint the panel white.
    // Second, it will output a value indicating the direction the robot should turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees.

    // print_colored_grid_string(vec![(2, 2), (1, 3), (2, 3)], (2, 2), 1);
    // println!();

    let mut paint_jobs = vec![];
    let mut robot_pos = (0, 0);
    let mut robot_dir = 0;
    let mut state = State::from_memory(memory);

    assert_eq!(get_color_of_panel(robot_pos, &paint_jobs), 0);

    loop {
        println!(
            "paint_jobs: {}, panels_painted_at_least_once: {}, robot_dir: {robot_dir:?}, robot_pos {robot_pos:?}",
            paint_jobs.len(),
            panels_painted_at_least_once(&paint_jobs)
        );

        let color = get_color_of_panel(robot_pos, &paint_jobs);
        state.input = Input(vec![color as i64].into());

        state = run_program_n_output(state, 2);

        let output = (
            state.output[state.output.len() - 1] as u8,
            state.output[state.output.len() - 2] as u8,
        );

        (paint_jobs, robot_pos, robot_dir) =
            process_output(paint_jobs, robot_pos, robot_dir, output);

        if state.halted {
            println!("program halted");
            break;
        }
    }

    print_colored_grid_string(&paint_jobs, robot_pos, robot_dir);

    let count = panels_painted_at_least_once(&paint_jobs);

    Ok(count)
}

fn get_color_of_panel(panel: Coord, paint_jobs: &[PaintJob]) -> u8 {
    match paint_jobs.iter().rev().find(|&e| e.0 == panel) {
        Some(e) => e.1,
        None => 0,
    }
}

type Coord = (i32, i32);

type PaintJob = (Coord, u8);

fn print_colored_grid_string(paint_jobs: &[PaintJob], robot_pos: Coord, robot_dir: i32) {
    let grid_string = construct_grid_string(paint_jobs, robot_pos, robot_dir);

    for c in grid_string.chars() {
        let cs = c.to_string();
        let cc = match c {
            '.' => cs.bold(),
            '#' => cs.bold(),
            _ => cs.bold().bright_green(),
        };
        print!("{cc}");
    }
}

fn construct_grid_string(paint_jobs: &[PaintJob], robot_pos: Coord, robot_dir: i32) -> String {
    let whites = paint_jobs
        .iter()
        .filter(|&e| e.1 == 1)
        .map(|e| e.0)
        .collect_vec();

    let min_x = whites.iter().map(|e| e.0).min().unwrap_or(0) - 2;
    let max_x = whites.iter().map(|e| e.0).max().unwrap_or(0) + 2;

    let min_y = whites.iter().map(|e| e.1).min().unwrap_or(0) - 2;
    let max_y = whites.iter().map(|e| e.1).max().unwrap_or(0) + 2;

    let mut s = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = (x, y);
            if c == robot_pos {
                let r = match robot_dir % 4 {
                    0 => "^",
                    1 => ">",
                    2 => "v",
                    3 => "<",
                    _ => unreachable!(),
                };
                s += r;
            } else if whites.contains(&c) {
                s += "#";
            } else {
                s += ".";
            }
        }
        s += "\n";
    }
    s
}

fn process_output(
    paint_jobs: Vec<PaintJob>,
    robot_pos: Coord,
    robot_dir: i32,
    output: (u8, u8),
) -> (Vec<PaintJob>, Coord, i32) {
    let (paint_color, turn_direction) = output;

    let mut new_paint_jobs = paint_jobs;
    new_paint_jobs.push((robot_pos, paint_color));

    let new_dir = match turn_direction {
        0 => ((robot_dir - 1) + 4) % 4,
        1 => ((robot_dir + 1) + 4) % 4,
        _ => panic!("{}", turn_direction),
    };

    let new_pos = match new_dir {
        0 => (robot_pos.0, robot_pos.1 - 1),
        1 => (robot_pos.0 + 1, robot_pos.1),
        2 => (robot_pos.0, robot_pos.1 + 1),
        3 => (robot_pos.0 - 1, robot_pos.1),
        _ => panic!("{}", new_dir),
    };

    (new_paint_jobs, new_pos, new_dir)
}

fn panels_painted_at_least_once(paint_jobs: &[PaintJob]) -> usize {
    paint_jobs.iter().map(|&e| e.0).unique().count()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn construct_grid_string_01() {
        assert_eq!(
            r#"......
            ......
            ...>..
            ..##..
            ......
            ......"#
                .replace(' ', ""),
            construct_grid_string(&[((2, 2), 1), ((1, 3), 1), ((2, 3), 1)], (2, 2), 1).trim_end()
        );
    }

    #[test]
    fn moves_01() {
        let actual = (vec![], (2, 2), 0);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);
        assert_eq!(actual, (vec![((2, 2), 1)], (1, 2), 3));

        let output = (0, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);
        assert_eq!(
            actual,
            (
                vec![((2, 2), 1), ((1, 2), 0), ((1, 3), 1), ((2, 3), 1)],
                (2, 2),
                0
            )
        );

        let output = (0, 1);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);
        assert_eq!(
            actual,
            (
                vec![
                    ((2, 2), 1),
                    ((1, 2), 0),
                    ((1, 3), 1),
                    ((2, 3), 1),
                    ((2, 2), 0),
                    ((3, 2), 1),
                    ((3, 1), 1)
                ],
                (2, 1),
                3
            )
        );
    }

    #[test]
    fn moves_02() {
        let actual = (vec![], (2, 2), 0);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (0, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (0, 1);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        assert_eq!(
            actual,
            (
                vec![
                    ((2, 2), 1),
                    ((1, 2), 0),
                    ((1, 3), 1),
                    ((2, 3), 1),
                    ((2, 2), 0),
                    ((3, 2), 1),
                    ((3, 1), 1),
                ],
                (2, 1),
                3
            )
        );
    }

    #[test]
    fn panels_painted_at_least_once_01() {
        assert_eq!(
            6,
            panels_painted_at_least_once(&[
                ((2, 2), 1),
                ((1, 2), 0),
                ((1, 3), 1),
                ((2, 3), 1),
                ((2, 2), 0),
                ((3, 2), 1),
                ((3, 1), 1),
            ])
        );
    }

    #[test]
    fn get_color_of_panel_01() {
        assert_eq!(
            1,
            get_color_of_panel(
                (1, 3),
                &[
                    ((2, 2), 1),
                    ((1, 2), 0),
                    ((1, 3), 1),
                    ((2, 3), 1),
                    ((2, 2), 0),
                    ((3, 2), 1),
                    ((3, 1), 1)
                ]
            )
        );
    }

    #[test]
    fn get_color_of_panel_02() {
        assert_eq!(
            0,
            get_color_of_panel(
                (2, 2),
                &[
                    ((2, 2), 1),
                    ((1, 2), 0),
                    ((1, 3), 1),
                    ((2, 3), 1),
                    ((2, 2), 0),
                    ((3, 2), 1),
                    ((3, 1), 1)
                ]
            )
        );
    }

    // #[test]
    // fn runs_part1() {
    //     assert_eq!(0, part1().unwrap());
    // }

    // #[test]
    // fn runs_part2() {
    //     assert_eq!(50008, part2());
    // }
}
