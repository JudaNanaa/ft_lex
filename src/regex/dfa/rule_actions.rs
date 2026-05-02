use std::collections::HashMap;

use super::Dfa;

pub fn assiociate_rule_actions(
    dfa: &Dfa,
    actions_table: &HashMap<usize, Vec<String>>,
) -> HashMap<usize, Vec<String>> {
    let mut new_hash = HashMap::new();

    for &state_id in &dfa.final_states {
        let mut action_for_state = Vec::new();
        if let Some(nfa_states) = dfa.nfa_states.get(&state_id) {
            for elem in nfa_states {
                if let Some(tab) = actions_table.get(elem) {
                    action_for_state.extend(tab.clone());
                }
            }
        }
        new_hash.insert(state_id, action_for_state);
    }

    new_hash
}
