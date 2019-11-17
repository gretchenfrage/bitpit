
#[macro_use]
extern crate nom;

pub mod memory;
pub mod code;

use code::tokens::Token;
use code::span::*;
use code::truthtable::IoTruthTable;

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

    let code = r##"
    b07afff: ^ y _ n = ~ I ( (( foo foo ^^^ __ "wow!" (( )) )) n * <ff >3 | O (( (( (((()))) )) )) )
    "##;

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
            println!("{:#?}", parts);


            /*

            for (i, token) in vec.into_iter().enumerate() {
                println!("{}: {:?}", i, token);
            }
            */
        }
    }
}