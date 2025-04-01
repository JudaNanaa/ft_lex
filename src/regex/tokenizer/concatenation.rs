use super::RegexToken;

fn need_concatenate(token: &RegexToken, next: &RegexToken) -> bool {
	match token {
		RegexToken::Char(_) => {
			match next {
				RegexToken::Char(_) | RegexToken::OpenGroup => {
					return true;
				},
				_ => return false,
			}
		}
		RegexToken::CloseGroup => {
			match next {
				RegexToken::Char(_) | RegexToken::OpenGroup => {
					return true;
				},
				_ => return false,
			}
		},
		_ => return false,
	}
}

pub fn add_concatenation_token(tokens: Vec<RegexToken>) -> Vec<RegexToken> {
	let mut dest = Vec::with_capacity(tokens.len() * 2);
	let mut token_it = tokens.iter().peekable();

	while let Some(token) = token_it.next() {
		dest.push(*token);
		if token_it.peek().is_none() {
			break;
		}
		if need_concatenate(token, *token_it.peek().unwrap()) == true {
			dest.push(RegexToken::Concatenation);
		}
	}
	return dest;
}
