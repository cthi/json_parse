use lex::Token;
use std;

pub type Tokens<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
