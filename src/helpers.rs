use crate::tokenize::token::Token;

pub fn find_closing_par(expr: &Vec<Token>, i_of_opening_par: usize) -> Option<usize> {
    let mut i = i_of_opening_par + 1;
    let mut level = 0;
    while i < expr.len() {
        match expr[i] {
            Token::ParL => level += 1,
            Token::ParR => {
                if level == 0 {
                    return Some(i);
                } else {
                    level -= 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::tokenize;

    #[test]
    fn find_closing_par_1() {
        let tokens = tokenize("(1 + 2)");
        assert_eq!(find_closing_par(&tokens, 0), Some(4));
    }

    #[test]
    fn find_closing_par_2() {
        let tokens = tokenize("3 * (1 + (2 * 3))");
        assert_eq!(find_closing_par(&tokens, 2), Some(10));
    }

    #[test]
    fn find_closing_par_3() {
        let tokens = tokenize("(1 + (2 * (3 - 4)))");
        assert_eq!(find_closing_par(&tokens, 0), Some(12));
        assert_eq!(find_closing_par(&tokens, 3), Some(11));
        assert_eq!(find_closing_par(&tokens, 6), Some(10));
    }
}
