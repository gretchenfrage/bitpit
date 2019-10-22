
#[macro_use]
extern crate nom;

mod memory;
mod code;

fn main() {
    let code = r##"

    fff: ^ y _ n = ~ I ( (( foo foo ^^^ __ "wow!" )) n | O (()))

    "##;

    let result = code::lexer::lex_verbose_err(code);

    match result {
        Err(e) => eprintln!("ERROR: {:#?}", e),
        Ok(vec) => {
            for (i, token) in vec.into_iter().enumerate() {
                println!("{}: {:?}", i, token);
            }
        }
    }
}
