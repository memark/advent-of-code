use crate::coord::Coord;
use itertools::{iproduct, Itertools};
use std::{collections::HashSet, hash::Hash, str::FromStr, string::ParseError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        iproduct!(0..4, 0..4)
            .filter_map(|(x, y)| {
                if y < self.0.len() && x < self.0[y].len() && self.0[y][x] {
                    Some(Coord::new(x as i32, y as i32))
                } else {
                    None
                }
            })
            .contains(&c)
    }

    pub fn real_coords(&self, upper_left: Coord) -> HashSet<Coord> {
        iproduct!(0..4, 0..4)
            .filter_map(|(x, y)| {
                if y < self.0.len() && x < self.0[y].len() && self.0[y][x] {
                    Some(Coord::new(x as i32, y as i32))
                } else {
                    None
                }
            })
            .map(|rc| rc + upper_left)
            .collect()
    }

    pub fn rocks() -> Vec<Rock> {
        [
            ROCK_1_DATA,
            ROCK_2_DATA,
            ROCK_3_DATA,
            ROCK_4_DATA,
            ROCK_5_DATA,
        ]
        .iter()
        .map(|r| r.parse::<Rock>().unwrap())
        .collect_vec()
    }
}

static ROCK_1_DATA: &str = "
####
";

static ROCK_2_DATA: &str = "
.#.
###
.#.
";

// Invertera denna i koden istället...
// static ROCK_3_DATA: &str = "
// ..#
// ..#
// ###
// ";
static ROCK_3_DATA: &str = "
###
..#
..#
";

static ROCK_4_DATA: &str = "
#
#
#
#
";

static ROCK_5_DATA: &str = "
##
##
";
