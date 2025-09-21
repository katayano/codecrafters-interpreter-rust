use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader, Write};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Bang,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    EqualEqual,
    BangEqual,
    UnexpectedToken,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
            Token::UnexpectedToken => write!(f, ""),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file = File::open(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to open file {}", filename).unwrap();
                std::process::exit(1);
            });

            let mut exit_code = 0;
            for (i, line_content) in BufReader::new(file).lines().enumerate() {
                if let Ok(content) = line_content {
                    exit_code = interpret_tokens(i + 1, content);
                    if exit_code != 0 {
                        break;
                    }
                }
            }
            println!("EOF  null");
            std::process::exit(exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

/// Interpreter that processes tokens and prints them.
/// # Arguments
/// * `line_number` - The current line number being processed.
/// * `tokens` - A string containing the tokens to be interpreted.
/// # Returns
/// * `i32` - Returns exit code
/// Exits with code 65 on lexical error.
fn interpret_tokens(line_number: usize, tokens: String) -> i32 {
    let mut exit_code = 0;
    let mut token_list = Vec::new();

    for token in tokens.chars() {
        let token = match token {
            '!' => Token::Bang,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            '/' => Token::Slash,
            '*' => Token::Star,
            '=' => Token::Equal,
            '<' => Token::Less,
            '>' => Token::Greater,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            _ => {
                writeln!(
                    io::stderr(),
                    "[line {}] Error: Unexpected character: {}",
                    line_number,
                    token
                )
                .unwrap();
                exit_code = 65;
                Token::UnexpectedToken
            }
        };

        match token {
            Token::Equal => match token_list.last() {
                Some(Token::Equal) => {
                    token_list.pop();
                    token_list.push(Token::EqualEqual);
                }
                Some(Token::Bang) => {
                    token_list.pop();
                    token_list.push(Token::BangEqual);
                }
                Some(Token::Less) => {
                    token_list.pop();
                    token_list.push(Token::LessEqual);
                }
                Some(Token::Greater) => {
                    token_list.pop();
                    token_list.push(Token::GreaterEqual);
                }
                _ => token_list.push(token),
            },
            _ => token_list.push(token),
        }
    }
    print_tokens(&token_list);

    exit_code
}

fn print_tokens(tokens: &[Token]) {
    for token in tokens {
        if *token != Token::UnexpectedToken {
            println!("{}", token);
        }
    }
}
