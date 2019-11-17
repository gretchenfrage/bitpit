
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
pub struct ActivationPattern(pub u128);

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

macro_rules! token_to_variants {
    (@methods $name:ident -> $subtype:ident (_); $($tail:tt)*) => {
        pub fn $name(self) -> Option<$subtype> {
            match self {
                Token::$subtype(inner) => Some(inner),
                _ => None,
            }
        }

        token_to_variants!(@methods $($tail)*);
    };
    (@methods $name:ident -> () if $variant:ident; $($tail:tt)*) => {
        pub fn $name(self) -> Option<()> {
            match self {
                Token::$variant => Some(()),
                _ => None,
            }
        }

        token_to_variants!(@methods $($tail)*);
    };
    (@methods) => {};

    ($($t:tt)*) => {
        impl Token {
            token_to_variants!(@methods $($t)*);
        }
    };
}

token_to_variants! {
    to_operator -> Operator(_);
    to_bit_literal -> BitLiteral(_);
    to_io_literal -> IoLiteral(_);
    to_memory_read -> MemoryRead(_);
    to_parenthesis -> Parenthesis(_);
    to_activation_pattern -> ActivationPattern(_);

    to_colon -> () if Colon;
    to_whitespace -> () if Whitespace;
    to_comment -> () if Comment;
}