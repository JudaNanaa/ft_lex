use std::collections::HashMap;

use super::{State, DFA};

pub fn assiociate_rule_actions(dfa: &DFA, actions_table: HashMap<usize, Vec<String>>) -> HashMap<State, Vec<String>> {

	let mut new_hash = HashMap::new();

	for state in &dfa.final_states {

		let mut action_for_state = Vec::new();
		for elem in &state.state {
			let tab = actions_table.get(elem).unwrap();
			action_for_state.extend(tab.clone());
		}
		new_hash.insert(state.clone(), action_for_state);
	}

	return new_hash;
}