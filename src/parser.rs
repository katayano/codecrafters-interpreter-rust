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
            _ => unimplemented!(),
        }
    }
}

impl From<Token> for Parser {
    fn from(token: Token) -> Self {
        Parser { token }
    }
}
