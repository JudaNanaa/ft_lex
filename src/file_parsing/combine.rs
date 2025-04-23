use std::collections::HashMap;

use crate::regex::{
    combine_nfa::{self, combine_nfa}, dfa::{dfa::construct_dfa, DFA}, NFA
};

use super::{rules, RuleAction};

fn create_hashmap_final_states(rules: &Vec<RuleAction>) -> HashMap<usize, Vec<String>> {
    let mut output = HashMap::new();
    let mut action_for_initial = Vec::new();

    for rule in rules {
        for state in &rule.nfa.final_states {
            match state {
                0 => {
                    action_for_initial.push(rule.action.clone());
                }
                _ => {
                    output.insert(*state, vec![rule.action.clone()]);
                }
            }
        }
    }

    if !action_for_initial.is_empty() {
        output.insert(0, action_for_initial);
    }
    return output;
}

fn get_all_nfa(rules: &Vec<RuleAction>) -> Vec<NFA> {
    let mut output = Vec::new();

    for rule in rules {
        output.push(rule.nfa.clone());
    }
    return output;
}

pub fn combine_rules(rules: Vec<RuleAction>) -> Result<(DFA, HashMap<usize, Vec<String>>), &'static str> {
    let mut rule_stack = Vec::new();
    let mut new_rules = Vec::new();

    for rule in rules {
        if rule.action == "|" {
            rule_stack.push(rule.clone());
        } else {
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
    let hash = create_hashmap_final_states(&new_rules);
    let nfas = get_all_nfa(&new_rules);
    let nfa_combine = combine_nfa(nfas);
	let dfa = construct_dfa(nfa_combine);
    return Ok((dfa, hash));
}
