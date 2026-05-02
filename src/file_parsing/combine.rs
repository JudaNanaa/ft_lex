use std::collections::{HashMap, HashSet};

use crate::{
    file_parsing::rules::RuleAction,
    regex::{
        combine_nfa::combine_nfa,
        dfa::{automaton::build_dfa, rule_actions::assiociate_rule_actions, Dfa},
        Nfa,
    },
};

fn map_final_states_to_actions(rules: &[RuleAction]) -> HashMap<usize, Vec<String>> {
    let mut final_state_actions = HashMap::new();
    let mut initial_state_actions = Vec::new();

    for rule in rules {
        for &final_state in &rule.nfa().final_states {
            if final_state == 0 {
                initial_state_actions.push(rule.action().to_string());
            } else {
                final_state_actions.insert(final_state, vec![rule.action().to_string()]);
            }
        }
    }

    if !initial_state_actions.is_empty() {
        final_state_actions.insert(0, initial_state_actions);
    }

    final_state_actions
}

fn extract_all_nfas(rules: &[RuleAction]) -> Vec<Nfa> {
    rules.iter().map(|rule| rule.nfa().clone()).collect()
}

type CombineResult = Result<(Dfa, HashMap<usize, Vec<String>>, Vec<RuleAction>), String>;

pub fn process_and_combine_rules(rules: Vec<RuleAction>) -> CombineResult {
    let mut pipe_buffer: Vec<RuleAction> = Vec::new();
    let mut rule_buffer: Vec<RuleAction> = Vec::new();
    let mut processed_rules = Vec::new();

    for rule in rules {
        if rule.action() == "|" {
            pipe_buffer.push(rule.clone());
            rule_buffer.push(rule.clone());
        } else {
            rule_buffer.push(rule.clone());
            let mut condition_state_list = Vec::new();
            while let Some(mut pending_rule) = pipe_buffer.pop() {
                condition_state_list.append(pending_rule.condition_state());
            }
            condition_state_list.append(rule.clone().condition_state());
            let mut seen = std::collections::HashSet::new();
            condition_state_list.retain(|x| seen.insert(x.clone()));
            while let Some(orig) = rule_buffer.pop() {
                processed_rules.push(RuleAction {
                    nfa: orig.nfa,
                    action: rule.action().to_string(),
                    condition_state: condition_state_list.clone(),
                    anchored_start: orig.anchored_start,
                    anchored_end: orig.anchored_end,
                    charsets: orig.charsets,
                });
            }
        }
    }

    if !pipe_buffer.is_empty() {
        return Err("Un symbole '|' n'a pas été suivi d'une action.".to_string());
    }

    let final_state_map = map_final_states_to_actions(&processed_rules);
    let nfa_list = extract_all_nfas(&processed_rules);

    let all_charsets: Vec<HashSet<char>> = processed_rules
        .iter()
        .flat_map(|r| r.charsets.iter().cloned())
        .collect();
    let (eq_classes, _num_classes) = crate::regex::partition::partition_refinement(&all_charsets);

    let combined_nfa = combine_nfa(nfa_list);
    let dfa = build_dfa(&combined_nfa, &eq_classes);

    let action_mapping = assiociate_rule_actions(&dfa, &final_state_map);

    Ok((dfa, action_mapping, processed_rules))
}
