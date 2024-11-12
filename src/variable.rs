use crate::tokenize::token::Token;
use std::collections::HashMap;

pub struct VarTable(HashMap<char, f64>);

impl VarTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(var_map: HashMap<char, f64>) -> Self {
        Self(var_map)
    }

    pub fn set(&mut self, name: char, val: f64) {
        self.0.insert(name, val);
    }

    pub fn get(&self, name: char) -> Option<f64> {
        self.0.get(&name).copied()
    }
}

pub fn insert_vars(expr: &mut Vec<Token>, var_table: &VarTable) {
    for tk in expr.iter_mut() {
        if let Token::Var(c) = *tk {
            *tk = Token::Num(var_table[c]);
        }
    }
}

impl std::ops::Index<char> for VarTable {
    type Output = f64;

    fn index(&self, index: char) -> &Self::Output {
        self.0
            .get(&index)
            .expect(&format!("eqrs: var {index} not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get() {
        let mut vt = VarTable::new();
        vt.set('x', 5.0);
        assert_eq!(vt.get('x').unwrap(), 5.0);
    }

    #[test]
    fn set_and_idx() {
        let mut vt = VarTable::new();
        vt.set('x', 10.0);
        assert_eq!(vt['x'], 10.0);
    }

    #[test]
    #[should_panic]
    fn idx_no_exist() {
        let vt = VarTable::new();
        vt['x'];
    }
}
