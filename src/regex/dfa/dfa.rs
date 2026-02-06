use std::collections::{HashSet, VecDeque};

#[cfg(feature = "dotfile")]
use crate::regex::dfa::dot::generate_dot_file;

use super::{DfaTransition, State, DFA};
use crate::regex::dfa::NewDfaTransition;
use crate::regex::NFA;

fn compute_new_final_states(dfa: &DFA) -> HashSet<usize> {
    let mut final_state_ids = HashSet::new();

    for state in &dfa.final_states {
        if let Some(&id) = dfa.state_to_id.get(state) {
            final_state_ids.insert(id);
        }
    }

    return final_state_ids;
}

fn map_final_states(dfa: &DFA, nfa_final_states: &HashSet<usize>) -> HashSet<State> {
    let mut dfa_final_states = HashSet::new();

    for state in dfa.transitions.keys() {
        if state.state.iter().any(|s| nfa_final_states.contains(s)) {
            dfa_final_states.insert(state.clone());
        }
    }

    return dfa_final_states;
}

fn find_target_state(nfa: &NFA, current_state: &State, input_char: &char) -> State {
    let mut target_states = HashSet::new();

    for nfa_state_id in &current_state.state {
        if let Some(transitions) = nfa.transitions.get(nfa_state_id) {
            for transition in transitions {
                if transition.input == *input_char {
                    target_states.insert(transition.target_state);
                }
            }
        }
    }

    if target_states.is_empty() {
        return State::trap();
    }

    let mut sorted_states: Vec<usize> = target_states.into_iter().collect();
    sorted_states.sort_unstable();
    return State {
        state: sorted_states,
    };
}

pub fn build_dfa(nfa: NFA) -> DFA {
    let mut dfa = DFA::new();
    let ascii_chars = (0..=255u8)
        .filter_map(|c| char::from_u32(c as u32))
        .collect::<Vec<char>>();

    let mut pending_states = VecDeque::from(vec![State { state: vec![0] }]);
    dfa.state_to_id.insert(State { state: vec![0] }, 0);
    let mut next_state_id = 1;

    while let Some(current_state) = pending_states.pop_front() {
        let mut state_transitions = Vec::with_capacity(ascii_chars.len());
        let mut new_state_transitions = Vec::with_capacity(ascii_chars.len());

        for &input_char in &ascii_chars {
            let target_state = find_target_state(&nfa, &current_state, &input_char);
            if !target_state.is_trap() {
                let state_id = match dfa.state_to_id.get(&target_state) {
                    Some(&id) => id,
                    None => {
                        let id = next_state_id;
                        dfa.state_to_id.insert(target_state.clone(), id);
                        next_state_id += 1;
                        id
                    }
                };

                state_transitions.push(DfaTransition {
                    input: input_char,
                    target_state,
                });

                new_state_transitions.push(NewDfaTransition {
                    input: input_char,
                    target_state: state_id,
                });
            }
        }

        let current_state_id = *dfa.state_to_id.get(&current_state).unwrap();
        dfa.new_transitions
            .insert(current_state_id, new_state_transitions.clone());
        dfa.transitions
            .insert(current_state, state_transitions.clone());

        for transition in state_transitions {
            if !dfa.transitions.contains_key(&transition.target_state)
                && !pending_states.contains(&transition.target_state)
            {
                pending_states.push_back(transition.target_state);
            }
        }
    }

    dfa.final_states = map_final_states(&dfa, &nfa.final_states);
    dfa.new_final_states = compute_new_final_states(&dfa);
    dfa.charset = nfa.compute_charset();
    dfa.new_final_states.remove(&0);
    dfa.trailing_context_final_states = nfa.trailing_context_final_states;

    dbg!(&dfa);

    #[cfg(feature = "dotfile")]
    match generate_dot_file(&dfa) {
        Ok(_) => {}
        Err(error) => eprintln!("Error generating dfa.dot: {}", error),
    }

    return dfa;
}
