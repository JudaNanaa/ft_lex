use crate::regex::NFA;

pub fn trailing_context(mut left: NFA, mut right: NFA) -> NFA {
    if right.final_states.contains(&0) {
        return left;
    }

    let right_initial = right.transitions.remove(&0).unwrap_or_default();

    right.final_states.remove(&0);

    for &state in &left.final_states {
        left.transitions
            .entry(state)
            .or_default()
            .extend(right_initial.clone());
    }

    left.final_states.extend(right.final_states.clone());

    left.transitions.extend(right.transitions);

    return NFA {
        transitions: left.transitions,
        final_states: left.final_states,
        trailing_context_final_states: Some(right.final_states),
    };
}
