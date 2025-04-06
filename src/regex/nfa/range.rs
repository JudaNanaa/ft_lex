use crate::regex::{
    nfa::{
        concatenate::concatenate,
        repeat_exact::repeat_exact,
        utils::{pop_last_two, shift_states},
    },
    utils::RemoveVecElement,
    NFA,
};

pub fn range(nfa: NFA, min: usize, max: usize) -> (NFA, usize) {
    assert!(min <= max, "Invalid range");

    if min == max {
        return repeat_exact(&nfa, min);
    }

    let mut nfa_parts: Vec<NFA> = Vec::new();
    let mut total_offset = 0;
    let mut accumulated_final_states = Vec::new();

    // Partie obligatoire : min répétitions
    if min > 0 {
        let (mandatory_nfa, _) = repeat_exact(&nfa, min);
        accumulated_final_states = mandatory_nfa.final_states.clone();
        total_offset += mandatory_nfa.transitions.len();
        nfa_parts.push(mandatory_nfa);
    } else {
        accumulated_final_states.push(0); // L’état initial est final si min == 0
    }

    // Parties optionnelles : (max - min) répétitions
    for _ in min..max {
        let optional_nfa = shift_states(nfa.clone(), total_offset);

        if let Some(transitions_from_initial) = optional_nfa.transitions.get(&0) {
            for transition in transitions_from_initial {
                accumulated_final_states.push_unique(transition.target_state);
            }
        }

        total_offset += optional_nfa.transitions.len();
        nfa_parts.push(optional_nfa);

        if nfa_parts.len() == 2 {
            let (left, right) = pop_last_two(&mut nfa_parts);
            nfa_parts.push(concatenate(left, right));
        }
    }

    let mut final_nfa = nfa_parts.pop().unwrap();
    for state in accumulated_final_states {
        final_nfa.final_states.push_unique(state);
    }

    return (final_nfa, total_offset + 1);
}
