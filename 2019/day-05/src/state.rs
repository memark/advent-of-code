use super::Int;

#[derive(Debug, PartialEq)]
pub struct State {
    pub mem: Vec<Int>,
    pub input: Vec<Int>,
    pub output: Vec<Int>,
}

impl State {
    pub fn from_mem(mem: Vec<Int>) -> Self {
        State { mem, input: vec![], output: vec![] }
    }

    pub fn with_input(mem: Vec<Int>, input: Vec<Int>) -> Self {
        State { mem, input, output: vec![] }
    }
}