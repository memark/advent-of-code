use super::Int;

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
            _ => unimplemented!(),
        }
    }
}