use crate::parse::{parse, post_process::post_process};

use super::{
    parse::{token::Token, operator::Op},
    variable::VarTable,
};

pub mod prelude {
    pub use super::{calc, eval};
}

pub fn eval(str: &str, var_table: Option<&VarTable>) -> f64 {
    calc(&post_process(&parse(str)), var_table)
}

pub fn calc(expr: &Vec<Token>, var_table: Option<&VarTable>) -> f64 {
    let mut expr = expr.clone();
    if let Some(var_table) = var_table {
        insert_vars(&mut expr, var_table);
    }

    // recursively evaluate parenthesis
    let mut i = 0;
    while i < expr.len() {
        if let Token::ParL = expr[i] {
            let mut level = 0;
            let mut j = i + 1;
            while j < expr.len() {
                match expr[j] {
                    Token::ParL => level += 1,
                    Token::ParR => {
                        if level == 0 {
                            expr[i] = Token::Num(calc(&Vec::from(&expr[(i + 1)..j]), None));
                            for _ in i..j {
                                expr.remove(i + 1);
                            }
                            break;
                        } else {
                            level -= 1;
                        }
                    },
                    _ => {},
                }
                j += 1;
            }
        }

        i += 1;
    }

    // POWER
    i = 0;
    while i < expr.len() {
        if let Token::Op(op) = expr[i] {
            // if i == 0 {
            //     return Err("leading operator".to_string());
            // }
            // if i == expr.len() - 1 {
            //     return Err("trailing operator".to_string());
            // }
            // else if i < expr.len() - 1 {
            //     if let Token::Op(_) = expr[i + 1] {
            //         return Err("adjacent operators".to_string());
            //     }
            // }
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
                },
                _ => {},
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
                },
                _ => {},
            }
        }
        

        i += 1;
    }

    if let Token::Num(val) = expr[0] {
        val
    } else {
        panic!("eqrs: eval error (expr: {expr:?})");
    }
}

pub fn insert_vars(expr: &mut Vec<Token>, var_table: &VarTable) {
    for tk in expr.iter_mut() {
        if let Token::Var(c) = *tk {
            *tk = Token::Num(var_table[c]);
        }
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
        assert!(approx_eq(eval("(-3 + 5)(-2) / (7 % 3) + 4 ^ (-2) - (6 * -3 + 2) / 2 + 8 % (-3)", None), 6.0625));
    }

    #[test]
    fn eval_no_vars_2() {
        assert!(approx_eq(eval("(-4 + 6)(3 - 5) / (10 % 4) + 5 ^ (-1) - (8 * -2 + 3) / 3 + 9 % (-4)", None), 3.5333));
    }

    #[test]
    fn eval_vars_1() {
        let mut vt = VarTable::new();
        let expr = "(x - 4)(y + 6) / (z % 5) + (xy)^(-1) - (3z + 2) / 2 + (x % y)";

        vt.set('x', -3.0);
        vt.set('y', 7.0);
        vt.set('z', 4.0);
        assert!(approx_eq(eval(expr, Some(&vt)), -32.7976));
    }

    #[test]
    fn eval_vars_2() {
        let mut vt = VarTable::new();
        let expr = "(x - 4)(y + 6) / (z % 5) + (xy)^(-1) - (3z + 2) / 2 + (x % y)";

        vt.set('x', 5.0);
        vt.set('y', -2.0);
        vt.set('z', 6.0);
        assert!(approx_eq(eval(expr, Some(&vt)), -5.1));
    }

    #[test]
    fn eval_vars_3() {
        let mut vt = VarTable::new();
        let expr = "(x - 4)(y + 6) / (z % 5) + (xy)^(-1) - (3z + 2) / 2 + (x % y)";

        vt.set('x', 8.0);
        vt.set('y', -3.0);
        vt.set('z', 7.0);
        assert!(approx_eq(eval(expr, Some(&vt)), -3.5417));
    }
}
