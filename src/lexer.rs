use lazy_static::lazy_static;
use num::BigInt;
use num::bigint::Sign;
use regex::Regex;
use crate::lexer::Token::{CloseBracket, OpenBrace, OpenBracket, Whitespace};

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
    Identifier(String),
    Keyword(String),
    Operator(String),
}

#[derive(Debug)]
pub struct TokenSource {
    token: Token,
    start: usize,
    end: usize,
}


enum Parser {
    Regex(Regex, fn(&str) -> Token),
    Function(fn(&str) -> Option<(Token, &str)>),
}

fn next_token(text: &str) -> Option<(Token, usize, &str)> {
    for parser in regexes.iter() {
        match parser {
            Parser::Regex(regex, processor) => {
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
            Parser::Function(f) => {
                //
            }
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

    static ref keywords: Vec<&'static str> = vec![
        "if",
        "fn",
    ];

    static ref regexes: Vec<Parser> = vec![
        
        Parser::Regex(Regex::new("^([0-9]*[eE][1-9][0-9]*|(([1-9][0-9]*\\.)|(\\.[0-9]+))([0-9]*)?([eE][\\-\\+]?[1-9][0-9]*)?)").unwrap(), | text | {
            let f = text.parse::<f64>().unwrap();
            Token::Float( f )
        }),
        
        Parser::Regex(Regex::new("^([0-9]+)").unwrap(), |integer_text: &str| {
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

        

        Parser::Regex(Regex::new("^\\(").unwrap(), | text | {
            Token::OpenParen
        }),

        Parser::Regex(Regex::new("^\\)").unwrap(), | text | {
            Token::CloseParen
        }),

        Parser::Regex(Regex::new("^(\\s)+").unwrap(), | text | {
            Token::Whitespace(text.to_string())
        }),

        Parser::Regex(Regex::new("^\\{").unwrap(), | text | {
            Token::OpenBrace
        }),

        Parser::Regex(Regex::new("^\\}").unwrap(), | text | {
            Token::CloseBrace
        }),

        Parser::Regex(Regex::new("^\"([^\"]*)\"").unwrap(), | text | {
            Token::String(text.to_string())
        }),

        Parser::Regex(Regex::new("\\[").unwrap(), |text| {
            OpenBracket
        }),

        Parser::Regex(Regex::new("\\]").unwrap(), |text| {
            CloseBracket
        }),

        Parser::Regex(Regex::new("^([a-zA-Z]*[a-zA-Z0-9]+)").unwrap(), | text | {
            for kw in keywords.iter() {
                if text == *kw {
                    return Token::Keyword(text.to_string())
                }
            }
            return Token::Identifier(text.to_string());
        }),

        Parser::Regex(Regex::new("^\\=").unwrap(), | text | {
            Token::Operator("=".to_string())
        })
    ];
}

