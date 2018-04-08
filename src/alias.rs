use lex::Token;
use std;

pub type Characters<'a> = std::iter::Peekable<std::str::Chars<'a>>;
pub type Tokens<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
