
use super::tokens::*;

pub struct Program {
    pub activation_pattern: Vec<bool>,
    pub codes: Vec<Instr>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StateSquare {
    pub in_no_out_no: bool,
    pub in_yes_out_no: bool,
    pub in_no_out_yes: bool,
    pub in_yes_out_yes: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Instr {
    // pushing values onto stack
    Push(StateSquare),
    ReadThenPush { offset: i128 },

    // operations (pop operands from stack, then push results)
    Both,
    Either,
    Different,
    Not,
    Same,
    Neither,
}

pub struct CompileError {
    // location (point or span)
    // message
    // kind
}

pub fn compile_program(tokens: Vec<Token>) -> Result<Program, ()> {
    unimplemented!()
}