use std::collections::HashSet;

use super::{concatenate::concatenate, repeat_exact::repeat_exact, utils::shift_states, NFA};

fn apply_kleene_star(nfa: &mut NFA) {
    let initial_transitions = nfa
        .transitions
        .get(&0)
        .cloned()
        .expect("No initial state, internal error");

    for &final_state in &nfa.final_states {
        let entry = nfa.transitions.entry(final_state).or_insert_with(Vec::new);
        let mut unique: HashSet<_> = entry.iter().cloned().collect();
        unique.extend(initial_transitions.clone());
        *entry = unique.into_iter().collect();
    }

    if !nfa.final_states.contains(&0) {
        nfa.final_states.push(0);
    }
}

pub fn at_least(nfa: NFA, count: usize) -> (NFA, usize) {
    if count == 0 {
        let mut kleene = nfa.clone();
        apply_kleene_star(&mut kleene);
        let next_id = kleene.final_states.iter().max().unwrap() + 1;
        return (kleene, next_id);
    }

    let (repeated, _) = repeat_exact(&nfa, count);
    let mut kleene_part = nfa.clone();
    apply_kleene_star(&mut kleene_part);

    let shifted_kleene = shift_states(kleene_part, repeated.transitions.len());
    let result = concatenate(repeated, shifted_kleene);
    let next_id = result.final_states.iter().max().unwrap() + 1;
    return (result, next_id);
}
