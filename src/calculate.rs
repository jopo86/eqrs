use crate::{
    helpers::find_closing_par,
    tokenize::{operator::Op, token::Token},
    variable::{insert_vars, VarTable},
};

pub fn calc(expr: &Vec<Token>, var_table: Option<&VarTable>) -> f64 {
    let mut expr = expr.clone();
    if let Some(var_table) = var_table {
        insert_vars(&mut expr, var_table);
    }

    // recursively evaluate parenthesis
    let mut i = 0;
    while i < expr.len() {
        if let Token::ParL = expr[i] {
            let j = find_closing_par(&expr, i).expect("failed to find closing parenthesis");
            expr[i] = Token::Num(calc(&Vec::from(&expr[(i + 1)..j]), None));
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
        val
    } else {
        panic!("eqrs: eval error (expr: {expr:?})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::tokenize;
    use crate::post_process::post_process;
    use crate::variable::VarTable;

    #[test]
    fn calc_1() {
        let tokens = post_process(&tokenize("2 + 2"));
        assert_eq!(calc(&tokens, None), 4.0);
    }

    #[test]
    fn calc_2() {
        let tokens = post_process(&tokenize("2 + 2 * 9 - 8"));
        assert_eq!(calc(&tokens, None), 12.0);
    }

    #[test]
    fn calc_3() {
        let tokens = post_process(&tokenize("3 * (2 + 1)"));
        assert_eq!(calc(&tokens, None), 9.0);
    }

    #[test]
    fn calc_4() {
        let tokens = post_process(&tokenize("4 ^ 2 + 3"));
        assert_eq!(calc(&tokens, None), 19.0);
    }

    #[test]
    fn calc_5() {
        let tokens = post_process(&tokenize("10 / 2 + 3 * 2"));
        assert_eq!(calc(&tokens, None), 11.0);
    }

    #[test]
    fn calc_6() {
        let tokens = post_process(&tokenize("-(3 + 4)"));
        assert_eq!(calc(&tokens, None), -7.0);
    }

    #[test]
    fn calc_with_vars_1() {
        let mut vt = VarTable::new();
        vt.set('x', 5.0);
        vt.set('y', 3.0);
        let tokens = post_process(&tokenize("x + y * 2"));
        assert_eq!(calc(&tokens, Some(&vt)), 11.0);
    }

    #[test]
    fn calc_with_vars_2() {
        let mut vt = VarTable::new();
        vt.set('a', 2.0);
        vt.set('b', 4.0);
        let tokens = post_process(&tokenize("a * b + 3"));
        assert_eq!(calc(&tokens, Some(&vt)), 11.0);
    }

    #[test]
    fn calc_with_vars_3() {
        let mut vt = VarTable::new();
        vt.set('m', 7.0);
        vt.set('n', 2.0);
        let tokens = post_process(&tokenize("m / n - 1"));
        assert_eq!(calc(&tokens, Some(&vt)), 2.5);
    }
}
