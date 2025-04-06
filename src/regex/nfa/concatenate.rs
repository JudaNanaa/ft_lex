use crate::regex::utils::RemoveVecElement;

use super::NFA;

pub fn concatenate(mut left: NFA, mut right: NFA) -> NFA {
    let right_initial = right.transitions.remove(&0).unwrap_or_default();
    let right_has_initial_final = right.final_states.contains(&0);

    right.final_states.remove_element(&0);

    for &state in &left.final_states {
        left.transitions
            .entry(state)
            .or_insert_with(Vec::new)
            .extend(right_initial.clone());

        if right_has_initial_final {
            right.final_states.push(state);
        }
    }

    left.transitions.extend(right.transitions);

    return NFA {
        transitions: left.transitions,
        final_states: right.final_states,
    };
}
