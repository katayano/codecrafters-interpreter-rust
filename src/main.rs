use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader, Write};

enum Token {
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::Star => write!(f, "STAR * null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
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
    for token in tokens.chars() {
        let token = match token {
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            '/' => Token::Slash,
            '*' => Token::Star,
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
                continue;
            }
        };
        println!("{}", token);
    }
    exit_code
}
