use std::{char, iter::Peekable, slice::Iter, str::Chars};

use crate::{
    file_parsing::rules,
    regex::{nfa::nfa::construct_nfa, regex_tokenizer},
};

use super::{Definition, FileInfo, RuleAction};

fn add_to_next_quote(rule: &mut String, file: &mut FileInfo) -> Result<(), &'static str>{

	rule.push('"');
	while let Some(char) = file.it.next() {
		match char {
			'\n' => {
				return Err("missing quote");
			}
			'"' => {
				break;
			}
			_ => {
				rule.push(char);
			}
		}
	}
	return Ok(());
}

fn extract_brace(rule: &mut String, file: &mut FileInfo, definitions: Vec<Definition>) -> Result<(), &'static str> {
	rule.push('{');
	let mut name = String::new();

	while let Some(char) = file.it.next() {
		match char {
			'}' => {
				break;
			}
			_ => {
				name.push(char);
			}
		}
	}
	todo!();
}

fn split_rule_action(file: &mut FileInfo, first_char: char, definitions: Vec<Definition>) -> Result<[String; 2], &'static str> {

	let mut rule = String::new();

	rule.push(first_char);
	while let Some(char) = file.it.next() {
		match char {
			'"' => {
				add_to_next_quote(&mut rule, file)?;
			},
			' ' | '\t' => {
				break;
			},
			'{' => {
				
			},
			'}' => {
				return Err("unrecognized rule");
			}
			_ => todo!()
		}
	}
	dbg!(rule);
	todo!();
}

pub fn parse_rules_part(file: &mut FileInfo, definitions: Vec<Definition>) -> Result<Vec<RuleAction>, String> {
    let rules: Vec<RuleAction> = Vec::new();

    while let Some(char) = file.it.next() {
        match char {
            '%' => {
                if let Some(next_char) = file.it.peek() {
                    if *next_char == '%' {
                        file.it.next();
                        return Ok(rules);
                    }
                }
            },
            c => {
				
			},
        }
    }

    todo!();
}
