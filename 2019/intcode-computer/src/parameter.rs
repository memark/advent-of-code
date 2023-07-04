use crate::state::State;
use crate::Int;

#[derive(Debug, PartialEq)]
pub(crate) enum Parameter {
    Position(Int),
    Immediate(Int),
    Relative(Int),
}
use Parameter::*;

impl Parameter {
    pub fn create(mode: Int, value: Int) -> Self {
        match mode {
            0 => Position(value),
            1 => Immediate(value),
            2 => Relative(value),
            _ => unimplemented!("{}", mode),
        }
    }

    pub fn eval_val(self, state: &State) -> Int {
        match self {
            Position(p) => state.memory.get(p),
            Immediate(i) => i,
            Relative(p) => state.memory.get(state.rb + p),
        }
    }

    pub fn eval_pos(self, state: &State) -> Int {
        match self {
            Position(p) => p,
            Immediate(_) => panic!("Positional parameter cannot be of type Immediate."),
            Relative(p) => state.rb + p,
        }
    }
}
