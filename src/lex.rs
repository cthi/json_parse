use alias::Characters;

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
}

pub trait Lexer {
    fn lex(&self) -> Vec<Token>;
}

impl Lexer for String {
    fn lex(&self) -> Vec<Token> {
        let mut chars = self.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        while let Some(&ch) = chars.peek() {
            match ch {
                '{' => {
                    chars.next();
                    tokens.push(Token::ObjectStart);
                }
                '}' => {
                    chars.next();
                    tokens.push(Token::ObjectEnd);
                }
                '[' => {
                    chars.next();
                    tokens.push(Token::ArrayStart);
                }
                ']' => {
                    chars.next();
                    tokens.push(Token::ArrayEnd);
                }
                ',' => {
                    chars.next();
                    tokens.push(Token::Comma);
                }
                ':' => {
                    chars.next();
                    tokens.push(Token::Colon);
                }
                '"' => lex_string(&mut chars, &mut tokens),
                't' => lex_true(&mut chars, &mut tokens),
                'f' => lex_false(&mut chars, &mut tokens),
                'n' => lex_null(&mut chars, &mut tokens),
                '\n' | '\t' | '\r' | ' ' => {
                    chars.next();
                    tokens.push(Token::Whitespace);
                }
                _ => panic!("Invalid character"),
            }
        }
        tokens
    }
}

fn lex_string(chars: &mut Characters, tokens: &mut Vec<Token>) {
    let mut string = String::new();

    if chars.next() != Some('"') {
        panic!("Invalid character");
    }

    loop {
        match chars.next() {
            Some(ch) => match ch {
                '"' => break,
                /*          '\' => tokens.push_str(lex_escapes(&mut chars)) */
                c => string.push(c),
            },
            None => panic!("Invalid Character"),
        }
    }

    tokens.push(Token::String(string))
}

/*
fn lex_escapes(chars: &mut Characters) {
  match chars.next() {
    Some(&ch) => match ch {
      '"' => String::new("\""),
      '\' => String::new("\\"),
      '/' => String::new("\/"),
      'b' => String::new("\b"),
      'f' => String::new("\f"),
      'n' => String::new("\n"),
      'r' => String::new("\r"),
      't' => String::new("\t"),
      'u' => lex_hex_digits(chars: &mut Characters),
      _ => panic!("Invalid Character"),
    },
    _ => panic!("Invalid Character")
  }
}

fn lex_hex_digits(chars: &mut Characters) {
  let digits = String::new();

  match chars.next() {
    Some(&ch) => match ch {
      '0' .. '9' => {  
    }
  }

  digits
}
*/
fn lex_true(chars: &mut Characters, tokens: &mut Vec<Token>) {
    match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some('t'), Some('r'), Some('u'), Some('e')) => tokens.push(Token::True),
        _ => panic!("Invalid character"),
    }
}

fn lex_false(chars: &mut Characters, tokens: &mut Vec<Token>) {
    match (
        chars.next(),
        chars.next(),
        chars.next(),
        chars.next(),
        chars.next(),
    ) {
        (Some('f'), Some('a'), Some('l'), Some('s'), Some('e')) => tokens.push(Token::False),
        _ => panic!("Invalid character"),
    }
}

fn lex_null(chars: &mut Characters, tokens: &mut Vec<Token>) {
    match (chars.next(), chars.next(), chars.next(), chars.next()) {
        (Some('n'), Some('u'), Some('l'), Some('l')) => tokens.push(Token::Null),
        _ => panic!("Invalid character"),
    }
}
