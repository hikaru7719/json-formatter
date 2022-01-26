use super::tokenizer::Token;

#[derive(Debug)]
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
    fn format_node(&self, indent: &String) -> String;
}

pub struct ObjectListNode {
    value: Vec<Box<dyn Node>>,
}

impl Node for ObjectListNode {
    fn print_node(&self) -> String {
        let mut buf = String::new();
        let mut count = 0;
        if count < self.value.len() {
            buf = self.value[count].print_node();
            count += 1;
        }
        while count < self.value.len() {
            buf = format!("{},{}", buf, self.value[count].print_node());
            count += 1;
        }
        return format!("{{{}}}", buf);
    }

    fn format_node(&self, indent: &String) -> String {
        let mut buf = String::new();
        let mut count = 0;
        if count < self.value.len() {
            buf = self.value[count].format_node(indent);
            count += 1;
        }
        while count < self.value.len() {
            buf = format!(
                "{},\n{}{}",
                buf,
                self.value[count].format_node(indent),
                indent
            );
            count += 1;
        }
        return format!("{{\n{}\n}}", buf);
    }
}

pub struct ObjectNode {
    key: Box<dyn Node>,
    value: Box<dyn Node>,
}

impl Node for ObjectNode {
    fn print_node(&self) -> String {
        return format!("{}:{}", self.key.print_node(), self.value.print_node());
    }

    fn format_node(&self, indent: &String) -> String {
        return format!(
            "{}{}: {}",
            indent,
            self.key.format_node(indent),
            self.value.format_node(indent)
        );
    }
}

pub struct ArrayNode {
    value: Vec<Box<dyn Node>>,
}

impl Node for ArrayNode {
    fn print_node(&self) -> String {
        let mut buf = String::new();
        let mut count = 0;
        if count < self.value.len() {
            buf = self.value[count].print_node();
            count += 1;
        }
        while count < self.value.len() {
            buf = format!("{},{}", buf, self.value[count].print_node());
            count += 1;
        }
        return format!("[{}]", buf);
    }

    fn format_node(&self, indent: &String) -> String {
        let mut buf = String::new();
        let mut count = 0;
        if count < self.value.len() {
            buf = format!("{}{}", indent, self.value[count].format_node(indent));
            count += 1;
        }
        while count < self.value.len() {
            buf = format!(
                "{},\n{}{}",
                buf,
                indent,
                self.value[count].format_node(indent)
            );
            count += 1;
        }
        return format!("[\n{}\n{}]", buf, indent);
    }
}

pub struct StringNode {
    value: String,
}

impl Node for StringNode {
    fn print_node(&self) -> String {
        return format!("{}", self.value);
    }

    fn format_node(&self, _: &String) -> String {
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

    fn format_node(&self, _: &String) -> String {
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

    fn format_node(&self, _: &String) -> String {
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
    fn format_node(&self, _: &String) -> String {
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
    return parse_objects(&token_list, &mut index);
}

pub fn parse_objects(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    let mut value: Vec<Box<dyn Node>> = Vec::new();
    if !expect_token(Token::LeftBracket, token_list, index) {
        return Err(ParseError::UnexpectedToken);
    }
    loop {
        match parse_object(token_list, index) {
            Ok(v) => value.push(v),
            Err(err) => return Err(err),
        }

        if !expect_token(Token::Commma, token_list, index) {
            break;
        }
    }
    if !expect_token(Token::RightBracket, token_list, index) {
        return Err(ParseError::UnexpectedToken);
    }

    return Ok(Box::new(ObjectListNode { value: value }));
}

pub fn parse_object(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    let key_resutl;
    let value_result;

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

    return Ok(Box::new(ObjectNode {
        key: key_resutl,
        value: value_result,
    }));
}

pub fn parse_array(
    token_list: &Vec<Token>,
    index: &mut usize,
) -> Result<Box<dyn Node>, ParseError> {
    let mut value: Vec<Box<dyn Node>> = Vec::new();

    if !expect_token(Token::LeftSquareBracket, token_list, index) {
        return Err(ParseError::UnexpectedToken);
    }

    loop {
        match parse_value(token_list, index) {
            Ok(v) => {
                value.push(v);
            }
            Err(err) => return Err(err),
        }
        print!("kitayo");
        if !expect_token(Token::Commma, token_list, index) {
            break;
        }
    }

    if !expect_token(Token::RightSquareBracket, token_list, index) {
        return Err(ParseError::UnexpectedToken);
    }

    return Ok(Box::new(ArrayNode { value: value }));
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
        Token::LeftBracket => parse_objects(token_list, index),
        Token::LeftSquareBracket => parse_array(token_list, index),
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

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("\"value\"".to_string()),
                Token::RightBracket,
            ])
            .unwrap()
            .print_node(),
            "{\"key\":\"value\"}".to_string()
        );

        assert_eq!(
            parse(vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("1".to_string()),
                Token::RightBracket,
            ])
            .unwrap()
            .print_node(),
            "{\"key\":1}".to_string()
        );

        assert_eq!(
            parse(vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("true".to_string()),
                Token::RightBracket,
            ])
            .unwrap()
            .print_node(),
            "{\"key\":true}".to_string()
        );

        assert_eq!(
            parse(vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("null".to_string()),
                Token::RightBracket,
            ])
            .unwrap()
            .print_node(),
            "{\"key\":null}".to_string()
        );

        assert_eq!(
            parse(vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("\"value\"".to_string()),
                Token::RightBracket,
                Token::RightBracket,
            ])
            .unwrap()
            .print_node(),
            "{\"key\":{\"key\":\"value\"}}".to_string()
        );

        assert_eq!(
            parse(vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::LeftSquareBracket,
                Token::Str("\"a\"".to_string()),
                Token::Commma,
                Token::Str("\"b\"".to_string()),
                Token::RightSquareBracket,
                Token::RightBracket,
            ])
            .unwrap()
            .print_node(),
            "{\"key\":[\"a\",\"b\"]}".to_string()
        );

        assert_eq!(
            parse(vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("\"value\"".to_string()),
                Token::Commma,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("\"value\"".to_string()),
                Token::RightBracket,
            ])
            .unwrap()
            .print_node(),
            "{\"key\":\"value\",\"key\":\"value\"}".to_string()
        );
    }

    #[test]
    fn test_format_node() {
        let indent = "  ".to_string();
        assert_eq!(
            ObjectListNode {
                value: vec![Box::new(ObjectNode {
                    key: Box::new(StringNode {
                        value: "\"key\"".to_string(),
                    }),
                    value: Box::new(StringNode {
                        value: "\"value\"".to_string(),
                    }),
                })],
            }
            .format_node(&indent),
            r#"{
                "key": "value"
              }"#
            .to_string()
        );
    }
}
