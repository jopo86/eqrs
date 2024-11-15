pub mod operator;
pub mod token;
pub mod function;

use token::{Token, TokenParseError};

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    TokenParseError(TokenParseError),
}

pub fn tokenize(str: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut vec = vec![];
    let chars: Vec<char> = str.chars().collect();

    let mut i = 0;
    while i < str.len() {
        let c = chars[i];
        
        if c.is_whitespace() {
            i += 1;
            continue;
        } else if c.is_ascii_digit() || c == '.' {
            let mut j = i;
            loop {
                j += 1;
                if j == str.len() || !chars[j].is_ascii_digit() && chars[j] != '.' {
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

            if i + 4 < str.len() && chars[i + 4] {
                // check 4 letter functions
                let slice = chars[i..i+3].iter().collect::<String>();
                if slice == "sqrt" {

                } else if slice == "cbrt" {

                } else if slice == "asin" {
                    
                } else if slice == "acos" {
                    
                } else if slice == "atan" {
                    
                } else if slice == "acsc" {
                    
                } else if slice == "asec" {
                    
                } else if slice == "atan" {
                    
                }
            }
            
            if i + 3 < str.len() {
                // check 3 letter functions
                let slice = chars[i..i+2].iter().collect::<String>();
                if slice == "sin" {

                } else if slice == "cos" {

                } else if slice == "tan" {
                    
                } else if slice == "csc" {
                    
                } else if slice == "sec" {
                    
                } else if slice == "cot" {
                    
                }
            }

            match Token::parse(&c) {
                Ok(tk) => vec.push(tk),
                Err(e) => return Err(TokenizeError::TokenParseError(e)),
            }
        }

        i += 1;
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use operator::Op;

    #[test]
    fn tokenize_1() {
        assert_eq!(
            tokenize("2 + 2"),
            Ok(vec![Token::Num(2.0), Token::Op(Op::Add), Token::Num(2.0)])
        );
    }

    #[test]
    fn tokenize_2() {
        assert_eq!(
            tokenize("2 + 2 * 9 - 8"),
            Ok(vec![
                Token::Num(2.0),
                Token::Op(Op::Add),
                Token::Num(2.0),
                Token::Op(Op::Mul),
                Token::Num(9.0),
                Token::Op(Op::Sub),
                Token::Num(8.0)
            ])
        );
    }

    #[test]
    fn tokenize_3() {
        assert_eq!(
            tokenize("x(7xy + 6) * 3y^2"),
            Ok(vec![
                Token::Var('x'),
                Token::ParL,
                Token::Num(7.0),
                Token::Var('x'),
                Token::Var('y'),
                Token::Op(Op::Add),
                Token::Num(6.0),
                Token::ParR,
                Token::Op(Op::Mul),
                Token::Num(3.0),
                Token::Var('y'),
                Token::Op(Op::Pow),
                Token::Num(2.0)
            ])
        );
    }

    #[test]
    fn tokenize_4() {
        assert_eq!(
            tokenize("x + &"),
            Err(TokenizeError::TokenParseError(
                TokenParseError::IsPunctuation
            ))
        );
    }
}
