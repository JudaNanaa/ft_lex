use std::collections::VecDeque;
use std::collections::HashSet;

use super::{DfaTransition, State, DFA};
use crate::regex::NFA;

fn get_target_state_for_input(nfa: &NFA, current_state: &State, input_char: &char) -> State {
	let mut store: HashSet<usize> = HashSet::new();
		
	for nfa_state_id in &current_state.state {
		if let Some(nfa_transitions) = nfa.transitions.get(nfa_state_id) {
			for transition in nfa_transitions {
				if transition.input == *input_char {
					store.insert(transition.target_state);
				}
			}
		}
	}

	if store.is_empty() {
		return State::trap();
	}
	let mut sorted_states: Vec<usize> = store.into_iter().collect();
	sorted_states.sort_unstable();
	return State {
		state: sorted_states,
	};
}

pub fn construct_dfa(nfa: NFA) -> DFA {
    let mut dfa = DFA::new();

    // All ASCII characters
    let alphabet = (0..=127u8)
        .filter_map(|c| char::from_u32(c as u32))
        .collect::<Vec<char>>();

    // Stack of DFA states to process
    let mut unprocessed_states = VecDeque::from(vec![State { state: vec![0] }]);

    while let Some(current_state) = unprocessed_states.pop_front() {
		
        let mut transitions_from_current = Vec::with_capacity(alphabet.len());

		for input_char in &alphabet {
			let state = get_target_state_for_input(&nfa, &current_state, input_char);
			if !state.is_trap() {
				transitions_from_current.push(DfaTransition {
					input: *input_char,
					target_state: state,
				});
			}
		}

        dfa.transitions
            .insert(current_state, transitions_from_current.clone());

        for transition in transitions_from_current {
            if !dfa.transitions.contains_key(&transition.target_state)
            && !unprocessed_states.contains(&transition.target_state)
            {
                unprocessed_states.push_back(transition.target_state);
            }
        }
    }

    dfa.final_states = nfa.final_states;
    println!("nb state dfa == {}", dfa.transitions.len());
    return dfa;
}
