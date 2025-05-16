use lazy_static::lazy_static;
use num::BigInt;
use num::bigint::Sign;
use regex::Regex;

enum Token {
    Int(BigInt),
    Float(f64),
}

fn next_token(text: &str) -> Option<(Token, &str)> {
    None
}

fn tokenize(text: &str) -> Vec<Token> {
    vec![]
}


lazy_static! {
    static ref regexes: Vec<(Regex, fn(&str) -> Token)> = vec![
        (Regex::new("^([0-9]+)$").unwrap(), |integer_text: &str| {
            let sign_char = integer_text.chars().nth(0).unwrap();
            let sign = match sign_char{
                '+' => Sign::Plus,
                '-' => Sign::Minus,
                _ => panic!("Invalid sign")
            };
            let integer_absolute_text = &integer_text[1..];
            let mut digits = vec![];
            for ch in integer_absolute_text.chars() {
                digits.push(ch as u32)
            }
            Token::Int(BigInt::new(sign, digits))
        })
    ];
}

