use crate::file_parsing::rules;

use super::RuleAction;

pub fn combine_nfa(rules: Vec<RuleAction>) -> Result<Vec<RuleAction>, &'static str> {

	let mut rule_stack = Vec::new();
	let mut new_rules = Vec::new();

	for rule in rules {
		if rule.action == "|" {
			rule_stack.push(rule.clone());
		}
		else {
			while let Some(mut elem) = rule_stack.pop() {
				elem.action = rule.action.clone();
				new_rules.push(elem);
			}
			new_rules.push(rule);
		}
	}
	if !rule_stack.is_empty() {
		return Err("| not close");
	}
	dbg!(&new_rules);
	return Ok(new_rules);
}