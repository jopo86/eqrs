pub mod operator;
pub mod token;

use token::Token;

pub fn tokenize(str: &str) -> Vec<Token> {
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
            vec.push(Token::parse(&c).expect("token parse should work with anything but a digit"))
        }

        i += 1;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use operator::Op;

    #[test]
    fn tokenize_1() {
        assert_eq!(
            tokenize("2 + 2"),
            vec![Token::Num(2.0), Token::Op(Op::Add), Token::Num(2.0)]
        );
    }

    #[test]
    fn tokenize_2() {
        assert_eq!(
            tokenize("2 + 2 * 9 - 8"),
            vec![
                Token::Num(2.0),
                Token::Op(Op::Add),
                Token::Num(2.0),
                Token::Op(Op::Mul),
                Token::Num(9.0),
                Token::Op(Op::Sub),
                Token::Num(8.0)
            ]
        );
    }

    #[test]
    fn tokenize_3() {
        assert_eq!(
            tokenize("x(7xy + 6) * 3y^2"),
            vec![
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
            ]
        );
    }
}
