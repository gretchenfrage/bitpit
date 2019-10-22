
use super::tokens::*;

use std::char;

use nom::error::{VerboseError, ParseError, convert_error};
use nom::{IResult, InputLength};
use nom::character::complete::hex_digit1;
use nom::character::complete::anychar;

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
    operator(&str) -> Operator,
    map!(
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
    )
);

named_any_err!(
    bit_literal(&str) -> BitLiteral,
    map!(
        one_of!("yn"),
        |c: char| match c {
            'y' => BitLiteral::Yes,
            'n' => BitLiteral::No,
            _ => unreachable!()
        }
    )
);

named_any_err!(
    io_literal(&str) -> IoLiteral,
    map!(
        one_of!("IO"),
        |c: char| match c {
            'I' => IoLiteral::Input,
            'O' => IoLiteral::Output,
            _ => unreachable!()
        }
    )
);

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
    hex_u128(&str) -> u128,
    map!(
        nom::character::complete::hex_digit1,
        hex_str_to_u128
    )
);

named_any_err!(
    memory_read(&str) -> MemoryRead,
    switch!(
        one_of!("*><"),
        '*' => value!(MemoryRead(0)) |
        '>' => map!(
            hex_u128,
            |offset: u128| MemoryRead(offset as i128)
        ) |
        '<' => map!(
            hex_u128,
            |offset: u128| MemoryRead(offset as i128 * -1)
        )
    )
);

named_any_err!(
    parenthesis(&str) -> Parenthesis,
    map!(
        one_of!("()"),
        |c: char| match c {
            '(' => Parenthesis::Open,
            ')' => Parenthesis::Close,
            _ => unreachable!()
        }
    )
);

named_any_err!(
    colon(&str) -> (),
    map!(
        char!(':'),
        |_| ()
    )
);

fn whitespace<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str>, {

    let mut split_index = None;

    for (index, c) in input.char_indices() {
        match c {
            ' ' | '\n' | '\t' => {
                split_index = Some(index);
            },
            c => {
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

fn comment<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str>, {

    named_any_err!(
        layer(&str) -> i32,
        preceded!(
            tag!("(("),
            map!(
                many_till!(
                    anychar,
                    alt!(
                        map!(complete!(tag!("))")), |_| -1_i32) |
                        map!(peek!(complete!(tag!("(("))), |_| 1_i32)
                    )
                ),
                |(_, delta)| delta
            )

        )
    );

    let (mut remaining, delta) = layer(input).map_err(nom::Err::convert)?;

    let mut depth: i32 = 1 + delta;
    while depth > 0 {
        let (remaining2, delta) = layer(remaining).map_err(nom::Err::convert)?;
        depth += delta;
        remaining = remaining2;
    }

    Ok((remaining, ()))
}

named_any_err!(
    token(&str) -> Token,
    alt!(
        // good for comment to be first
        complete!( map!(comment, |_| Token::Comment) ) |

        complete!( map!(operator, Token::Operator) ) |
        complete!( map!(bit_literal, Token::BitLiteral) ) |
        complete!( map!(io_literal, Token::IoLiteral) ) |
        complete!( map!(memory_read, Token::MemoryRead) ) |
        complete!( map!(parenthesis, Token::Parenthesis) ) |
        complete!( map!(hex_u128, |n| Token::ActivationPattern(ActivationPattern(n))) ) |
        complete!( map!(colon, |_| Token::Colon) ) |
        complete!( map!(whitespace, |_| Token::Whitespace) )
    )
);

pub fn lex<'a, E>(code: &'a str) -> Result<Vec<Token>, E>
    where
        E: ParseError<&'a str>, {

    many0!(code, complete!(token))
        .map_err(nom::Err::convert)
        .map_err(|nom_error| match nom_error {
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
            nom::Err::Incomplete(e) => unreachable!(),
        })
        .and_then(|(rem, vec)| {
            if rem.is_empty() {
                Ok(vec)
            } else {
                println!("rem = {:#?}", rem);
                Err(E::from_error_kind(rem, nom::error::ErrorKind::Alt))
            }
        })

}

pub fn lex_verbose_err(code: &str) -> Result<Vec<Token>, String> {
    lex::<VerboseError<_>>(code)
        .map_err(|e| convert_error(code, e))
}
