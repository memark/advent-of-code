use crate::utils::parse_ints;
use crate::Int;
use std::collections::VecDeque;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Input(pub VecDeque<Int>);

impl Input {
    pub fn parse(s: &str) -> Self {
        Self(parse_ints(s).into())
    }
}
