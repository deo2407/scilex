#![allow(unused, dead_code)]

use std::env;
use std::io::Read;
use std::fs::File;

use crate::lexer::Lexer;

mod lexer;
mod token;
mod parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect(); 

    if args.len() != 2 {
        return Err("usage: scilex FILENAME".into());
    }

    let filename = &args[1]; 
    let mut file = File::open(filename)?;     

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens = Lexer::lex_all(contents); 

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

