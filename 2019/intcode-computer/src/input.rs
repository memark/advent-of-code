use crate::Int;
use crate::utils::parse_ints;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Input(pub Vec<Int>);

impl Input {
    pub fn parse(s: &str) -> Self {
        Self(parse_ints(s))
    }
}
