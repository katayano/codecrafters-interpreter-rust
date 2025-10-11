use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader, Write};

mod parser;
mod reserved_words;
mod token;
mod tokenizer;

use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(
            io::stderr(),
            "Usage: {} (tokenize | parse) <filename>",
            args[0]
        )
        .unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            tokenize(filename);
        }
        "parse" => {
            parse(filename);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(filename: &str) {
    let file = File::open(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to open file {}", filename).unwrap();
        std::process::exit(1);
    });

    let mut has_lexical_error = false;
    for (i, line_content) in BufReader::new(file).lines().enumerate() {
        if let Ok(content) = line_content {
            let token_list = interpret_tokens(i + 1, content);
            if let Err(token_list) = token_list {
                has_lexical_error = true;
                print_tokens(&token_list, "tokenizer");
            } else {
                print_tokens(&token_list.unwrap(), "tokenizer");
            }
        }
    }
    println!("EOF  null");
    std::process::exit(if has_lexical_error { 65 } else { 0 });
}

fn parse(filename: &str) {
    let file = File::open(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to open file {}", filename).unwrap();
        std::process::exit(1);
    });

    let mut has_lexical_error = false;
    for (i, line_content) in BufReader::new(file).lines().enumerate() {
        if let Ok(content) = line_content {
            let token_list = interpret_tokens(i + 1, content);
            if let Err(token_list) = token_list {
                has_lexical_error = true;
                print_tokens(&token_list, "parser");
            } else {
                print_tokens(&token_list.unwrap(), "parser");
            }
        }
    }
    std::process::exit(if has_lexical_error { 65 } else { 0 });
}

/// Interpreter that processes tokens and prints them.
/// # Arguments
/// * `line_number` - The current line number being processed.
/// * `tokens` - A string containing the tokens to be interpreted.
/// # Returns
/// * `Result<(), ()>` - Returns Err(()) if there was a lexical error, Ok(()) otherwise.
fn interpret_tokens(line_number: usize, tokens: String) -> Result<Vec<Token>, Vec<Token>> {
    let mut token_list = Vec::new();

    let mut string_literal_tmp = String::new();

    for token in tokens.chars() {
        let token = if token_list.last() == Some(&Token::UnterminatedString(line_number)) {
            // If there is an unterminated string, keep appending characters until we find the closing quote
            if token == '"' {
                let string_literal = string_literal_tmp.clone();
                string_literal_tmp.clear();
                token_list.pop(); // Remove the UnterminatedString token
                token_list.push(Token::StringLiterals(string_literal));
            } else {
                string_literal_tmp.push(token);
            }
            continue;
        } else {
            match token {
                '!' => Token::Bang,
                ',' => Token::Comma,
                '.' => {
                    if let Some(Token::NumberLiterals(num)) = token_list.last() {
                        let number_literal = format!("{}.", num);
                        Token::NumberLiterals(number_literal)
                    } else {
                        Token::Dot
                    }
                }
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
                ' ' => Token::Space,
                '\t' => Token::Tab,
                '\n' => Token::Newline,
                '0'..='9' => match token_list.last() {
                    Some(Token::NumberLiterals(num)) => {
                        Token::NumberLiterals(format!("{}{}", num, token))
                    }
                    Some(Token::Identifier(id)) => Token::Identifier(format!("{}{}", id, token)),
                    _ => Token::NumberLiterals(token.to_string()),
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = match token_list.last() {
                        Some(Token::Identifier(id)) => format!("{}{}", id, token),
                        _ => token.to_string(),
                    };
                    Token::Identifier(identifier)
                }
                '"' => Token::UnterminatedString(line_number),
                _ => Token::UnexpectedToken(line_number, token),
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
            Token::Slash if matches!(token_list.last(), Some(&Token::Slash)) => {
                token_list.pop();
                token_list.push(Token::Comment);
                // Ignore the rest of the line after a comment
                break;
            }
            Token::NumberLiterals(_)
                if matches!(token_list.last(), Some(Token::NumberLiterals(_))) =>
            {
                token_list.pop(); // Remove the last number literal
                token_list.push(token);
            }
            Token::Identifier(ref id)
                if matches!(token_list.last(), Some(Token::Identifier(_))) =>
            {
                token_list.pop(); // Remove the last identifier
                token_list.push(
                    // Check if the identifier is a reserved word
                    if reserved_words::RESERVED_WORDS.contains(&id.as_str()) {
                        Token::ReservedWord(id.to_string())
                    } else {
                        token
                    },
                );
            }
            _ => token_list.push(token),
        }
    }

    if token_list.iter().any(|t| {
        matches!(
            t,
            Token::UnexpectedToken(_, _) | Token::UnterminatedString(_)
        )
    }) {
        Err(token_list)
    } else {
        Ok(token_list)
    }
}

fn print_tokens(tokens: &[Token], printed_by: &str) {
    for token in tokens {
        match token {
            Token::UnexpectedToken(line_number, token) => {
                writeln!(
                    io::stderr(),
                    "[line {}] Error: Unexpected character: {}",
                    line_number,
                    token
                )
                .unwrap();
            }
            Token::UnterminatedString(line_number) => {
                writeln!(
                    io::stderr(),
                    "[line {}] Error: Unterminated string.",
                    line_number
                )
                .unwrap();
            }
            Token::Comment => break,
            // space and tab and newline are ignored
            Token::Space | Token::Tab | Token::Newline => continue,
            _ => {
                if printed_by == "tokenizer" {
                    println!("{}", tokenizer::Tokenizer::from(token.clone()));
                } else if printed_by == "parser" {
                    println!("{}", parser::Parser::from(token.clone()));
                }
            }
        }
    }
}
