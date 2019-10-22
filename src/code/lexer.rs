
use super::tokens::*;

use std::char;

use nom::error::{VerboseError, ParseError};
use nom::{IResult, InputLength};
use nom::character::complete::hex_digit1;
use nom::character::complete::anychar;

named!(
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

named!(
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

named!(
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

named!(
    hex_u128(&str) -> u128,
    map!(
        nom::character::complete::hex_digit1,
        hex_str_to_u128
    )
);

named!(
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

named!(
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

named!(
    colon(&str) -> (),
    map!(
        char!(':'),
        |_| ()
    )
);

named!(
    whitespace(&str) -> (),
    map!(
        eat_separator!(" \t\n"),
        |_| ()
    )
);

fn comment<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str>,
        E: From<(&'a str, nom::error::ErrorKind)>, {

    named!(
        layer(&str) -> i32,
        preceded!(
            tag!("(("),
            map!(
                many_till!(
                    anychar,
                    alt!(
                        map!(tag!("))"), |_| -1_i32) |
                        map!(peek!(tag!("((")), |_| 1_i32)
                    )
                ),
                |(_, delta)| delta
            )

        )
    );

    let (mut remaining, delta) = layer(input).map_err(nom::Err::convert)?;

    let mut depth: i32 = 1 - delta;
    while depth > 0 {
        let (remaining2, delta) = layer(remaining).map_err(nom::Err::convert)?;
        depth += delta;
        remaining = remaining2;
    }

    Ok((remaining, ()))
}

named!(
    token(&str) -> Token,
    alt!(
        map!(operator, Token::Operator) |
        map!(bit_literal, Token::BitLiteral) |
        map!(io_literal, Token::IoLiteral) |
        map!(memory_read, Token::MemoryRead) |
        map!(parenthesis, Token::Parenthesis) |
        map!(hex_u128, |n| Token::ActivationPattern(ActivationPattern(n))) |
        map!(colon, |_| Token::Colon) |
        map!(whitespace, |_| Token::Whitespace) |
        map!(comment, |_| Token::Comment)
    )
);

named!(
    tokens_complete(&str) -> Vec<Token>,
    complete!(
        terminated!(
            many0!(
                token
            ),
            eof!()
        )
    )
);

pub fn lex<'a, E>(code: &'a str) -> Result<Vec<Token>, E>
    where
        E: ParseError<&'a str>,
        E: From<(&'a str, nom::error::ErrorKind)>, {

    tokens_complete(code)
        .map(|(rem, vec)| {
            assert!(rem.is_empty());
            vec
        })
        .map_err(nom::Err::convert)
        .map_err(|nom_error| match nom_error {
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
            nom::Err::Incomplete(_) => unreachable!(),
        })

}

pub fn lex_verbose_err(code: &str) -> Result<Vec<Token>, (&str, nom::error::ErrorKind)> {
    lex(code)
}