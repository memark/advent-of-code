use super::Int;

use crate::state::State;

#[derive(Debug, PartialEq)]
pub enum Parameter {
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
            Position(p) => state.mem[p as usize],
            Immediate(i) => i,
            Relative(p) => state.mem[(state.rb + p) as usize],
        }
    }

    pub fn eval_pos(self) -> Int {
        match self {
            Position(p) => p,
            Immediate(_) => panic!(),
            Relative(p) => todo!(),
        }
    }
}