use std::{char, collections::LinkedList, str::Chars};

#[derive(Debug)]
pub enum RegexToken {
	Char(char),
	Or,
    Star,
	Escape(char),
	String(String),
}

// i want to paerse this -> "a|b*c"

fn get_string_under_quotes(chars: &mut Chars<'_>, quote_to_match: char) -> String {
    let mut dest = String::new();
    let mut last_seen_backslash = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' if !last_seen_backslash => last_seen_backslash = true,
            q if q == quote_to_match && !last_seen_backslash => break,
            _ => {
                if last_seen_backslash {
                    dest.push('\\');
                }
                dest.push(c);
                last_seen_backslash = false;
            }
        }
    }
    return dest;
}

pub fn regex_tokenizer(regex: &str) -> LinkedList<RegexToken> {
    let mut token_list: LinkedList<RegexToken> = LinkedList::new();
    let mut chars = regex.chars();

    while let Some(char) = chars.next() {
        match char {
            '"' | '\'' => {
                let str = get_string_under_quotes(&mut chars, char);
                token_list.push_back(RegexToken::String(str));
            }
            '\\' => {
                if let Some(c) = chars.next() {
                    token_list.push_back(RegexToken::Escape(c));
                } else {
                    token_list.push_back(RegexToken::Escape('\\'));
                }
            }
            '|' => token_list.push_back(RegexToken::Or),
            '*' => token_list.push_back(RegexToken::Star),
            _ => token_list.push_back(RegexToken::Char(char)),
        }
    }
    return token_list;
}
