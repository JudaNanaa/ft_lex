use super::{Transition, NFA};

pub fn shift_states(nfa: NFA, offset: usize) -> NFA {
    let mut new_nfa = NFA::new();

    for (state, transitions) in nfa.transitions {
        let new_key = if state == 0 { 0 } else { state + offset };
        let updated_transitions: Vec<Transition> = transitions
            .into_iter()
            .map(|mut t| {
                t.target_state += offset;
                t
            })
            .collect();
        new_nfa.transitions.insert(new_key, updated_transitions);
    }

    for final_state in nfa.final_states {
        let new_final = if final_state == 0 {
            0
        } else {
            final_state + offset
        };
        new_nfa.final_states.push(new_final);
    }

    return new_nfa;
}

pub fn pop_last_two(stack: &mut Vec<NFA>) -> (NFA, NFA) {
    let second = stack.pop().expect("Internal error");
    let first = stack.pop().expect("Internal error");
    return (first, second);
}
