
use super::{
    ProgramParts, TokenTree, ExprSubprogram,
    error::{Error, ErrorKind}
};
use crate::code::span::{self, Span, Spanned, HasSpan};
use crate::code::tokens::*;
use crate::code::bytecode::*;
use crate::code::truthtable::IoTruthTable;

/// Parse a flat sequence of tokens into a sequence of token trees.
pub fn parse_scopes<'a, T>(tokens: T) -> Result<Vec<TokenTree<'a>>, Error<'a>>
    where T: IntoIterator<Item=Spanned<'a, Token>>
{
    fn top<T>(vec: &mut Vec<T>) -> &mut T {
        let i = vec.len() - 1;
        &mut vec[i]
    }

    let mut scope_stack: Vec<Vec<TokenTree<'a>>> = vec![Vec::new()];
    let mut paren_depth: usize = 0;

    for token in tokens {
        match &token.0 {
            Token::Parenthesis(Parenthesis::Open) => {
                // begin a new layer of scope
                paren_depth += 1;
                scope_stack.push(Vec::new());
            }
            Token::Parenthesis(Parenthesis::Close) => {
                // exit a new layer of scope
                if paren_depth > 0 {
                    // if valid
                    paren_depth -= 1;
                    let scope = scope_stack.pop().unwrap();
                    let tt = TokenTree::ParenScope(scope);
                    top(&mut scope_stack).push(tt);
                } else {
                    return Err(Error {
                        message: format!("unmatched close parenthesis"),
                        location: token.1,
                        kind: ErrorKind::UnbalancedParenthesis,
                        next_error: None,
                    });
                }
            },
            _ => {
                // simply add the token to the current scope
                let tt = TokenTree::Token(token);
                top(&mut scope_stack).push(tt);
            }
        };
    }

    // make sure that all parenthesis have been closed
    if paren_depth > 0 {
        return Err(Error {
            message: format!("{} unclosed parenthesis", paren_depth),
            location: Span::None,
            kind: ErrorKind::UnbalancedParenthesis,
            next_error: None,
        });
    }

    // success
    assert_eq!(scope_stack.len(), 1);
    return Ok(scope_stack.pop().unwrap());
}

/// Split syntax into parts of the program.
pub fn program_parts<'a>(mut tokens: &[TokenTree<'a>]) -> Result<ProgramParts<'a>, Error<'a>> {

    fn take_variant<'a, V>(
        tokens: &mut &[TokenTree<'a>],
        method: fn(Token) -> Option<V>,
        required: &str,
    ) -> Result<Spanned<'a, V>, Error<'a>> {
        let elem: Option<&TokenTree<'a>> = tokens.get(0);
        let elem: &TokenTree<'a> = elem
            .ok_or_else(|| Error {
                message: format!("required {}, found end of tokens", required),
                location: Span::None,
                kind: ErrorKind::WrongTokenType,
                next_error: None,
            })?;
        let elem: Spanned<'a, Token> = match elem {
            &TokenTree::Token(single) => single,
            &TokenTree::ParenScope(_) => {
                return Err(Error {
                    message: format!("required {}, found parenthesis", required),
                    location: elem.span(),
                    kind: ErrorKind::WrongTokenType,
                    next_error: None,
                })
            }
        };
        let Spanned(elem, span) = elem;
        let elem: V = method(elem)
            .ok_or_else(|| Error {
                message: format!("required {}, found {:?}", required, elem),
                location: span,
                kind: ErrorKind::WrongTokenType,
                next_error: None,
            })?;
        let elem: Spanned<'a, V> = Spanned(elem, span);

        *tokens = &(*tokens)[1..];

        Ok(elem)
    }

    let activation: Spanned<'a, ActivationPattern> = take_variant(
        &mut tokens,
        Token::to_activation_pattern,
        "activation pattern",
    )?;

    take_variant(&mut tokens, Token::to_colon, "colon")?;

    let prefix_rule: Vec<TokenTree<'a>> = tokens.to_vec();

    let program_parts = ProgramParts {
        activation,
        prefix_rule,
    };
    Ok(program_parts)
}

