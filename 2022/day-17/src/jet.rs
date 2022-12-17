use itertools::Itertools;
use std::{fmt::Display, str::FromStr, string::ParseError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct JetPattern(Vec<Jet>);

impl FromStr for JetPattern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(JetPattern(
            s.chars()
                .map(|c| match c {
                    '>' => Jet::PushRight,
                    '<' => Jet::PushLeft,
                    _ => panic!(),
                })
                .collect_vec(),
        ))
    }
}

impl JetPattern {
    pub fn stream(&self) -> impl Iterator<Item = &Jet> {
        self.0.iter().cycle()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Jet {
    PushLeft,
    PushRight,
}

impl Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
