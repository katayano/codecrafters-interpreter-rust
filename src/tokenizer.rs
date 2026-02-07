use std::fmt;

use crate::token::Token;

pub struct Tokenizer {
    token: Token,
}

impl fmt::Display for Tokenizer {
    // String format is "<TOKEN_TYPE> <LEXEME> <LITERAL>"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token {
            Token::Bang => write!(f, "BANG ! null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::Less => write!(f, "LESS < null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::Space => write!(f, ""),
            Token::Tab => write!(f, ""),
            Token::Newline => write!(f, ""),
            Token::Comment => write!(f, ""),
            Token::ReservedWord(word) => write!(f, "{} {} null", word.to_uppercase(), word),
            Token::StringLiterals(str) => write!(f, "STRING \"{}\" {}", str, str),
            Token::NumberLiterals(num_literal) => {
                // For Example, "123" -> "123.0", "123.45" -> "123.45", "123.000" -> "123.0"
                let num = num_literal.parse::<f64>().unwrap();
                if num.to_string().contains(".") {
                    write!(f, "NUMBER {} {}", num_literal, num)
                } else {
                    write!(f, "NUMBER {} {}.0", num_literal, num)
                }
            }
            Token::Identifier(id) => write!(f, "IDENTIFIER {} null", id),
            Token::UnexpectedToken(_, _) => write!(f, ""),
            Token::UnterminatedString(_) => write!(f, ""),
            Token::EOF => write!(f, "EOF null"),
        }
    }
}

impl From<Token> for Tokenizer {
    fn from(token: Token) -> Self {
        Tokenizer { token }
    }
}
