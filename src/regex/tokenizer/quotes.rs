use crate::regex::utils::expand_escape;

use super::Operator::CloseParen;
use super::Operator::OpenParen;
use super::Token;
use super::Token::Operator;
use std::str::Chars;

fn string_to_tokens(str: String) -> Vec<Token> {
    let mut token_string: Vec<Token> = Vec::new();
    let str_chars: Chars<'_> = str.chars();

    token_string.push(Operator(OpenParen));
    for char in str_chars {
        token_string.push(Token::Char(char));
    }
    token_string.push(Operator(CloseParen));
    return token_string;
}

pub fn get_string_under_quotes(chars: &mut Chars<'_>, quote_to_match: char) -> Vec<Token> {
    let mut dest: String = String::new();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(expended) = expand_escape(chars) {
                    dest.push(expended);
                } else {
                    panic!("unclose quotes");
                }
            }
            q if q == quote_to_match => {
                return string_to_tokens(dest);
            }
            _ => {
                dest.push(c);
            }
        }
    }
    panic!("Unclose quotes");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::Chars;

    fn to_chars(input: &str) -> Chars<'_> {
        input.chars()
    }

    #[test]
    fn test_basic_double_quotes() {
        let mut chars = to_chars("hello\"");
        let tokens = get_string_under_quotes(&mut chars, '"');
        let expected = vec![
            Operator(OpenParen),
            Token::Char('h'),
            Token::Char('e'),
            Token::Char('l'),
            Token::Char('l'),
            Token::Char('o'),
            Operator(CloseParen),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_basic_single_quotes() {
        let mut chars = to_chars("world'");
        let tokens = get_string_under_quotes(&mut chars, '\'');
        let expected = vec![
            Operator(OpenParen),
            Token::Char('w'),
            Token::Char('o'),
            Token::Char('r'),
            Token::Char('l'),
            Token::Char('d'),
            Operator(CloseParen),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_hard_double_quotes() {
        let mut chars = to_chars("wor\\\"ld\"");
        let tokens = get_string_under_quotes(&mut chars, '"');
        let expected = vec![
            Operator(OpenParen),
            Token::Char('w'),
            Token::Char('o'),
            Token::Char('r'),
            Token::Char('"'),
            Token::Char('l'),
            Token::Char('d'),
            Operator(CloseParen),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic(expected = "Unclose quotes")]
    fn test_unclosed_quotes_should_panic() {
        let mut chars = to_chars("oops no end");
        let _ = get_string_under_quotes(&mut chars, '"');
    }
}
