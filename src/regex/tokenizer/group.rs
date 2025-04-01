use std::str::Chars;

use crate::regex::{
    tokenizer::{charset::extract_charset, quantifier::Quantifier, utils::expand_escape},
    RegexToken,
};

pub fn extract_group(chars: &mut Chars<'_>) -> Vec<RegexToken> {
    let mut token_group: Vec<RegexToken> = Vec::new();

    token_group.push(RegexToken::OpenGroup);
    while let Some(char) = chars.next() {
        match char {
            ')' => {
                token_group.push(RegexToken::CloseGroup);
                return token_group;
            }
            '\\' => {
                if let Some(c) = chars.next() {
                    token_group.push(RegexToken::Char(expand_escape(c)));
                } else {
                    token_group.push(RegexToken::Char('\\'));
                }
            }
            '[' => {
                let mut token_charset = extract_charset(chars);
                token_group.append(&mut token_charset);
            }
            '(' => {
                let mut group = extract_group(chars);
                token_group.append(&mut group);
            }
            '|' => token_group.push(RegexToken::Or),
            '/' => token_group.push(RegexToken::TrailingContent),
            '?' => token_group.push(RegexToken::Quantifier(Quantifier::Range(0, 1))),
            '*' => token_group.push(RegexToken::Quantifier(Quantifier::AtLeast(0))),
            '+' => token_group.push(RegexToken::Quantifier(Quantifier::AtLeast(1))),
            _ => token_group.push(RegexToken::Char(char)),
        }
    }
    panic!("Unclose parenthesis");
}
