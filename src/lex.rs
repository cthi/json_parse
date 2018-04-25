use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    True,
    False,
    Null,
    Comma,
    Colon,
    Whitespace,
    Integer(i32),
    Float(f64),
    String(String),
    NoMoreTokens,
}

#[derive(Debug)]
pub enum LexError {
    InvalidToken,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            match next(&mut self.chars) {
                Ok(Token::NoMoreTokens) => break,
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            }
        }

        Ok(tokens)
    }
}

fn next(mut chars: &mut Peekable<Chars>) -> Result<Token, LexError> {
    if let Some(&ch) = chars.peek() {
        match ch {
            '{' => {
                chars.next();
                Ok(Token::ObjectStart)
            }
            '}' => {
                chars.next();
                Ok(Token::ObjectEnd)
            }
            '[' => {
                chars.next();
                Ok(Token::ArrayStart)
            }
            ']' => {
                chars.next();
                Ok(Token::ArrayEnd)
            }
            ',' => {
                chars.next();
                Ok(Token::Comma)
            }
            ':' => {
                chars.next();
                Ok(Token::Colon)
            }
            '0' => {
                chars.next();
                Ok(Token::Integer(0))
            }
            '1'...'9' => lex_number(&mut chars),
            '"' => lex_string(&mut chars),
            't' => lex_true(&mut chars),
            'f' => lex_false(&mut chars),
            'n' => lex_null(&mut chars),
            '\n' | '\t' | '\r' | ' ' => {
                chars.next();
                Ok(Token::Whitespace)
            }
            _ => Err(LexError::InvalidToken),
        }
    } else {
        Ok(Token::NoMoreTokens)
    }
}

fn lex_string(chars: &mut Peekable<Chars>) -> Result<Token, LexError> {
    let mut string = String::new();

    if chars.next() != Some('"') {
        return Err(LexError::InvalidToken);
    }

    loop {
        if let Some(ch) = chars.next() {
            if ch == '"' {
                break;
            } else {
                string.push(ch);
            }
        } else {
            return Err(LexError::InvalidToken);
        }
    }

    Ok(Token::String(string))
}

fn lex_true(chars: &mut Peekable<Chars>) -> Result<Token, LexError> {
    match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some('t'), Some('r'), Some('u'), Some('e')) => Ok(Token::True),
        _ => Err(LexError::InvalidToken),
    }
}

fn lex_false(chars: &mut Peekable<Chars>) -> Result<Token, LexError> {
    match (
        chars.next(),
        chars.next(),
        chars.next(),
        chars.next(),
        chars.next(),
    ) {
        (Some('f'), Some('a'), Some('l'), Some('s'), Some('e')) => Ok(Token::False),
        _ => Err(LexError::InvalidToken),
    }
}

fn lex_null(chars: &mut Peekable<Chars>) -> Result<Token, LexError> {
    match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some('n'), Some('u'), Some('l'), Some('l')) => Ok(Token::Null),
        _ => Err(LexError::InvalidToken),
    }
}

fn lex_number(chars: &mut Peekable<Chars>) -> Result<Token, LexError> {
    if let Ok(Token::Integer(integer)) = lex_digits(chars) {
        if let Some('.') = chars.peek() {
            chars.next();
            if let Ok(Token::Integer(decimal)) = lex_digits(chars) {
                if let Ok(f) = [integer.to_string(), decimal.to_string()]
                    .join(".")
                    .parse::<f64>()
                {
                    Ok(Token::Float(f))
                } else {
                    Err(LexError::InvalidToken)
                }
            } else {
                Err(LexError::InvalidToken)
            }
        } else {
            Ok(Token::Integer(integer))
        }
    } else {
        Err(LexError::InvalidToken)
    }
}

fn lex_digits(chars: &mut Peekable<Chars>) -> Result<Token, LexError> {
    if chars.peek().map(|c| c.is_digit(10)).is_none() {
        return Err(LexError::InvalidToken);
    }
    let mut digits = 0;
    while let Some(Some(digit)) = chars.peek().map(|c| c.to_digit(11)) {
        digits = digits * 10 + digit;
        chars.next();
    }
    Ok(Token::Integer(digits as i32))
}
