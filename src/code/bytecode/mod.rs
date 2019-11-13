
pub mod compile;

use super::truthtable::*;

pub struct CompiledProgram {
    pub activation_pattern: Vec<bool>,
    pub instrs: Vec<Instr>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Instr {
    // pushing values onto stack
    Push(IoTruthTable<TruthTableArray>),
    ReadThenPush { offset: i128 },

    // operations (pop operands from stack, then push results)
    Both,
    Either,
    Different,
    Not,
    Same,
    Neither,
}

/*
pub struct CompileError {
    // location (point or span)
    // message
    // kind
}

pub fn compile_program(tokens: Vec<Token>) -> Result<CompiledProgram, ()> {
    unimplemented!()
}
*/