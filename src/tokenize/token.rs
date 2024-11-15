use super::operator::*;
use super::function::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Op(Op),
    ParL,
    ParR,
    Var(char),
    Fn(Fn),
}

#[derive(Debug, PartialEq)]
pub enum TokenParseError {
    IsDigit,
    IsPunctuation,
}

impl Token {
    pub fn parse(c: &char) -> Result<Self, TokenParseError> {
        if c.is_ascii_digit() {
            Err(TokenParseError::IsDigit)
        } else if Op::is_valid(c) {
            Ok(Token::Op(Op::parse(c).unwrap()))
        } else if *c == '(' || *c == '[' || *c == '{' {
            Ok(Token::ParL)
        } else if *c == ')' || *c == ']' || *c == '}' {
            Ok(Token::ParR)
        } else if c.is_ascii_punctuation() {
            Err(TokenParseError::IsPunctuation)
        } else {
            Ok(Token::Var(*c))
        }
    }

    pub fn is_valid(c: &char) -> bool {

        !(c.is_ascii_digit() || (c.is_ascii_punctuation() && !(Op::is_valid(c) || Self::is_par_l(c) || Self::is_par_r(c))))
    }

    pub fn is_par_l(c: &char) -> bool {
        *c == '(' || *c == '[' || *c == '{'
    }

    pub fn is_par_r(c: &char) -> bool {
        *c == ')' || *c == ']' || *c == '}'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_1() {
        assert!(Token::is_valid(&'a'));
    }

    #[test]
    fn is_valid_2() {
        assert!(Token::is_valid(&'Z'));
    }

    #[test]
    fn is_valid_4() {
        assert!(Token::is_valid(&'('));
    }

    #[test]
    fn is_valid_5() {
        assert!(Token::is_valid(&')'));
    }

    #[test]
    fn is_valid_6() {
        assert!(Token::is_valid(&'+'));
    }

    #[test]
    fn is_valid_7() {
        assert!(!Token::is_valid(&'1'));
    }

    #[test]
    fn is_valid_8() {
        assert!(!Token::is_valid(&'!'));
    }

    #[test]
    fn is_valid_9() {
        assert!(!Token::is_valid(&','));
    }

    #[test]
    fn is_valid_3() {
        assert!(!Token::is_valid(&'_'));
    }
}
