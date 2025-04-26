use std::collections::HashMap;

use super::{State, DFA};

pub fn assiociate_rule_actions(
    dfa: &DFA,
    actions_table: HashMap<usize, Vec<String>>,
) -> HashMap<usize, Vec<String>> {
    let mut new_hash = HashMap::new();

    for state in &dfa.final_states {
        let mut action_for_state = Vec::new();
        for elem in &state.state {
            if let Some(tab) = actions_table.get(elem) {
				action_for_state.extend(tab.clone());
			}
        }
        new_hash.insert(*dfa.test.get(state).unwrap(), action_for_state);
    }

    return new_hash;
}
