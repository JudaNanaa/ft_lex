use crate::regex::utils::expand_escape;

use super::Operator::*;
use super::Quantifier::*;
use super::Token::Operator;
use super::{
    concatenation::add_concatenation_token, postfix::to_postfix, quotes::get_string_under_quotes, *,
};
use std::str::Chars;

const WHITESPACE: &str = " \n\r\t";

pub fn regex_tokenizer(regex: &String) -> Vec<Token> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut chars: Chars<'_> = regex.chars();

    while let Some(current_char) = chars.next() {
        match current_char {
            '"' => {
                let mut string_tokens = get_string_under_quotes(&mut chars, '"');
                token_list.append(&mut string_tokens);
            }
            '\\' => {
                if let Some(escaped_char) = chars.next() {
                    token_list.push(Token::Char(expand_escape(escaped_char)));
                } else {
                    token_list.push(Token::Char('\\'));
                }
            }
            '[' => {
                let mut charset_tokens = extract_charset(&mut chars);
                token_list.append(&mut charset_tokens);
            }
            '{' => {
                let quantifier = extract_repetition_range(&mut chars);
                token_list.push(Operator(Quantifier(quantifier)));
            }
            '(' => {
                token_list.push(Operator(OpenParen));
            }
            ')' => {
                token_list.push(Operator(CloseParen));
            }
            '|' => token_list.push(Operator(Or)),
            '/' => token_list.push(Operator(TrailingContent)),
            '?' => token_list.push(Operator(Quantifier(Range(0, 1)))),
            '*' => token_list.push(Operator(Quantifier(AtLeast(0)))),
            '+' => token_list.push(Operator(Quantifier(AtLeast(1)))),
            '.' => {
                let mut charset_tokens = expand_dot();
                token_list.append(&mut charset_tokens);
            }
            c => {
                if WHITESPACE.contains(c) {
                    break;
                }
                token_list.push(Token::Char(current_char))
            }
        }
    }
    token_list = add_concatenation_token(token_list);
    token_list = to_postfix(token_list);
    dbg!(&token_list);
    return token_list;
}
