
#[macro_use]
extern crate nom;

pub mod memory;
pub mod code;

use code::tokens::Token;
use code::span::*;
use code::truthtable::IoTruthTable;
use code::bytecode::compile::{self, ProgramParts};

fn main() {
    /*
    let mut bitfield = IoTruthTable::yes_unconditional()
        .pack_bitfield();

    bitfield = !bitfield;
    bitfield |= IoTruthTable::input_conditional().pack_bitfield();

    println!("{:#?}", bitfield);

    let IoTruthTable(mut byte) = bitfield;

    let mut b2 = IoTruthTable(&mut byte);
    b2 &= IoTruthTable::new_zeroed_bitfield();
    b2.copy_from(!b2.owned());

    println!("{:#?}", b2);
    */

    /*

    let a = IoTruthTable::yes_unconditional().pack_bitfield();
    let b = IoTruthTable::input_conditional().pack_bitfield();

    println!("{:#?}", (a, b));

    println!("{:#?}", a | b);

    println!("{:#?}", a & b);

    println!("{:#?}", a ^ b);

    println!("{:#?}", !(a ^ b));
    */

    let code = include_str!("code.bitpit");

    macro_rules! unwrap {
        ($e:expr) => {match $e {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{:#?}", e);
                std::process::exit(1);
            },
        }}
    }

    let tokens = code::lexer::lex_verbose_err(code);
    let mut tokens = unwrap!(tokens);
    println!("tokens:");
    for &Spanned(t, s) in &tokens {
        println!("- {:?}", s);
    }
    println!();

    tokens.retain(|&Spanned(ref token, _)| match token {
        &Token::Whitespace => false,
        &Token::Comment => false,
        _ => true,
    });

    let tt = compile::inner::parse_scopes(tokens.iter().cloned());
    let tt = unwrap!(tt);
    println!("token tree:");
    compile::debug::print_tt(&tt, true);
    println!();

    let parts = compile::inner::program_parts(&tt);
    let parts = unwrap!(parts);
    println!("program parts:");
    println!("{:?}", parts);
    println!();

    let expressions = compile::inner::syntax_to_expression(&parts.prefix_rule);
    let expressions = unwrap!(expressions);

    for (i, v) in expressions.iter().enumerate() {
        println!("expr #{}", i);
        for &Spanned(i, s) in v {
            println!("- {:?}", s);
        }
        println!();
    }

    /*

    let result = code::lexer::lex_verbose_err(code);

    match result {
        Err(e) => eprintln!("ERROR:\n\n{}", e),
        Ok(mut vec) => {


            vec.retain(|&Spanned(ref token, _)| match token {
                &Token::Whitespace => false,
                &Token::Comment => false,
                _ => true,
            });

            let scopes = code::bytecode::compile::parse_scopes(vec);

            let scopes = scopes
                .map_err(|e| println!("error: {:#?}", e))
                .unwrap();


            code::bytecode::compile::print_tt(&scopes, false);

            let parts = code::bytecode::compile::program_parts(&scopes)
                .map_err(|e| println!("error: {:#?}", e))
                .unwrap();
            //println!("{:#?}", parts);

            let codes = code::bytecode::compile::syntax_to_expression(&parts.prefix_rule)
                .map_err(|e| println!("error: {:#?}", e))
                .unwrap();

            std::dbg!(codes.len());
            //\println!("{:#?}", codes);


            /*

            for (i, token) in vec.into_iter().enumerate() {
                println!("{}: {:?}", i, token);
            }
            */
        }
    }
    */
}