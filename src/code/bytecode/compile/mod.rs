
pub mod error;

/// Debugging utilities.
pub mod debug;

/// Inner details.
pub mod inner;

use crate::code::span::{self, Span, Spanned, HasSpan};
use crate::code::tokens::*;
use crate::code::bytecode::*;
use crate::code::truthtable::IoTruthTable;

/// A representation of tokens which can represent recursive parenthesis
/// scoping. The actual Token variant should not contain a parenthesis
/// token.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TokenTree<'a> {
    Token(Spanned<'a, Token>),
    ParenScope(Vec<TokenTree<'a>>),
}

/// Program syntax, split into its parts.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ProgramParts<'a> {
    /// Program activation pattern.
    pub activation: Spanned<'a, ActivationPattern>,
    /// Behavior rule as a prefix-notation token tree.
    pub prefix_rule: Vec<TokenTree<'a>>,
}

/// Bytecode instructions which independently produce a single expression.
pub type ExprSubprogram<'a> = Vec<Spanned<'a, Instr>>;

impl<'a> HasSpan<'a> for TokenTree<'a> {
    fn span(&self) -> Span<'a> {
        match self {
            &TokenTree::Token(Spanned(_, span)) => span,
            &TokenTree::ParenScope(ref vec) => span::merge_all(vec),
        }
    }
}
