
#[macro_use]
extern crate nom;

pub mod memory;
pub mod code;

use code::bytecode::compile;

fn main() {
    macro_rules! unwrap {
        ($e:expr) => {match $e {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{:#?}", e);
                std::process::exit(1);
            },
        }}
    }

    let code = include_str!("code.bitpit");
    let program = compile::compile(code);
    let program = unwrap!(program);
    println!("{:#?}", program);
}