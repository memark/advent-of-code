use super::Int;

use crate::state::State;

#[derive(Debug, PartialEq)]
pub enum Parameter {
    Position(Int),
    Immediate(Int),
}
use Parameter::*;

impl Parameter {
    pub fn create(mode: Int, value: Int) -> Self {
        match mode {
            0 => Position(value),
            1 => Immediate(value),
            _ => unimplemented!("{}", mode),
        }
    }

    pub fn eval(self, state: &State) -> Int {
        match self {
            Position(p) => state.mem[p as usize],
            Immediate(i) => i,
        }
    }

    pub fn extract_pos(self) -> Int {
        match self {
            Position(p) => p,
            Immediate(_) => panic!(),
        }
    }
}