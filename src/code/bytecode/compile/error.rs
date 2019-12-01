
use crate::code::span::Span;

/// Compile error.
///
/// References its location in source code.
#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub message: String,
    pub location: Span<'a>,
    pub kind: ErrorKind,
    pub next_error: Option<Box<Error<'a>>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ErrorKind {
    UnbalancedParenthesis,
    WrongTokenType,
    TooManyExprsInParenthesis,
    TooManyExprsInProgram,
    NotEnoughOperands,

    Lexer,
}

impl<'a> Error<'a> {
    /// Chain a sequence of errors into a single error.
    pub fn chain<I: IntoIterator<Item=Self>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();

        let mut head: Error<'a> = match iter.next() {
            Some(head) => head,
            None => return None,
        };

        {
            let mut tail: &mut Option<Box<Error<'a>>> = &mut head.next_error;
            for error in iter {
                *tail = Some(Box::new(error));
                tail = &mut tail.as_mut().unwrap().next_error;
            }
        }

        Some(head)
    }
}