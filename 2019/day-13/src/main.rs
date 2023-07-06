// https://adventofcode.com/2019/day/13

use std::{io, thread::sleep, time::Duration};

use anyhow::Result;
use colored::Colorize;
use intcode_computer::{
    input::Input,
    memory::Memory,
    program::{run_program, run_program_n_output, run_program_one_output},
    state::State,
    Int,
};
use itertools::Itertools;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Terminal,
};

fn main() -> Result<()> {
    // 207
    // println!("Part 1: {}", part1()?);
    part1()?;
    // println!("Part 2: {:?}", part2());

    Ok(())
}

fn part1() -> Result<usize> {
    let file = include_str!("../input.txt");
    let memory = Memory::parse(file);

    // every three output instructions specify the x position (distance from the left), y position (distance from the top), and tile id. The tile id is interpreted as follows:

    // 0 is an empty tile. No game object appears in this tile.
    // 1 is a wall tile. Walls are indestructible barriers.
    // 2 is a block tile. Blocks can be broken by the ball.
    // 3 is a horizontal paddle tile. The paddle is indestructible.
    // 4 is a ball tile. The ball moves diagonally and bounces off objects.

    let mut state = State::from_memory(memory);
    state.memory.set(0, 2);

    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);

    // state.input.0.push_back(1);
    // state.input.0.push_back(1);
    // state.input.0.push_back(1);
    // state.input.0.push_back(1);
    // state.input.0.push_back(1);
    // state.input.0.push_back(1);
    // state.input.0.push_back(1);
    // state.input.0.push_back(1);
    // state.input.0.push_back(1);

    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    // state.input.0.push_back(0);
    state.input.0.push_back(0);

    // for _ in 0..100 {
    //     state.input.0.push_back(0);
    // }
    // println!("{}", state.input.0.len());

    // state = run_program(state);
    // println!("finished!");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut ball_column = 0;
    let mut paddle_column = 0;
    let mut joystick = 0;

    loop {
        if paddle_column == ball_column {
            joystick = 0;
        }
        if paddle_column < ball_column {
            joystick = 1;
        }
        if paddle_column > ball_column {
            joystick = -1;
        }

        // if state.output.len() > 2522 {
        if state.input.0.is_empty() {
            state.input.0.push_back(joystick);
        }

        state = run_program_n_output(state, 3);
        let (x, y, id) = state
            .output
            .iter()
            .rev()
            .take(3)
            .rev()
            .next_tuple()
            .unwrap();
        // println!("{:?}", (x, y, id));

        if *id == 4 {
            ball_column = *x;
        }

        if *id == 3 {
            paddle_column = *x;
        }

        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = Paragraph::new(format!(
                "{:?}",
                (ball_column, paddle_column, joystick, state.input.0.len())
            ))
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    // .title("Copyright")
                    .border_type(BorderType::Plain),
            );

            rect.render_widget(copyright, chunks[2]);

            let grid_string = construct_grid_string(&state);

            let text = grid_string.lines().map(Spans::from).collect_vec();

            let paragraph = Paragraph::new(text.clone());

            rect.render_widget(paragraph, chunks[1]);
        })?;

        if state.halted {
            break;
        }

        // if state.input.0.len() < 100 {
        // if state.output.len() > 2522 {
        if state.input.0.is_empty() {
            // println!("{}", state.output.len());
            sleep(Duration::from_millis(1000));
        } else if state.output.len() > 2522 {
            sleep(Duration::from_millis(100));
        }
    }

    // print_colored_grid_string(&state);

    Ok(0)
}

type Coord = (i32, i32);

fn print_colored_grid_string(state: &State) {
    let grid_string = construct_grid_string(state);

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

fn construct_grid_string(state: &State) -> String {
    let tuples = state
        .output
        .iter()
        .tuples::<(_, _, _)>()
        .filter(|t| *t.0 != -1)
        .collect_vec();

    let min_x = *tuples.iter().map(|e| e.0).min().unwrap_or(&0);
    let max_x = *tuples.iter().map(|e| e.0).max().unwrap_or(&0);

    let min_y = *tuples.iter().map(|e| e.1).min().unwrap_or(&0);
    let max_y = *tuples.iter().map(|e| e.1).max().unwrap_or(&0);

    let mut s = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // Om vi ska behålla all output behöver vi hitta den _sista_ som matchar.
            let Some(t) = tuples.iter().filter(|(&xx, &yy, _)| xx == x && yy == y).last() else {
                s += ".";
                continue;
            };

            s += match t.2 {
                0 => ".",
                1 => "#",
                2 => "*",
                3 => "=",
                4 => "o",
                _ => panic!("unexpected {:?}", t),
            }

            //         let c = (x, y);
            //         if c == robot_pos {
            //             let r = match robot_dir % 4 {
            //                 0 => "^",
            //                 1 => ">",
            //                 2 => "v",
            //                 3 => "<",
            //                 _ => unreachable!(),
            //             };
            //             s += r;
            //         } else if whites.contains(&c) {
            //             s += "#";
            //         } else {
            //             s += ".";
            //         }
        }
        s += "\n";
    }
    s
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn construct_grid_string_01() {
        assert_eq!(
            r#".....
            .....
            ..>..
            .##..
            ....."#
                .replace(' ', ""),
            construct_grid_string(vec![(2, 2), (1, 3), (2, 3)], (2, 2), 1).trim_end()
        );
    }

    #[test]
    fn moves_01() {
        let actual = (vec![], (2, 2), 0);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);
        assert_eq!(actual, (vec![(2, 2)], (1, 2), 3));

        let output = (0, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);
        assert_eq!(actual, (vec![(2, 2), (1, 3), (2, 3)], (2, 2), 0));

        let output = (0, 1);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output(actual.0, actual.1, actual.2, output);
        assert_eq!(actual, (vec![(1, 3), (2, 3), (3, 2), (3, 1)], (2, 1), 3));
    }

    #[test]
    fn moves_02() {
        let actual = (vec![], (2, 2), 0);

        let output = (1, 0);
        let actual = process_output_2(actual.0, actual.1, actual.2, output);

        let output = (0, 0);
        let actual = process_output_2(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output_2(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output_2(actual.0, actual.1, actual.2, output);

        let output = (0, 1);
        let actual = process_output_2(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output_2(actual.0, actual.1, actual.2, output);

        let output = (1, 0);
        let actual = process_output_2(actual.0, actual.1, actual.2, output);

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
            panels_painted_at_least_once(vec![
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
