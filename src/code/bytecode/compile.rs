
use crate::code::tokens::*;
use crate::code::span::{self, Span, Spanned};

#[derive(Debug)]
pub struct Error {
    pub message: String,
    //pub location: ErrorLocation,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorLocation {
    // unimplemented
}

#[derive(Debug)]
pub enum ErrorKind {
    UnbalancedParenthesis,
}

/// A representation of tokens which can represent recursive parenthesis
/// scoping. The actual Token variant should not contain a parenthesis
/// token.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TokenTree<'a> {
    Token(Spanned<'a, Token>),
    ParenScope(Vec<TokenTree<'a>>),
}

pub fn parse_scopes<'a, T>(tokens: T) -> Result<Vec<TokenTree<'a>>, Error>
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
                        kind: ErrorKind::UnbalancedParenthesis,
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
            kind: ErrorKind::UnbalancedParenthesis,
        });
    }

    // success
    assert_eq!(scope_stack.len(), 1);
    return Ok(scope_stack.pop().unwrap());
}