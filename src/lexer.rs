use lazy_static::lazy_static;
use num::BigInt;
use num::bigint::Sign;
use regex::Regex;
use crate::lexer::Token::Whitespace;

#[derive(Debug)]
pub enum Token {
    Int(BigInt),
    Float(f64),
    String(String),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Whitespace(String),
    Word(String),
}

#[derive(Debug)]
pub struct TokenSource {
    token: Token,
    start: usize,
    end: usize,
}

fn next_token(text: &str) -> Option<(Token, usize, &str)> {
    for (regex, processor) in regexes.iter() {
        match Regex::find(regex, text) {
            Some(x) => {
                let length = x.end();
                let match_text = x.as_str();
                let match_start = x.start();
                let match_end = x.end();
                return Some((processor(match_text), length, &text[match_end..]))
            },
            None => {}
        }
    }
    None
}

pub fn tokenize(text: &str) -> Vec<TokenSource> {
    let mut position = 0;
    let mut current_text = text;
    let mut tokens: Vec<TokenSource> = Vec::new();
    while true {
        match next_token(current_text) {
            Some((token, length, text)) => {
                current_text = text;
                let token = TokenSource {
                    token,
                    start: position,
                    end: position + length,
                };
                position += length;
                tokens.push(token);
            }
            None => {
                break
            }
        }
    }

    tokens
}


lazy_static! {
    static ref regexes: Vec<(Regex, fn(&str) -> Token)> = vec![
        (Regex::new("^([0-9]+)").unwrap(), |integer_text: &str| {
            let first_char = integer_text.chars().nth(0).unwrap();
            let (sign, absolute_text) = match first_char {
                '+' => (Sign::Plus, &integer_text[1..]),
                '-' => (Sign::Minus, &integer_text[1..]),
                _ => (Sign::Plus, integer_text),
            };
            let mut digits = vec![];
            for ch in absolute_text.chars() {
                //digits.push((ch as u8) - 48)
                digits.push(ch as u8)
            }
            let mut bigint = BigInt::parse_bytes(digits.as_slice(), 10).unwrap();
            if sign == Sign::Minus {
                bigint = -bigint;
            }
            Token::Int(bigint)
        }),
        
        (Regex::new("^\\(").unwrap(), | text | {
            Token::OpenParen
        }),
        
        (Regex::new("^(\\s)+").unwrap(), | text | {
            Whitespace(text.to_string())
        })
    ];
}

