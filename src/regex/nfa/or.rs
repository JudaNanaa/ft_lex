use super::NFA;

pub fn or(left: NFA, right: NFA) -> NFA {
    let mut transitions = left.transitions;
    let mut final_states = left.final_states;

    for (state, mut trans) in right.transitions {
        transitions
            .entry(state)
            .and_modify(|list| list.append(&mut trans))
            .or_insert(trans);
    }

    for state in right.final_states {
        if !final_states.contains(&state) {
            final_states.push(state);
        }
    }

    return NFA {
        transitions,
        final_states,
    };
}
