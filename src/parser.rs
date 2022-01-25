use super::tokenizer::Token;

pub enum ParseError {
    IsNotString,
    IsNotNumber,
    IsNotBool,
    IsNotNull,
    InvalidToken,
    UnexpectedToken,
}
pub trait Node {
    fn print_node(&self) -> String;
}

pub struct ObjectNode {
    key: Box<dyn Node>,
    value: Box<dyn Node>,
}

impl Node for ObjectNode {
    fn print_node(&self) -> String {
        return format!("{{{}:{}}}", self.key.print_node(), self.value.print_node());
    }
}

pub struct StringNode {
    value: String,
}

impl Node for StringNode {
    fn print_node(&self) -> String {
        return format!("{}", self.value);
    }
}

pub struct NumberNode {
    value: String,
}

impl Node for NumberNode {
    fn print_node(&self) -> String {
        return format!("{}", self.value);
    }
}

pub struct BoolNode {
    value: String,
}

impl Node for BoolNode {
    fn print_node(&self) -> String {
        return format!("{}", self.value);
    }
}

pub struct NullNode {
    value: String,
}

impl Node for NullNode {
    fn print_node(&self) -> String {
        return format!("{}", self.value);
    }
}

fn expect_token(token_type: Token, token_list: &Vec<Token>, index: &mut usize) -> bool {
    if token_type == token_list[*index] {
        *index += 1;
        true
    } else {
        false
    }
}

pub fn parse(token_list: Vec<Token>) -> Result<Box<dyn Node>, ParseError> {
    let mut index = 0;
    return parse_object(&token_list, &mut index);
}

pub fn parse_object(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    let key_resutl;
    let value_result;
    if !expect_token(Token::LeftBracket, token_list, index) {
        return Err(ParseError::UnexpectedToken);
    }

    match parse_string(token_list, index) {
        Ok(key) => {
            key_resutl = key;
        }
        Err(err) => return Err(err),
    }
    if !expect_token(Token::Colorn, token_list, index) {
        return Err(ParseError::UnexpectedToken);
    }

    match parse_value(token_list, index) {
        Ok(value) => {
            value_result = value;
        }
        Err(err) => return Err(err),
    }

    if !expect_token(Token::Colorn, token_list, index) {
        return Err(ParseError::UnexpectedToken);
    }

    return Ok(Box::new(ObjectNode {
        key: key_resutl,
        value: value_result,
    }));
}

pub fn parse_value(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    match token_list[*index] {
        Token::Str(_) => parse_string(token_list, index),
        Token::Num(_) => parse_number(token_list, index),
        Token::Bool(_) => parse_bool(token_list, index),
        Token::Null => parse_null(token_list, index),
        _ => Err(ParseError::InvalidToken),
    }
}

pub fn parse_string(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    if let Token::Str(string) = &token_list[*index] {
        *index += 1;
        Ok(Box::new(StringNode {
            value: string.clone(),
        }))
    } else {
        Err(ParseError::IsNotString)
    }
}

pub fn parse_number(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    if let Token::Num(string) = &token_list[*index] {
        *index += 1;
        Ok(Box::new(NumberNode {
            value: string.clone(),
        }))
    } else {
        Err(ParseError::IsNotNumber)
    }
}

pub fn parse_bool(token_list: &Vec<Token>, index: &mut usize) -> Result<Box<dyn Node>, ParseError> {
    if let Token::Bool(string) = &token_list[*index] {
        *index += 1;
        Ok(Box::new(BoolNode {
            value: string.clone(),
        }))
    } else {
        Err(ParseError::IsNotBool)
    }
}

pub fn parse_null(token_list: &Vec<Token>, index: &mut usize) -> Result<Box<dyn Node>, ParseError> {
    if let Token::Null = &token_list[*index] {
        *index += 1;
        Ok(Box::new(NullNode {
            value: "null".to_string(),
        }))
    } else {
        Err(ParseError::IsNotNull)
    }
}
