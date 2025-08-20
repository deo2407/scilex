#![allow(unused, dead_code)]

use std::env;
use std::io::Read;
use std::fs::File;

mod lexer;
mod token;
mod parser;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect(); 

    if args.len() != 2 {
        println!("usage: {} FILENAME", args[0]);
    }

    let filename = &args[1]; 
    let mut file = File::open(filename)?;     

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lx = lexer::Lexer::new(contents);

    loop {
        let token = lx.scan_token();
        println!("{:?}", token);

        if token.token_type == token::TokenType::EOF {
            break;
        }
    }

    Ok(())
}
