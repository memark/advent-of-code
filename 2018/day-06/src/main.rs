#![allow(dead_code, unused_imports, unused_variables)]

use colored::Colorize;
use itertools::Itertools;
use num::Complex;
use std::fs;

fn main() {
    let input = "`1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9
    `"
    .replace('`', "");
    let input = read_and_trim_input();

    let lines = get_lines(&input);
    // dbg!(&lines);

    let coords = lines
        .iter()
        .map(|lines| {
            lines
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect_vec();
    // dbg!(&coords);

    let part_1a = {
        let min_x = *coords.iter().map(|(x, y)| x).min().unwrap();
        // let min_x = 0;
        let max_x = *coords.iter().map(|(x, y)| x).max().unwrap();
        // let max_x = 9;
        let min_y = *coords.iter().map(|(x, y)| y).min().unwrap();
        // let min_y = 0;
        let max_y = *coords.iter().map(|(x, y)| y).max().unwrap();
        // let max_y = 9;
        // dbg!(min_x, max_x, min_y, max_y);

        let closest_coord_by_location = (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .map(|l| (l, closest_coord(&coords, l)))
            .collect_vec();
        // dbg!(&closest_coord_by_location);

        // print(
        //     &closest_coord_by_location,
        //     &coords,
        //     min_y,
        //     max_y,
        //     min_x,
        //     max_x,
        // );

        let closest_coord_by_location_larger = (min_x - 1..=max_x + 1)
            .cartesian_product(min_y - 1..=max_y + 1)
            .map(|l| (l, closest_coord(&coords, l)))
            .collect_vec();

        let counts = closest_coord_by_location
            .iter()
            .filter_map(|&cc| cc.1)
            .counts();
        // dbg!(&counts);

        let counts_larger = closest_coord_by_location_larger
            .iter()
            .filter_map(|&cc| cc.1)
            .counts();
        // dbg!(&counts_larger);
        let same = counts
            .iter()
            .filter(|&c| counts_larger.get(c.0).unwrap() == c.1)
            .collect_vec();
        // dbg!(&same);

        let safest = same.iter().max_by_key(|&s| s.1).unwrap();
        // dbg!(&safest);

        safest.1.to_owned()
    };
    println!("{}: {}", stringify!(part_1a), part_1a);

    let part_2a = "?";
    println!("{}: {}", stringify!(part_2a), part_2a);
}

fn print(
    closest_coord_by_location: &[((i32, i32), Option<(i32, i32)>)],
    coords: &[(i32, i32)],
    min_y: i32,
    max_y: i32,
    min_x: i32,
    max_x: i32,
) {
    let prepare_print = closest_coord_by_location
        .iter()
        .map(|&(l, c)| {
            let char = get_char_for_location(c, coords, l);
            (l, char)
        })
        .collect_vec();
    // dbg!(&prepare_print);

    // Känns knöligt, vore det bättre att ha koordinaterna i en array av array?
    println!();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let char = &prepare_print.iter().find(|&pp| pp.0 == (x, y)).unwrap().1;
            print!("{}", char);
        }
        println!();
    }
    println!();
}

fn get_char_for_location(c: Option<(i32, i32)>, coords: &[(i32, i32)], l: (i32, i32)) -> String {
    match c {
        Some(sc) => {
            let letters = ('a'..='z').collect_vec();
            let position = coords.iter().position(|&cc| cc == sc).unwrap();
            let char = if position < letters.len() {
                letters[position]
            } else {
                '?'
            };
            if l == sc {
                char.to_ascii_uppercase().to_string().bold().to_string()
            } else {
                char.to_string()
            }
        }
        None => '.'.to_string(),
    }
}

fn closest_coord(coords: &[(i32, i32)], l: (i32, i32)) -> Option<(i32, i32)> {
    let coord_dists = coords
        .iter()
        .map(|&c| (c, manhattan_distance(l, c)))
        .collect_vec();
    let closest_coord_dist = &coord_dists.iter().min_by_key(|&(c, d)| d).unwrap();
    let closest_coords = coord_dists
        .iter()
        .filter(|&&(c, d)| d == closest_coord_dist.1)
        .collect_vec();

    if closest_coords.len() == 1 {
        Some(closest_coords[0].0)
    } else {
        None
    }
}

fn read_and_trim_input() -> String {
    // Kanske inte ha trim här?
    let file = fs::read_to_string("input").unwrap();
    file.trim().to_owned()
}

fn get_lines(s: &str) -> Vec<&str> {
    s.trim_matches('`')
        .split_terminator('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect_vec()
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    num::abs(x2 - x1) + num::abs(y2 - y1)
}

// fn manhattan_distance(a: Complex<i32>, b: Complex<i32>) -> i32 {
//     num::abs(a.re - b.re) + num::abs(a.im - b.im)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn manhattan_distance_test() {
    //     assert_eq!(
    //         manhattan_distance(Complex::new(1, 1), Complex::new(5, 10)),
    //         4 + 9
    //     );
    // }

    #[test]
    fn part_1_examples() {
        assert_eq!(1, 1);
    }

    #[test]
    fn part_1_input() {
        let input = read_and_trim_input();

        assert_eq!(1, 1);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(1, 1);
    }

    #[test]
    fn part_2_input() {
        let input = read_and_trim_input();

        assert_eq!(1, 1);
    }
}
