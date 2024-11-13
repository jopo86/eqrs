pub mod calculate;
mod helpers;
pub mod post_process;
pub mod tokenize;
pub mod variable;

use crate::{
    calculate::{calc, CalcError},
    post_process::post_process,
    tokenize::{tokenize, TokenizeError},
    variable::VarTable,
};

#[derive(Debug, PartialEq)]
pub enum EvalError {
    TokenizeError(TokenizeError),
    CalcError(CalcError),
}

pub fn eval(str: &str, var_table: Option<&VarTable>) -> Result<f64, EvalError> {
    match tokenize(str) {
        Ok(tokens) => match calc(&post_process(&tokens), var_table) {
            Ok(val) => Ok(val),
            Err(e) => Err(EvalError::CalcError(e)),
        },
        Err(e) => Err(EvalError::TokenizeError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-4;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < TOLERANCE
    }

    #[test]
    fn eval_no_vars_1() {
        assert!(approx_eq(
            eval(
                "(-3 + 5)(-2) / (7 % 3) + 4 ^ (-2) - (6 * -3 + 2) / 2 + 8 % (-3)",
                None
            )
            .unwrap(),
            6.0625
        ));
    }

    #[test]
    fn eval_no_vars_2() {
        assert!(approx_eq(
            eval(
                "(-4 + 6)(3 - 5) / (10 % 4) + 5 ^ (-1) - (8 * -2 + 3) / 3 + 9 % (-4)",
                None
            )
            .unwrap(),
            3.5333
        ));
    }

    #[test]
    fn eval_vars_1() {
        let mut vt = VarTable::new();
        let expr = "(x - 4)(y + 6) / (z % 5) + (xy)^(-1) - (3z + 2) / 2 + (x % y)";

        vt.set('x', -3.0);
        vt.set('y', 7.0);
        vt.set('z', 4.0);
        assert!(approx_eq(eval(expr, Some(&vt)).unwrap(), -32.7976));
    }

    #[test]
    fn eval_vars_2() {
        let mut vt = VarTable::new();
        let expr = "(x - 4)(y + 6) / (z % 5) + (xy)^(-1) - (3z + 2) / 2 + (x % y)";

        vt.set('x', 5.0);
        vt.set('y', -2.0);
        vt.set('z', 6.0);
        assert!(approx_eq(eval(expr, Some(&vt)).unwrap(), -5.1));
    }

    #[test]
    fn eval_vars_3() {
        let mut vt = VarTable::new();
        let expr = "(x - 4)(y + 6) / (z % 5) + (xy)^(-1) - (3z + 2) / 2 + (x % y)";

        vt.set('x', 8.0);
        vt.set('y', -3.0);
        vt.set('z', 7.0);
        assert!(approx_eq(eval(expr, Some(&vt)).unwrap(), -3.5417));
    }
}
