
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Operator {
    Both,
    Either,
    Different,
    Not,
    Same,
    Neither,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum BitLiteral {
    Yes,
    No,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IoLiteral {
    Input,
    Output,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MemoryRead(pub i128);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Parenthesis {
    Open,
    Close,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ActivationPattern(pub i128);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Token {
    Operator(Operator),
    BitLiteral(BitLiteral),
    IoLiteral(IoLiteral),
    MemoryRead(MemoryRead),
    Parenthesis(Parenthesis),
    ActivationPattern(ActivationPattern),
    Colon,
    Whitespace,
    Comment,
}