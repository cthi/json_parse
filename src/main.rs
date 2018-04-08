#![feature(option_filter)]
use std::io::{self, Read};

mod alias;
mod lex;
mod parse;

use lex::Lexer;
use lex::Token;
use parse::parse_object;

fn main() {
    let mut buffer = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut buffer) {
        let mut tokens = buffer.lex();
        tokens.retain(|&ref token| token != &Token::Whitespace);

        println!("{:?}", parse_object(&mut tokens.iter().peekable()));
    } else {
        panic!("Error reading input.");
    }
}
