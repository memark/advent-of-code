use crate::coord::Coord;
use itertools::Itertools;
use std::{str::FromStr, string::ParseError};

// Prova [[bool; 4]; 4] ?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rock(Vec<Vec<bool>>);

impl FromStr for Rock {
    type Err = ParseError;

    // Borde jag alltid returnera en 4x4 istället? Blir det enklare?

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rock(
            s.trim()
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!(),
                        })
                        .collect_vec()
                })
                .collect_vec(),
        ))
    }
}

impl Rock {
    pub fn covers_relative_coord(&self, c: Coord) -> bool {
        self.relative_coords().contains(&c)
    }

    pub fn relative_coords(&self) -> Vec<Coord> {
        let mut res = vec![];
        for y in 0..4 {
            for x in 0..4 {
                // TODO: Reverse
                // TODO: Use iproduct and iter
                if y < self.0.len() && x < self.0[y].len() && self.0[y][x] {
                    res.push(Coord::new(x as i32, y as i32))
                }
            }
        }
        res
    }

    pub fn real_coords(&self, upper_left: Coord) -> Vec<Coord> {
        self.relative_coords()
            .iter()
            .map(|rc| *rc + upper_left)
            .collect_vec()
    }

    pub fn stream() -> impl Iterator<Item = Rock> {
        [
            rock_1_data,
            rock_2_data,
            rock_3_data,
            rock_4_data,
            rock_5_data,
        ]
        .into_iter()
        .map(|r| r.parse::<Rock>().unwrap())
        .cycle()
    }
}

static rock_1_data: &str = "
####
";

static rock_2_data: &str = "
.#.
###
.#.
";

// Invertera denna i koden istället...
// static rock_3_data: &str = "
// ..#
// ..#
// ###
// ";
static rock_3_data: &str = "
###
..#
..#
";

static rock_4_data: &str = "
#
#
#
#
";

static rock_5_data: &str = "
##
##
";
