use crate::utils::parse_ints;
use crate::Int;

use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Memory(HashMap<Int, Int>);

impl Memory {
    pub fn parse(s: &str) -> Self {
        Self(ints_to_hashmap(parse_ints(s)))
    }

    pub fn get(&self, index: Int) -> Int {
        // Assume unlimited memory. Every non-set address should have the value 0.
        *self.0.get(&index).unwrap_or(&0)
    }

    pub fn set(&mut self, index: Int, value: Int) {
        self.0.insert(index, value);
    }
}

fn ints_to_hashmap(ints: Vec<Int>) -> HashMap<Int, Int> {
    ints.into_iter()
        .enumerate()
        .map(|(i, x)| (i as Int, x))
        .collect()
}
