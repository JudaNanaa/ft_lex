use crate::regex::utils::expand_escape;

use super::Operator::CloseParen;
use super::Operator::OpenParen;
use super::Operator::Or;
use super::Token::Operator;
use super::*;
use std::{char, str::Chars};

#[derive(PartialEq)]
enum CharsetState {
    Continue,
    Exit,
}

fn create_charset_group(charset: String, is_negative: bool) -> Vec<Token> {
    let mut tokens_charset = Vec::new();

    tokens_charset.push(Operator(OpenParen));
    if is_negative == false {
        let mut chars_it = charset.chars().peekable();
        while let Some(char) = chars_it.next() {
            tokens_charset.push(Token::Char(char));
            if chars_it.peek().is_some() {
                tokens_charset.push(Operator(Or));
            }
        }
    } else {
        let all_chars = (0..=255u8) // Using ASCII range for simplicity
            .filter_map(|c| char::from_u32(c as u32))
            .collect::<Vec<char>>();

        let mut iter = all_chars.iter().peekable();

        let charset_chars: Vec<char> = charset.chars().collect();

        while let Some(c) = iter.next() {
            if !charset_chars.contains(c) {
                tokens_charset.push(Token::Char(*c));
                if iter.peek().is_some() {
                    tokens_charset.push(Operator(Or));
                }
            }
        }
    }
    tokens_charset.push(Operator(CloseParen));
    return tokens_charset;
}

fn check_if_negative_charset(chars: &mut Chars<'_>, charset: &mut String) -> (bool, CharsetState) {
    let mut is_negative = false;

    if let Some(char) = chars.next() {
        match char {
            '^' => is_negative = true,
            ']' => return (is_negative, CharsetState::Exit),
            '\\' => {
                if let Some(escaped_char) = expand_escape(chars) {
                    charset.push(escaped_char);
                } else {
                    panic!("No Ending bracket");
                }
            }
            _ => charset.push(char),
        }
    }
    return (is_negative, CharsetState::Continue);
}

fn expand_minus(mut char_begin: char, char_end: char) -> String {
    let mut range_chars = String::new();

    if char_begin > char_end {
        panic!("Range values reversed. Start char code is greater than end char code.");
    }

    while char_begin <= char_end {
        range_chars.push(char_begin);

        char_begin = match (char_begin as u32).checked_add(1).and_then(char::from_u32) {
            Some(next_char) => next_char,
            None => break,
        };
    }

    return range_chars;
}

fn minus_gesture(chars: &mut Chars<'_>, charset: &mut String) -> CharsetState {
    if let Some(char) = chars.next() {
        match char {
            ']' => {
                charset.push('-');
                return CharsetState::Exit;
            }
            '\\' => {
                if let Some(escaped_char) = expand_escape(chars) {
                    charset.push(escaped_char);
                } else {
                    panic!("No Ending bracket");
                }
            }
            _ => {
                charset.push(char);
            }
        }
        if charset.len() < 2 {
            charset.push('-');
            return CharsetState::Continue;
        }
        let char_end = charset.pop().unwrap();
        let char_begin = charset.pop().unwrap();
        charset.push_str(&expand_minus(char_begin, char_end));
    } else {
        panic!("No Ending bracket");
    }
    return CharsetState::Continue;
}

pub fn extract_charset(chars: &mut Chars<'_>) -> Vec<Token> {
    let mut charset = String::new();

    let (is_negative, state) = check_if_negative_charset(chars, &mut charset);

    if state == CharsetState::Exit {
        return create_charset_group(charset, is_negative);
    }
    while let Some(char) = chars.next() {
        match char {
            ']' => {
                return create_charset_group(charset, is_negative);
            }
            '\\' => {
                if let Some(escaped_char) = expand_escape(chars) {
                    charset.push(escaped_char);
                } else {
                    panic!("No Ending bracket");
                }
            }
            '-' => {
                if minus_gesture(chars, &mut charset) == CharsetState::Exit {
                    return create_charset_group(charset, is_negative);
                }
            }
            _ => {
                charset.push(char);
            }
        }
    }
    panic!("No Ending bracket");
}

pub fn expand_dot() -> Vec<Token> {
    let mut dest = Vec::new();
    let mut all_chars: Vec<char> = (0..=255u8)
        .filter_map(|c| char::from_u32(c as u32))
        .collect();

    all_chars.remove('\n' as usize);

    let mut iter = all_chars.iter().peekable();
    dest.push(Operator(OpenParen));
    while let Some(char) = iter.next() {
        dest.push(Token::Char(*char));
        if iter.peek().is_some() {
            dest.push(Operator(Or));
        }
    }
    dest.push(Operator(CloseParen));
    return dest;
}

// ------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens_to_string(tokens: &[Token]) -> String {
        tokens
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_extract_charset_basic() {
        let mut chars = "[abc]".chars();
        chars.next(); // skip '['
        let tokens = extract_charset(&mut chars);

        let expected = vec![
            Operator(OpenParen),
            Token::Char('a'),
            Operator(Or),
            Token::Char('b'),
            Operator(Or),
            Token::Char('c'),
            Operator(CloseParen),
        ];

        assert_eq!(tokens, expected, "{}", tokens_to_string(&tokens));
    }

    #[test]
    fn test_extract_charset_negative() {
        let mut chars = "[^x]".chars();
        chars.next();
        let tokens = extract_charset(&mut chars);

        // Just check it’s negative and contains lots of tokens (not x)
        assert!(tokens.contains(&Operator(OpenParen)));
        assert!(tokens.contains(&Operator(CloseParen)));
        assert!(!tokens.contains(&Token::Char('x')));
    }

    #[test]
    fn test_extract_charset_range() {
        let mut chars = "[a-c]".chars();
        chars.next();
        let tokens = extract_charset(&mut chars);

        let expected = vec![
            Operator(OpenParen),
            Token::Char('a'),
            Operator(Or),
            Token::Char('b'),
            Operator(Or),
            Token::Char('c'),
            Operator(CloseParen),
        ];

        assert_eq!(tokens, expected, "{}", tokens_to_string(&tokens));
    }

    #[test]
    fn test_extract_charset_with_escape() {
        let mut chars = "[\\n]".chars();
        chars.next();
        let tokens = extract_charset(&mut chars);

        let expected = vec![Operator(OpenParen), Token::Char('\n'), Operator(CloseParen)];

        assert_eq!(tokens, expected, "{}", tokens_to_string(&tokens));
    }

    #[test]
    fn test_extract_charset_with_dash_as_char() {
        let mut chars = "[-]".chars();
        chars.next();
        let tokens = extract_charset(&mut chars);

        let expected = vec![Operator(OpenParen), Token::Char('-'), Operator(CloseParen)];

        assert_eq!(tokens, expected, "{}", tokens_to_string(&tokens));
    }

    #[test]
    fn test_expand_dot() {
        let tokens = expand_dot();

        assert!(tokens.contains(&Token::Char('a')));
        assert!(!tokens.contains(&Token::Char('\n')));
        assert!(tokens.contains(&Operator(Or)));
        assert!(tokens.len() > 50); // should include most printable ASCII chars
    }
}
