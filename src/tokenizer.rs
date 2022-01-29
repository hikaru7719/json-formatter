#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,        // {
    RightBracket,       // }
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    Commma,             // ,
    Colorn,             // :
    Str(String),
    Num(String),
    Bool(String),
    Null,
}

#[derive(Debug)]
pub enum TokenizeError {
    InvalidString,
    NotTrue,
    NotFalse,
    NotNull,
    InvalidCharactar,
}

pub fn tokenize(str_vec: Vec<char>) -> Result<Vec<Token>, TokenizeError> {
    let mut vec: Vec<Token> = Vec::new();
    let mut count = 0;
    while count < str_vec.len() {
        match str_vec[count] {
            '{' => {
                vec.push(Token::LeftBracket);
                count += 1;
            }
            '}' => {
                vec.push(Token::RightBracket);
                count += 1;
            }
            '[' => {
                vec.push(Token::LeftSquareBracket);
                count += 1;
            }
            ']' => {
                vec.push(Token::RightSquareBracket);
                count += 1;
            }
            ',' => {
                vec.push(Token::Commma);
                count += 1;
            }
            ':' => {
                vec.push(Token::Colorn);
                count += 1;
            }
            '"' => {
                distinguish_string(&str_vec, &mut count, &mut vec)?;
            }
            'f' => {
                distinguish_false(&str_vec, &mut count, &mut vec)?;
            }
            't' => {
                distinguish_true(&str_vec, &mut count, &mut vec)?;
            }
            'n' => {
                distinguish_null(&str_vec, &mut count, &mut vec)?;
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                distinguish_number(&str_vec, &mut count, &mut vec);
            }
            '\n' | '\r' | ' ' | '\t' => {
                consume_whitespace(&str_vec, &mut count);
            }
            _ => return Err(TokenizeError::InvalidCharactar),
        }
    }
    return Ok(vec);
}

fn distinguish_string(
    str_vec: &Vec<char>,
    count: &mut usize,
    vec: &mut Vec<Token>,
) -> Result<(), TokenizeError> {
    let mut buf = str_vec[*count].to_string();
    *count += 1;

    while *count < str_vec.len() {
        if str_vec[*count] == '"' {
            buf.push(str_vec[*count]);
            vec.push(Token::Str(buf.clone()));
            *count += 1;
            return Ok(());
        }
        buf.push(str_vec[*count]);
        *count += 1;
    }
    return Err(TokenizeError::InvalidString);
}

fn distinguish_false(
    str_vec: &Vec<char>,
    count: &mut usize,
    vec: &mut Vec<Token>,
) -> Result<(), TokenizeError> {
    if let Some(bool) = str_vec.get(*count..*count + 5) {
        let buf = bool.iter().collect::<String>();
        if buf == "false".to_string() {
            vec.push(Token::Bool(buf.clone()));
            *count += 5;
            return Ok(());
        }
    }
    return Err(TokenizeError::NotFalse);
}

fn distinguish_true(
    str_vec: &Vec<char>,
    count: &mut usize,
    vec: &mut Vec<Token>,
) -> Result<(), TokenizeError> {
    if let Some(bool) = str_vec.get(*count..*count + 4) {
        let buf = bool.iter().collect::<String>();
        if buf == "true".to_string() {
            vec.push(Token::Bool(buf.clone()));
            *count += 4;
            return Ok(());
        }
    }
    return Err(TokenizeError::NotTrue);
}

fn distinguish_null(
    str_vec: &Vec<char>,
    count: &mut usize,
    vec: &mut Vec<Token>,
) -> Result<(), TokenizeError> {
    if let Some(null) = str_vec.get(*count..*count + 4) {
        if null.iter().collect::<String>() == "null".to_string() {
            vec.push(Token::Null);
            *count += 4;
            return Ok(());
        }
    }
    return Err(TokenizeError::NotNull);
}

fn distinguish_number(str_vec: &Vec<char>, count: &mut usize, vec: &mut Vec<Token>) {
    let mut buf = String::new();
    while *count < str_vec.len() {
        if str_vec[*count].is_digit(10) {
            buf.push(str_vec[*count]);
            *count += 1;
            continue;
        }
        vec.push(Token::Num(buf.clone()));
        return;
    }

    vec.push(Token::Num(buf.clone()));
    return;
}

fn consume_whitespace(str_vec: &Vec<char>, count: &mut usize) {
    while *count < str_vec.len() {
        if str_vec[*count].is_whitespace() {
            *count += 1;
            continue;
        }
        return;
    }
    return;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("{".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::LeftBracket]
        );
        assert_eq!(
            tokenize("}".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::RightBracket]
        );
        assert_eq!(
            tokenize("[".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::LeftSquareBracket]
        );
        assert_eq!(
            tokenize("]".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::RightSquareBracket]
        );
        assert_eq!(
            tokenize(",".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::Commma]
        );
        assert_eq!(
            tokenize(":".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::Colorn]
        );
        assert_eq!(
            tokenize("\"hoge\"".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::Str("\"hoge\"".to_string())]
        );
        assert_eq!(
            tokenize("false".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::Bool("false".to_string())]
        );
        assert_eq!(
            tokenize("true".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::Bool("true".to_string())]
        );
        assert_eq!(
            tokenize("null".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::Null]
        );
        assert_eq!(
            tokenize("100".chars().collect::<Vec<char>>()).unwrap(),
            vec![Token::Num("100".to_string())]
        );

        assert_eq!(
            tokenize(
                "{  \"key\"  \t :  \"value\" \r\n}"
                    .chars()
                    .collect::<Vec<char>>()
            )
            .unwrap(),
            vec![
                Token::LeftBracket,
                Token::Str("\"key\"".to_string()),
                Token::Colorn,
                Token::Str("\"value\"".to_string()),
                Token::RightBracket
            ]
        );
    }

    #[test]
    fn test_distinguish_string() {
        let str_vec = "\"hoge\"".chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_string(&str_vec, &mut count, &mut vec).unwrap();
        assert_eq!(vec, vec![Token::Str("\"hoge\"".to_string())]);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_distinguish_false() {
        let str_vec = "false".chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_false(&str_vec, &mut count, &mut vec).unwrap();
        assert_eq!(vec, vec![Token::Bool("false".to_string())]);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_distinguish_true() {
        let str_vec = "true".chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_true(&str_vec, &mut count, &mut vec).unwrap();
        assert_eq!(vec, vec![Token::Bool("true".to_string())]);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_distinguish_null() {
        let str_vec = "null".chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_null(&str_vec, &mut count, &mut vec).unwrap();
        assert_eq!(vec, vec![Token::Null]);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_distinguish_number() {
        let str_vec = "12".chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_number(&str_vec, &mut count, &mut vec);
        assert_eq!(vec, vec![Token::Num("12".to_string())]);
        assert_eq!(count, 2);
    }
}
