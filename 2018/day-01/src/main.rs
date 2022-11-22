use anyhow::Result;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    let file = fs::read_to_string("input")?;
    let freqs = file
        .split_terminator('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // ---

    let part_1 = freqs.iter().sum::<i32>();
    println!("part 1: {:?}", part_1);

    // ---

    let part_2a = 'part_2: {
        let mut seen = HashSet::new();
        let mut curr = 0;
        for f in freqs.iter().cycle() {
            curr += f;
            if !seen.insert(curr) {
                break 'part_2 curr;
            }
        }
        0
    };
    println!("part 2a: {:?}", part_2a);

    // ---

    let part_2b = freqs
        .iter()
        .cycle()
        .fold_while((0, HashSet::new()), |mut acc, item| {
            let curr = acc.0 + item;
            if !acc.1.insert(curr) {
                Done((curr, acc.1))
            } else {
                Continue((curr, acc.1))
            }
        })
        .into_inner()
        .0;
    println!("part 2b: {:?}", part_2b);

    // ---

    let part_2c = freqs
        .iter()
        .cycle()
        .scan(0, |state, &item| {
            *state += item;
            Some(*state)
        })
        .duplicates()
        .next()
        .unwrap();
    println!("part 2c: {:?}", part_2c);

    Ok(())
}
