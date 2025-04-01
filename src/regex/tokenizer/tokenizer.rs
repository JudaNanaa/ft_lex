use super::{concatenation::add_concatenation_token, postfix::postfix_notation, quotes::get_string_under_quotes, *};
use std::{char, str::Chars};

const WHITESPACE: &str = " \n\r\t";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
	Or,
	TrailingContent,
	OpenGroup,
	CloseGroup,
	Quantifier(Quantifier),
	Concatenation,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Char(char),
	Operator(Operator),
}

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
                token_list.push(Token::Operator(Operator::Quantifier(quantifier)));
            }
            '(' => {
				token_list.push(Token::Operator(Operator::OpenGroup));
            }
            ')' => {
				token_list.push(Token::Operator(Operator::CloseGroup));
            }
            '|' => token_list.push(Token::Operator(Operator::Or)),
            '/' => token_list.push(Token::Operator(Operator::TrailingContent)),
            '?' => token_list.push(Token::Operator(Operator::Quantifier(Quantifier::Range(0, 1)))),
            '*' => token_list.push(Token::Operator(Operator::Quantifier(Quantifier::AtLeast(0)))),
            '+' => token_list.push(Token::Operator(Operator::Quantifier(Quantifier::AtLeast(1)))),
            c => {
                if WHITESPACE.contains(c) {
                    break;
                }
                token_list.push(Token::Char(current_char))
            }
        }
    }
	token_list = add_concatenation_token(token_list);
	token_list = postfix_notation(token_list);
    return token_list;
}
