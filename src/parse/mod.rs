use lex::Token;
use std::iter::Peekable;
use std::slice::Iter;

type Tokens<'a> = Peekable<Iter<'a, Token>>;

#[derive(Debug, PartialEq)]
pub enum Object {
    Empty,
    Nonempty(Box<Members>),
}

#[derive(Debug, PartialEq)]
pub enum Members {
    Pair(String, Value),
    Pairs(String, Value, Box<Members>),
}

#[derive(Debug, PartialEq)]
pub enum Array {
    Empty,
    Nonempty(Box<Elements>),
}

#[derive(Debug, PartialEq)]
pub enum Elements {
    Single(Value),
    Many(Value, Box<Elements>),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(JSONNumber),
    Object(Object),
    Array(Array),
    True,
    False,
    Null,
}

#[derive(Debug, PartialEq)]
pub enum JSONNumber {
    Integer(i32),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ExpectedToken,
}

pub fn parse_object(mut tokens: &mut Tokens) -> Result<Object, ParseError> {
    if tokens
        .next()
        .filter(|t| **t == Token::ObjectStart)
        .is_none()
    {
        return Err(ParseError::ExpectedToken);
    }

    if tokens.peek().filter(|t| ***t == Token::ObjectEnd).is_some() {
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
        if tokens.peek().filter(|t| ***t == Token::Comma).is_none() {
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
        match *t {
            Token::String(ref string) => {
                tokens.next();
                Ok(Value::String(string.clone()))
            }
            Token::Integer(number) => {
                tokens.next();
                Ok(Value::Number(JSONNumber::Integer(number)))
            }
            Token::Float(number) => {
                tokens.next();
                Ok(Value::Number(JSONNumber::Float(number)))
            }
            Token::True => {
                tokens.next();
                Ok(Value::True)
            }
            Token::False => {
                tokens.next();
                Ok(Value::False)
            }
            Token::Null => {
                tokens.next();
                Ok(Value::Null)
            }
            Token::ObjectStart => parse_object(&mut tokens).map(Value::Object),
            Token::ArrayStart => parse_array(&mut tokens).map(Value::Array),
            _ => Err(ParseError::ExpectedToken),
        }
    } else {
        Err(ParseError::ExpectedToken)
    }
}

fn parse_array(mut tokens: &mut Tokens) -> Result<Array, ParseError> {
    if tokens.next().filter(|t| **t == Token::ArrayStart).is_none() {
        return Err(ParseError::ExpectedToken);
    }

    if tokens.peek().filter(|t| ***t == Token::ArrayEnd).is_some() {
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
        if tokens.peek().filter(|t| ***t == Token::Comma).is_none() {
            return Ok(Elements::Single(value));
        }
        tokens.next();
        parse_elements(&mut tokens).map(|elements| Elements::Many(value, Box::new(elements)))
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use lex::Token;

    #[test]
    fn test_parse_value_string() {
        let result = parse_value(&mut vec![Token::String("string".to_string())].iter().peekable());
        assert_eq!(result, Ok(Value::String("string".to_string())));
    }

    #[test]
    fn test_parse_value_number() {
        let result = parse_value(&mut vec![Token::Integer(5)].iter().peekable());
        assert_eq!(result, Ok(Value::Number(JSONNumber::Integer(5))));
    }

    #[test]
    fn test_parse_value_true() {
        let result = parse_value(&mut vec![Token::True].iter().peekable());
        assert_eq!(result, Ok(Value::True));
    }

    #[test]
    fn test_parse_value_false() {
        let result = parse_value(&mut vec![Token::False].iter().peekable());
        assert_eq!(result, Ok(Value::False));
    }

    #[test]
    fn test_parse_value_null() {
        let result = parse_value(&mut vec![Token::Null].iter().peekable());
        assert_eq!(result, Ok(Value::Null));
    }

    #[test]
    fn test_parse_value_no_token() {
        let result = parse_value(&mut vec![].iter().peekable());
        assert_eq!(result, Err(ParseError::ExpectedToken));
    }

    #[test]
    fn test_parse_value_invalid_token() {
        let result = parse_value(&mut vec![Token::ObjectStart].iter().peekable());
        assert_eq!(result, Err(ParseError::ExpectedToken));
    }

    #[test]
    fn test_parse_object_empty() {
        let result =
            parse_object(&mut vec![Token::ObjectStart, Token::ObjectEnd].iter().peekable());
        assert_eq!(result, Ok(Object::Empty));
    }

    #[test]
    fn test_parse_object_member_string() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::String("value".to_string()),
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::String("value".to_string())
            ))))
        );
    }

    #[test]
    fn test_parse_object_members() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key1".to_string()),
            Token::Colon,
            Token::String("value1".to_string()),
            Token::Comma,
            Token::String("key2".to_string()),
            Token::Colon,
            Token::String("value2".to_string()),
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pairs(
                "key1".to_string(),
                Value::String("value1".to_string()),
                Box::new(Members::Pair(
                    "key2".to_string(),
                    Value::String("value2".to_string())
                ))
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_int() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Integer(5),
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::Number(JSONNumber::Integer(5))
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_float() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Float(0.5),
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::Number(JSONNumber::Float(0.5))
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_true() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::True,
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::True
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_false() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::False,
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::False
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_null() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Null,
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::Null
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_array_empty() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::ArrayStart,
            Token::ArrayEnd,
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::Array(Array::Empty)
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_array_element() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::ArrayStart,
            Token::Integer(5),
            Token::ArrayEnd,
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::Array(Array::Nonempty(Box::new(Elements::Single(Value::Number(
                    JSONNumber::Integer(5)
                )))))
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_array_elements() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::ArrayStart,
            Token::Integer(5),
            Token::Comma,
            Token::String("elements".to_string()),
            Token::ArrayEnd,
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::Array(Array::Nonempty(Box::new(Elements::Many(
                    Value::Number(JSONNumber::Integer(5)),
                    Box::new(Elements::Single(Value::String("elements".to_string())))
                ))))
            ))))
        );
    }

    #[test]
    fn test_parse_object_member_object() {
        let result = parse_object(&mut vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::ObjectStart,
            Token::ObjectEnd,
            Token::ObjectEnd,
        ].iter()
            .peekable());
        assert_eq!(
            result,
            Ok(Object::Nonempty(Box::new(Members::Pair(
                "key".to_string(),
                Value::Object(Object::Empty)
            ))))
        );
    }
}
