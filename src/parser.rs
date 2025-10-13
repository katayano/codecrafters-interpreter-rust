use std::fmt;

use crate::token::Token;

pub struct Parser {
    token: Token,
}

impl fmt::Display for Parser {
    // String format is "<TOKEN_TYPE> <LEXEME> <LITERAL>"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token {
            Token::ReservedWord(word) => write!(f, "{}", word),
            Token::StringLiterals(str) => write!(f, "{}", str),
            Token::NumberLiterals(num_literal) => {
                // For Example, "123" -> "123.0", "123.45" -> "123.45", "123.000" -> "123.0"
                let num = num_literal.parse::<f64>().unwrap();
                if num.to_string().contains(".") {
                    write!(f, "{}", num)
                } else {
                    write!(f, "{}.0", num)
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl From<Token> for Parser {
    fn from(token: Token) -> Self {
        Parser { token }
    }
}
