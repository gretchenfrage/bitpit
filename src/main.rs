
#[macro_use]
extern crate nom;

mod memory;
mod code;

use code::tokens::Token;

fn main() {
    let code = r##"
    fff: ^ y _ n = ~ I ( (( foo foo ^^^ __ "wow!" )) n | O (()) )
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
}
