
use super::tokens::*;
use super::span::{self, Spanned, Span};

use std::char;

use nom::IResult;
use nom::error::{VerboseError, ParseError, convert_error};
use nom::character::complete::anychar;

pub fn lex<'a, E>(code: &'a str) -> Result<Vec<Spanned<'a, Token>>, E>
    where
        E: ParseError<&'a str>, {

    many0!(code, complete!(token))
        .map_err(nom::Err::convert)
        .map_err(|nom_error| match nom_error {
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
            nom::Err::Incomplete(_e) => unreachable!(),
        })
        .and_then(|(rem, vec)| {
            if rem.is_empty() {
                Ok(vec)
            } else {
                Err(E::from_error_kind(rem, nom::error::ErrorKind::Alt))
            }
        })

}

pub fn lex_verbose_err(code: &str) -> Result<Vec<Spanned<Token>>, String> {
    lex::<VerboseError<_>>(code)
        .map_err(|e| convert_error(code, e))
}

macro_rules! spanned {
    ($i:expr, $submac:ident!( $($args:tt)* )) => {{
        let input = $i;
        match $submac!(input, $($args)* ) {
            Err(e) => Err(e),
            Ok((rem, elem)) => {
                let span = span::until(input, rem);
                Ok((
                    rem,
                    Spanned(elem, span),
                ))
            },
        }
    //spanned!($i, |i| $submac!(i, $($args)*))
    }};
    ($i:expr, $f:expr) => {{
        let input = $i;
        match $f(input) {
            Err(e) => Err(e),
            Ok((rem, elem)) => {
                let span = span::until(input, rem);
                Ok((
                    rem,
                    Spanned(elem, span),
                ))
            },
        }
    }};
}

macro_rules! named_any_err {
    (
        $name:ident(&str) -> $output:ty,
        $submac:ident!( $($args:tt)* )
    ) => {
        fn $name<'a, E>(input: &'a str) -> IResult<&'a str, $output, E>
            where
                E: ParseError<&'a str> {

            $submac!(input, $($args)*)

        }
    };
}

named_any_err!(
    operator(&str) -> Spanned<Operator>,
    spanned!(map!(
        one_of!("&|^~=_"),
        |c: char| match c {
            '&' => Operator::Both,
            '|' => Operator::Either,
            '^' => Operator::Different,
            '~' => Operator::Not,
            '=' => Operator::Same,
            '_' => Operator::Neither,
            _ => unreachable!()
        }
    ))
);

named_any_err!(
    bit_literal(&str) -> Spanned<BitLiteral>,
    spanned!(map!(
        one_of!("yn"),
        |c: char| match c {
            'y' => BitLiteral::Yes,
            'n' => BitLiteral::No,
            _ => unreachable!()
        }
    ))
);

named_any_err!(
    io_literal(&str) -> Spanned<IoLiteral>,
    spanned!(map!(
        one_of!("IO"),
        |c: char| match c {
            'I' => IoLiteral::Input,
            'O' => IoLiteral::Output,
            _ => unreachable!()
        }
    ))
);

/// Panics if invalid.
fn hex_str_to_u128(string: &str) -> u128 {
    let mut accum: u128 = 0;
    let mut scale: u128 = 1;

    for digit_char in string.chars().rev() {
        let digit_u32: u32 = digit_char.to_digit(16)
            .expect("invalid hex digit");

        accum += digit_u32 as u128 * scale;
        scale *= 16;
    }

    accum
}

named_any_err!(
    hex_u128(&str) -> Spanned<u128>,
    spanned!(map!(
        nom::character::complete::hex_digit1,
        hex_str_to_u128
    ))
);

named_any_err!(
    memory_read(&str) -> Spanned<MemoryRead>,
    spanned!(switch!(
        one_of!("*><"),
        '*' => value!(MemoryRead(0)) |
        '>' => map!(
            hex_u128,
            |offset: Spanned<u128>| MemoryRead(offset.0 as i128)
        ) |
        '<' => map!(
            hex_u128,
            |offset: Spanned<u128>| MemoryRead(offset.0 as i128 * -1)
        )
    ))
);

named_any_err!(
    parenthesis(&str) -> Spanned<Parenthesis>,
    spanned!(map!(
        one_of!("()"),
        |c: char| match c {
            '(' => Parenthesis::Open,
            ')' => Parenthesis::Close,
            _ => unreachable!()
        }
    ))
);

named_any_err!(
    colon(&str) -> Spanned<()>,
    spanned!(map!(
        char!(':'),
        |_| ()
    ))
);

named_any_err!(
    whitespace(&str) -> Spanned<()>,
    spanned!(whitespace_nospan)
);

fn whitespace_nospan<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str>, {

    let mut split_index = None;

    for (index, c) in input.char_indices() {
        match c {
            ' ' | '\n' | '\t' => {
                split_index = Some(index);
            },
            _c => {
                break;
            }
        };
    }

    match split_index {
        None => {
            Err(nom::Err::Error(
                E::from_error_kind(input, nom::error::ErrorKind::Many0)
            ))
        },
        Some(index) => {
            let (_, rest) = input.split_at(index + 1);
            Ok((rest, ()))
        }
    }
}

named_any_err!(
    comment(&str) -> Spanned<()>,
    spanned!(comment_nospan)
);

fn comment_nospan<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str>, {

    let (mut remaining, _) = tag!(input, "((")?;

    named_any_err!(
        iteration(&str) -> i32,
        complete!(
            map!(
                many_till!(
                    anychar,
                    alt!(
                        map!(tag!("))"), |_| -1) |
                        map!(tag!("(("), |_| 1)
                    )
                ),
                |(_, delta)| delta
            )
        )
    );

    let mut depth: i32 = 1;
    while depth > 0 {
        let (remaining2, delta) = iteration(remaining).map_err(nom::Err::convert)?;

        remaining = remaining2;
        depth += delta;
    }

    Ok((remaining, ()))
}

named_any_err!(
    token(&str) -> Spanned<Token>,
    alt!(
        // good for comment to be first
        complete!( map!(comment, span::mapping(|_| Token::Comment)) ) |

        complete!( map!(operator, span::mapping(Token::Operator)) ) |
        complete!( map!(bit_literal, span::mapping(Token::BitLiteral)) ) |
        complete!( map!(io_literal, span::mapping(Token::IoLiteral)) ) |
        complete!( map!(memory_read, span::mapping(Token::MemoryRead)) ) |
        complete!( map!(parenthesis, span::mapping(Token::Parenthesis)) ) |
        complete!( map!(hex_u128, span::mapping(
                |n| Token::ActivationPattern(ActivationPattern(n))
            )) ) |
        complete!( map!(colon, span::mapping(|_| Token::Colon)) ) |
        complete!( map!(whitespace, span::mapping(|_| Token::Whitespace)) )
    )
);
