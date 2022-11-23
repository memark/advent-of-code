#![allow(dead_code, unused_variables)]

use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() -> Result<()> {
    let file = r"#1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2";
    let file = fs::read_to_string("input")?;

    let lines = file.split_terminator('\n').map(|x| x.trim()).collect_vec();
    let claims = &lines.iter().map(|l| Claim::from(*l)).collect_vec();

    let max_x = claims.iter().map(|c| c.left + c.width).max().unwrap();
    let max_y = claims.iter().map(|c| c.top + c.height).max().unwrap();

    // ---

    let part_1a = {
        let mut num = 0;
        for x in 0..=max_x {
            for y in 0..=max_y {
                let num_covered = claims.iter().filter(|c| c.covers(&x, &y)).count();
                if num_covered >= 2 {
                    num += 1;
                }
            }
        }
        num
    };
    println!("{}: {}", stringify!(part_1a), part_1a);

    // ---

    let part_1b = (0..=max_x)
        .cartesian_product(0..=max_y)
        .filter(|(x, y)| claims.iter().filter(|c| c.covers(x, y)).take(2).count() == 2)
        .count();
    println!("{}: {}", stringify!(part_1b), part_1b);

    // ---

    let part_2a = {
        let coords_covered_by_one_claim = (0..=max_x)
            .cartesian_product(0..=max_y)
            .filter(|(x, y)| claims.iter().filter(|c| c.covers(x, y)).count() == 1)
            .collect_vec();

        claims
            .iter()
            .filter(|c| {
                c.all_coords()
                    .all(|(x, y)| coords_covered_by_one_claim.contains(&(x, y)))
            })
            .exactly_one()
            .unwrap()
            .id
    };
    println!("{}: {}", stringify!(part_2a), part_2a);

    // ---

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn covers(&self, x: &u32, y: &u32) -> bool {
        (self.left..self.left + self.width).contains(x)
            && (self.top..self.top + self.height).contains(y)
    }

    fn all_coords(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        (self.left..self.left + self.width).cartesian_product(self.top..self.top + self.height)
    }
}

impl From<&str> for Claim {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }
        let c = RE.captures(s).unwrap();
        let pu = |s: &str| s.parse().unwrap();

        Claim {
            id: pu(&c[1]),
            left: pu(&c[2]),
            top: pu(&c[3]),
            width: pu(&c[4]),
            height: pu(&c[5]),
        }
    }
}
