use super::*;
use std::{char, str::Chars};
use super::Token::Operator as Operator;
use super::Operator::OpenGroup as OpenGroup;
use super::Operator::Or as Or;

#[derive(PartialEq)]
enum CharsetState {
    Continue,
    Exit,
}

fn create_charset_group(charset: String, is_negative: bool) -> Vec<Token> {
    let mut tokens_charset = Vec::new();

    tokens_charset.push(Operator(OpenGroup));
    if is_negative == false {
        let mut chars_it = charset.chars();
        while let Some(char) = chars_it.next() {
            tokens_charset.push(Token::Char(char));
            tokens_charset.push(Operator(Or));
        }
    } else {
        let all_chars = (0..=127u8) // Using ASCII range for simplicity
            .filter_map(|c| char::from_u32(c as u32))
            .collect::<Vec<char>>();

        // Filter out characters that are in the charset
        let charset_chars: Vec<char> = charset.chars().collect();

        for c in all_chars {
            if !charset_chars.contains(&c) {
                tokens_charset.push(Token::Char(c));
                tokens_charset.push(Operator(Or));
            }
        }
    }
    if let Some(token) = tokens_charset.last() {
        if *token == Operator(Or) {
            tokens_charset.pop();
        }
    }
    tokens_charset.push(Operator(OpenGroup));
    return tokens_charset;
}

fn check_if_negative_charset(chars: &mut Chars<'_>, charset: &mut String) -> (bool, CharsetState) {
    let mut is_negative = false;

    if let Some(char) = chars.next() {
        match char {
            '^' => is_negative = true,
            ']' => return (is_negative, CharsetState::Exit),
            '\\' => {
                if let Some(c) = chars.next() {
                    charset.push(expand_escape(c));
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
                if let Some(c) = chars.next() {
                    charset.push(expand_escape(c));
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
                if let Some(c) = chars.next() {
                    charset.push(expand_escape(c));
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
