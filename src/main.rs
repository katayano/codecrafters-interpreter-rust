use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader, Write};

mod reserved_words;

#[derive(Debug, PartialEq, Clone)]
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
    Space,
    Tab,
    Newline,
    Comment,
    ReservedWord(String),
    StringLiterals(String),
    NumberLiterals(String),
    Identifier(String),
    UnexpectedToken(usize, char),
    UnterminatedString(usize),
}

impl fmt::Display for Token {
    // String format is "<TOKEN_TYPE> <LEXEME> <LITERAL>"
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

            let mut has_lexical_error = false;
            for (i, line_content) in BufReader::new(file).lines().enumerate() {
                if let Ok(content) = line_content {
                    if let Err(_) = interpret_tokens(i + 1, content) {
                        has_lexical_error = true;
                    }
                }
            }
            println!("EOF  null");
            std::process::exit(if has_lexical_error { 65 } else { 0 });
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
/// * `Result<(), ()>` - Returns Err(()) if there was a lexical error, Ok(()) otherwise.
fn interpret_tokens(line_number: usize, tokens: String) -> Result<(), ()> {
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
    print_tokens(&token_list);

    if token_list.iter().any(|t| {
        matches!(
            t,
            Token::UnexpectedToken(_, _) | Token::UnterminatedString(_)
        )
    }) {
        Err(())
    } else {
        Ok(())
    }
}

fn print_tokens(tokens: &[Token]) {
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
                println!("{}", token);
            }
        }
    }
}
