use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Fn {
    Sqrt(Vec<Token>),
    Cbrt(Vec<Token>),
    Sin(Vec<Token>),
    Cos(Vec<Token>),
    Tan(Vec<Token>),
    Csc(Vec<Token>),
    Sec(Vec<Token>),
    Cot(Vec<Token>),
    Asin(Vec<Token>),
    Acos(Vec<Token>),
    Atan(Vec<Token>),
    Acsc(Vec<Token>),
    Asec(Vec<Token>),
    Acot(Vec<Token>),
}
