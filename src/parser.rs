use super::tokenizer::Token;

pub enum ParseError {
    IsNotString,
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

fn expect_token(
    token_type: Token,
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<(), ParseError> {
    if token_type == token_list[*index] {
        *index += 1;
        Ok(())
    } else {
        Err(ParseError::UnexpectedToken)
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
    expect_token(Token::LeftBracket, token_list, index)?;
    let key: Box<dyn Node> = parse_string(token_list, index)?;
    expect_token(Token::Colorn, token_list, index)?;
    let value = parse_value(token_list, index)?;
    expect_token(Token::RightBracket, token_list, index)?;
    return Ok(Box::new(ObjectNode {
        key: key,
        value: value,
    }));
}

pub fn parse_value(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    match token_list[*index] {
        Token::Str(_) => parse_string(token_list, index),
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
