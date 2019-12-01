
pub mod compile;

use super::truthtable::*;

/// Fully formed runnable bytecode program.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
    Push(IoTruthTable<u8>),
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

macro_rules! enum_from_samey {
    (
    ($to:path) from ($from:path) {
        $(
        $variant:ident
        ),* $(,)?
    }
    ) => {
        impl From<$from> for $to {
            fn from(from: $from) -> $to {
                match from {
                    $(
                    <$from>::$variant => <$to>::$variant,
                    )*
                }
            }
        }
    }
}

enum_from_samey! {
    (OpInstr) from (crate::code::tokens::Operator) {
        Both,
        Either,
        Different,
        Not,
        Same,
        Neither,
    }
}