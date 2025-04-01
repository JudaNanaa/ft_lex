use std::str::Chars;

use super::RegexToken;

fn string_to_tokens(str: String) -> Vec<RegexToken> {
    let mut token_string: Vec<RegexToken> = Vec::new();
    let mut str_chars: Chars<'_> = str.chars();

    token_string.push(RegexToken::OpenGroup);
    while let Some(char) = str_chars.next() {
        token_string.push(RegexToken::Char(char));
    }
    token_string.push(RegexToken::CloseGroup);
    return token_string;
}

pub fn get_string_under_quotes(chars: &mut Chars<'_>, quote_to_match: char) -> Vec<RegexToken> {
    let mut dest: String = String::new();
    let mut last_seen_backslash: bool = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' if !last_seen_backslash => last_seen_backslash = true,
            q if q == quote_to_match && !last_seen_backslash => {
                return string_to_tokens(dest);
            }
            _ => {
                if last_seen_backslash {
                    dest.push('\\');
                }
                dest.push(c);
                last_seen_backslash = false;
            }
        }
    }
    panic!("Unclose quotes");
}
