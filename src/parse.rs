use alias::Tokens;
use lex::Token;

#[derive(Debug)]
pub enum Object {
    Empty,
    Nonempty(Box<Members>),
}

#[derive(Debug)]
pub enum Members {
    Pair(String, Value),
    Pairs(String, Value, Box<Members>),
}

#[derive(Debug)]
pub enum Array {
    Empty,
    Nonempty(Box<Elements>),
}

#[derive(Debug)]
pub enum Elements {
    Single(Value),
    Many(Value, Box<Elements>),
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(i32),
    Object(Object),
    Array(Array),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedToken,
}

pub fn parse_object(mut tokens: &mut Tokens) -> Result<Object, ParseError> {
    if let None = tokens.next().filter(|t| **t == Token::ObjectStart) {
        return Err(ParseError::ExpectedToken);
    }

    if let Some(_) = tokens.peek().filter(|t| ***t == Token::ObjectEnd) {
        tokens.next();
        return Ok(Object::Empty);
    }

    parse_members(&mut tokens).and_then(|members| {
        tokens
            .next()
            .filter(|t| **t == Token::ObjectEnd)
            .map_or(Err(ParseError::ExpectedToken), |_| {
                Ok(Object::Nonempty(Box::new(members)))
            })
    })
}

fn parse_members(mut tokens: &mut Tokens) -> Result<Members, ParseError> {
    parse_pair(&mut tokens).and_then(|(key, value)| {
        if let None = tokens.peek().filter(|t| ***t == Token::Comma) {
            return Ok(Members::Pair(key, value));
        }
        tokens.next();
        parse_members(&mut tokens).map(|members| Members::Pairs(key, value, Box::new(members)))
    })
}

fn parse_pair(mut tokens: &mut Tokens) -> Result<(String, Value), ParseError> {
    match (tokens.next(), tokens.next()) {
        (Some(&Token::String(ref key)), Some(&Token::Colon)) => {
            parse_value(&mut tokens).map(|value| (key.clone(), value))
        }
        _ => Err(ParseError::ExpectedToken),
    }
}

fn parse_value(mut tokens: &mut Tokens) -> Result<Value, ParseError> {
    if let Some(t) = tokens.peek().map(|t| *t) {
        match t {
            &Token::String(ref string) => {
                tokens.next();
                Ok(Value::String(string.clone()))
            }
            &Token::Integer(number) => {
                tokens.next();
                Ok(Value::Number(number))
            }
            &Token::True => {
                tokens.next();
                Ok(Value::True)
            }
            &Token::False => {
                tokens.next();
                Ok(Value::False)
            }
            &Token::Null => {
                tokens.next();
                Ok(Value::Null)
            }
            &Token::ObjectStart => parse_object(&mut tokens).map(|obj| Value::Object(obj)),
            &Token::ArrayStart => parse_array(&mut tokens).map(|arr| Value::Array(arr)),
            _ => Err(ParseError::ExpectedToken),
        }
    } else {
        Err(ParseError::ExpectedToken)
    }
}

fn parse_array(mut tokens: &mut Tokens) -> Result<Array, ParseError> {
    if let None = tokens.next().filter(|t| **t == Token::ArrayStart) {
        return Err(ParseError::ExpectedToken);
    }

    if let Some(_) = tokens.peek().filter(|t| ***t == Token::ArrayEnd) {
        tokens.next();
        return Ok(Array::Empty);
    }

    parse_elements(&mut tokens).and_then(|elements| {
        tokens
            .next()
            .filter(|t| **t == Token::ArrayEnd)
            .map_or(Err(ParseError::ExpectedToken), |_| {
                Ok(Array::Nonempty(Box::new(elements)))
            })
    })
}

fn parse_elements(mut tokens: &mut Tokens) -> Result<Elements, ParseError> {
    parse_value(&mut tokens).and_then(|value| {
        if let None = tokens.peek().filter(|t| ***t == Token::Comma) {
            return Ok(Elements::Single(value));
        }
        tokens.next();
        parse_elements(&mut tokens).map(|elements| Elements::Many(value, Box::new(elements)))
    })
}
