use crate::{
    helpers::find_closing_par,
    tokenize::{operator::Op, token::Token},
};

pub fn post_process(vec: &Vec<Token>) -> Vec<Token> {
    let mut vec = vec.clone();

    let mut i = 0;

    // "-n" --> "(0-n)"
    // "-(...)" --> "(0-(...))"
    while i < vec.len() - 1 {
        
        if let Token::Op(Op::Sub) = vec[i] {
            if i == 0 || matches!(vec[i - 1], Token::Op(_)) || matches!(vec[i - 1], Token::ParL) {
                vec.insert(i, Token::Num(0.0));
                vec.insert(i, Token::ParL);
                vec.insert(
                    if matches!(vec[i + 3], Token::ParL) {
                        find_closing_par(&vec, i + 3).expect("failed to find closing parenthesis")
                    } else {
                        i + 4
                    },
                    Token::ParR,
                );
            }
        }

        i += 1;
    }

    // insert multiply between Nx, xN, xy, x(, )x, N(, )N, and )(
    i = 0;
    while i < vec.len() - 1 {
        match (vec[i], vec[i + 1]) {
            (Token::Num(_), Token::Var(_))
            | (Token::Var(_), Token::Num(_))
            | (Token::Var(_), Token::Var(_))
            | (Token::Var(_), Token::ParL)
            | (Token::ParR, Token::Var(_))
            | (Token::Num(_), Token::ParL)
            | (Token::ParR, Token::Num(_))
            | (Token::ParR, Token::ParL) => {
                vec.insert(i + 1, Token::Op(Op::Mul));
            }
            _ => {}
        }

        i += 1;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::{operator::Op, token::Token, tokenize};

    #[test]
    fn post_process_1() {
        assert_eq!(
            post_process(&tokenize("2 + 2")),
            vec![Token::Num(2.0), Token::Op(Op::Add), Token::Num(2.0)]
        );
    }

    #[test]
    fn post_process_2() {
        assert_eq!(
            post_process(&tokenize("2 + 2 * 9 - 8")),
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
    fn post_process_3() {
        assert_eq!(
            post_process(&tokenize("x(7xy + 6) * 3y^2")),
            vec![
                Token::Var('x'),
                Token::Op(Op::Mul),
                Token::ParL,
                Token::Num(7.0),
                Token::Op(Op::Mul),
                Token::Var('x'),
                Token::Op(Op::Mul),
                Token::Var('y'),
                Token::Op(Op::Add),
                Token::Num(6.0),
                Token::ParR,
                Token::Op(Op::Mul),
                Token::Num(3.0),
                Token::Op(Op::Mul),
                Token::Var('y'),
                Token::Op(Op::Pow),
                Token::Num(2.0)
            ]
        );
    }

    #[test]
    fn post_process_4() {
        assert_eq!(
            post_process(&tokenize("2x + 3(4 + 5)")),
            vec![
                Token::Num(2.0),
                Token::Op(Op::Mul),
                Token::Var('x'),
                Token::Op(Op::Add),
                Token::Num(3.0),
                Token::Op(Op::Mul),
                Token::ParL,
                Token::Num(4.0),
                Token::Op(Op::Add),
                Token::Num(5.0),
                Token::ParR
            ]
        );
    }

    #[test]
    fn post_process_5() {
        assert_eq!(
            post_process(&tokenize("-(3 + 4)")),
            vec![
                Token::ParL,
                Token::Num(0.0),
                Token::Op(Op::Sub),
                Token::ParL,
                Token::Num(3.0),
                Token::Op(Op::Add),
                Token::Num(4.0),
                Token::ParR,
                Token::ParR
            ]
        );
    }
}
