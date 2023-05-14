use crate::Int;
use crate::state::State;

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

    pub fn eval(self, state: &State) -> Int {
        match self {
            Position(p) => *state.memory.0.get(&p).unwrap_or(&0),
            Immediate(i) => i,
            Relative(p) => *state.memory.0.get(&(state.rb + p)).unwrap_or(&0),
        }
    }

    pub fn eval_pos(self, state: &State) -> Int {
        match self {
            Position(p) => p,
            Immediate(_) => panic!(),
            Relative(p) => state.rb + p,
        }
    }
}