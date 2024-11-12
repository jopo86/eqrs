use super::operator::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(f64),
    Op(Op),
    ParL,
    ParR,
    Var(char),
}

#[derive(Debug)]
pub enum TokenParseErr {
    IsDigit,
}

impl Token {
    pub fn parse(c: &char) -> Result<Self, TokenParseErr> {
        if c.is_ascii_digit() {
            return Err(TokenParseErr::IsDigit);
        } else if is_op(c) {
            Ok(Token::Op(Op::parse(c).unwrap()))
        } else if *c == '(' || *c == '[' || *c == '{' {
            Ok(Token::ParL)
        } else if *c == ')' || *c == ']' || *c == '}' {
            Ok(Token::ParR)
        } else {
            Ok(Token::Var(*c))
        }
    }
}
