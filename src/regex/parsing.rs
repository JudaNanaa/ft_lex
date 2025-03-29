use std::collections::LinkedList;

use super::tokenizer::{self, RegexToken};


enum AstNode {
	Char(char),
	Concat(Box<AstNode>, Box<AstNode>),
	Star(Box<AstNode>),
	Plus(Box<AstNode>),
	Optional(Box<AstNode>),
	Or(Box<AstNode>, Box<AstNode>),
	Quantifier(Box<AstNode>, u32, Option<u32>)
}

pub fn regex_parsing(tokens: LinkedList<RegexToken>) {
	let mut nodes: Vec<AstNode> = Vec::new();

	for token in tokens {
		let node = match token {
			RegexToken::Char(c) | RegexToken::Escape(c) => AstNode::Char(c),
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
		};
	}
}