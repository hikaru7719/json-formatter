#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,        // {
    RightBracket,       // }
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    Commma,             // ,
    Colorn,             // :
    Str(String),
    Num(i64),
    Bool(bool),
    Null,
}

pub fn tokenize(str_vec: Vec<char>) -> Vec<Token> {
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
                count += 1;
                distinguish_string(&str_vec, &mut count, &mut vec);
            }
            'f' => {
                distinguish_false(&str_vec, &mut count, &mut vec);
            }
            't' => {
                distinguish_true(&str_vec, &mut count, &mut vec);
            }
            _ => (),
        }
    }
    vec
}

fn distinguish_string(str_vec: &Vec<char>, count: &mut usize, vec: &mut Vec<Token>) {
    let mut buf = String::new();
    while *count < str_vec.len() {
        if str_vec[*count] == '"' {
            vec.push(Token::Str(buf.clone()));
            *count += 1;
            return;
        }
        buf.push(str_vec[*count]);
        *count += 1;
    }
    panic!("can't find \"");
}

fn distinguish_false(str_vec: &Vec<char>, count: &mut usize, vec: &mut Vec<Token>) {
    if let Some(bool) = str_vec.get(*count..*count + 5) {
        if bool.iter().collect::<String>() == "false".to_string() {
            vec.push(Token::Bool(false));
            *count += 5;
            return;
        }
    }
    panic!("not false primitive");
}

fn distinguish_true(str_vec: &Vec<char>, count: &mut usize, vec: &mut Vec<Token>) {
    if let Some(bool) = str_vec.get(*count..*count + 4) {
        if bool.iter().collect::<String>() == "true".to_string() {
            vec.push(Token::Bool(true));
            *count += 4;
            return;
        }
    }
    panic!("not true primitive");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("{".chars().collect::<Vec<char>>()),
            vec![Token::LeftBracket]
        );
        assert_eq!(
            tokenize("}".chars().collect::<Vec<char>>()),
            vec![Token::RightBracket]
        );
        assert_eq!(
            tokenize("[".chars().collect::<Vec<char>>()),
            vec![Token::LeftSquareBracket]
        );
        assert_eq!(
            tokenize("]".chars().collect::<Vec<char>>()),
            vec![Token::RightSquareBracket]
        );
        assert_eq!(
            tokenize(",".chars().collect::<Vec<char>>()),
            vec![Token::Commma]
        );
        assert_eq!(
            tokenize(":".chars().collect::<Vec<char>>()),
            vec![Token::Colorn]
        );
        assert_eq!(
            tokenize("\"hoge\"".chars().collect::<Vec<char>>()),
            vec![Token::Str("hoge".to_string())]
        );
        assert_eq!(
            tokenize("false".chars().collect::<Vec<char>>()),
            vec![Token::Bool(false)]
        );
        assert_eq!(
            tokenize("true".chars().collect::<Vec<char>>()),
            vec![Token::Bool(true)]
        );
    }

    #[test]
    fn test_distinguish_string() {
        let str_vec = "\"hoge\"".chars().collect::<Vec<char>>();
        let mut count = 1;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_string(&str_vec, &mut count, &mut vec);
        assert_eq!(vec, vec![Token::Str("hoge".to_string())]);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_distinguish_false() {
        let str_vec = "false".chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_false(&str_vec, &mut count, &mut vec);
        assert_eq!(vec, vec![Token::Bool(false)]);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_distinguish_true() {
        let str_vec = "true".chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut vec: Vec<Token> = Vec::new();
        distinguish_true(&str_vec, &mut count, &mut vec);
        assert_eq!(vec, vec![Token::Bool(true)]);
        assert_eq!(count, 4);
    }
}
