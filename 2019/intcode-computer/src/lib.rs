use itertools::Itertools;
use std::collections::HashMap;

pub mod instruction;
pub mod parameter;
pub mod program;
pub mod state;

pub type Int = i64;

pub fn parse_ints(s: &str) -> Vec<Int> {
    if s.is_empty() {
        vec![]
    } else {
        s.split(',')
            .map(|ss| ss.trim().parse().unwrap())
            .collect_vec()
    }
}

pub fn ints_to_hashmap(mem: Vec<Int>) -> HashMap<Int, Int> {
    mem.into_iter()
        .enumerate()
        .map(|(i, x)| (i as Int, x))
        .collect()
}