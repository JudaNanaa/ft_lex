use super::{group::extract_group, quotes::get_string_under_quotes, *};
use std::{char, str::Chars};

const BLANK: &str = " \n\r\t";

#[derive(Debug, PartialEq)]
pub enum RegexToken {
    Char(char),
    Or,
    TrailingContent,
    OpenGroup,
    CloseGroup,
    Quantifier(Quantifier),
}

pub fn regex_tokenizer(regex: &String) -> Vec<RegexToken> {
    let mut token_list: Vec<RegexToken> = Vec::new();

    let mut chars: Chars<'_> = regex.chars();

    while let Some(char) = chars.next() {
        match char {
            '"' => {
                let mut token_str: Vec<RegexToken> = get_string_under_quotes(&mut chars, '"');
                token_list.append(&mut token_str);
            }
            '\\' => {
                if let Some(c) = chars.next() {
                    token_list.push(RegexToken::Char(expand_escape(c)));
                } else {
                    token_list.push(RegexToken::Char('\\'));
                }
            }
            '[' => {
                let mut token_charset = extract_charset(&mut chars);
                token_list.append(&mut token_charset);
            }
            '{' => {
                let quantifier = extract_repetition_range(&mut chars);
                token_list.push(RegexToken::Quantifier(quantifier));
            }
            '(' => {
                let mut group = extract_group(&mut chars);
                token_list.append(&mut group);
            }
            '|' => token_list.push(RegexToken::Or),
            '/' => token_list.push(RegexToken::TrailingContent),
            '?' => token_list.push(RegexToken::Quantifier(Quantifier::Range(0, 1))),
            '*' => token_list.push(RegexToken::Quantifier(Quantifier::AtLeast(0))),
            '+' => token_list.push(RegexToken::Quantifier(Quantifier::AtLeast(1))),
            c => {
                if BLANK.contains(c) {
                    break;
                }
                token_list.push(RegexToken::Char(char))
            }
        }
    }
    return token_list;
}
