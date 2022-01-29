use json_parser::{parser, tokenizer};

fn main() {
    let mut buf = String::new();
    loop {
        match std::io::stdin().read_line(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                continue;
            }
            Err(_) => {
                break;
            }
        }
    }

    let vec_token = tokenizer::tokenize(buf.chars().collect()).unwrap();
    let result = parser::parse(vec_token).unwrap();
    println!("{}", result.format_node(&("  ".to_string()), &mut 0));
}
