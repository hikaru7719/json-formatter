#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,        // {
    RightBracket,       // }
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    Commma,             // ,
    Colorn,             // *
    Str(String),
    Num(i64),
    Bool(bool),
    Null,
}

pub fn tokenize(strings: &str) -> Vec<Token> {
    let mut vec: Vec<Token> = Vec::new();
    let mut buf: String = "".to_string();
    for s in strings.chars() {
        buf.push(s);
        match buf.as_str() {
            "{" => vec.push(Token::LeftBracket),
            _ => (),
        }
    }
    vec
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("{"), vec![Token::LeftBracket]);
    }
}
