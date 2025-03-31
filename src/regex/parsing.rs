
use super::tokenizer::RegexToken;

enum AstNode {
	Char(char),
	Concat(Box<AstNode>, Box<AstNode>),
	Group(Box<AstNode>), // ()
	CharSet(String, bool), // for []
	Star(Box<AstNode>),
	Plus(Box<AstNode>),
	Optional(Box<AstNode>),
	Or(Box<AstNode>, Box<AstNode>),
	Quantifier(Box<AstNode>, u32, Option<u32>)
}

[-z-]


(z|b)
fn charset_creating(tokens_iterator: &mut std::slice::Iter<'_, RegexToken>) -> Box<AstNode> {
	let mut dest = String::new();
	let mut negative = false;

	for &token in tokens_iterator.next() {
		match token {
			RegexToken::CloseCharSet => {
				return Box::new(AstNode::CharSet(dest, negative));
			}
			RegexToken::OpenGroup => dest.push('('),
			RegexToken::CloseGroup => dest.push(')'),
			RegexToken::Optional => dest.push('?'),
			RegexToken::Star => dest.push('*'),
			RegexToken::Char(c) => dest.push(c),
		}
	}
	panic!("NO UNCLOSED CHARSET");
}

pub fn regex_parsing(tokens: Vec<RegexToken>) {
	let mut nodes: Vec<AstNode> = Vec::new();

	let mut tokens_it: std::slice::Iter<'_, RegexToken> = tokens.iter();

	for &token in tokens_it.next() {
		let node = match token {
			RegexToken::Char(c) => {
				AstNode::Char(c)
			},

			// RegexToken::Or => {
			// 	if let Some(last_node) = nodes.pop() {
			// 		AstNode::Or((), ())
			// 	} else {
			// 		panic!("Error parsing OR");
			// 	}
			// }
			RegexToken::Star => {
				if let Some(last_node) = nodes.pop() {
					AstNode::Star(Box::new(last_node))
				} else {
					panic!("Error parsing STAR");
				}
			},
			RegexToken::Optional => {
				if let Some(last_node) = nodes.pop() {
					AstNode::Optional(Box::new(last_node))
				} else {
					panic!("Error parsing STAR");
				}
			},
		};
	}
}