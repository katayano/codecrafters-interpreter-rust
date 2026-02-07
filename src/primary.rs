use crate::token::Token;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Primary {
    True,
    False,
    Nil,
    StringLiterals(String),
    NumberLiterals(String),
    Expression(Box<Primary>),
}

impl fmt::Display for Primary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Primary::True => write!(f, "true"),
            Primary::False => write!(f, "false"),
            Primary::Nil => write!(f, "nil"),
            Primary::StringLiterals(str) => write!(f, "{}", str),
            Primary::NumberLiterals(num_literal) => {
                let num = num_literal.parse::<f64>().unwrap();
                if num.to_string().contains(".") {
                    write!(f, "{}", num)
                } else {
                    write!(f, "{}.0", num)
                }
            }
            Primary::Expression(content) => write!(f, "(group {})", content),
        }
    }
}

impl Primary {
    pub fn new(tokens: Vec<Token>) -> Self {
        match &tokens[0] {
            Token::ReservedWord(word) if word == "true" => Primary::True,
            Token::ReservedWord(word) if word == "false" => Primary::False,
            Token::ReservedWord(word) if word == "nil" => Primary::Nil,
            Token::StringLiterals(str) => Primary::StringLiterals(str.clone()),
            Token::NumberLiterals(num_literal) => Primary::NumberLiterals(num_literal.clone()),
            Token::LeftParen => {
                let primary = Primary::Expression(Box::new(Primary::new(
                    tokens[1..tokens.len() - 2].to_vec(),
                )));

                // Expected RightParen
                if let Token::RightParen = tokens[tokens.len() - 2] {
                    primary
                } else {
                    panic!("Expected RightParen token");
                }
            }
            _ => unimplemented!(),
        }
    }
}
