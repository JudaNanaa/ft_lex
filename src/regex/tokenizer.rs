use std::str::Chars;

#[derive(Debug)]
pub enum RegexToken {
	Char(char),
	Or,
    Star,
	Optional,
	OpenCharSet,
	CloseCharSet,
	OpenGroup,
	CloseGroup,
}

// i want to paerse this -> "a|b*c"

fn get_string_under_quotes(chars: &mut Chars<'_>, quote_to_match: char) -> Vec<RegexToken> {
    let mut dest: String = String::new();
    let mut last_seen_backslash: bool = false;

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
    return string_to_tokens(dest);
}

fn string_to_tokens(str: String) -> Vec<RegexToken> {
    let mut token_string: Vec<RegexToken> = Vec::new();
	let mut str_chars: Chars<'_> = str.chars(); 

	while let Some(char) = str_chars.next() {
		token_string.push(RegexToken::Char(char));
	}
	return token_string;
}

fn expand_escape(c: char) -> char {
	match c {
		'n' => return '\n',
		't' => return '\t',
		'r' => return '\r',
		_ => return c,
	}
}

pub fn regex_tokenizer(regex: &String) -> Vec<RegexToken> {
    let mut token_list: Vec<RegexToken> = Vec::new();

    let mut chars: Chars<'_> = regex.chars();

    while let Some(char) = chars.next() {
        match char {
            '"' | '\'' => {
                let mut token_str: Vec<RegexToken> = get_string_under_quotes(&mut chars, char);
                token_list.append(&mut token_str);
            }
            '\\' => {
                if let Some(c) = chars.next() {
                    token_list.push(RegexToken::Char(expand_escape(c)));
                } else {
                    token_list.push(RegexToken::Char('\\'));
                }
            }
            '[' => token_list.push(RegexToken::OpenCharSet),
            ']' => token_list.push(RegexToken::CloseCharSet),
            '(' => token_list.push(RegexToken::OpenGroup),
            ')' => token_list.push(RegexToken::CloseGroup),
            '?' => token_list.push(RegexToken::Optional),
            '|' => token_list.push(RegexToken::Or),
            '*' => token_list.push(RegexToken::Star),
            _ => token_list.push(RegexToken::Char(char)),
        }
    }
    return token_list;
}

