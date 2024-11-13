use super::operator::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(f64),
    Op(Op),
    ParL,
    ParR,
    Var(char),
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
        let is_paren =
            |c: &char| *c == '(' || *c == '[' || *c == '{' || *c == ')' || *c == ']' || *c == '}';

        !(c.is_ascii_digit() || (c.is_ascii_punctuation() && !(Op::is_valid(c) || is_paren(c))))
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
