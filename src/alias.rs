use std;
use lex::Token;

pub type Characters<'a> = std::iter::Peekable<std::str::Chars<'a>>;
pub type Tokens<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
