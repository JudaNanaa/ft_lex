use super::Token;
use super::Token::Operator;
use super::Operator::*;

fn need_concatenate(token: &Token, next: &Token) -> bool {
	match token {
		Token::Char(_) => {
			match next {
				Token::Char(_) | Operator(OpenGroup) => {
					return true;
				},
				_ => return false,
			}
		}
		Operator(CloseGroup) => {
			match next {
				Token::Char(_) | Operator(OpenGroup) => {
					return true;
				},
				_ => return false,
			}
		},
		Operator(Quantifier(_)) => {
			match next {
				Token::Char(_) | Operator(OpenGroup) => {
					return true;
				},
				_ => return false,
			}
		}
		_ => return false,
	}
}

pub fn add_concatenation_token(tokens: Vec<Token>) -> Vec<Token> {
	let mut dest = Vec::with_capacity(tokens.len() * 2);
	let mut token_it = tokens.iter().peekable();

	while let Some(token) = token_it.next() {
		dest.push(*token);
		if token_it.peek().is_none() {
			break;
		}
		if need_concatenate(token, token_it.peek().unwrap()) == true {
			dest.push(Operator(Concatenation));
		}
	}
	return dest;
}
