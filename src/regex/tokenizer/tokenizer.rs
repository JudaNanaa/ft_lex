use super::{concatenation::add_concatenation_token, group::extract_group, quotes::get_string_under_quotes, *};
use std::{char, str::Chars};

const WHITESPACE: &str = " \n\r\t";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RegexToken {
    Char(char),
    Or,
    TrailingContent,
    OpenGroup,
    CloseGroup,
    Quantifier(Quantifier),
	Concatenation,
}

pub fn regex_tokenizer(regex: &String) -> Vec<RegexToken> {
    let mut token_list: Vec<RegexToken> = Vec::new();
    let mut chars: Chars<'_> = regex.chars();

    while let Some(current_char) = chars.next() {
        match current_char {
            '"' => {
                let mut string_tokens = get_string_under_quotes(&mut chars, '"');
                token_list.append(&mut string_tokens);
            }
            '\\' => {
                if let Some(escaped_char) = chars.next() {
                    token_list.push(RegexToken::Char(expand_escape(escaped_char)));
                } else {
                    token_list.push(RegexToken::Char('\\'));
                }
            }
            '[' => {
                let mut charset_tokens = extract_charset(&mut chars);
                token_list.append(&mut charset_tokens);
            }
            '{' => {
                let quantifier = extract_repetition_range(&mut chars);
                token_list.push(RegexToken::Quantifier(quantifier));
            }
            '(' => {
                let mut group_tokens = extract_group(&mut chars);
                token_list.append(&mut group_tokens);
            }
            '|' => token_list.push(RegexToken::Or),
            '/' => token_list.push(RegexToken::TrailingContent),
            '?' => token_list.push(RegexToken::Quantifier(Quantifier::Range(0, 1))),
            '*' => token_list.push(RegexToken::Quantifier(Quantifier::AtLeast(0))),
            '+' => token_list.push(RegexToken::Quantifier(Quantifier::AtLeast(1))),
            c => {
                if WHITESPACE.contains(c) {
                    break;
                }
                token_list.push(RegexToken::Char(current_char))
            }
        }
    }
	token_list = add_concatenation_token(token_list);
    return token_list;
}
