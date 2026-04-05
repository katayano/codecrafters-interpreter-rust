use crate::primary::Primary;
use crate::token::Token;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Unary {
    Minus(Box<Unary>),
    Bang(Box<Unary>),
    Primary(Primary),
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Unary::Minus(inner) => write!(f, "(- {})", inner),
            Unary::Bang(inner) => write!(f, "(! {})", inner),
            Unary::Primary(primary) => write!(f, "{}", primary),
        }
    }
}

impl Unary {
    pub fn new(tokens: Vec<Token>) -> Self {
        match &tokens[0] {
            Token::Minus => Unary::Minus(Box::new(Unary::new(tokens[1..].to_vec()))),
            Token::Bang => Unary::Bang(Box::new(Unary::new(tokens[1..].to_vec()))),
            _ => Unary::Primary(Primary::new(tokens)),
        }
    }
}
