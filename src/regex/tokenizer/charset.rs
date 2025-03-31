use std::{char, str::Chars};

use crate::regex::tokenizer::{tokenizer::RegexToken, utils::expand_escape};

#[derive(PartialEq)]
enum CharsetState {
	Continue,
	Exit,
}

fn check_if_negative_charset(chars: &mut Chars<'_>, charset: &mut String) -> bool {
	let mut is_negative = false;

	if let Some(char) = chars.next() {
		match char {
			'^' => {
				is_negative = true;
			},
			'\\' => {
				if let Some(c) = chars.next() {
					charset.push(expand_escape(c));
				} else {
					panic!("No Ending bracket");
				}
			}
			_ => {
				charset.push(char);
			},
		}
	}
	return is_negative;
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


pub fn extract_charset(chars: &mut Chars<'_>) -> RegexToken {
	let mut charset = String::new();
	
	let is_negative = check_if_negative_charset(chars, &mut charset);
	
	while let Some(char) = chars.next() {
		match char {
			']' => {
				return RegexToken::Charset(charset, is_negative);
			},
			'\\' => {
				if let Some(c) = chars.next() {
					charset.push(expand_escape(c));
				} else {
					panic!("No Ending bracket");
				}
			},
			'-' => {
				if minus_gesture(chars, &mut charset) == CharsetState::Exit {
					return RegexToken::Charset(charset, is_negative);
				}
			}
			_ => {
				charset.push(char);
			},
		}
	}
	panic!("No Ending bracket");
}