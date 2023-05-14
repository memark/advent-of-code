use std::collections::HashMap;

use itertools::Itertools;

use super::Int;

#[derive(Debug, PartialEq)]
pub struct State {
    pub memory: Memory,
    pub input: Input,
    pub output: Vec<Int>,
    pub ip: Int,
    pub rb: Int,
}

impl State {
    pub fn from_memory(memory: Memory) -> Self {
        State { memory, input: Input::default(), output: vec![], ip: 0, rb: 0 }
    }

    pub fn with_input(memory: Memory, input: Input) -> Self {
        State { memory, input, output: vec![], ip: 0, rb: 0 }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Memory(pub HashMap<Int, Int>);

impl Memory {
    pub fn parse(s: &str) -> Self {
        Self(ints_to_hashmap(parse_ints(s)))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Input(pub Vec<Int>);

impl Input {
    pub fn parse(s: &str) -> Self {
        Self(parse_ints(s))
    }
}

fn ints_to_hashmap(ints: Vec<Int>) -> HashMap<Int, Int> {
    ints.into_iter()
        .enumerate()
        .map(|(i, x)| (i as Int, x))
        .collect()
}

fn parse_ints(s: &str) -> Vec<Int> {
    if s.is_empty() {
        vec![]
    } else {
        s.split(',')
            .map(|ss| ss.trim().parse().unwrap())
            .collect_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn parses_ints_with_newlines() {
        let input = "1,9,10,3,
    2,3,11,0,
    99,
    30,40,50";

        let actual = parse_ints(input);

        assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], actual);
    }

    #[rstest]
    fn parses_ints_empty() {
        let input = "";

        let actual = parse_ints(input);

        assert_eq!(vec![0; 0], actual);
    }
}