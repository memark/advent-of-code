use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
    let file = fs::read_to_string("input")?;
    let ids = file.split_terminator('\n').collect_vec();

    // let ids = [
    //     "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    // ];

    // ---

    let part_1a = {
        let twice_or_thrice = ids
            .iter()
            .map(|id| {
                let cc = id.chars().counts();
                let twice = cc.iter().any(|(_k, v)| *v == 2);
                let thrice = cc.iter().any(|(_k, v)| *v == 3);
                (twice, thrice)
            })
            .collect_vec();

        let twos = twice_or_thrice.iter().filter(|x| x.0).count();
        let threes = twice_or_thrice.iter().filter(|x| x.1).count();

        twos * threes
    };
    println!("part 1a: {:?}", part_1a);

    // ---

    let part_1b = {
        let smth = ids
            .iter()
            .map(|id| {
                let cc = id.chars().counts();
                let twice = cc.iter().any(|(_k, v)| *v == 2);
                let thrice = cc.iter().any(|(_k, v)| *v == 3);
                (twice, thrice)
            })
            .fold((0, 0), |acc, item| {
                (acc.0 + i32::from(item.0), acc.1 + i32::from(item.1))
            });

        smth.0 * smth.1
    };
    println!("part 1b: {:?}", part_1b);

    // ---

    let part_1c = {
        [2, 3]
            .iter()
            .map(|t| {
                ids.iter()
                    .filter(|id| id.chars().counts().iter().any(|(_k, v)| *v == *t))
                    .count()
            })
            .product::<usize>()
    };
    println!("part 1c: {:?}", part_1c);

    // ---

    let part_1d = {
        [2, 3]
            .iter()
            .map(|t| {
                ids.iter()
                    .filter(|id| id.chars().counts().iter().any(|(_k, v)| *v == *t))
                    .count()
            })
            .product::<usize>()
    };
    println!("{}: {}", stringify!(part_1d), part_1d);

    // ---

    let twos = ids
        .iter()
        .filter(|id| id.chars().counts().iter().any(|(_k, v)| *v == 2))
        .count();
    let threes = ids
        .iter()
        .filter(|id| id.chars().counts().iter().any(|(_k, v)| *v == 3))
        .count();
    let part_1e = twos * threes;

    println!("{}: {}", stringify!(part_1e), part_1e);

    // ---

    let ids_with = |x| {
        ids.iter()
            .filter(|id| id.chars().counts().iter().any(|(_k, v)| *v == x))
            .count()
    };
    let part_1f = ids_with(2) * ids_with(3);
    println!("{}: {}", stringify!(part_1f), part_1f);

    // ---
    println!();
    // ---

    let ids = [
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];

    // ---

    let part_2a = {
        let mut res = "".to_owned();
        'outer: for id1 in ids.iter() {
            for id2 in ids.iter() {
                let num_diff = id1
                    .chars()
                    .zip(id2.chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .count();
                if num_diff == 1 {
                    res = id1
                        .chars()
                        .zip(id2.chars())
                        .filter(|(c1, c2)| c1 == c2)
                        .map(|(c1, _c2)| c1)
                        .collect::<String>();

                    break 'outer;
                }
            }
        }
        res
    };
    println!("{}: {}", stringify!(part_2a), part_2a);

    // ---

    let part_2b = ids
        .iter()
        .cartesian_product(ids.iter())
        .map(|(id1, id2)| {
            id1.chars()
                .zip(id2.chars())
                .filter(|(c1, c2)| c1 == c2)
                .map(|(c1, _c2)| c1)
                .collect_vec()
        })
        .find(|x| x.len() == ids[0].len() - 1)
        .unwrap()
        .into_iter()
        .collect::<String>();
    println!("{}: {}", stringify!(part_2b), part_2b);

    // ---

    Ok(())
}

// cartesian_product
