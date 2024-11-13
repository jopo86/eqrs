use crate::{
    helpers::find_closing_par,
    tokenize::{operator::Op, token::Token},
    variable::{insert_vars, VarTable},
};

#[derive(Debug, PartialEq)]
pub enum CalcError {
    NoTokens,
    VarsNotFound,
    MismatchedPars,
    LeadingOperator,
    TrailingOperator,
    AdjacentOperators,
    Unknown,
}

pub fn calc(expr: &Vec<Token>, var_table: Option<&VarTable>) -> Result<f64, CalcError> {
    let mut expr = expr.clone();

    // check no tokens
    if expr.len() == 0 {
        return Err(CalcError::NoTokens);
    }

    // attempt to insert variables
    if let Some(var_table) = var_table {
        match insert_vars(&expr, var_table) {
            Ok(new_expr) => expr = new_expr,
            Err(_) => return Err(CalcError::VarsNotFound),
        }
    }

    // check leading operator
    if let Token::Op(_) = expr[0] {
        return Err(CalcError::LeadingOperator);
    }

    // check trailing operator
    if let Token::Op(_) = expr[expr.len() - 1] {
        return Err(CalcError::TrailingOperator);
    }


    // check adjacent operators
    for i in 0..expr.len() - 1 {
        if let (Token::Op(_), Token::Op(_)) = (expr[i], expr[i + 1]) {
            return Err(CalcError::AdjacentOperators);
        }
    }

    // check remaining variables
    for i in 0..expr.len() {
        if let Token::Var(_) = expr[i] {
            return Err(CalcError::VarsNotFound)
        }
    }

    let mut i = 0;

    // recursively evaluate parenthesis
    while i < expr.len() {
        if let Token::ParL = expr[i] {
            let j;
            match find_closing_par(&expr, i) {
                Some(idx) => j = idx,
                None => return Err(CalcError::MismatchedPars),
            }
            expr[i] = Token::Num(calc(&Vec::from(&expr[(i + 1)..j]), None)?);
            for _ in i..j {
                expr.remove(i + 1);
            }
        }

        i += 1;
    }


    // POWER
    i = 0;
    while i < expr.len() {
        if let Token::Op(op) = expr[i] {
            if let Op::Pow = op {
                if let (Token::Num(a), Token::Num(b)) = (expr[i - 1], expr[i + 1]) {
                    expr[i - 1] = Token::Num(op.calc(a, b));
                    expr.remove(i);
                    expr.remove(i);
                    i -= 1;
                } else {
                    // err unknown
                }
            }
        }

        i += 1;
    }

    // MULTIPLY / DIVIDE / MODULUS
    i = 0;
    while i < expr.len() {
        if let Token::Op(op) = expr[i] {
            match op {
                Op::Mul | Op::Div | Op::Mod => {
                    if let (Token::Num(a), Token::Num(b)) = (expr[i - 1], expr[i + 1]) {
                        expr[i - 1] = Token::Num(op.calc(a, b));
                        expr.remove(i);
                        expr.remove(i);
                        i -= 1;
                    } else {
                        // err unknown
                    }
                }
                _ => {}
            }
        }

        i += 1;
    }

    // ADD / SUBTRACT
    i = 0;
    while i < expr.len() {
        if let Token::Op(op) = expr[i] {
            match op {
                Op::Add | Op::Sub => {
                    if let (Token::Num(a), Token::Num(b)) = (expr[i - 1], expr[i + 1]) {
                        expr[i - 1] = Token::Num(op.calc(a, b));
                        expr.remove(i);
                        expr.remove(i);
                        i -= 1;
                    } else {
                        // err unknown
                    }
                }
                _ => {}
            }
        }

        i += 1;
    }

    if let Token::Num(val) = expr[0] {
        Ok(val)
    } else {
        Err(CalcError::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::post_process::post_process;
    use crate::tokenize::tokenize;
    use crate::variable::VarTable;

    #[test]
    fn calc_1() {
        let tokens = post_process(&tokenize("2 + 2").unwrap());
        assert_eq!(calc(&tokens, None).unwrap(), 4.0);
    }

    #[test]
    fn calc_2() {
        let tokens = post_process(&tokenize("2 + 2 * 9 - 8").unwrap());
        assert_eq!(calc(&tokens, None).unwrap(), 12.0);
    }

    #[test]
    fn calc_3() {
        let tokens = post_process(&tokenize("3 * (2 + 1)").unwrap());
        assert_eq!(calc(&tokens, None).unwrap(), 9.0);
    }

    #[test]
    fn calc_4() {
        let tokens = post_process(&tokenize("4 ^ 2 + 3").unwrap());
        assert_eq!(calc(&tokens, None).unwrap(), 19.0);
    }

    #[test]
    fn calc_5() {
        let tokens = post_process(&tokenize("10 / 2 + 3 * 2").unwrap());
        assert_eq!(calc(&tokens, None).unwrap(), 11.0);
    }

    #[test]
    fn calc_6() {
        let tokens = post_process(&tokenize("-(3 + 4)").unwrap());
        assert_eq!(calc(&tokens, None).unwrap(), -7.0);
    }

    #[test]
    fn calc_with_vars_1() {
        let mut vt = VarTable::new();
        vt.set('x', 5.0);
        vt.set('y', 3.0);
        let tokens = post_process(&tokenize("x + y * 2").unwrap());
        assert_eq!(calc(&tokens, Some(&vt)).unwrap(), 11.0);
    }

    #[test]
    fn calc_with_vars_2() {
        let mut vt = VarTable::new();
        vt.set('a', 2.0);
        vt.set('b', 4.0);
        let tokens = post_process(&tokenize("a * b + 3").unwrap());
        assert_eq!(calc(&tokens, Some(&vt)).unwrap(), 11.0);
    }

    #[test]
    fn calc_with_vars_3() {
        let mut vt = VarTable::new();
        vt.set('m', 7.0);
        vt.set('n', 2.0);
        let tokens = post_process(&tokenize("m / n - 1").unwrap());
        assert_eq!(calc(&tokens, Some(&vt)).unwrap(), 2.5);
    }
}
