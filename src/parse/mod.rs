pub mod operator;
pub mod post_process;
pub mod token;

use token::Token;

pub mod prelude {
    pub use super::{token::*, operator::*, parse};
}

pub fn parse(str: &str) -> Vec<Token> {
    let mut vec = vec![];
    let chars: Vec<char> = str.chars().collect();

    let mut i = 0;
    while i < str.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        } else if c.is_ascii_digit() {
            let mut j = i;
            loop {
                j += 1;
                if !chars[j].is_ascii_digit() && chars[j] != '.' || j == str.len() {
                    break;
                }
            }
            vec.push(Token::Num(
                chars[i..j]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .expect("parse should work with all digits"),
            ));
            i = j - 1;
        } else {
            vec.push(Token::parse(&c).expect("token parse should work with anything but a digit"))
        }

        i += 1;
    }

    vec
}
