
#[macro_use]
extern crate nom;

pub mod memory;
pub mod code;

use code::tokens::Token;
use code::truthtable::IoTruthTable;

fn main() {
    let mut bitfield = IoTruthTable::yes_unconditional()
        .pack_bitfield();

    bitfield = !bitfield;
    bitfield |= IoTruthTable::input_conditional().pack_bitfield();

    println!("{:#?}", bitfield);

    let IoTruthTable(mut byte) = bitfield;

    let mut b2 = IoTruthTable(&mut byte);
    b2 &= IoTruthTable::new_zeroed_bitfield();

    println!("{:#?}", b2);

    /*
    let code = r##"
    b07afff: ^ y _ n = ~ I ( (( foo foo ^^^ __ "wow!" (( )) )) n * <ff >3 | O (( (( (((()))) )) )) )
    "##;

    let result = code::lexer::lex_verbose_err(code);

    match result {
        Err(e) => eprintln!("ERROR:\n\n{}", e),
        Ok(mut vec) => {


            vec.retain(|token| match token {
                &Token::Whitespace => false,
                &Token::Comment => false,
                _ => true,
            });


            for (i, token) in vec.into_iter().enumerate() {
                println!("{}: {:?}", i, token);
            }
        }
    }
    */
}