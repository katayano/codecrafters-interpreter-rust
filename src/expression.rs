use crate::token::Token;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    True,
    False,
    Nil,
    StringLiterals(String),
    NumberLiterals(String),
    Group(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::True => write!(f, "true"),
            Expression::False => write!(f, "false"),
            Expression::Nil => write!(f, "nil"),
            Expression::StringLiterals(str) => write!(f, "{}", str),
            Expression::NumberLiterals(num_literal) => {
                let num = num_literal.parse::<f64>().unwrap();
                if num.to_string().contains(".") {
                    write!(f, "{}", num)
                } else {
                    write!(f, "{}.0", num)
                }
            }
            Expression::Group(content) => write!(f, "(group {})", content),
        }
    }
}

impl From<Vec<Token>> for Expression {
    fn from(tokens: Vec<Token>) -> Self {
        match &tokens[0] {
            Token::ReservedWord(word) if word == "true" => Expression::True,
            Token::ReservedWord(word) if word == "false" => Expression::False,
            Token::ReservedWord(word) if word == "nil" => Expression::Nil,
            Token::StringLiterals(str) => Expression::StringLiterals(str.clone()),
            Token::NumberLiterals(num_literal) => Expression::NumberLiterals(num_literal.clone()),
            Token::LeftParen => {
                if &tokens[1] != &Token::EOF
                    && &tokens[2] != &Token::EOF
                    && &tokens[2] == &Token::RightParen
                {
                    if let Token::StringLiterals(str) = &tokens[1] {
                        Expression::Group(str.clone())
                    } else {
                        unimplemented!()
                    }
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        }
    }
}
