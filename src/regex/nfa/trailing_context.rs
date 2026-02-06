use std::collections::{HashMap, HashSet};

use crate::regex::NFA;

fn create_trailing_context_map(
    left_final_state: &HashSet<usize>,
    right_final_state: &HashSet<usize>,
) -> HashMap<usize, HashSet<usize>> {
    let mut trailing_ctx_map = HashMap::new();
    for &state in right_final_state {
        trailing_ctx_map.insert(state, left_final_state.clone());
    }

    return trailing_ctx_map;
}

pub fn trailing_context(mut left: NFA, mut right: NFA) -> NFA {
    if right.final_states.contains(&0) {
        return left;
    }

    let right_initial = right.transitions.remove(&0).unwrap_or_default();

    for &state in &left.final_states {
        left.transitions
            .entry(state)
            .or_default()
            .extend(right_initial.clone());
    }

    let trailing_context_map = create_trailing_context_map(&left.final_states, &right.final_states);

    // left.final_states.extend(right.final_states.clone());

    left.transitions.extend(right.transitions);

    return NFA {
        transitions: left.transitions,
        final_states: left.final_states,
        trailing_context_final_states: Some(trailing_context_map),
    };
}
