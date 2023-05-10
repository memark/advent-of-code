use super::Int;

#[derive(Debug, PartialEq)]
pub struct State {
    pub mem: Vec<Int>,
    pub input: Vec<Int>,
    pub output: Vec<Int>,
    pub ip: Int,
}

impl State {
    pub fn from_mem(mem: Vec<Int>) -> Self {
        State { mem, input: vec![], output: vec![], ip: 0 }
    }

    pub fn with_input(mem: Vec<Int>, input: Vec<Int>) -> Self {
        State { mem, input, output: vec![], ip: 0 }
    }
}