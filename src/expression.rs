use crate::token::Token;
use crate::unary::Unary;

use std::fmt;
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Unary(Unary),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Unary(unary) => write!(f, "{}", unary),
        }
    }
}

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
        Expression::Unary(Unary::new(tokens))
    }
}
