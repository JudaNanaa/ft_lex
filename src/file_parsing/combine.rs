use std::collections::HashMap;

use crate::regex::{
    combine_nfa::combine_nfa,
    dfa::{dfa::construct_dfa, rule_actions::assiociate_rule_actions, State, DFA},
    NFA,
};

use super::RuleAction;

fn map_final_states_to_actions(rules: &[RuleAction]) -> HashMap<usize, Vec<String>> {
    let mut final_state_actions = HashMap::new();
    let mut initial_state_actions = Vec::new();

    for rule in rules {
        for &final_state in &rule.nfa.final_states {
            if final_state == 0 {
                initial_state_actions.push(rule.action.clone());
            } else {
                final_state_actions.insert(final_state, vec![rule.action.clone()]);
            }
        }
    }

    if !initial_state_actions.is_empty() {
        final_state_actions.insert(0, initial_state_actions);
    }

    return final_state_actions;
}

fn extract_all_nfas(rules: &[RuleAction]) -> Vec<NFA> {
    return rules.iter().map(|rule| rule.nfa.clone()).collect();
}

pub fn process_and_combine_rules(
    rules: Vec<RuleAction>,
) -> Result<(DFA, HashMap<usize, Vec<String>>), &'static str> {
    let mut pipe_buffer = Vec::new();
    let mut processed_rules = Vec::new();

    for rule in rules {
        if rule.action == "|" {
            pipe_buffer.push(rule.clone());
        } else {
            while let Some(mut pending_rule) = pipe_buffer.pop() {
                pending_rule.action = rule.action.clone();
                processed_rules.push(pending_rule);
            }
            processed_rules.push(rule);
        }
    }

    if !pipe_buffer.is_empty() {
        return Err("Un symbole '|' n’a pas été suivi d’une action.");
    }

    let final_state_map = map_final_states_to_actions(&processed_rules);
    let nfa_list = extract_all_nfas(&processed_rules);
    let combined_nfa = combine_nfa(nfa_list);
    let dfa = construct_dfa(combined_nfa);

    let action_mapping = assiociate_rule_actions(&dfa, final_state_map.clone());

    return Ok((dfa, action_mapping));
}
