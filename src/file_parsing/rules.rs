use std::{iter::Peekable, slice::Iter, str::Chars};

use crate::regex::{nfa::nfa::construct_nfa, regex_tokenizer};

use super::RuleAction;


fn extract_action(line: &str, start_index: usize, line_it: &mut Peekable<Iter<'_, &str>>) -> String {
	if start_index == line.len() {
		return String::new();
	}
	let action = String::new();
	let char_it  = line.chars();


	todo!();
}

pub fn parse_rules_part(line_it: &mut Peekable<Chars<'_>>) -> RuleAction {
	
	// let action = extract_action(line, end_regex, line_it);
	todo!();
}