#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl Op {
    pub fn calc(&self, a: f64, b: f64) -> f64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
            Self::Mod => a % b,
            Self::Pow => a.powf(b),
        }
    }
}

#[derive(Debug)]
pub enum OpParseErr {
    NotOp,
}

pub fn is_op(c: &char) -> bool {
    ['+', '-', '*', '/', '%', '^'].contains(c)
}

impl Op {
    pub fn parse(c: &char) -> Result<Self, OpParseErr> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            '%' => Ok(Self::Mod),
            '^' => Ok(Self::Pow),
            _ => Err(OpParseErr::NotOp),
        }
    }
}
