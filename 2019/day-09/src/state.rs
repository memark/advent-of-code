use std::collections::HashMap;

use super::Int;

pub type Mem = HashMap<Int, Int>;

#[derive(Debug, PartialEq)]
pub struct State {
    pub mem: Mem,
    pub input: Vec<Int>,
    pub output: Vec<Int>,
    pub ip: Int,
    pub rb: Int,
}

impl State {
    pub fn from_mem(mem: Mem) -> Self {
        State { mem, input: vec![], output: vec![], ip: 0, rb: 0 }
    }

    pub fn with_input(mem: Mem, input: Vec<Int>) -> Self {
        State { mem, input, output: vec![], ip: 0, rb: 0 }
    }
}