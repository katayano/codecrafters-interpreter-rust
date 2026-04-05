use crate::primary::Primary;
use crate::token::Token;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    operator: Option<Token>,
    operand: Option<Box<Unary>>,
    primary: Option<Primary>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(operator) = &self.operator {
            if let Some(operand) = &self.operand {
                match operator {
                    Token::Minus => write!(f, "(- {})", operand),
                    Token::Bang => write!(f, "(! {})", operand),
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        } else if let Some(primary) = &self.primary {
            write!(f, "{}", primary)
        } else {
            unreachable!()
        }
    }
}

impl Unary {
    pub fn new(tokens: Vec<Token>) -> Self {
        match &tokens[0] {
            Token::Minus | Token::Bang => Unary {
                operator: Some(tokens[0].clone()),
                operand: Some(Box::new(Unary::new(tokens[1..].to_vec()))),
                primary: None,
            },
            _ => Unary {
                operator: None,
                operand: None,
                primary: Some(Primary::new(tokens)),
            },
        }
    }
}
