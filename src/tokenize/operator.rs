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

#[derive(Debug, PartialEq)]
pub enum OpParseError {
    NotOp,
}

impl Op {
    pub fn parse(c: &char) -> Result<Self, OpParseError> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            '%' => Ok(Self::Mod),
            '^' => Ok(Self::Pow),
            _ => Err(OpParseError::NotOp),
        }
    }

    pub fn is_valid(c: &char) -> bool {
        ['+', '-', '*', '/', '%', '^'].contains(c)
    }
}
