use crate::token::Token;
use std::fmt;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(String),
    Grouping(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Binary(left, operator, right) => match operator {
                Token::Plus => write!(f, "(+ {} {})", left, right),
                Token::Minus => write!(f, "(- {} {})", left, right),
                Token::Star => write!(f, "(* {} {})", left, right),
                Token::Slash => write!(f, "(/ {} {})", left, right),
                Token::EqualEqual => write!(f, "(== {} {})", left, right),
                Token::BangEqual => write!(f, "(!= {} {})", left, right),
                Token::Greater => write!(f, "(> {} {})", left, right),
                Token::GreaterEqual => write!(f, "(>= {} {})", left, right),
                Token::Less => write!(f, "(< {} {})", left, right),
                Token::LessEqual => write!(f, "(<= {} {})", left, right),
                _ => unreachable!(),
            },
            Expr::Unary(operator, right) => match operator {
                Token::Minus => write!(f, "(- {})", right),
                Token::Bang => write!(f, "(! {})", right),
                _ => unreachable!(),
            },
            Expr::Literal(value) => write!(f, "{}", value),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&[Token::EqualEqual, Token::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn match_token(&mut self, types: &[Token]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.position == self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.position - 1]
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(&[
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[Token::Star, Token::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[Token::Bang, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&[Token::False]) {
            Expr::Literal("false".to_string())
        } else if self.match_token(&[Token::True]) {
            Expr::Literal("true".to_string())
        } else if self.match_token(&[Token::Nil]) {
            Expr::Literal("nil".to_string())
        } else {
            match self.peek().clone() {
                Token::NumberLiterals(num_literal) => {
                    self.advance();
                    let num = num_literal.parse::<f64>().unwrap();
                    if num.to_string().contains(".") {
                        Expr::Literal(num.to_string())
                    } else {
                        Expr::Literal(format!("{}.0", num))
                    }
                }
                Token::StringLiterals(string) => {
                    self.advance();
                    Expr::Literal(string.clone())
                }
                _ if self.match_token(&[Token::LeftParen]) => {
                    let expr = self.expression();
                    self.consume(&Token::RightParen, "Expected ')' after expression.");
                    Expr::Grouping(Box::new(expr))
                }
                _ => panic!("Expected expression."),
            }
        }
    }

    fn consume(&mut self, token_type: &Token, message: &str) {
        if self.check(token_type) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    pub fn print_parser(&mut self) {
        let expr = self.expression();
        println!("{}", expr);
    }
}
