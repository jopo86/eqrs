use super::{operator::Op, token::Token};

pub fn post_process(vec: &Vec<Token>) -> Vec<Token> {
    let mut vec = vec.clone();

    let mut i = 0;
    while i < vec.len() - 1 {
        match (vec[i], vec[i + 1]) {
            (Token::Num(_), Token::Var(_))
            | (Token::Var(_), Token::Num(_))
            | (Token::Var(_), Token::Var(_))
            | (Token::ParR, Token::ParL) => {
                vec.insert(i + 1, Token::Op(Op::Mul)); // insert multiply in between Nx, xN, xy, and )(
            }
            _ => {}
        }

        if let Token::Op(Op::Sub) = vec[i] {
            if i == 0 || matches!(vec.get(i - 1), Some(Token::Op(_))) || matches!(vec.get(i - 1), Some(Token::ParL)) {
                vec.insert(i, Token::Num(0.0));
                vec.insert(i, Token::ParL);
                vec.insert(i + 4, Token::ParR);
                // "-n" --> "(0-n)"
            }
        }

        i += 1;
    }

    vec
}
