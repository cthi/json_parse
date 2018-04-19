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
    if io::stdin().read_to_string(&mut buffer).is_ok() {
        let mut lexer = Lexer {
            chars: buffer.chars().peekable(),
        };
        match lexer.lex() {
            Ok(mut tokens) => {
                tokens.retain(|token| token != &Token::Whitespace);
                println!("{:?}", parse_object(&mut tokens.iter().peekable()));
            }
            Err(err) => println!("{:?}", err),
        }
    } else {
        panic!("Error reading input.");
    }
}
