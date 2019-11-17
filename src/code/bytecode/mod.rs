
pub mod compile;

use super::truthtable::*;

/// Fully formed runnable bytecode program.
pub struct CompiledProgram {
    pub activation: Vec<bool>,
    pub instrs: Vec<Instr>,
}

/// Bytecode instruction.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Instr {
    Value(PushInstr),
    Operation(OpInstr),
}

/// Pushing values onto stack.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PushInstr {
    Push(IoTruthTable<TruthTableArray>),
    ReadThenPush { offset: i128 },
}

/// Operations (pop operands from stack, then push results).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum OpInstr {
    Both,
    Either,
    Different,
    Not,
    Same,
    Neither,
}

impl OpInstr {
    /// Number of operands.
    pub fn arity(&self) -> usize {
        use OpInstr::*;

        match *self {
            Both      => 2,
            Either    => 2,
            Different => 2,
            Same      => 2,
            Neither   => 2,

            Not       => 1,
        }
    }
}