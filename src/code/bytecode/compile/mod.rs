
pub mod error;

/// Debugging utilities.
pub mod debug;

/// Inner details.
pub mod inner;

use self::error::{Error, ErrorKind};
use crate::code::span::{self, Span, Spanned, HasSpan};
use crate::code::tokens::*;
use crate::code::bytecode::*;
use crate::code::lexer::lex_verbose_err;

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

pub fn compile(code: &str) -> Result<CompiledProgram, Error> {
    // lex
    lex_verbose_err(code)
        .map_err(|message| Error {
            message,
            location: Span::None,
            kind: ErrorKind::Lexer,
            next_error:  None,
        })
        // strip
        .map(|mut tokens| {
            tokens.retain(|&Spanned(ref token, _)| match token {
                &Token::Whitespace => false,
                &Token::Comment => false,
                _ => true,
            });
            tokens
        })
        // compile
        .and_then(inner::parse_scopes)
        .and_then(inner::program_parts)
        .and_then(|parts|
            inner::syntax_to_expression(&parts.prefix_rule)
                .and_then(move |mut expressions| {

                    // verify correct number of expressions
                    if expressions.len() != 1 {
                        return Err(Error {
                            message: format!(
                                "program requires 1 expression in behavior rule, found {}",
                                expressions.len()
                            ),
                            location: expressions.span(),
                            kind: ErrorKind::TooManyExprsInProgram,
                            next_error: None,
                        });
                    }
                    let expression: ExprSubprogram = expressions.remove(0);

                    // de-span the expression instructions
                    let instrs: Vec<Instr> = expression
                        .into_iter()
                        .map(Spanned::into_inner)
                        .collect();

                    // convert the behavior rule from a u128 to a Vec<boolean>
                    let activation: Vec<bool> = {
                        let Spanned(ActivationPattern(field), _) = parts.activation;

                        let mut vec = Vec::new();
                        let mut scanned_mask: u128 = 0x0;
                        'bits: for i in 0..128 {
                            if (field & (!scanned_mask)) == 0x0 {
                                break 'bits;
                            }

                            let bit_mask: u128 = 0x1 << i;
                            let bit: u128 = field & bit_mask;
                            let bit_bool: bool = bit != 0x0;
                            vec.push(bit_bool);

                            scanned_mask |= bit_mask;
                        }
                        vec.reverse();

                        vec
                    };

                    // compose the compiled program
                    let program = CompiledProgram {
                        activation,
                        instrs,
                    };

                    Ok(program)
                })
        )
}

impl<'a> HasSpan<'a> for TokenTree<'a> {
    fn span(&self) -> Span<'a> {
        match self {
            &TokenTree::Token(Spanned(_, span)) => span,
            &TokenTree::ParenScope(ref vec) => span::merge_all(vec),
        }
    }
}