/// Convert syntax for an expression into bytecode.
pub fn syntax_to_expression<'a>(
    syntax: &[TokenTree<'a>]
) -> Result<Vec<ExprSubprogram<'a>>, Error<'a>> {

    #[derive(Debug, Clone)]
    struct Prefix {
        op: OpInstr,
        beneath: usize,
        requires: usize,
    }

    let mut expr_stack: Vec<ExprSubprogram<'a>> = vec![];
    let mut prefix_stack: Vec<Spanned<'a, Prefix>> = vec![];

    for tt in syntax.iter() {
        match tt_to_expr_token(tt)? {

            ExprToken::Literal(instr) => {
                let expr = instr.map(Instr::Value);

                expr_stack.push(vec![expr]);
            }

            ExprToken::Op(instr) => {
                let prefix = instr.map(|op| Prefix {
                    op,
                    beneath: expr_stack.len(),
                    requires: op.arity(),
                });

                prefix_stack.push(prefix);
            }

            ExprToken::Scope(sub_tt) => {
                let mut sub_program_vec = syntax_to_expression(sub_tt)?;
                if sub_program_vec.len() == 1 {

                    let sub_program = sub_program_vec.remove(0);
                    expr_stack.push(sub_program);

                } else {

                    return Err(Error {
                        message: format!(
                            "parenthesis should contain 1 expr, these contained {}",
                            sub_program_vec.len()
                        ),
                        location: span::merge_all(sub_tt),
                        kind: ErrorKind::TooManyExprsInParenthesis,
                        next_error: None,
                    });

                }
            }
        };

        'collapse: while prefix_stack.len() > 0 {
            let &Spanned(ref top, _) = &prefix_stack[prefix_stack.len() - 1];

            if expr_stack.len() == (top.beneath + top.requires) {
                let Spanned(top, span) = prefix_stack.pop().unwrap();

                let operands: Vec<ExprSubprogram<'a>> = {
                    let mut vec = Vec::new();
                    for _ in 0..top.requires {
                        vec.push(expr_stack.pop().unwrap());
                    }
                    vec
                };

                let expr: ExprSubprogram<'a> ={
                    let mut vec = operands.into_iter()
                        .flat_map(|expr| expr)
                        .collect::<Vec<_>>();
                    vec.push(Spanned(
                        Instr::Operation(top.op),
                        span,
                    ));
                    vec
                };

                expr_stack.push(expr);
            } else {
                assert!(expr_stack.len() < (top.beneath + top.requires));

                break 'collapse;
            }
        }
    }

    if prefix_stack.len() > 0 {
        let error = Error::chain({
            prefix_stack.into_iter()
                .map(|Spanned(_, span)| Error {
                    message: format!("operator missing sufficient number of operands"),
                    location: span,
                    kind: ErrorKind::NotEnoughOperands,
                    next_error: None,
                })
        }).unwrap();
        return Err(error);
    }

    Ok(expr_stack)
}

/// Token within an expression, categorized between operands and operators.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum ExprToken<'a, 's> {
    Literal(Spanned<'a, PushInstr>),
    Op(Spanned<'a, OpInstr>),
    Scope(&'s [TokenTree<'a>]),
}

/// Helper method for syntax_to_expression.
fn tt_to_expr_token<'a, 's>(token: &'s TokenTree<'a>) -> Result<ExprToken<'a, 's>, Error<'a>> {
    match token {

        &TokenTree::Token(Spanned(token, span)) => {
            match token {

                Token::Operator(inner) => {
                    let op = OpInstr::from(inner);
                    Ok(ExprToken::Op(Spanned(op, span)))
                },

                Token::BitLiteral(inner) => {
                    let table = match inner {
                        BitLiteral::Yes => IoTruthTable::yes_unconditional(),
                        BitLiteral::No => IoTruthTable::no_unconditional(),
                    };
                    let push = PushInstr::Push(table.pack_bitfield());
                    Ok(ExprToken::Literal(Spanned(push, span)))
                },

                Token::IoLiteral(inner) => {
                    let table = match inner {
                        IoLiteral::Input => IoTruthTable::input_conditional(),
                        IoLiteral::Output => IoTruthTable::output_conditional(),
                    };
                    let push = PushInstr::Push(table.pack_bitfield());
                    Ok(ExprToken::Literal(Spanned(push, span)))
                },

                Token::MemoryRead(MemoryRead(offset)) => {
                    let push = PushInstr::ReadThenPush { offset };
                    Ok(ExprToken::Literal(Spanned(push, span)))
                },

                _ => {
                    Err(Error {
                        message: format!("required some expression token, found {:?}", token),
                        location: span,
                        kind: ErrorKind::WrongTokenType,
                        next_error: None,
                    })
                },

            }
        }

        &TokenTree::ParenScope(ref vec) => {
            Ok(ExprToken::Scope(Vec::as_slice(vec)))
        }

    }
}
